bindgen!({
    path: "wit",
    world: "backend",
    async: true,
});

mod sql_import;
mod tcp_import;

use sql_import::DatabaseHost;
use tcp_import::TcpHost;

use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};

use wasmtime_wasi::{bindings::Imports, WasiCtxBuilder, WasiP1Ctx};

pub trait Import {
    fn new() -> Self;
}

pub struct Ctx {
    database: DatabaseHost,
    server: TcpHost,
    wasi_p1: WasiP1Ctx,
}

fn build_store(engine: Engine) -> Result<Store<Ctx>, anyhow::Error> {
    let database = DatabaseHost::new();
    let server = TcpHost::new();

    let wasi_p1 = WasiCtxBuilder::new()
        .inherit_network()
        .allow_tcp(true)
        .allow_ip_name_lookup(true)
        .build_p1();

    let store = Store::new(
        &engine,
        Ctx {
            database,
            server,
            wasi_p1,
        },
    );

    Ok(store)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);

    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);

    sql_import::add_to_linker(&mut linker)?;
    tcp_import::add_to_linker(&mut linker)?;
    Imports::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_p1)?;

    let component = Component::from_file(&engine, "guest-component.wasm")?;
    let mut store = build_store(engine)?;

    let (backend, _instance) = Backend::instantiate_async(&mut store, &component, &linker).await?;
    println!("Instantiated database");

    let _ = backend
        .bachelor_backend_sockets_handler()
        .call_socket_handle(&mut store)
        .await?;

    Ok(())
}
