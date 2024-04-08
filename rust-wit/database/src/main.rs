bindgen!({
    path: "wit",
    world: "database",
    async: true,
    with: {
        "backend:database/sql/connection": DatabaseConnectionHost,
    }
});

use backend::database::sql;
use sqlx::SqliteConnection;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};

use wasmtime::component::Resource;

use wasmtime_wasi::bindings::Imports;
use wasmtime_wasi::WasiP1Ctx;
use wasmtime_wasi::{ResourceTable, WasiCtxBuilder};

use sqlx::prelude::*;
use sqlx::sqlite::SqliteConnectOptions;

pub struct DatabaseTypesHost {
    res_table: ResourceTable,
}
pub struct DatabaseConnectionHost {
    connection: SqliteConnection,
}

#[async_trait::async_trait]
impl sql::HostConnection for DatabaseTypesHost {
    fn drop(&mut self, _: Resource<DatabaseConnectionHost>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl sql::Host for DatabaseTypesHost {
    async fn open_connection(
        &mut self,
        url: String,
        create_if_missing: bool,
    ) -> Result<Result<Resource<DatabaseConnectionHost>, u32>, wasmtime::Error> {
        println!("Opening {}...", url);
        let options = SqliteConnectOptions::new()
            .filename(url)
            .create_if_missing(create_if_missing);
        let conn = options.connect().await?;

        let database_connection = DatabaseConnectionHost { connection: conn };
        let res = self.res_table.push(database_connection)?;

        Ok(Ok(res))
    }

    async fn create_table(
        &mut self,
        query: String,
        conn: Resource<DatabaseConnectionHost>,
    ) -> Result<(), wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;
        sqlx::query(query.as_str())
            .execute(&mut host_conn.connection)
            .await?;
        Ok(())
    }

    async fn drop_connection(
        &mut self,
        conn: Resource<DatabaseConnectionHost>,
    ) -> Result<Result<(), u32>, wasmtime::Error> {
        let host_conn = self.res_table.delete(conn)?;
        host_conn.connection.close().await?;
        Ok(Ok(()))
    }

    async fn select(
        &mut self,
        conn: Resource<DatabaseConnectionHost>,
    ) -> Result<Result<String, u32>, wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;

        let rows = sqlx::query("SELECT * FROM test")
            .fetch_all(&mut host_conn.connection)
            .await?;

        let mut result_string = String::new();
        for row in rows {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            result_string.push_str(&format!("ID: {}, Name: {}\n", id, name));
        }

        Ok(Ok(result_string))
    }

    async fn insert(
        &mut self,
        conn: Resource<DatabaseConnectionHost>,
        name: String,
    ) -> Result<(), wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;
        sqlx::query("INSERT INTO test (name) VALUES (?)")
            .bind(name)
            .execute(&mut host_conn.connection)
            .await?;
        Ok(())
    }

    async fn delete(
        &mut self,
        conn: Resource<DatabaseConnectionHost>,
        name: String,
    ) -> Result<(), wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;
        sqlx::query("DELETE FROM test WHERE name = ?")
            .bind(name)
            .execute(&mut host_conn.connection)
            .await?;
        Ok(())
    }

    async fn print_to_host(&mut self, str: String) -> Result<(), wasmtime::Error> {
        println!("{str}");
        Ok(())
    }
}

pub struct Ctx {
    database: DatabaseTypesHost,
    wasi: WasiP1Ctx,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let database = DatabaseTypesHost {
        res_table: ResourceTable::new(),
    };
    let wasi = WasiCtxBuilder::new().build_p1();
    let mut store = Store::new(&engine, Ctx { database, wasi });

    let mut linker = Linker::new(&engine);
    sql::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.database)?;
    Imports::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "guest-component.wasm")?;

    println!("Read file.");
    let (database, _instance) =
        Database::instantiate_async(&mut store, &component, &linker).await?;

    println!("Instantiated database");
    let result = database
        .backend_database_handler()
        .call_handle(store)
        .await?;

    Ok(())
}
