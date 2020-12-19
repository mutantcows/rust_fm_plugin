# fm_plugin
### A Rust wrapper for the FileMaker plug-in SDK

`Cargo.toml:`

```toml
[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]

[dependencies]
fm_plugin = "0.1.0"

[build-dependencies]
fm_plugin = "0.1.0"

[package.metadata.cargo-post.dependencies]
directories = "*"
toml = "*"
serde = { version = "1.0", features = ["derive"] }
fm_plugin = "0.1.0"
```

`config.toml:`

```toml
[filemaker]
ext_path = "path/to/filemaker/Extensions"
bin_path = "path/to/filemaker/executable.exe"

[plugin]
name = "my_plugin"
```

`build.rs:`

```rust
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fm_plugin::kill_filemaker(env!("CARGO_MANIFEST_DIR"))?;
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
use fm_plugin::{Plugin, register_plugin, log, FMError, ExternalFunction, FileMakerFunction};
use fm_plugin::PluginFlag::*;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn id() -> &'static [u8; 4] {
        &b"MyPl"
    }

    fn name() -> &'static str {
        "MY PLUGIN"
    }

    fn register_functions() -> Vec<ExternalFunction> {
        vec![ExternalFunction {
            id: 100,
            name: "MyPlugin_MyFunction",
            definition: "MyPlugin_MyFunction( arg1 ; arg2 )",
            description: "Does some really great stuff.",
            min_args: 2,
            max_args: 2,
            compatible_flags: DisplayInAllDialogs | FutureCompatible,
            function_ptr: Some(MyFunction::extern_func),
            }
        ]
    }

    ...
}

pub struct MyFunction;

impl FileMakerFunction for MyFunction {
    fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        //log some info to the desktop (plugin.log)
        log("some troublshooting info");

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
3. Bundle the plug-in
4. Clear the log file
5. Move the plug-in to the FileMaker extensions folder
6. Launch FileMaker
