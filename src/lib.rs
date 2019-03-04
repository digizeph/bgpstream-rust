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
    use std::ffi::CString;

    #[test]
    fn bs() {
        unsafe{
            // bgpreader -w 1445306400,1445306402 -c route-views.sfmix

            let mut collector_str = CString::new("route-views.sfmix").expect("CString::new failed");
            printf(collector_str.as_ptr());
            let bs = bgpstream_create();
            let datasource_id = bgpstream_get_data_interface_id(bs);
            let datasource_info = bgpstream_get_data_interface_info(bs, datasource_id);
            // FIXME: cannot find function `bgpstream_record_create` in this scope
            let mut bs_record = bgpstream_record_create();
            let w = window{start:1445306400, end:1445306402};
            bgpstream_add_filter(bs, 2, collector_str.as_ptr());
            bgpstream_add_interval_filter(bs, w.start, w.end);
            let get_next_ret = bgpstream_get_next_record(bs, &mut bs_record);
        }
    }
}
