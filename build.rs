use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bgpstream");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .whitelist_function("bgpstream_record_create")
        .whitelist_function("bgpstream_create")
        .whitelist_function("printf")
        .whitelist_function("bgpstream_get_data_interface_id")
        .whitelist_function("bgpstream_get_data_interface_info")
        .whitelist_function("bgpstream_add_filter")
        .whitelist_function("bgpstream_add_interval_filter")
        .whitelist_function("bgpstream_get_next_record")
        .whitelist_type("window")
        .whitelist_type("bgpstream_record_t")
        .whitelist_type("bgpstream_record")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
