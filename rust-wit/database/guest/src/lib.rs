use wasi::sockets::instance_network::instance_network;
use wasi::sockets::network::{ErrorCode, IpAddressFamily, Ipv4SocketAddress};
use wasi::sockets::udp::{IncomingDatagram, IpSocketAddress, Pollable, UdpSocket};
use wasi::sockets::udp_create_socket::create_udp_socket;

wit_bindgen::generate!({
    path: "../wit",
    world: "database",
});

// use bachelor::database::readwrite::{exec, query};
// use bachelor::database::types::{open_connection, prepare_statement};

struct Component;

impl exports::bachelor::database::handler::Guest for Component {
    fn start_server() -> Result<UdpSocket, ErrorCode> {
        let addr_family = IpAddressFamily::Ipv4;
        let socket = create_udp_socket(addr_family)?;
        Ok(socket)
    }

    fn handle(sock: UdpSocket) -> Result<(), ErrorCode> {
        let addr = IpSocketAddress::Ipv4(Ipv4SocketAddress {
            port: 8080,
            address: (192, 168, 0, 217),
        });

        let net = instance_network();

        sock.start_bind(&net, addr)?;
        println!("Started binding udp socket");

        let pollable: Pollable = sock.subscribe();
        pollable.block();

        let (sock, data) = Self::socket_receive(sock)?;
        let message: String = String::from_utf8(data).expect("Error converting bytes to string");
        println!("Received message: {}", message);

        let json: serde_json::Value =
            serde_json::from_str(message.as_str()).expect("Error parsing JSON");

        if let Some(id) = json.get("id") {
            if let Some(id_str) = id.as_str() {}
        }

        Ok(())
    }

    fn socket_receive(sock: UdpSocket) -> Result<(UdpSocket, Vec<u8>), ErrorCode> {
        let (inc_data_stream, _) = sock.stream(None)?;
        let datagrams: Vec<IncomingDatagram> = inc_data_stream.receive(1)?;
        if let Some(datagram) = datagrams.get(0) {
            let data = datagram.data.clone();

            Ok((sock, data))
        } else {
            println!("No datagrams received");
            Err(ErrorCode::InvalidArgument)
        }
    }

    fn add(x: i32, y: i32) -> i32 {
        println!("{} + {} = {}", x, y, x + y);
        x + y
    }
}

export!(Component);
