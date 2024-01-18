pub mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "http",
    });
}
