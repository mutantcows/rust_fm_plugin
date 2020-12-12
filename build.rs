#[cfg(target_os = "macos")]
fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    println!(
        r"cargo:rustc-link-search=framework={}/libraries/Mac",
        manifest
    );
}

#[cfg(target_os = "windows")]
fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    println!(r"cargo:rustc-link-search={}/libraries/Win/x64", manifest);
}
