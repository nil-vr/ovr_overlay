use std::path::{Path, PathBuf};

use normpath::PathExt;

fn main() {
    // include path openvr/headers
    let include_path = relative("openvr/headers");
    // This assumes all your C++ bindings are in main.rs
    let mut b =
        autocxx_build::Builder::new(relative("src/lib.rs"), &[&include_path]).expect_build();
    // arbitrary library name, pick anything
    b.flag_if_supported("-std=c++14").compile("autocxx-demo");

    println!("cargo:rerun-if-changed={:?}", relative("src/lib.rs"));
    println!("cargo:rerun-if-changed={:?}", relative("openvr/bin/"));
    println!("cargo:rerun-if-changed={:?}", relative("openvr/lib/"));

    // Link the C++ libraries
    #[cfg(target_os = "windows")]
    let input_files = {
        // Maybe we need these???

        // println!(
        //     "cargo:rustc-link-search=native={:?}",
        //     relative("openvr/lib/win64")
        // );

        [
            relative("openvr/bin/win64/openvr_api.dll"),
            relative("openvr/lib/win64/openvr_api.lib"),
        ]
    };

    #[cfg(not(target_os = "windows"))]
    panic!("Haven't tested on platforms other than windows!");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    for f in input_files {
        let file_name = f.file_name().unwrap();
        std::fs::copy(&f, out_dir.join(file_name))
            .expect(&format!("Failed to copy {:?} to {:?}", file_name, &out_dir));
    }

    println!("cargo:rustc-link-lib=dylib=openvr_api");
    println!("cargo:rustc-link-search=native={:?}", out_dir);
}

fn relative(s: &str) -> std::path::PathBuf {
    let result = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    result.join(s).normalize().unwrap().into_path_buf()
}