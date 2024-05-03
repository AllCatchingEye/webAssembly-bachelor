#[allow(warnings)]
mod bindings;

use bindings::example::component::adder::add;

fn main() {
    let result = add(3, 2);
    println!("Result of add: {}", result);
}
