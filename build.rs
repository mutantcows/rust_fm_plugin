#[cfg(target_os = "macos")]
fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    println!(
        r"cargo:rustc-link-search=framework={}/libraries/Mac",
        manifest
    );

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     .clang_arg("--language=c++")
    //     .clang_arg("-std=c++14")
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("headers/FMWrapper.h")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file("bindings.rs")
    //     .expect("Couldn't write bindings!");
}

#[cfg(target_os = "windows")]
fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    println!(r"cargo:rustc-link-search={}/libraries/Win/x64", manifest);
}
