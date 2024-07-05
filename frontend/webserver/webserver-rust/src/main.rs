bindgen!({
    world: "webserver",
    path: "../../wit"
});

use frontend::webserver::plot;
use plotly::{self, Plot, Scatter};
use wasmtime::component::bindgen;

use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

struct WebServerState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl plot::Host for WebServerState {
    fn plot_temperature(&mut self, ids: Vec<u32>, temperatures: Vec<i32>) {
        let mut plot = Plot::new();

        let trace = Scatter::new(ids, temperatures);
        plot.add_trace(trace);

        // plot.write_image("temperatures.png", plotly::ImageFormat::PNG, 800, 600, 1.0);
        plot.write_html("temperature.html")
    }
    fn plot_humidity(&mut self, ids: Vec<u32>, humidities: Vec<u32>) {
        let mut plot = Plot::new();
        let trace = Scatter::new(ids, humidities);
        plot.add_trace(trace);

        // plot.write_image("humidities.png", plotly::ImageFormat::PNG, 800, 600, 1.0);
        plot.write_html("humidity.html")
    }
}

impl WasiView for WebServerState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.wasm_backtrace(true);

    let engine = Engine::new(&config)?;

    let mut builder = WasiCtxBuilder::new();
    builder.allow_tcp(true);
    builder.inherit_network();
    builder.allow_ip_name_lookup(true);
    builder.inherit_env();
    builder.inherit_stdio();

    let mut store = Store::new(
        &engine,
        WebServerState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );

    println!("Wrapping function...");
    let mut linker: Linker<WebServerState> = Linker::new(&engine);
    plot::add_to_linker(&mut linker, |state: &mut WebServerState| state)?;
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    linker
        .root()
        .func_wrap("print-host", |_store, params: (String,)| {
            let msg: String = params.0;
            println!("{msg}");
            Ok(())
        })?;

    // Load the component from disk
    println!("Reading component bytes...");
    let bytes = std::fs::read("../plugged.wasm")?;
    println!("Creating component from bytes...");
    let component = Component::new(&engine, bytes)?;

    println!("Instantiate component...");
    let (ws_instance, _instance) = Webserver::instantiate(&mut store, &component, &linker)?;

    println!("Call component...");
    ws_instance.call_start_webserver(&mut store)?;

    Ok(())
}
