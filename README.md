# fm_plugin
### A Rust wrapper for the FileMaker plug-in SDK

`Cargo.toml:`

```toml
[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]

[dependencies]
fm_plugin = "0.1.13"

[build-dependencies]
fm_plugin = "0.1.13"

[package.metadata.cargo-post.dependencies]
fm_plugin = "0.1.13"
toml = "0.5"
serde = { version = "1.0", features = ["derive"] }
```

`config.toml:`

```toml
[filemaker]
ext_path = "/path/to/Extentions"
bin_path = "/Applications/FileMaker Pro.app"
kill = true
launch = true

[plugin]
name = "plugin name"
bundle = true
move_to_ext = true

[code_signing]
sign = true
signtool_path = "/path/to/signtool.exe"
cert_path = "/path/to/cert.p12"
cert_pass = "password"
timestamp_url = "http://cert.timestamp.server.com"

[log]
path = "/path/to/plugin.log"
clear_on_launch = true
```

`build.rs:`

```rust
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fm_plugin::kill_filemaker()?;
    Ok(())
}
```

`post_build.rs:`

```rust
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fm_plugin::post_build::bundle_plugin()?;
    Ok(())
}
```

`lib.rs:`

```rust
use fm_plugin::prelude::*;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn id() -> &'static [u8; 4] {
        &b"MyPl"
    }

    fn name() -> &'static str {
        "MY PLUGIN"
    }

    fn register_functions() -> Vec<Registration> {
        vec![Registration::Function {
            id: 100,
            name: "MyPlugin_MyFunction",
            definition: "MyPlugin_MyFunction( arg1 ; arg2 )",
            description: "Does some really great stuff.",
            min_args: 2,
            max_args: 2,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_ext_version: ExternVersion::V160,
            min_fm_version: "18.0.2",
            allowed_versions: AllowedVersions {developer: true, pro: true, web: true, sase: true, runtime: true},
            function_ptr: Some(MyFunction::extern_func),
            }
        ]
    }

    ...
}

pub struct MyFunction;

impl FileMakerFunction for MyFunction {
    fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        //log some info to the file set in config.toml
        log("some troubleshooting info")?;

        ...

        FMError::NoError
    }
}

register_plugin!(MyPlugin);
```

`cargo install cargo-post`

If you set up the build/post_build scripts as shown above, running `cargo post build --release` will:

1. Quit FileMaker
2. Compile the library
3. Clear the log file
4. Bundle the plug-in
5. Move the plug-in to the FileMaker extensions folder
6. Sign the plug-in
7. Launch FileMaker
