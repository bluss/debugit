
//!
//! This build script detects if we are nightly or not
//!

extern crate version_check;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Some(true) = version_check::supports_features() {
        println!("cargo:rustc-cfg=use_nightly");
    }
}
