bindgen!({
    path: "wit",
    world: "database",
    async: true,
    with: {
        "backend:database/sql/connection": DatabaseConnectionHost,
        "backend:database/tcp/socket": DatabaseTcpSocket,
        "backend:database/tcp/tcp-stream": DatabaseTcpStream,
    }
});

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use backend::database::{sql, tcp};

use sqlx::prelude::*;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqliteConnection;

use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};

use wasmtime::component::Resource;
use wasmtime_wasi::{
    bindings::{wasi::filesystem, Imports},
    pipe::MemoryOutputPipe,
    ResourceTable, WasiCtx, WasiCtxBuilder, WasiP1Ctx, WasiView,
};

use wasmtime_wasi_http::{
    bindings::wasi::http as wasi_http, body::HyperIncomingBody, proxy, WasiHttpCtx, WasiHttpView,
};

pub struct DatabaseHost {
    res_table: ResourceTable,
}
pub struct DatabaseConnectionHost {
    connection: SqliteConnection,
}

pub struct TcpHost {
    res_table: ResourceTable,
}

pub struct DatabaseTcpSocket {
    res_table: ResourceTable,
    listener: TcpListener,
}

pub struct DatabaseTcpStream {
    stream: TcpStream,
}

impl tcp::HostSocket for TcpHost {
    fn drop(&mut self, res: Resource<DatabaseTcpSocket>) -> Result<(), wasmtime::Error> {
        self.res_table.delete(res)?;
        Ok(())
    }
}

impl tcp::HostTcpStream for TcpHost {
    fn drop(&mut self, res: Resource<DatabaseTcpStream>) -> Result<(), wasmtime::Error> {
        self.res_table.delete(res)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl tcp::Host for TcpHost {
    async fn create_socket(
        &mut self,
        addr: String,
    ) -> Result<Result<Resource<DatabaseTcpSocket>, u32>, wasmtime::Error> {
        let listener = TcpListener::bind(addr)?;

        let database_socket = DatabaseTcpSocket {
            res_table: ResourceTable::new(),
            listener,
        };
        let res = self.res_table.push(database_socket)?;

        Ok(Ok(res))
    }

    async fn accept(
        &mut self,
        sock: Resource<DatabaseTcpSocket>,
    ) -> Result<Result<Resource<DatabaseTcpStream>, u32>, wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;

        let res;
        loop {
            match host_sock.listener.incoming().next() {
                Some(stream) => {
                    let stream = stream?;

                    let host_stream = DatabaseTcpStream { stream };
                    res = host_sock.res_table.push(host_stream)?;
                    break;
                }
                None => continue, // Break the loop if there's no more incoming stream
            };
        }

        Ok(Ok(res))
    }

    async fn read(
        &mut self,
        sock: Resource<DatabaseTcpSocket>,
        inc_str: Resource<DatabaseTcpStream>,
    ) -> Result<Result<String, u32>, wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;
        let host_stream = host_sock.res_table.get_mut(&inc_str)?;

        let mut message = String::new();
        host_stream.stream.read_to_string(&mut message)?;

        Ok(Ok(message))
    }

    async fn close_stream(
        &mut self,
        sock: Resource<DatabaseTcpSocket>,
        stream: Resource<DatabaseTcpStream>,
    ) -> Result<(), wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;
        let host_stream = host_sock.res_table.get_mut(&stream)?;
        host_stream.stream.shutdown(std::net::Shutdown::Both)?;
        host_sock.res_table.delete(stream)?;

        Ok(())
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
    table: ResourceTable,
    database: DatabaseHost,
    server: TcpHost,
    wasi: WasiCtx,
    wasi_p1: WasiP1Ctx,
    http: WasiHttpCtx,
    stdout: MemoryOutputPipe,
    stderr: MemoryOutputPipe,
}

impl WasiHttpView for Ctx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
}

impl WasiView for Ctx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

fn add_wasi_bindings_to_linker(mut linker: Linker<Ctx>) -> Result<Linker<Ctx>> {
    //filesystem
    filesystem::preopens::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_p1)?;
    filesystem::types::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_p1)?;

    // Imports::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_p1)?;

    Ok(linker)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);

    let engine = Engine::new(&config)?;

    let database = DatabaseHost {
        res_table: ResourceTable::new(),
    };
    let server = TcpHost {
        res_table: ResourceTable::new(),
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let stdout = MemoryOutputPipe::new(4096);
    let stderr = MemoryOutputPipe::new(4096);
    let wasi = WasiCtxBuilder::new()
        .inherit_network()
        .inherit_stdio()
        .inherit_env()
        .inherit_args()
        .inherit_stdout()
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .env("HTTP_SERVER", addr.to_string())
        .build();

    let wasi_p1 = WasiCtxBuilder::new()
        .inherit_network()
        .allow_tcp(true)
        .allow_ip_name_lookup(true)
        .build_p1();
    let http = WasiHttpCtx;

    let table = ResourceTable::new();
    let mut store = Store::new(
        &engine,
        Ctx {
            table,
            database,
            server,
            wasi,
            wasi_p1,
            http,
            stderr,
            stdout,
        },
    );

    let mut linker = Linker::new(&engine);
    // wasmtime_wasi_http::proxy::add_to_linker(&mut linker)?;
    tcp::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.server)?;
    sql::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.database)?;
    Imports::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_p1)?;
    // linker = add_wasi_bindings_to_linker(linker)?;

    let component = Component::from_file(&engine, "guest-component.wasm")?;

    let (database, _instance) =
        Database::instantiate_async(&mut store, &component, &linker).await?;
    println!("Instantiated database");

    let result = database
        .backend_database_sockets_handler()
        .call_socket_handle(&mut store)
        .await?;
    println!("Read file.");

    // let (database, _instance) =
    //     wasmtime_wasi_http::proxy::Proxy::instantiate_async(&mut store, &component, &linker)
    //         .await?;
    // println!("Instantiated database");

    // let req = hyper::Request::builder()
    //     .header("custom-forbidden-header", "yes")
    //     .uri("http://example.com:8080/test-path")
    //     .method(http::Method::GET);
    // let req_body = req.body(HyperIncomingBody::default())?;
    // let req = store.data_mut().new_incoming_request(req_body)?;
    //
    // let (sender, receiver) = tokio::sync::oneshot::channel();
    // let out = store.data_mut().new_response_outparam(sender)?;
    //
    // let handle = wasmtime_wasi::with_ambient_tokio_runtime(|| async move {
    //     database
    //         .wasi_http_incoming_handler()
    //         .call_handle(&mut store, req, out)
    //         .await?;
    //
    //     Ok::<_, anyhow::Error>(())
    //     // Ok(())
    // });

    // let resp = match receiver.await {
    //     Ok(Ok(resp)) => {
    //         use http_body_util::BodyExt;
    //         let (parts, body) = resp.into_parts();
    //         let collected = BodyExt::collect(body).await?;
    //         Some(Ok(hyper::Response::from_parts(parts, collected)))
    //     }
    //     Ok(Err(e)) => Some(Err(e)),
    //
    //     // Fall through below to the `resp.expect(...)` which will hopefully
    //     // return a more specific error from `handle.await`.
    //     Err(_) => None,
    // };

    // handle.await.context("Component execution")?;

    // resp.expect("wasm never called set-response-outparam")?;

    Ok(())
}
