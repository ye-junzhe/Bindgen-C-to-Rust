cargo install bindgen

bindgen c_struct.h > rust_bindings/bindgen_from_c.rs

rustc rust_main.rs

./rust_main
