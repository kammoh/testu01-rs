use std::{env, path::PathBuf};

use cargo_emit::*;
use cmake::Config;

fn main() {
    let src_dir = "TestU01";
    let dst = Config::new(src_dir)
        .profile("Release")
        .cflag("-O3")
        .cflag("-march=native")
        .cflag("-mtune=native")
        .build();

    rerun_if_changed!("wrapper.h");
    let mut match_options = glob::MatchOptions::new();
    match_options.case_sensitive = false;
    for src in glob::glob_with(format!("{}/**/*", src_dir).as_str(), match_options).unwrap() {
        rerun_if_changed!(src.unwrap().display());
    }
    rerun_if_changed!("wrapper.h");

    rustc_link_search!(format!("{}", dst.join("lib").display()) => "native");
    rustc_link_lib!("testu01" => "static");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .allowlist_item("unif01.*")
        .allowlist_item("bbattery.*")
        .allowlist_item("swrite.*")
        .allowlist_item("sres.*")
        .allowlist_item("smultin.*")
        .allowlist_item("sentrop.*")
        .allowlist_item("snpair.*")
        .allowlist_item("sknuth.*")
        .allowlist_item("swalk.*")
        .allowlist_item("sspectral.*")
        .allowlist_item("scomp.*")
        .allowlist_item("sstring.*")
        .allowlist_item("sspacings.*")
        .allowlist_item("scatter.*")
        .allowlist_item("smarsa.*")
        .allowlist_item("statcol.*")
        .allowlist_item("testu01.*")
        .allowlist_item("gofw.*")
        .allowlist_item("ulcg.*")
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
