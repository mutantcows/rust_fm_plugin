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
//! pub struct MyFunction;
//!
//! impl FileMakerFunction for MyFunction {
//!     fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
//!         //log some info to the desktop (plugin.log)
//!         log("some troubleshooting info");
//!
//!         ...
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
//!
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn id() -> &'static [u8; 4] {
//!         &b"MyPl"
//!     }
//!
//!     fn name() -> &'static str {
//!         "MY PLUGIN"
//!     }
//!
//!     fn register_functions() -> Vec<ExternalFunction> {
//!         vec![ExternalFunction {
//!             id: 100,
//!             name: "MyPlugin_MyFunction",
//!             definition: "MyPlugin_MyFunction( arg1 ; arg2 )",
//!             description: "Does some really great stuff.",
//!             min_args: 2,
//!             max_args: 2,
//!             compatible_flags: DisplayInAllDialogs | FutureCompatible,
//!             min_version: ExternVersion::V160,
//!             function_ptr: Some(MyFunction::extern_func),
//!             }
//!         ]
//!     }
//!     ...
//! }
//! ```
//! Lastly you'll need to register the plug-in.
//! ```rust
//! register_plugin!(MyPlugin);
//! ```
//! [`Plugin`]: trait.Plugin.html
//! [`FileMakerFunction`]: ffi/calc_engine/trait.FileMakerFunction.html

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod config;
pub mod ffi;
pub mod helpers;
pub mod post_build;
pub use config::kill_filemaker;
pub use ffi::*;
pub use helpers::{log, write_to_u16_buff};

pub mod prelude {
    //! Re-exports everything necessary for the register_plugin macro.
    pub use crate::PluginFlag::*;
    pub use crate::{
        fmx_ExternCallStruct, fmx_ptrtype, register_plugin, write_to_u16_buff, ExternStringType,
        ExternVersion, FMError, FMExternCallType, FileMakerFunction, IdleType, Plugin, QuadChar,
        Registration,
    };
}

/// Implement this trait for your plugin struct. The different functions are used to give FileMaker information about the plugin. You also need to register all your functions/script steps in the trait implementation.
///
/// # Example
/// ```rust
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn id() -> &'static [u8; 4] {
///         &b"MyPl"
///     }
///
///     fn name() -> &'static str {
///         "MY PLUGIN"
///     }
///
///     fn register_functions() -> Vec<ExternalFunction> {
///         vec![ExternalFunction {
///             id: 100,
///             name: "MyPlugin_MyFunction",
///             definition: "MyPlugin_MyFunction( arg1 ; arg2 )",
///             description: "Does some really great stuff.",
///             min_args: 2,
///             max_args: 2,
///             compatible_flags: DisplayInAllDialogs | FutureCompatible,
///             min_version: ExternVersion::V160,
///             function_ptr: Some(MyFunction::extern_func),
///             }
///         ]
///     }
///     ...
/// }
/// ```
pub trait Plugin {
    /// Unique 4 letter identifier for the plug-in.
    fn id() -> &'static [u8; 4];
    /// Plug-in's name.
    fn name() -> &'static str;
    /// Description of the plug-in.
    fn description() -> &'static str;
    /// Url to send users to from the help in FileMaker. The function's name that the user  will be appended to the url when clicked.
    fn url() -> &'static str;

    /// Register all custom functions/script steps
    fn register_functions() -> Vec<Registration>;

    /// Defaults to false
    fn enable_configure_button() -> bool {
        false
    }
    /// Defaults to true
    fn enable_init_and_shutdown() -> bool {
        true
    }
    /// Defaults to false
    fn enable_idle() -> bool {
        false
    }
    /// Defaults to false
    fn enable_file_and_session_shutdown() -> bool {
        false
    }

    fn session_shutdown(_session_id: fmx_ptrtype) {}
    fn file_shutdown(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype) {}
    fn preferences() {}
    fn idle(_session_id: fmx_ptrtype) {}
    fn not_idle(_session_id: fmx_ptrtype) {}
    fn script_paused(_session_id: fmx_ptrtype) {}
    fn script_running(_session_id: fmx_ptrtype) {}
    fn un_safe(_session_id: fmx_ptrtype) {}
}

