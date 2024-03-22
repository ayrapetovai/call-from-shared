#[no_mangle]
pub extern "C" fn twice(i: i32) -> i32 {
    i + i
}
