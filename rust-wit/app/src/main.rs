#[allow(warnings)]
mod bindings;

use bindings::backend::adder::add::add;
use bindings::backend::server::server_handle::handle;

fn main() -> Result<(), anyhow::Error> {
    let x = 5;
    let y = 3;
    let result = add(x, y);
    println!("x + y = {result}");

    handle()?;

    Ok(())
}
