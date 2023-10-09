use std::{env, path::PathBuf};

use cargo_emit::*;
use cmake::Config;

fn main() {
    let dst = Config::new("TestU01")
        .profile("Release")
        .cflag("-O3")
        .cflag("-march=native")
        .cflag("-mtune=native")
        .build();

    rerun_if_changed!("wrapper.h");

    rustc_link_search!(format!("{}", dst.join("lib").display()) => "native");
    rustc_link_lib!("testu01" => "static");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_item("unif01.*")
        .allowlist_item("bbattery.*")
        .clang_arg(format!("-I{}", dst.join("include").display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
