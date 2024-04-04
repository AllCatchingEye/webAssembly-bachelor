bindgen!({
    path: "wit",
    world: "database",
    async: true,
});

use bachelor::database::readwrite;
use bachelor::database::types;
use bachelor::database::types::{Connection, Error, Statement};
use bachelor::database::types::{DataType, Row};
use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};

use wasmtime_wasi::bindings;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiP1Ctx;

struct DatabaseTypesHost;
struct ReadwriteHost;

#[async_trait::async_trait]
impl types::Host for DatabaseTypesHost {
    async fn drop_statement(&mut self, _s: Statement) -> Result<(), anyhow::Error> {
        Ok(())
    }

    async fn prepare_statement(
        &mut self,
        _query: String,
        _params: Vec<String>,
    ) -> Result<Result<u32, u32>, anyhow::Error> {
        Ok(Ok(0))
    }

    async fn drop_error(&mut self, _conn: Connection) -> Result<(), anyhow::Error> {
        Ok(())
    }

    async fn trace_error(&mut self, _conn: Connection) -> Result<String, anyhow::Error> {
        Ok(">>> called trace_error".to_string())
    }

    async fn drop_connection(&mut self, _conn: Connection) -> Result<(), anyhow::Error> {
        Ok(())
    }

    async fn open_connection(
        &mut self,
        _name: String,
    ) -> Result<Result<Connection, Error>, anyhow::Error> {
        Ok(Ok(0))
    }
}

#[async_trait::async_trait]
impl readwrite::Host for ReadwriteHost {
    async fn query(
        &mut self,
        _conn: Connection,
        _q: Statement,
    ) -> Result<Result<Vec<Row>, Error>, anyhow::Error> {
        let row = Row {
            field_name: String::from("id"),
            value: DataType::Int32(1),
        };

        let rows: Vec<Row> = vec![row];
        Ok(Ok(rows))
    }

    async fn exec(
        &mut self,
        _c: Connection,
        _q: Statement,
    ) -> Result<Result<Connection, Error>, anyhow::Error> {
        Ok(Ok(0))
    }
}

pub struct Ctx {
    database_types: DatabaseTypesHost,
    readwrite: ReadwriteHost,
    wasi: WasiP1Ctx,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let database_types = DatabaseTypesHost;
    let readwrite = ReadwriteHost;
    let wasi = WasiCtxBuilder::new().build_p1();
    let mut store = Store::new(
        &engine,
        Ctx {
            database_types,
            readwrite,
            wasi,
        },
    );

    let mut linker = Linker::new(&engine);
    readwrite::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.readwrite)?;
    types::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.database_types)?;
    bindings::Imports::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "target/wasm32-wasi/debug/guest_component.wasm")?;
    println!("Read file.");
    let (database, _instance) =
        Database::instantiate_async(&mut store, &component, &linker).await?;
    println!("Instantiated database");
    let result = database
        .bachelor_database_handler()
        .call_add(&mut store, 3, 4)
        .await?;
    println!("Result of add from database is: {}", result);

    Ok(())
}
