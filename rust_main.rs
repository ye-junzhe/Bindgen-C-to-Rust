mod rust_bindings;

use rust_bindings::bindgen_from_c::CoolStruct;

fn main() {
    let cool_struct = CoolStruct{ x: 10, y: 20 };
    println!("Inside CoolStruct => x: {}, y: {}", cool_struct.x, cool_struct.y);
}
