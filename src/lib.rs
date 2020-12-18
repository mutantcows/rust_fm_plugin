#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::str::from_utf8;

pub mod ffi;
mod functions;
pub mod helpers;
mod script_steps;
use ffi::*;
use functions::*;
use helpers::*;

trait Plugin {
    fn id() -> &'static [u8; 4];
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn url() -> &'static str;
    fn enable_configure_button() -> bool {
        false
    }
    fn enable_init_and_shutdown() -> bool {
        true
    }
    fn enable_idle() -> bool {
        false
    }
    fn enable_shutdown() -> bool {
        false
    }
    fn register_functions() -> Vec<ExternalFunction>;

    fn get_string(
        which_string: ExternStringType,
        _win_lang_id: fmx_uint32,
        out_buffer_size: fmx_uint32,
        out_buffer: *mut fmx_unichar16,
    ) {
        use ExternStringType::*;
        let string = match which_string {
            Name => Self::name().to_string(),
            AppConfig => Self::description().to_string(),
            Options => {
                let mut options: String = from_utf8(Self::id()).unwrap().to_string();
                options.push('1');
                options.push(if Self::enable_configure_button() {
                    'Y'
                } else {
                    'n'
                });
                options.push('n');
                options.push(if Self::enable_init_and_shutdown() {
                    'Y'
                } else {
                    'n'
                });
                options.push(if Self::enable_idle() { 'Y' } else { 'n' });
                options.push(if Self::enable_shutdown() { 'Y' } else { 'n' });
                options.push('n');
                options
            }
            HelpUrl => Self::url().to_string(),
            Blank => "".to_string(),
        };
        unsafe { write_to_u16_buff(out_buffer, out_buffer_size, &string) };
    }

    fn initialize(version: ExternVersion) -> u64 {
        let plugin_id = QuadChar::new(Self::id());

        if version < ExternVersion::V160 {
            return ExternVersion::DoNotEnable as u64;
        }

        for f in Self::register_functions() {
            if f.register(&plugin_id) != 0 {
                return ExternVersion::DoNotEnable as u64;
            }
        }
        ExternVersion::V190 as u64
    }

    fn session_notifications(_session_id: fmx_ptrtype);

    fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype);

    fn preferences();

    fn shutdown(version: ExternVersion) {
        let plugin_id = QuadChar::new(Self::id());
        if version >= ExternVersion::V160 {
            for f in Self::register_functions() {
                f.unregister(&plugin_id);
            }
        }
    }

    fn idle_callback(idle_level: fmx_IdleLevel, _session_id: fmx_ptrtype) {
        use IdleType::*;
        match IdleType::from(idle_level) {
            Idle => Self::idle(),
            NotIdle => {}
            ScriptPaused => {}
            ScriptRunning => {}
            Unsafe => {}
        }
    }

    fn idle();
    fn not_idle();
    fn script_paused();
    fn script_running();
    fn un_safe();
}

// struct MyPlugin;

// impl Plugin for MyPlugin {
//     fn id() -> &'static [u8; 4] {
//         &b"RUST"
//     }

//     fn name() -> &'static str {
//         "RUST_PLUGIN"
//     }

//     fn description() -> &'static str {
//         "Great Plugin"
//     }

//     fn url() -> &'static str {
//         "http://wow.com"
//     }

//     fn register_functions() -> Vec<ExternalFunction> {
//         vec![ExternalFunction {
//             id: 100,
//             name: "RUST_ConvertToBase",
//             definition: "RUST_ConvertToBase( number ; base )",
//             description: "Converts the number into a string using the specified base",
//             min_args: 2,
//             max_args: 2,
//             compatible_flags: PluginFlag::DisplayInAllDialogs as u32
//                 | PluginFlag::FutureCompatible as u32,
//             function_ptr: Some(ConvertToBase::extern_func),
//         },
//          ExternalFunction{
//             id: 200,
//             name: "RUST_ExecuteSQL",
//             definition: "RUST_ExecuteSQL( fileName ; sqlQuery { ; arguments... } )",
//             description: "Performs SQL Query",
//             min_args: 2,
//             max_args: -1,
//             compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
//             function_ptr: Some(ExecuteSQL::extern_func),
//         },
//          ExternalFunction{
//             id: 300,
//             name: "RUST_ExecuteSQLTextResult",
//             definition: "RUST_ExecuteSQLTextResult( fileName ; sqlQuery ; fieldSeparator ; rowSeparator { ; arguments... } )",
//             description: "Performs SQL Query",
//             min_args: 4,
//             max_args: -1,
//             compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
//             function_ptr: Some(ExecuteSQLTextResult::extern_func),
//         },
//          ExternalFunction{
//             id: 400,
//             name: "RUST_PDFToJSON",
//             definition: "RUST_PDFToJSON( path )",
//             description: "Converts fields in pdf to JSON object.",
//             min_args: 1,
//             max_args: 1,
//             compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
//             function_ptr: Some(PDFToJSON::extern_func),
//         },
//          ExternalFunction{
//             id: 500,
//             name: "RUST_InsertFile",
//             definition: "RUST_InsertFile( path )",
//             description: "Inserts file into container.",
//             min_args: 1,
//             max_args: 1,
//             compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
//             function_ptr: Some(InsertFile::extern_func),
//         }]
//     }

//     fn session_notifications(_session_id: fmx_ptrtype) {}
//     fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype) {}
//     fn preferences() {}
//     fn idle() {}
//     fn not_idle() {}
//     fn script_paused() {}
//     fn script_running() {}
//     fn un_safe() {}
// }

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
                Init => (*pb).result = $x::initialize((*pb).extnVersion),
                Idle => $x::idle_callback((*pb).parm1, (*pb).parm2),
                Shutdown => $x::shutdown((*pb).extnVersion),
                AppPrefs => $x::preferences(),
                GetString => $x::get_string(
                    (*pb).parm1.into(),
                    (*pb).parm2 as u32,
                    (*pb).parm3 as u32,
                    (*pb).result as *mut u16,
                ),
                SessionShutdown => $x::session_notifications((*pb).parm2),
                FileShutdown => $x::file_notifications((*pb).parm2, (*pb).parm3),
            }
        }
    };
}

// register_plugin!(MyPlugin);
