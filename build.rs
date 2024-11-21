//! Downloads and statically links the `mach-dxcompiler` C library.

use std::env::*;
use std::path::*;
use std::process::*;

/// Downloads and links the static DXC binary.
fn main() {
    let target = var("TARGET").expect("Failed to get target triple");
    
    let out_dir = var("OUT_DIR").expect("Failed to get output directory");
    let lib_folder = PathBuf::from(out_dir).join(&target);
    let file_path = lib_folder.join("machdxcompiler.tar.gz");
    download_url(&get_target_url(&target), &file_path);
    extract_tar_gz(&file_path, &lib_folder);

    println!("cargo:rustc-link-lib=static=machdxcompiler");
    println!("cargo:rustc-link-search=native={}", lib_folder.display());
}

/// Gets the URL from which the DXC binary should be downloaded.
fn get_target_url(target: &str) -> String {
    const BASE_URL: &str = "https://github.com/hexops/mach-dxcompiler/releases/download/2023.12.14%2B0b7073b.1/";

    BASE_URL.to_string() + match target {
        "x86_64-pc-windows-msvc" => "x86_64-windows-msvc_ReleaseFast_lib.tar.gz",
        "x86_64-pc-windows-gnu" => "x86_64-windows-gnu_ReleaseFast_lib.tar.gz",
        _ => panic!("Unsupported target '{target}' for mach-dxcompiler")
    }
}

/// Downloads the provided URL to a file.
fn download_url(url: &str, file_path: &Path) {
    Command::new("curl")
        .arg("--location")
        .arg("-o")
        .arg(file_path)
        .arg(url)
        .spawn()
        .expect("Failed to start Curl to download DXC binary")
        .wait()
        .expect("Failed to download DXC binary");
}

/// Extracts the file at the provided path as a `.tar.gz` file.
/// The contents are extracted to the current directory.
fn extract_tar_gz(path: &Path, output_dir: &Path) {
    Command::new("tar")
        .current_dir(output_dir)
        .arg("-xzf")
        .arg(path)
        .spawn()
        .expect("Failed to start Tar to extract DXC binary")
        .wait()
        .expect("Failed to extract DXC binary");
}