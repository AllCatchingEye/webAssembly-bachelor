#[allow(warnings)]
mod bindings;

use bindings::exports::backend::server::server_handle::Guest;
use bindings::wasi::sockets::instance_network::instance_network;
use bindings::wasi::sockets::network::{ErrorCode, IpAddressFamily, Ipv4SocketAddress};
use bindings::wasi::sockets::udp::{IncomingDatagram, IpSocketAddress, Pollable, UdpSocket};
use bindings::wasi::sockets::udp_create_socket::create_udp_socket;

use crate::bindings::wasi::sockets::network::Network;

struct Component;

impl Guest for Component {
    fn handle() -> Result<(), ErrorCode> {
        let addr_family = IpAddressFamily::Ipv4;
        let sock = create_udp_socket(addr_family)?;

        let addr = IpSocketAddress::Ipv4(Ipv4SocketAddress {
            port: 0,
            address: (0, 0, 0, 0),
        });

        let net: Network = instance_network();
        println!("{}", net.handle());

        sock.start_bind(&net, addr)?;
        println!("Started binding udp socket");

        sock.finish_bind()?;
        println!("Finished binding udp socket");

        let pollable: Pollable = sock.subscribe();
        pollable.block();

        let data = Self::socket_receive(&sock)?;
        let message: String = String::from_utf8(data).expect("Error converting bytes to string");
        println!("Received message: {}", message);

        let json: serde_json::Value =
            serde_json::from_str(message.as_str()).expect("Error parsing JSON");

        if let Some(id) = json.get("id") {
            if let Some(id_str) = id.as_str() {}
        }

        Ok(())
    }

    fn socket_receive(sock: &UdpSocket) -> Result<Vec<u8>, ErrorCode> {
        let (inc_data_stream, _) = sock.stream(None)?;
        let datagrams: Vec<IncomingDatagram> = inc_data_stream.receive(1)?;
        if let Some(datagram) = datagrams.get(0) {
            let data = datagram.data.clone();

            Ok(data)
        } else {
            println!("No datagrams received");
            Err(ErrorCode::InvalidArgument)
        }
    }
}

bindings::export!(Component with_types_in bindings);
