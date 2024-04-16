wit_bindgen::generate!({
    path: "../wit",
    world: "database",
});

use backend::database::sql::{
    create_table, delete, drop_connection, insert, open_connection, print_to_host, select,
    Error as SqlError,
};
//use exports::backend::database::sql_handler::Guest as SQLHandleGuest;

// // HTTP Server
// use exports::wasi::http::incoming_handler::Guest as HttpIncGuest;
// use wasi::http::types::IncomingBody;
// pub use wasi::http::types::{
//     Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
// };
// pub use wasi::io::streams::InputStream;

use backend::database::tcp::{accept, close_stream, create_socket, read, Error as TcpError};

struct Component;

// impl HttpIncGuest for Component {
//     fn handle(request: IncomingRequest, outparam: ResponseOutparam) {
//         let hdrs = Fields::new();
//         let resp = OutgoingResponse::new(hdrs);
//         let body = resp.body().expect("outgoing response");
//
//         ResponseOutparam::set(outparam, Ok(resp));
//
//         let out = body.write().expect("outgoing stream");
//         out.blocking_write_and_flush(b"Hello, wasi:http/proxy world!\n")
//             .expect("writing response");
//
//         drop(out);
//         OutgoingBody::finish(body, None).unwrap();
//     }
// }

// impl SQLHandleGuestGuest for Component {
//     fn db_test() -> Result<(), Error> {
//         print_to_host("Started handle...");
//
//         print_to_host("Opening connection to db...");
//         let conn = open_connection("sqlite:data.db", true).unwrap();
//
//         print_to_host("Creating table...");
//         create_table(
//             "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)",
//             &conn,
//         );
//
//         print_to_host("Inserting into db...");
//         insert(&conn, "Alice");
//         let res = select(&conn)?;
//         let message = format!("Select results after delete:\n{:?}", res);
//         print_to_host(message.as_str());
//
//         print_to_host("Delete from db...");
//         delete(&conn, "Alice");
//         let res = select(&conn)?;
//         let message = format!("Select results after delete:\n{:?}", res);
//         print_to_host(message.as_str());
//
//         print_to_host("Drop connection to db");
//         drop_connection(conn)?;
//
//         print_to_host("Finished handler.");
//
//         Ok(())
//     }
// }

impl exports::backend::database::sockets_handler::Guest for Component {
    fn socket_handle() -> Result<(), TcpError> {
        let addr = "127.0.0.1:8080";
        let socket = create_socket(addr)?;
        print_to_host("Created socket");

        loop {
            print_to_host("Listening for incoming connection...");
            let stream = accept(&socket)?;

            println!("Reading from stream...");
            let message = read(&socket, &stream)?;

            print_to_host(message.as_str());

            println!("Closing connection...");
            close_stream(&socket, stream);
        }
    }
    // fn socket_handle() -> Result<(), ErrorCode> {
    //     let addr_family = IpAddressFamily::Ipv4;
    //     let sock = create_tcp_socket(addr_family)?;
    //
    //     let addr = IpSocketAddress::Ipv4(Ipv4SocketAddress {
    //         port: 8080,
    //         address: (127, 0, 0, 1),
    //     });
    //
    //     let net: network = instance_network();
    //     print_to_host(format!("{:?}", net).as_str());
    //     print_to_host(format!("{:?}", sock.address_family()).as_str());
    //
    //     sock.set_keep_alive_enabled(true)?;
    //
    //     let pollable = sock.subscribe();
    //
    //     print_to_host(format!("staring bind...").as_str());
    //     sock.start_bind(&net, addr)?;
    //     pollable.block();
    //     sock.finish_bind()?;
    //     print_to_host(format!("finished bind:\n{:?}", sock.local_address()?).as_str());
    //
    //     print_to_host(format!("staring listen...").as_str());
    //     sock.start_listen()?;
    //     pollable.block();
    //     sock.finish_listen()?;
    //     print_to_host(format!("finished listening").as_str());
    //
    //     pollable.block();
    //     let (client_sock, inc_stream, out_stream) = sock.accept()?;
    //     print_to_host(format!("accepted new client").as_str());
    //
    //     loop {
    //         // let datagrams: vec<incomingdatagram> = inc_stream.read(150000)?;
    //         // if let some(datagram) = datagrams.get(0) {
    //         //     print_to_host(format!("received datagram: {:?}", datagram).as_str());
    //         //     let data = datagram.data.clone();
    //         //
    //         //     let message: string =
    //         //         string::from_utf8(data).expect("error converting bytes to string");
    //         //     print_to_host(format!("( received message: {}", message).as_str());
    //         //
    //         //     let json: serde_json::value =
    //         //         serde_json::from_str(message.as_str()).expect("error parsing json");
    //         //
    //         //     if let some(id) = json.get("id") {
    //         //         if let some(id_str) = id.as_str() {
    //         //             print_to_host(format!("name: {}", id_str).as_str());
    //         //         }
    //         //     }
    //         // } else {
    //         //     continue;
    //         // }
    //     }
    //
    //     // ok(())
    // }

    //fn socket_receive(sock: UdpSocket) -> Result<Vec<u8>, ErrorCode> {}
}

export!(Component);
