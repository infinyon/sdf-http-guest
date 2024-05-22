use wasmtime::component::Component;
use wasmtime::component::Linker;
use wasmtime::Config;
use wasmtime::Engine;
use wasmtime::Store;
use wasmtime_wasi::add_to_linker_sync;
use wasmtime_wasi::ResourceTable;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiView;
use wasmtime_wasi_http::WasiHttpCtx;

mod test_bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "test-world",
        async: false,
        trappable_imports: true
    });
}

use test_bindings::TestWorld;
use wasmtime_wasi_http::WasiHttpView;

struct HttpContext {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    http: WasiHttpCtx,
}

impl WasiHttpView for HttpContext {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for HttpContext {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

fn create_context() -> HttpContext {
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();
    HttpContext {
        table: ResourceTable::new(),
        wasi_ctx,
        http: WasiHttpCtx::new(),
    }
}

fn load_component() -> (Store<HttpContext>, Component, Linker<HttpContext>) {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).expect("new engine");

    let mut linker: Linker<HttpContext> = Linker::new(&engine);

    add_to_linker_sync(&mut linker).expect("link");

    let wasm_file = std::env::var("WASM_FILE").expect("WASM_FILE");
    println!("Loading wasm file: {}", wasm_file);
    let component = Component::from_file(&engine, wasm_file).expect("component");
    let store = Store::new(&engine, create_context());
    (store, component, linker)
}

#[test]
fn test_http_sync() {
    let (mut store, component, linker) = load_component();

    let (binding, _) =
        TestWorld::instantiate(&mut store, &component, &linker).expect("instantiate");

    binding
        .sdf_test_test_guest()
        .call_run(store)
        .expect("init state");
}
