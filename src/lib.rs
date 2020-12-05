#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ptr::null_mut;

mod ffi;
mod functions;
mod helpers;
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

fn plugin_init(version: fmx_int16) -> u64 {
    let mut extern_version: u64 = ExternVersion::DoNotEnable as u64;
    let plugin_id = QuadChar::new(PLUGIN_ID);

    let flags: fmx_uint32 =
        PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32;

    if version >= ExternVersion::V160 as i16 {
        let convert_to_base_func = ExternalFunction::new(
            100,
            "RUST_ConvertToBase",
            "RUST_ConvertToBase( number ; base )",
            "Converts the number into a string using the specified base",
            2,
            2,
            flags,
            Some(rust_convert_to_base),
        );

        if convert_to_base_func.register(&plugin_id) != 0 {
            return extern_version;
        }

        let execute_sql_func = ExternalFunction::new(
            200,
            "RUST_ExecuteSQL",
            "RUST_ExecuteSQL( fileName ; sqlQuery { ; arguments... } )",
            "Performs SQL Query",
            2,
            -1,
            flags,
            Some(rust_execute_sql),
        );

        if execute_sql_func.register(&plugin_id) != 0 {
            return extern_version;
        }

        let execute_sql_text_result_func = ExternalFunction::new(
            300,
            "RUST_ExecuteSQLTextResult",
            "RUST_ExecuteSQLTextResult( fileName ; sqlQuery ; fieldSeparator ; rowSeparator { ; arguments... } )",
            "Performs SQL Query",
            4,
            -1,
            flags,
            Some(rust_execute_sql_text_result),
        );

        if execute_sql_text_result_func.register(&plugin_id) != 0 {
            return extern_version;
        }

        let pdf_to_json_func = ExternalFunction::new(
            400,
            "RUST_PDFToJSON",
            "RUST_PDFToJSON( path )",
            "Converts fields in pdf to JSON object.",
            1,
            1,
            flags,
            Some(rust_pdf_to_json),
        );

        if pdf_to_json_func.register(&plugin_id) != 0 {
            return extern_version;
        }

        extern_version = ExternVersion::V190 as u64;
    }

    extern_version
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

fn plugin_shutdown(version: fmx_int16) {
    let plugin_id = QuadChar::new(PLUGIN_ID).ptr as *const fmx_QuadChar;
    if version >= 57 {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_ExprEnv_UnRegisterExternalFunction(plugin_id, 100, &mut _x) };
        _x.check();
        unsafe { FM_ExprEnv_UnRegisterExternalFunction(plugin_id, 200, &mut _x) };
        _x.check();
    }
}

fn plugin_prefs() {}

fn plugin_get_string(
    which_string: fmx_uint32,
    _win_lang_id: fmx_uint32,
    out_buffer_size: fmx_uint32,
    out_buffer: *mut fmx_unichar16,
) {
    use ExternStringType::*;
    let string = match ExternStringType::from(which_string) {
        Name => PLUGIN_NAME,
        AppConfig => PLUGIN_DESCRIPTION,
        Options => PLUGIN_OPTIONS,
        HelpUrl => PLUGIN_URL,
        Blank => "",
    };
    write_to_u16_buff(out_buffer, out_buffer_size, string);
}

fn session_notifications(_session_id: fmx_ptrtype) {}

fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype) {}
