bindgen!({
    path: "wit",
    world: "database",
    async: true,
    with: {
        "bachelor:backend/sql/connection": DatabaseConnectionHost,
    }
});

use crate::{Ctx, Import};
use bachelor::backend::sql;

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

impl Import for DatabaseHost {
    fn new() -> Self {
        DatabaseHost {
            res_table: ResourceTable::new(),
        }
    }
}

#[async_trait::async_trait]
impl sql::HostConnection for DatabaseHost {
    fn drop(&mut self, _: Resource<DatabaseConnectionHost>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl sql::Host for DatabaseHost {
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
    ) -> Result<(Result<(), u32>), wasmtime::Error> {
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
    ) -> Result<Result<Option<String>, u32>, wasmtime::Error> {
        let host_conn = self.res_table.get_mut(&conn)?;

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
        let query_result = prepared_query.fetch_all(&mut host_conn.connection).await;

        match query_result {
            Ok(rows) => {
                println!("Returning query results...");
                Ok(Ok(Some(build_query_result(rows)?)))
            }
            Err(sqlx::Error::RowNotFound) => {
                println!("Executed query");
                Ok(Ok(None))
            }
            Err(err) => {
                eprintln!("Error executing query: {}", err);
                Ok(Err(1))
            }
        }
    }

    async fn print_to_host(&mut self, str: String) -> Result<(), wasmtime::Error> {
        println!("{str}");
        Ok(())
    }
}

pub fn add_to_linker(linker: &mut Linker<Ctx>) -> Result<()> {
    sql::add_to_linker(linker, |ctx: &mut Ctx| &mut ctx.database)?;
    Ok(())
}

fn build_query_result(rows: Vec<SqliteRow>) -> Result<String, anyhow::Error> {
    let mut query_result: String = String::new();
    for row in rows {
        let columns = row.columns();
        for column in columns {
            let value: Option<String> = match column.type_info().name() {
                "INTEGER" => {
                    // Decode as Option<i32> for INTEGER columns
                    row.try_get::<Option<i32>, _>(column.name())?
                        .map(|v| v.to_string())
                }
                _ => {
                    // Decode as Option<String> for other column types
                    row.try_get::<Option<String>, _>(column.name())?
                }
            };
            match value {
                Some(v) => {
                    let row_str = &format!("Column {}: {}\n", column.name(), v);
                    query_result.push_str(row_str);
                }
                None => {
                    let row_str = &format!("Column {}: NULL\n", column.name());
                    query_result.push_str(row_str);
                }
            }
        }
    }

    Ok(query_result)
}
