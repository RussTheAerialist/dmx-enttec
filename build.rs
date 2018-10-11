extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=ftd2xx");
    println!("cargo:rustc-link-lib=usb-1.0");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("FT_CreateDeviceInfoList")
        .whitelist_function("FT_GetDeviceInfoList")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
