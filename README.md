# HTTP Client Library for WASM Component Model

HTTP Client Library for used in the WASM Component Model.  It uses [WASI-HTTP](https://github.com/WebAssembly/wasi-http).
It is part of [SDF](https://www.fluvio.io/sdf/quickstart) project.

This crate re-exports `http` crate.


## Usage in the SDF inline operator

While this can be used in the any WASM Component.  It is primarily used in the SDF dataflow.  Here is an example of how to use it in the SDF.

### Using Get api

Here `sdf-http` is used as rust dependency in the operator.
Noted that a single get call can be used to get the response.

```
  get:
    sources:
      - type: topic
        id: input-topic
    transforms:
      - operator: map
        dependencies:
          - name: sdf-http
            version: "0.4.3"
        run: |
          fn invoke_http(input: String) -> Result<String> {
            
            let uri = format!("http://localhost:3000/hello/{}", input);
            Ok(sdf_http::get(uri)?.text()?)
          }
    sinks:
      - type: topic
        id: output-topic

```

### Using Post api

For post operation, new body based builder is provider.  Instead of setting up uri and other parameters.  It starts with body.  It is optimized for JSON based operation.

Here is a simple example:
```
post:
    sources:
      - type: topic
        id: input

    transforms:
      - operator: map
        dependencies:
          - name: sdf-http
            version: "0.4.3",
            features: ["serde_json"]
        run: |
          fn invoke_post(input: String) -> Result<Out> {
             
             let input = sdf_http::serde_json::json!({
                "name": input
             });
             let response = sdf_http::json(input.to_string())
                  .bearer("123")
                  .post("http://localhost:3000/create")?;

             let out = sdf_http::serde_json::from_slice(response.as_slice())?;
          
            Ok(out)
             
          }
    sinks:
      - type: topic
        id: output
```

The builder provides existing api for builder as well as frequently used methods like `bearer` for setting up bearer token.
In addition, it re-exports both `serde` and `serde_json` so you don't have to import them separately.


# License

Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
