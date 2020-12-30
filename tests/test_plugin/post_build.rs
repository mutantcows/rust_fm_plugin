#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fm_plugin::post_build::bundle_plugin()?;
    Ok(())
}
