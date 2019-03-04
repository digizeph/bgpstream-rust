#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern fn print_hello_from_rust() {
    println!("Hello from Rust");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ffi::CString;

    #[test]
    fn bgpstream_create() {
        unsafe{
            let mut bs: bgpstream_t = mem::zeroed();
            // bgpstream_debug("BS: create start");
            let c_str = CString::new("Hello, world!").expect("CString::new failed");
            printf(c_str.as_ptr());
            bgpstream_create(); // FIXME: this will crash
        }
    }
}
