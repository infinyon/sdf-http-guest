
mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "test-world",
    });
}

bindings::export!(TestComponent with_types_in bindings);

use bindings::exports::sdf::test::test_guest::Guest as TestGuest;

struct  TestComponent;

impl TestGuest for TestComponent {
    fn run() -> String {
       "hello world".to_string()
    }
}