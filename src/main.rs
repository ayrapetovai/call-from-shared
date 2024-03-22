#![crate_type = "bin"]

// https://docs.rs/libloading/latest/libloading/
use libloading::{Library, Symbol};

fn main() {
    unsafe {
        let lib = Library::new("./libshared.so").unwrap();
        let twice = lib
            .get::<Symbol<extern "C" fn(i32) -> i32>>(b"twice")
            .unwrap();

        println!("{}", twice(1));
    }
}
