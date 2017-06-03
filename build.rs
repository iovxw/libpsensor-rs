extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::io::{Error, ErrorKind};

const PSENSOR_DIR: &'static str = "psensor-1.2.0";
const LIBPSENSOR_DIR: &'static str = "psensor-1.2.0/src/lib";

fn main() {
    Command::new("./configure")
        .current_dir(PSENSOR_DIR)
        .env("CFLAGS", "-fPIC")
        .env("CONFIG_FILES", "src/lib/Makefile")
        .output()
        .and_then(|output| if output.status.success() {
                      Ok(())
                  } else {
                      let err = String::from_utf8_lossy(&output.stderr);
                      Err(Error::new(ErrorKind::Other, err.to_string()))
                  })
        .expect("failed to configure psensor");

    Command::new("make")
        .current_dir(LIBPSENSOR_DIR)
        .output()
        .and_then(|output| if output.status.success() {
                      Ok(())
                  } else {
                      let err = String::from_utf8_lossy(&output.stderr);
                      Err(Error::new(ErrorKind::Other, err.to_string()))
                  })
        .expect("failed to make libpsensor");

    println!("cargo:rustc-link-search={}", LIBPSENSOR_DIR);
    println!("cargo:rustc-link-lib=static=psensor");

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .clang_arg("-fPIC")
        .clang_arg("-Ipsensor-1.2.0")
        .clang_arg("-Ipsensor-1.2.0/src/lib")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
