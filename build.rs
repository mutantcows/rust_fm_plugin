#[cfg(feature = "ffi")]
use std::env;

fn main() {
#[cfg(feature = "ffi")]
    link_search();
}

#[cfg(feature = "ffi")]
fn link_search() {
    let manifest = env!("CARGO_MANIFEST_DIR");

    if cfg!(target_os = "windows") {
        println!(r"cargo:rustc-link-search={}/libraries/Win/x64", manifest);
    } else if cfg!(target_os = "macos") {
        println!(
            r"cargo:rustc-link-search=framework={}/libraries/Mac",
            manifest
        );
    } else if cfg!(target_os = "linux") {
        println!(r"cargo:rustc-link-search={}/libraries/Linux", manifest);
    }
}

#[cfg(any())]
fn run_bindgen() {
    let bindings = bindgen::Builder::default()
        .clang_arg("--language=c++")
        // .clang_arg("-std=c++14")
        .clang_arg("-std=c++1y")
        .clang_arg("-stdlib=libc++")
        .header("headers/FMWrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
