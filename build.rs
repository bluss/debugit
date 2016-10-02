
//!
//! This build script detects if we are nightly or not
//!

extern crate rustc_version;

use rustc_version::Channel;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    match rustc_version::version_meta().channel {
        Channel::Dev | Channel::Nightly => {
            println!("cargo:rustc-cfg=use_nightly");
        }
        _otherwise => { }
    }
}
