mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "test-world",
    });
}

bindings::export!(TestComponent with_types_in bindings);

use bindings::exports::sdf::test::test_guest::Guest as TestGuest;

struct TestComponent;

impl TestGuest for TestComponent {
    fn run() -> String {
        let url = "https://httpbin.org:443/user-agent";
        println!("url: {}", url);
        let request = sdf_http::http::Request::builder()
            .uri(url)
            .body("")
            .unwrap();
        let response = sdf_http::blocking::send(request).unwrap();
        unsafe { String::from_utf8_unchecked(response.into_body()) }
    }
}
