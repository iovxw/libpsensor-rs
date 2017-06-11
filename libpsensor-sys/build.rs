extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

static LIBPSENSOR_SRC: &str = "psensor-1.2.0/src/lib/";

fn main() {
    gcc::Config::new()
        .file("wrapper.c")
        .include(".")
        .include(LIBPSENSOR_SRC)
        .compile("libpsensor.a");

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .clang_arg("-I.")
        .clang_arg(format!("-I{}", LIBPSENSOR_SRC))
        .header("wrapper.h")
        .whitelisted_function("^psensor_.*")
        .whitelisted_type("^psensor_.*")
        .constified_enum("psensor_type")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
