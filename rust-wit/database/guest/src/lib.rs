wit_bindgen::generate!({
    path: "../wit",
    world: "database",
});

use backend::database::sql::{
    create_table, delete, drop_connection, insert, open_connection, print_to_host, select, Error,
};

struct Component;

impl exports::backend::database::handler::Guest for Component {
    fn handle() -> Result<(), Error> {
        print_to_host("Started handle...");

        print_to_host("Opening connection to db...");
        let conn = open_connection("sqlite:data.db", true).unwrap();

        print_to_host("Creating table...");
        create_table(
            "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)",
            &conn,
        );

        print_to_host("Inserting into db...");
        insert(&conn, "Alice");
        let res = select(&conn)?;
        let message = format!("Select results after delete:\n{:?}", res);
        print_to_host(message.as_str());

        print_to_host("Delete from db...");
        delete(&conn, "Alice");
        let res = select(&conn)?;
        let message = format!("Select results after delete:\n{:?}", res);
        print_to_host(message.as_str());

        print_to_host("Drop connection to db");
        drop_connection(conn)?;

        print_to_host("Finished handler.");

        Ok(())
    }
}

export!(Component);
