#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use phf::phf_map;
use std::ptr::null_mut;

pub mod ffi;
mod functions;
pub mod helpers;
mod script_steps;
use ffi::*;
use functions::*;
use helpers::*;

const PLUGIN_ID: &[u8; 4] = b"RUST";
const PLUGIN_NAME: &str = "RustPlugIn";
const PLUGIN_DESCRIPTION: &str = "Small example plug-in compiled with Rust.";
const PLUGIN_URL: &str = "http://httpbin.org/get?id=";

/*
first 4 chars are the plugin id
char 5 always 1
char 6 "Y" or "n" to enable configure button in prefs
char 7 alwast "n"
char 8 "Y" or "n" to enable init and shutdown callbacks
char 9 "Y" or "n" for idle callback
char 10 "Y" or "n" for session/file shutdwon callbacks
char 11 always "n"
*/
const PLUGIN_OPTIONS: &str = "RUST1nnYnnn";

const FUNCTIONS: phf::Map<fmx_int16, ExternalFunction> = phf_map! {
    100i16 => ExternalFunction{
        id: 100,
        name: "RUST_ConvertToBase",
        definition: "RUST_ConvertToBase( number ; base )",
        description: "Converts the number into a string using the specified base",
        min_args: 2,
        max_args:2,
        compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
        function_ptr: Some(ConvertToBase::extern_func),
    },

    200i16 => ExternalFunction{
        id: 200,
        name: "RUST_ExecuteSQL",
        definition: "RUST_ExecuteSQL( fileName ; sqlQuery { ; arguments... } )",
        description: "Performs SQL Query",
        min_args: 2,
        max_args: -1,
        compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
        function_ptr: Some(ExecuteSQL::extern_func),
    },

    300i16 => ExternalFunction{
        id: 300,
        name: "RUST_ExecuteSQLTextResult",
        definition: "RUST_ExecuteSQLTextResult( fileName ; sqlQuery ; fieldSeparator ; rowSeparator { ; arguments... } )",
        description: "Performs SQL Query",
        min_args: 4,
        max_args: -1,
        compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
        function_ptr: Some(ExecuteSQLTextResult::extern_func),
    },

    400i16 => ExternalFunction{
        id: 400,
        name: "RUST_PDFToJSON",
        definition: "RUST_PDFToJSON( path )",
        description: "Converts fields in pdf to JSON object.",
        min_args: 1,
        max_args: 1,
        compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
        function_ptr: Some(PDFToJSON::extern_func),
    },

    500i16 => ExternalFunction{
        id: 500,
        name: "RUST_InsertFile",
        definition: "RUST_InsertFile( path )",
        description: "Inserts file into container.",
        min_args: 1,
        max_args: 1,
        compatible_flags: PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32,
        function_ptr: Some(InsertFile::extern_func),
    },
};

#[no_mangle]
pub static mut gfmx_ExternCallPtr: *mut fmx_ExternCallStruct = null_mut();

#[no_mangle]
unsafe extern "C" fn FMExternCallProc(pb: *mut fmx_ExternCallStruct) {
    // Setup global defined in fmxExtern.h (this will be obsoleted in a later header file)
    gfmx_ExternCallPtr = pb;
    use FMExternCallType::*;

    // Message dispatcher
    match FMExternCallType::from((*pb).whichCall) {
        Init => (*pb).result = plugin_init((*pb).extnVersion),
        Idle => plugin_idle((*pb).parm1, (*pb).parm2),
        Shutdown => plugin_shutdown((*pb).extnVersion),
        AppPrefs => plugin_prefs(),
        GetString => plugin_get_string(
            (*pb).parm1.into(),
            (*pb).parm2 as u32,
            (*pb).parm3 as u32,
            (*pb).result as *mut u16,
        ),
        SessionShutdown => session_notifications((*pb).parm2),
        FileShutdown => file_notifications((*pb).parm2, (*pb).parm3),
    }
}

fn plugin_init(version: ExternVersion) -> u64 {
    let plugin_id = QuadChar::new(PLUGIN_ID);

    if version < ExternVersion::V160 {
        return ExternVersion::DoNotEnable as u64;
    }

    for f in FUNCTIONS.values() {
        if f.register(&plugin_id) != 0 {
            return ExternVersion::DoNotEnable as u64;
        }
    }
    ExternVersion::V190 as u64
}

fn plugin_idle(idle_level: fmx_IdleLevel, _session_id: fmx_ptrtype) {
    use IdleType::*;
    match IdleType::from(idle_level) {
        Idle => {}
        NotIdle => {}
        ScriptPaused => {}
        ScriptRunning => {}
        Unsafe => {}
    }
}

fn plugin_shutdown(version: ExternVersion) {
    let plugin_id = QuadChar::new(PLUGIN_ID);
    if version >= ExternVersion::V160 {
        for f in FUNCTIONS.values() {
            f.unregister(&plugin_id);
        }
    }
}

fn plugin_prefs() {}

fn plugin_get_string(
    which_string: ExternStringType,
    _win_lang_id: fmx_uint32,
    out_buffer_size: fmx_uint32,
    out_buffer: *mut fmx_unichar16,
) {
    use ExternStringType::*;
    let string = match which_string {
        Name => PLUGIN_NAME,
        AppConfig => PLUGIN_DESCRIPTION,
        Options => PLUGIN_OPTIONS,
        HelpUrl => PLUGIN_URL,
        Blank => "",
    };
    unsafe { write_to_u16_buff(out_buffer, out_buffer_size, string) };
}

fn session_notifications(_session_id: fmx_ptrtype) {}

fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype) {}

struct Plugin<T>
where
    T: FileMakerFunction,
{
    name: String,
    functions: Vec<T>,
}
