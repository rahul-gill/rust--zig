use std::{env};


use std::process::{Command};

//zig build-lib -fPIC -fPIC -fcompiler-rt  --name ziglib zig/lib.zig
fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_source_file = format!("{}/zig/lib.zig", crate_dir);


    println!("cargo:rerun-if-changed={}", lib_source_file.as_str());
    Command::new("zig").args(["build-lib", "-fPIC", "-static", "-fcompiler-rt",
        lib_source_file.as_str(), "--name", "ziglib"])
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", crate_dir.as_str());
    // println!("cargo:rustc-link-arg=");
    println!("cargo:rustc-link-lib=static=ziglib");
}