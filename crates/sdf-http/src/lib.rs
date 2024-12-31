pub mod http {
    pub use http::*;
}

use anyhow::anyhow;
pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;

mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "http",
        generate_all
    });
}

pub use wrapper::*;

// high level API similar to Reqwest
mod wrapper {
    use std::ops::{Deref, DerefMut};

    use http::{Error, Response, Uri};

    pub struct ByteResponse(Response<Vec<u8>>);

    impl Deref for ByteResponse {
        type Target = Response<Vec<u8>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for ByteResponse {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl ByteResponse {
        pub fn text(self) -> anyhow::Result<String> {
            Ok(String::from_utf8(self.0.into_body())?)
        }
    }

    pub fn get<T>(uri: T) -> anyhow::Result<ByteResponse>
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<Error>,
    {
        let request = crate::http::Request::builder().uri(uri).body("")?;
        let response = crate::blocking::send(request)?;
        Ok(ByteResponse(response))
    }
}

pub mod blocking {
    use anyhow::anyhow;

    use crate::bindings::wasi::http::outgoing_handler;
    use crate::bindings::wasi::http::types::{OutgoingBody, OutgoingRequest};
    use crate::bindings::wasi::io::streams::StreamError;

    use super::http::{Request, Response};
    use super::Result;

    pub fn send<T: AsRef<[u8]>>(request: Request<T>) -> Result<Response<Vec<u8>>> {
        let request_wasi = OutgoingRequest::try_from(&request)?;

        let request_body = request_wasi
            .body()
            .map_err(|_| anyhow!("outgoing request write failed"))?;
        let output_stream = request_body
            .write()
            .map_err(|_| anyhow!("request has no input stream"))?;
        output_stream.write(request.body().as_ref())?;
        drop(output_stream);

        let response_fut = outgoing_handler::handle(request_wasi, None)?;
        OutgoingBody::finish(request_body, None)?;

        let response_wasi = match response_fut.get() {
            Some(result) => result.map_err(|_| anyhow!("response already taken"))?,
            None => {
                let pollable = response_fut.subscribe();
                pollable.block();
                response_fut
                    .get()
                    .ok_or_else(|| anyhow!("response available"))?
                    .map_err(|_| anyhow!("response already taken"))?
            }
        }?;

        let mut response_builder = Response::builder();
        response_builder =
            response_builder.status(http::StatusCode::from_u16(response_wasi.status())?);

        for (header, values) in response_wasi.headers().entries() {
            response_builder = response_builder.header(header, values);
        }

        let body_wasi = response_wasi
            .consume()
            .map_err(|()| anyhow!("response has no body stream"))?;

        let input_stream = body_wasi
            .stream()
            .map_err(|()| anyhow!("response body has no stream"))?;
        let input_stream_pollable = input_stream.subscribe();

        let mut body = Vec::new();
        loop {
            input_stream_pollable.block();
            let mut body_chunk = match input_stream.read(1024 * 1024) {
                Ok(c) => c,
                Err(StreamError::Closed) => break,
                Err(e) => Err(anyhow!("input stream read failed: {e:?}"))?,
            };
            if !body_chunk.is_empty() {
                body.append(&mut body_chunk);
            }
        }
        Ok(response_builder.body(body)?)
    }
}

impl<T> TryFrom<&http::Request<T>> for bindings::wasi::http::types::OutgoingRequest {
    type Error = Error;

    fn try_from(request: &http::Request<T>) -> std::result::Result<Self, Self::Error> {
        let headers = request.headers().try_into()?;
        let request_wasi = Self::new(headers);

        let method = request.method().into();
        let scheme = request.uri().scheme().map(|s| s.into());
        let authority = request.uri().authority().map(|a| a.as_str());
        let path_and_query = request.uri().path_and_query().map(|a| a.as_str());

        request_wasi
            .set_method(&method)
            .map_err(|_| anyhow!("invalid method"))?;
        request_wasi
            .set_scheme(scheme.as_ref())
            .map_err(|_| anyhow!("invalid scheme"))?;
        request_wasi
            .set_authority(authority)
            .map_err(|_| anyhow!("invalid authority"))?;
        request_wasi
            .set_path_with_query(path_and_query)
            .map_err(|_| anyhow!("invalid path_and_query"))?;

        Ok(request_wasi)
    }
}

impl From<&http::Method> for bindings::wasi::http::types::Method {
    fn from(value: &http::Method) -> Self {
        match value.as_str() {
            "OPTIONS" => Self::Options,
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "HEAD" => Self::Head,
            "TRACE" => Self::Trace,
            "CONNECT" => Self::Connect,
            "PATCH" => Self::Patch,
            other => Self::Other(other.to_string()),
        }
    }
}

impl From<&http::uri::Scheme> for bindings::wasi::http::types::Scheme {
    fn from(value: &http::uri::Scheme) -> Self {
        match value.as_str() {
            "https" | "HTTPS" => Self::Https,
            _ => Self::Http,
        }
    }
}

impl TryFrom<&http::HeaderMap> for bindings::wasi::http::types::Headers {
    type Error = bindings::wasi::http::types::HeaderError;

    fn try_from(value: &http::HeaderMap) -> std::result::Result<Self, Self::Error> {
        let headers = bindings::wasi::http::types::Headers::new();
        for key in value.keys() {
            let all: Vec<Vec<u8>> = value
                .get_all(key)
                .iter()
                .flat_map(|v| v.to_str().ok())
                .map(|v| v.as_bytes().to_vec())
                .collect();
            let key: String = key.to_string();
            headers.set(&key, &all)?;
        }
        Ok(headers)
    }
}
