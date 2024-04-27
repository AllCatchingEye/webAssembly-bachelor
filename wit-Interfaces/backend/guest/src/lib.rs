wit_bindgen::generate!({
    path: "../wit",
    world: "backend",
});

use bachelor::backend::sql::{
    create_table, drop_connection, execute_query, open_connection, print_to_host, DbOperation,
};

use bachelor::backend::tcp::{
    accept, close_stream, create_socket, parse_data, read, Error, MessageData, TestMessageData,
};

struct Component;

impl exports::bachelor::backend::sockets_handler::Guest for Component {
    fn init_db() -> Result<(), Error> {
        let conn = open_connection("sqlite:data.db", true)?;
        create_table("DROP TABLE test", &conn);
        create_table(
            "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)",
            &conn,
        );
        drop_connection(conn)?;
        Ok(())
    }

    fn handle_test_message(data: TestMessageData) -> Result<(), Error> {
        let conn = open_connection("sqlite:data.db", true)?;
        match data.operation {
            DbOperation::Select => {
                let query = "SELECT * FROM test";
                if let Some(results) = execute_query(&conn, query, None)? {
                    print_to_host(&format!("Results: \n{}", results));
                }
            }
            DbOperation::Insert => {
                let query = "INSERT INTO test (name) VALUES (?)";
                let values = vec![data.name];
                let val_data = serde_json::to_string(&values).expect("Serializing values");

                execute_query(&conn, query, Some(&val_data))?;

                let query = "SELECT * FROM test";
                if let Some(results) = execute_query(&conn, query, None)? {
                    print_to_host(&format!("Results: \n{}", results));
                }
            }
            DbOperation::Delete => {
                let query = "DELETE FROM test WHERE name = ?";
                let values = vec![data.name];

                let val_data = serde_json::to_string(&values).expect("Serializing values");
                execute_query(&conn, query, Some(&val_data))?;

                let query = "SELECT * FROM test";
                if let Some(results) = execute_query(&conn, query, None)? {
                    print_to_host(&format!("Results: \n{}", results));
                }
            }
            DbOperation::Unknown => {}
        }
        drop_connection(conn)?;
        Ok(())
    }

    fn socket_handle() -> Result<(), Error> {
        let addr = "127.0.0.1:8080";
        let socket = create_socket(addr)?;
        print_to_host("Created socket");

        Component::init_db()?;

        loop {
            print_to_host("Listening for incoming connection...");
            let stream = accept(&socket)?;

            print_to_host("Reading from stream...");
            let message = read(&socket, &stream)?;

            let data = parse_data(&message)?;
            match data {
                MessageData::Dht11(_data) => {}
                MessageData::TestMessage(data) => Component::handle_test_message(data)?,
                MessageData::None => print_to_host("Unknown message type"),
            }

            print_to_host("Closing connection...");
            close_stream(&socket, stream);
        }
    }
}

export!(Component);
