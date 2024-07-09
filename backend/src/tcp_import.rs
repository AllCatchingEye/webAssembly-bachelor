bindgen!({

    path: "wit",
    world: "server",
    async: true,
    with: {
        "bachelor:backend/tcp/socket": BackendSocket,
        "bachelor:backend/tcp/tcp-stream": BackendStream,
    }
});

use bachelor::backend::tcp;
use bachelor::backend::tcp::{DbOperation, Dht11Message, Message, TestMessage};

use crate::{Ctx, Import};

use serde::Deserialize;
use serde_json::Value;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use wasmtime::{
    component::{bindgen, Linker, ResourceTable},
    Result,
};

use wasmtime::component::{ComponentType, Resource};

pub struct TcpHost {
    res_table: ResourceTable,
}

pub struct BackendSocket {
    res_table: ResourceTable,
    listener: TcpListener,
}

pub struct BackendStream {
    stream: TcpStream,
}

#[derive(ComponentType, Deserialize)]
#[component(enum)]
enum HostDbOperation {
    Select,
    Insert,
    Delete,
    Unknown,
}

fn host_to_guest_db_operation(val: HostDbOperation) -> DbOperation {
    println!("Mapping Host to Guest DbOperation");
    match val {
        HostDbOperation::Select => DbOperation::Select,
        HostDbOperation::Insert => DbOperation::Insert,
        HostDbOperation::Delete => DbOperation::Delete,
        HostDbOperation::Unknown => DbOperation::Unknown,
    }
}

#[derive(ComponentType, Deserialize)]
#[component(record)]
struct Dht11Wrapper {
    message_type: String,
    operation: HostDbOperation,
    id: Option<u32>,
    temperature: Option<i32>,
    humidity: Option<i32>,
}

#[derive(ComponentType, Deserialize)]
#[component(record)]
struct TestMessageWrapper {
    message_type: String,
    operation: HostDbOperation,
    id: Option<u32>,
    name: Option<String>,
}

impl Import for TcpHost {
    fn new() -> Self {
        TcpHost {
            res_table: ResourceTable::new(),
        }
    }
}

impl tcp::HostSocket for TcpHost {
    fn drop(&mut self, res: Resource<BackendSocket>) -> Result<(), wasmtime::Error> {
        self.res_table.delete(res)?;
        Ok(())
    }
}

impl tcp::HostTcpStream for TcpHost {
    fn drop(&mut self, res: Resource<BackendStream>) -> Result<(), wasmtime::Error> {
        self.res_table.delete(res)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl tcp::Host for TcpHost {
    async fn create_socket(
        &mut self,
        addr: String,
    ) -> Result<Result<Resource<BackendSocket>, u32>, wasmtime::Error> {
        let listener = TcpListener::bind(addr)?;

        let database_socket = BackendSocket {
            res_table: ResourceTable::new(),
            listener,
        };
        let res = self.res_table.push(database_socket)?;

        Ok(Ok(res))
    }

    async fn accept(
        &mut self,
        sock: Resource<BackendSocket>,
    ) -> Result<Result<Resource<BackendStream>, u32>, wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;

        let res;
        loop {
            match host_sock.listener.incoming().next() {
                Some(stream) => {
                    let stream = stream?;

                    let host_stream = BackendStream { stream };
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
        sock: Resource<BackendSocket>,
        inc_str: Resource<BackendStream>,
    ) -> Result<Result<String, u32>, wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;
        let host_stream = host_sock.res_table.get_mut(&inc_str)?;

        let mut buffer = [0; 1024];
        let bytes_read = host_stream.stream.read(&mut buffer)?;

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        let message = message.trim_end_matches('\0');

        Ok(Ok(message.to_string()))
    }

    async fn write(
        &mut self,
        sock: Resource<BackendSocket>,
        inc_str: Resource<BackendStream>,
        msg: String,
    ) -> Result<Result<(), u32>, wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;
        let host_stream = host_sock.res_table.get_mut(&inc_str)?;

        host_stream.stream.write_all(msg.as_bytes())?;

        Ok(Ok(()))
    }

    async fn close_stream(
        &mut self,
        sock: Resource<BackendSocket>,
        stream: Resource<BackendStream>,
    ) -> Result<(), wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;
        let host_stream = host_sock.res_table.get_mut(&stream)?;
        host_stream.stream.shutdown(std::net::Shutdown::Both)?;
        host_sock.res_table.delete(stream)?;

        Ok(())
    }

    async fn parse_data(
        &mut self,
        message: String,
    ) -> Result<Result<Message, u32>, wasmtime::Error> {
        let json_body: Value =
            serde_json::from_str(&message).expect("Something went wrong while parsing");

        println!("{:#?}", json_body);
        let message_type = json_body.get("message_type").and_then(|v| v.as_str());

        println!("{:#?}", message_type);
        let message_data: Message;
        match message_type {
            Some("test") => {
                println!("Building host test message");
                let test_message: TestMessageWrapper = serde_json::from_value(json_body)?;
                println!("Building guest test message");
                let guest_test_message = TestMessage {
                    message_type: test_message.message_type,
                    operation: host_to_guest_db_operation(test_message.operation),
                    id: test_message.id,
                    name: test_message.name,
                };

                message_data = Message::Test(guest_test_message);
                Ok(Ok(message_data))
            }
            Some("dht11") => {
                let dht11_message: Dht11Wrapper = serde_json::from_value(json_body)?;
                let guest_dht11_message = Dht11Message {
                    message_type: dht11_message.message_type,
                    operation: host_to_guest_db_operation(dht11_message.operation),
                    id: dht11_message.id,
                    temperature: dht11_message.temperature,
                    humidity: dht11_message.humidity,
                };

                message_data = Message::Dht11(guest_dht11_message);
                Ok(Ok(message_data))
            }
            _ => Ok(Err(0)),
        }
    }
}

pub fn add_to_linker(linker: &mut Linker<Ctx>) -> Result<()> {
    tcp::add_to_linker(linker, |ctx: &mut Ctx| &mut ctx.server)?;
    Ok(())
}
