
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CString;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern {
    fn print_bs_record(bs_record: *mut bgpstream_record_t);
}

pub fn try_bs() {
    unsafe{
        // bgpreader -w 1445306400,1445306402 -c route-views.sfmix

        let mut collector_str = CString::new("route-views.sfmix").expect("CString::new failed");
        let bs: *mut bgpstream_t = bgpstream_create();
        let bs_record: *mut bgpstream_record_t = bgpstream_record_create();
        let w = window{start:1445306400, end:1445306402};
        bgpstream_add_filter(bs, 2, collector_str.as_ptr());
        bgpstream_add_interval_filter(bs, w.start, w.end);
        bgpstream_start(bs);
        let mut get_next_ret = 1;
        let mut count = 0;
        while get_next_ret > 0{
            get_next_ret = bgpstream_get_next_record(bs, bs_record);
            print_bs_record(bs_record);
            count += 1;
            if count > 10 {
                break
            }
        }
    }

}

#[no_mangle]
pub extern fn print_hello_from_rust() {
    println!("Hello from Rust");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs() {
        unsafe{
            // bgpreader -w 1445306400,1445306402 -c route-views.sfmix
            try_bs();
        }
    }

}
