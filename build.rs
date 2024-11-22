//! Downloads and statically links the `mach-dxcompiler` C library.

use std::env::*;
use std::path::*;
use std::process::*;

/// Downloads and links the static DXC binary.
fn main() {
    let target = var("TARGET").expect("Failed to get target triple");

    let out_dir = PathBuf::from(var("OUT_DIR").expect("Failed to get output directory"));
    let file_path = out_dir.join("machdxcompiler.tar.gz");
    download_url(&get_target_url(&target, static_crt()), &file_path);
    extract_tar_gz(&file_path, &out_dir);

    println!("cargo:rustc-link-lib=static=machdxcompiler");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}

/// Gets the URL from which the DXC binary should be downloaded.
fn get_target_url(target: &str, static_crt: bool) -> String {
    const BASE_URL: &str = "https://github.com/DouglasDwyer/mach-dxcompiler/releases/download/2024.11.22%2Bbfceb9a.1/";

    BASE_URL.to_string() + match (target, static_crt) {
        ("x86_64-pc-windows-msvc", false) => "x86_64-windows-msvc_ReleaseFast_Dynamic_lib.tar.gz",
        ("x86_64-pc-windows-msvc", true) => "x86_64-windows-msvc_ReleaseFast_lib.tar.gz",
        ("x86_64-pc-windows-gnu", _) => "x86_64-windows-gnu_ReleaseFast_lib.tar.gz",
        _ => panic!("Unsupported target '{target}' for mach-dxcompiler")
    }
}

/// Downloads the provided URL to a file.
fn download_url(url: &str, file_path: &Path) {
    let result = Command::new("curl")
        .arg("--location")
        .arg("-o")
        .arg(file_path)
        .arg(url)
        .spawn()
        .expect("Failed to start Curl to download DXC binary")
        .wait()
        .expect("Failed to download DXC binary");
    if !result.success() {
        panic!("{result}");
    }
}

/// Extracts the file at the provided path as a `.tar.gz` file.
/// The contents are extracted to the current directory.
fn extract_tar_gz(path: &Path, output_dir: &Path) {
    let result = Command::new("tar")
        .current_dir(output_dir)
        .arg("-xzf")
        .arg(path)
        .spawn()
        .expect("Failed to start Tar to extract DXC binary")
        .wait()
        .expect("Failed to extract DXC binary");
    if !result.success() {
        panic!("{result}");
    }
}

/// Determines whether the CRT is being statically or dynamically linked.
fn static_crt() -> bool {
    var("CARGO_ENCODED_RUSTFLAGS")
        .unwrap_or_default()
        .contains("target-feature=+crt-static")
}