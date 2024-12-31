# HTTP Client Library for WASM Component Model

HTTP Client Library for used in the WASM Component Model.  It uses [WASI-HTTP](https://github.com/WebAssembly/wasi-http).
It is part of [SDF](https://www.fluvio.io/sdf/quickstart) project.

This crate re-exports `http` crate.


## Usage in the SDF inline operator

While this can be used in the any WASM Component.  It is primarily used in the SDF dataflow.  Here is an example of how to use it in the SDF.


Here `sdf-http` is used as rust dependency in the operator.

```
  hello:
    sources:
      - type: topic
        id: input-topic
    transforms:
      - operator: map
        dependencies:
          - name: sdf-http
            version: "0.4.2"
        run: |
          fn invoke_http(input: String) -> Result<String> {
            
            let uri = format!("http://localhost:3000/hello/{}", input);
            Ok(sdf_http::get(uri)?.text()?)
          }
    sinks:
      - type: topic
        id: output-topic

```


# License

Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
