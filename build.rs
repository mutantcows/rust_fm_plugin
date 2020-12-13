use std::env;
use std::error::Error;
use std::path::Path;
use std::process;
mod config;
use config::{read_config, BuildError, Config};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let config = read_config(Path::new(manifest)).unwrap();

    if env::var("PROFILE").unwrap() == "release" {
        kill_filemaker(&config)?;
    }

    if cfg!(target_os = "windows") {
        println!(r"cargo:rustc-link-search={}/libraries/Win/x64", manifest);
    } else if cfg!(target_os = "macos") {
        println!(
            r"cargo:rustc-link-search=framework={}/libraries/Mac",
            manifest
        );
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn kill_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    let app_path = Path::new(&config.filemaker.bin_path);
    let app = app_path.file_name().ok_or(BuildError::FileMaker)?;
    process::Command::new("taskkill")
        .arg("/IM")
        .arg(app)
        .arg("/F")
        .spawn()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn kill_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    let app_path = Path::new(&config.filemaker.bin_path);
    let app = app_path.file_stem().ok_or(BuildError::FileMaker)?;
    process::Command::new("pkill").arg(app).spawn().ok();
    Ok(())
}

#[allow(dead_code)]
fn run_bindgen() {
    let bindings = bindgen::Builder::default()
        .clang_arg("--language=c++")
        .clang_arg("-std=c++14")
        .header("headers/FMWrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
