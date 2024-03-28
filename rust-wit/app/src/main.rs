#[allow(warnings)]
mod bindings;

use bindings::bachelor::server::server_handle::hello_world;

fn main() {
    let greeting = hello_world();

    println!("{greeting}");
}
