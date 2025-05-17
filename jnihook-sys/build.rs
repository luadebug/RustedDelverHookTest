#![allow(warnings)]
use std::env::var;

fn main() {
    let deps = vec!["user32", "psapi", "ntdll", "shell32"];
    for dep in deps {
        println!("cargo:rustc-link-lib={}", dep);
    }
    // Get current directory
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/../dep", manifest_dir);
    println!("cargo:rustc-link-lib=jvm");       // Link against jvm.lib
    println!("cargo:rustc-link-lib=jawt");      // Link against jawt.lib
    println!("cargo:rustc-link-lib=jnihook");   // Link against jnihook.lib
}
