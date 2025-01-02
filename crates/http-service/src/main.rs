use poem::http::StatusCode;
use poem::web::headers::HeaderMapExt;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::Tracing,
    post,
    web::{
        headers::{authorization::Bearer, Authorization},
        Json, Path,
    },
    EndpointExt, FromRequest, Request, RequestBody, Route, Server,
};
use poem::{Error, Result as PoemResult};
use serde::Deserialize;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello-{name}")
}

#[derive(Debug, Deserialize)]
struct Message {
    #[allow(unused)]
    name: String,
}

struct MessageRequest(Json<Message>);

/// requires authentication bearer token
impl<'a> FromRequest<'a> for MessageRequest {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> PoemResult<Self> {
        let bearer_token: Option<Authorization<Bearer>> = req
            .headers()
            .typed_try_get()
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

        match bearer_token {
            Some(token) => {
                if token.token() == "123" {
                    let bytes = body.take()?.into_bytes().await?;
                    match serde_json::from_slice(&bytes) {
                        Ok(json) => return Ok(MessageRequest(Json(json))),
                        Err(_err) => {
                            return Err(Error::from_status(StatusCode::BAD_REQUEST));
                        }
                    }
                } else {
                    Err(Error::from_status(StatusCode::UNAUTHORIZED))
                }
            }
            None => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
        }
    }
}

/// requires authentication bearer token
#[handler]
fn create(req: MessageRequest) -> Json<serde_json::Value> {
    Json(serde_json::json! ({
        "code": 0,
        "message": req.0.name,
    }))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/create", post(create))
        .with(Tracing);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
