//! Downloads and statically links the `mach-dxcompiler` C library.

use std::env::*;
use std::path::*;

fn main() {
    let target = var("TARGET").expect("Failed to get manifest directory");
    let dir = var("CARGO_MANIFEST_DIR").expect("Failed to get manifest directory");
    let path = PathBuf::from(dir).join("lib").join(Path::new(&target));

    if path.exists() {
        println!("cargo:rustc-link-lib=static=machdxcompiler");
        println!("cargo:rustc-link-search=native={}", path.display());
    }
    else {
        panic!("Unsupported target '{target}' for mach-dxcompiler");
    }
}