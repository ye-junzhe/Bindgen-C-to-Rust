mod rust_bindings;

use rust_bindings::bindgen_from_c::{
    CoolStruct,
    cool_function
};

fn main() {
    let cool_struct = CoolStruct{ x: 10, y: 20 };
    unsafe {
        cool_function(cool_struct);
    }
}
