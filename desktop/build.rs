use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let sdl3_dir = PathBuf::from("C:\\Users\\aleksa\\Desktop\\projects\\rust\\SDL3-devel-3.2.20-VC\\SDL3-3.2.20"); // Adjust to your SDL3 path
    let mut lib_dir = sdl3_dir.join("lib");
    let mut dll_dir = sdl3_dir.join("lib");

    if target.contains("x86_64") {
        lib_dir.push("x64");

        dll_dir.push("x64");

    } else {
        lib_dir.push("x86");
        dll_dir.push("x86");
    }

    // Tell Cargo where to find SDL3.lib
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=SDL3");

    // Copy SDL3.dll to the output directory
    let dll_file = dll_dir.join("SDL3.dll");
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("SDL3.dll");
    std::fs::copy(&dll_file, &target_dir).expect("Failed to copy SDL3.dll");
}