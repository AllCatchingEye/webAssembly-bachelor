#[allow(warnings)]
mod bindings;

use bindings::exports::backend::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

bindings::export!(Component with_types_in bindings);
