//! A wrapper around the FileMaker plug-in SDK.
//!
//! Replicates much of the functionality found in the C++ library provided by FileMaker, which is mostly wrapping the C ffi, as well as some convenience functions.
//!
//! Has only been tested with FileMaker 18 and 19 (windows, macos, and linux); your mileage may vary with older versions.
//!
//! # Quick Start
//!
//! You'll want to make your project a library with a crate-type of `cdylib`.
//!
//! ```toml
//! [lib]
//! path = "src/lib.rs"
//! crate-type = ["cdylib"]
//! ```
//!
//! Each custom function/script step must be configured in a [`FileMakerFunction`] implementation.
//!
//! ```rust
//! # use fm_plugin::prelude::*;
//! # use fm_plugin::{ExprEnv, DataVect, Data, log};
//! pub struct MyFunction;
//!
//! impl FileMakerFunction for MyFunction {
//!     fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
//!         //log some info to the desktop (plugin.log)
//!         log("some troubleshooting info");
//!
//!         FMError::NoError
//!     }
//! }
//! ```
//!
//! Next you'll need to implement [`Plugin`] for your plugin's struct, defining all the information about the plug-in, as well as registering all the functions.
//!
//! ```rust
//! use fm_plugin::prelude::*;
//! # use fm_plugin::{ExprEnv, DataVect, Data, FMError};
//! # struct MyFunction;
//! # impl FileMakerFunction for MyFunction {
//! # fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
//! #     FMError::NoError
//! # }
//! # }
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn id() -> &'static [u8; 4] { &b"MyPl" }
//!     fn name() -> &'static str { "MY PLUGIN" }
//!     fn description() -> &'static str { "Does all sorts of great things." }
//!     fn url() -> &'static str { "http://myplugin.com" }
//!
//!     fn register_functions() -> Vec<Registration> {
//!         vec![Registration::Function {
//!             id: 100,
//!             name: "MyPlugin_MyFunction",
//!             definition: "MyPlugin_MyFunction( arg1 ; arg2 )",
//!             description: "Does some really great stuff.",
//!             min_args: 2,
//!             max_args: 2,
//!             display_in_dialogs: true,
//!             compatibility_flags: Compatibility::Future as u32,
//!             min_version: ExternVersion::V160,
//!             function_ptr: Some(MyFunction::extern_func),
//!             }
//!         ]
//!     }
//! }
//! ```
//! Lastly you'll need to register the plug-in.
//! ```rust
//! # use fm_plugin::prelude::*;
//! # struct MyPlugin;
//! # impl Plugin for MyPlugin {
//! #    fn id() -> &'static [u8; 4] { &b"MyPl" }
//! #    fn name() -> &'static str { "MY PLUGIN" }
//! #    fn description() -> &'static str { "Does all sorts of great things." }
//! #    fn url() -> &'static str { "http://myplugin.com" }
//! #    fn register_functions() -> Vec<Registration> { Vec::new() }
//! # }
//! register_plugin!(MyPlugin);
//! ```
//! [`Plugin`]: trait.Plugin.html
//! [`FileMakerFunction`]: ffi/calc_engine/trait.FileMakerFunction.html

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod config;
pub mod ffi;
pub mod helpers;
pub mod plugin;
pub mod post_build;
#[doc(inline)]
pub use config::kill_filemaker;
#[doc(inline)]
pub use ffi::*;
#[doc(inline)]
pub use helpers::{log, write_to_u16_buff};
#[doc(inline)]
pub use plugin::*;

pub mod prelude {
    //! Re-exports everything necessary for the register_plugin macro.
    pub use crate::{
        fmx_ExternCallStruct, fmx_ptrtype, register_plugin, write_to_u16_buff, Compatibility, Data,
        ExternStringType, ExternVersion, FMError, FMExternCallType, FileMakerFunction, IdleType,
        Plugin, PluginInternal, QuadChar, Registration, ScriptControl, Text,
    };
    pub use lazy_static::lazy_static;
    pub use std::collections::HashMap;
    pub use std::sync::RwLock;
}