/// Sets up the entry point for every FileMaker call into the plug-in. The function then dispatches the calls to the various trait functions you can implement.
/// Impl [`Plugin`][Plugin] for your plugin struct, and then call the macro on it.
///
/// # Example
/// ```rust
/// use fm_plugin::prelude::*;
///
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin { ... }
///
/// register_plugin!(MyPlugin);
/// ```
///
/// # Macro Contents
///```rust
/// #[no_mangle]
/// pub static mut gfmx_ExternCallPtr: *mut fmx_ExternCallStruct = std::ptr::null_mut();
///
/// #[no_mangle]
/// unsafe extern "C" fn FMExternCallProc(pb: *mut fmx_ExternCallStruct) {
///     // Setup global defined in fmxExtern.h (this will be obsoleted in a later header file)
///     gfmx_ExternCallPtr = pb;
///     use FMExternCallType::*;
///
///     // Message dispatcher
///     match FMExternCallType::from((*pb).whichCall) {
///         Init => (*pb).result = initialize((*pb).extnVersion) as u64,
///         Idle => {
///             use IdleType::*;
///             match IdleType::from((*pb).parm1) {
///                 Idle => $x::idle((*pb).parm2),
///                 NotIdle => $x::not_idle((*pb).parm2),
///                 ScriptPaused => $x::script_paused((*pb).parm2),
///                 ScriptRunning => $x::script_running((*pb).parm2),
///                 Unsafe => $x::un_safe((*pb).parm2),
///             }
///         }
///         Shutdown => shutdown((*pb).extnVersion),
///         AppPrefs => $x::preferences(),
///         GetString => get_string(
///             (*pb).parm1.into(),
///             (*pb).parm2 as u32,
///             (*pb).parm3 as u32,
///             (*pb).result as *mut u16,
///         ),
///         SessionShutdown => $x::session_shutdown((*pb).parm2),
///         FileShutdown => $x::file_shutdown((*pb).parm2, (*pb).parm3),
///     }
/// }
///
/// fn get_string(
///     which_string: ExternStringType,
///     _win_lang_id: u32,
///     out_buffer_size: u32,
///     out_buffer: *mut u16,
/// ) {
///     use ExternStringType::*;
///     let string = match which_string {
///         Name => $x::name().to_string(),
///         AppConfig => $x::description().to_string(),
///         Options => {
///             let mut options: String = ::std::str::from_utf8($x::id()).unwrap().to_string();
///             options.push('1');
///             options.push(if $x::enable_configure_button() {
///                 'Y'
///             } else {
///                 'n'
///             });
///             options.push('n');
///             options.push(if $x::enable_init_and_shutdown() {
///                 'Y'
///             } else {
///                 'n'
///             });
///             options.push(if $x::enable_idle() { 'Y' } else { 'n' });
///             options.push(if $x::enable_file_and_session_shutdown() {
///                 'Y'
///             } else {
///                 'n'
///             });
///             options.push('n');
///             options
///         }
///         HelpUrl => $x::url().to_string(),
///         Blank => "".to_string(),
///     };
///     unsafe { write_to_u16_buff(out_buffer, out_buffer_size, &string) }
/// }
///
/// fn initialize(version: ExternVersion) -> ExternVersion {
///     let plugin_id = QuadChar::new($x::id());
///     for f in $x::register_functions() {
///         if version < f.min_version() {
///             continue;
///         }
///         if f.register(&plugin_id) != FMError::NoError {
///             return ExternVersion::DoNotEnable;
///         }
///     }
///     ExternVersion::V190
/// }
///
/// fn shutdown(version: ExternVersion) {
///     let plugin_id = QuadChar::new($x::id());
///     for f in $x::register_functions() {
///         if version < f.min_version() {
///             continue;
///         }
///         f.unregister(&plugin_id);
///     }
/// }
/// ```
#[macro_export]
macro_rules! register_plugin {
    ($x:ident) => {
        #[no_mangle]
        pub static mut gfmx_ExternCallPtr: *mut fmx_ExternCallStruct = std::ptr::null_mut();

        #[no_mangle]
        unsafe extern "C" fn FMExternCallProc(pb: *mut fmx_ExternCallStruct) {
            // Setup global defined in fmxExtern.h (this will be obsoleted in a later header file)
            gfmx_ExternCallPtr = pb;
            use FMExternCallType::*;

            // Message dispatcher
            match FMExternCallType::from((*pb).whichCall) {
                Init => (*pb).result = initialize((*pb).extnVersion) as u64,
                Idle => {
                    use IdleType::*;
                    match IdleType::from((*pb).parm1) {
                        Idle => $x::idle((*pb).parm2),
                        NotIdle => $x::not_idle((*pb).parm2),
                        ScriptPaused => $x::script_paused((*pb).parm2),
                        ScriptRunning => $x::script_running((*pb).parm2),
                        Unsafe => $x::un_safe((*pb).parm2),
                    }
                }
                Shutdown => shutdown((*pb).extnVersion),
                AppPrefs => $x::preferences(),
                GetString => get_string(
                    (*pb).parm1.into(),
                    (*pb).parm2 as u32,
                    (*pb).parm3 as u32,
                    (*pb).result as *mut u16,
                ),
                SessionShutdown => $x::session_shutdown((*pb).parm2),
                FileShutdown => $x::file_shutdown((*pb).parm2, (*pb).parm3),
            }
        }

        fn get_string(
            which_string: ExternStringType,
            _win_lang_id: u32,
            out_buffer_size: u32,
            out_buffer: *mut u16,
        ) {
            use ExternStringType::*;
            let string = match which_string {
                Name => $x::name().to_string(),
                AppConfig => $x::description().to_string(),
                Options => {
                    let mut options: String = ::std::str::from_utf8($x::id()).unwrap().to_string();
                    options.push('1');
                    options.push(if $x::enable_configure_button() {
                        'Y'
                    } else {
                        'n'
                    });
                    options.push('n');
                    options.push(if $x::enable_init_and_shutdown() {
                        'Y'
                    } else {
                        'n'
                    });
                    options.push(if $x::enable_idle() { 'Y' } else { 'n' });
                    options.push(if $x::enable_file_and_session_shutdown() {
                        'Y'
                    } else {
                        'n'
                    });
                    options.push('n');
                    options
                }
                HelpUrl => $x::url().to_string(),
                Blank => "".to_string(),
            };
            unsafe { write_to_u16_buff(out_buffer, out_buffer_size, &string) }
        }

        fn initialize(version: ExternVersion) -> ExternVersion {
            let plugin_id = QuadChar::new($x::id());
            for f in $x::register_functions() {
                if version < f.min_version() {
                    continue;
                }
                if f.register(&plugin_id) != FMError::NoError {
                    return ExternVersion::DoNotEnable;
                }
            }
            ExternVersion::V190
        }

        fn shutdown(version: ExternVersion) {
            let plugin_id = QuadChar::new($x::id());
            for f in $x::register_functions() {
                if version < f.min_version() {
                    continue;
                }
                f.unregister(&plugin_id);
            }
        }
    };
}
