bindgen!({
    path: "wit",
    world: "database",
    async: true,
    with: {
        "bachelor:backend/sql/connection": DatabaseConnectionHost,
    }
});

use crate::{Ctx, Import};
use bachelor::backend::sql::{add_to_linker as sql_linker, Host, HostConnection, MessageType};

use serde::Serialize;
use serde_json::Value;

use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{prelude::*, Column, Execute};
use sqlx::{SqliteConnection, TypeInfo};

use wasmtime::{
    component::{bindgen, Linker, ResourceTable},
    Result,
};

use wasmtime::component::Resource;

pub struct DatabaseHost {
    res_table: ResourceTable,
}

pub struct DatabaseConnectionHost {
    connection: SqliteConnection,
}

#[derive(Serialize)]
struct Dht11Data {
    id: u32,
    temperature: i32,
    humidity: u32,
}

impl From<SqliteRow> for Dht11Data {
    fn from(row: SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            temperature: row.get("temperature"),
            humidity: row.get("humidity"),
        }
    }
}

#[derive(Serialize)]
struct TestData {
    id: u32,
    name: String,
}

impl From<SqliteRow> for TestData {
    fn from(row: SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}

impl Import for DatabaseHost {
    fn new() -> Self {
        DatabaseHost {
            res_table: ResourceTable::new(),
        }
    }
}

#[async_trait::async_trait]
impl HostConnection for DatabaseHost {
    fn drop(&mut self, _: Resource<DatabaseConnectionHost>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl Host for DatabaseHost {
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
    ) -> Result<Result<(), u32>, wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;
        sqlx::query(query.as_str())
            .execute(&mut host_conn.connection)
            .await?;
        Ok(Ok(()))
    }

    async fn drop_connection(
        &mut self,
        conn: Resource<DatabaseConnectionHost>,
    ) -> Result<Result<(), u32>, wasmtime::Error> {
        let host_conn = self.res_table.delete(conn)?;
        host_conn.connection.close().await?;
        Ok(Ok(()))
    }

    async fn execute_query(
        &mut self,
        conn: Resource<DatabaseConnectionHost>,
        query: String,
        values: Option<String>,
        m_type: MessageType,
    ) -> Result<Result<Option<String>, u32>, wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;

        println!("Preparing query... ");
        let mut prepared_query = sqlx::query(&query);

        if let Some(values) = values {
            println!("Binding values...");
            let parsed_values: Vec<Value> = serde_json::from_str(&values)?;
            println!("{:#?}", parsed_values);
            for value in parsed_values {
                let binding_value = match value {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    _ => return Err(wasmtime::Error::msg("Error while binding values")),
                };
                println!("Binding values: {}", binding_value);
                prepared_query = prepared_query.bind(binding_value);
            }
        }

        println!("Executing query: {:#?}", prepared_query.sql());

        let mut query_result: Option<String> = None;
        let rows = prepared_query.fetch_all(&mut host_conn.connection).await?;
        if !rows.is_empty() {
            match m_type {
                MessageType::Test => {
                    println!("Test message type");
                    let test_data_rows: Vec<TestData> =
                        rows.into_iter().map(TestData::from).collect();
                    query_result = Some(serde_json::to_string(&test_data_rows)?);
                }
                MessageType::Dht11 => {
                    println!("Dht11 message type");
                    let dht_data_rows: Vec<Dht11Data> =
                        rows.into_iter().map(Dht11Data::from).collect();
                    query_result = Some(serde_json::to_string(&dht_data_rows)?);
                }
                _ => {
                    println!("Unknown message type")
                }
            }
        }

        match query_result {
            Some(rows) => {
                println!("Returning query results...");
                Ok(Ok(Some(rows)))
            }
            None => Ok(Ok(None)),
        }
    }

    async fn print_to_host(&mut self, str: String) -> Result<(), wasmtime::Error> {
        println!("{str}");
        Ok(())
    }
}

pub fn add_to_linker(linker: &mut Linker<Ctx>) -> Result<()> {
    sql_linker(linker, |ctx: &mut Ctx| &mut ctx.database)?;
    Ok(())
}
