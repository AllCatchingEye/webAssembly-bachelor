bindgen!({

    path: "wit",
    world: "server",
    async: true,
    with: {
        "bachelor:backend/tcp/socket": DatabaseTcpSocket,
        "bachelor:backend/tcp/tcp-stream": DatabaseTcpStream,
    }
});

use bachelor::backend::tcp;
use bachelor::backend::tcp::{DbOperation, Dht11Data, MessageData, TestMessageData};

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

pub struct DatabaseTcpSocket {
    res_table: ResourceTable,
    listener: TcpListener,
}

pub struct DatabaseTcpStream {
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
struct Dht11 {
    message_type: String,
    operation: HostDbOperation,
    id: Option<u32>,
    temperature: Option<i32>,
    humidity: Option<i32>,
}

#[derive(ComponentType, Deserialize)]
#[component(record)]
struct TestMessage {
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

    async fn write(
        &mut self,
        sock: Resource<DatabaseTcpSocket>,
        inc_str: Resource<DatabaseTcpStream>,
        msg: String,
    ) -> Result<Result<(), u32>, wasmtime::Error> {
        let host_sock = self.res_table.get_mut(&sock)?;
        let host_stream = host_sock.res_table.get_mut(&inc_str)?;

        host_stream.stream.write_all(msg.as_bytes())?;

        Ok(Ok(()))
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

    async fn parse_data(
        &mut self,
        message: String,
    ) -> Result<Result<MessageData, u32>, wasmtime::Error> {
        let json_body: Value = serde_json::from_str(&message).unwrap();
        let message_type = json_body.get("message_type").and_then(|v| v.as_str());

        let message_data: MessageData;
        match message_type {
            Some("test") => {
                println!("Building host test message");
                let test_message: TestMessage = serde_json::from_value(json_body)?;
                println!("Building guest test message");
                let guest_test_message = TestMessageData {
                    message_type: test_message.message_type,
                    operation: host_to_guest_db_operation(test_message.operation),
                    id: test_message.id,
                    name: test_message.name,
                };

                message_data = MessageData::TestMessage(guest_test_message);
                Ok(Ok(message_data))
            }
            Some("dht11") => {
                let dht11_message: Dht11 = serde_json::from_value(json_body)?;
                let guest_dht11_message = Dht11Data {
                    message_type: dht11_message.message_type,
                    operation: host_to_guest_db_operation(dht11_message.operation),
                    id: dht11_message.id,
                    temperature: dht11_message.temperature,
                    humidity: dht11_message.humidity,
                };

                message_data = MessageData::Dht11(guest_dht11_message);
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
