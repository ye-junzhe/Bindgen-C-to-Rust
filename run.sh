# cargo install bindgen

bindgen c_struct.c > rust_bindings/bindgen_from_c.rs

clang c_struct.c -c

rustc rust_main.rs -l c_struct.o -L .

./rust_main
