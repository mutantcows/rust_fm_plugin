#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ptr::null_mut;

mod ffi;
mod functions;
mod helpers;
mod script_steps;
mod wrappers;
use ffi::*;
use functions::*;
use helpers::*;
use wrappers::*;

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
pub static mut gFMX_ExternCallPtr: FMX_ExternCallPtr = null_mut();

#[no_mangle]
unsafe extern "C" fn FMExternCallProc(pb: FMX_ExternCallPtr) {
    // Setup global defined in FMXExtern.h (this will be obsoleted in a later header file)
    gFMX_ExternCallPtr = pb;
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
    let mut sdk_version: u64 = SDKVersion::DoNotEnable as u64;
    let plugin_id = QuadChar::new(PLUGIN_ID);

    let flags: fmx_uint32 =
        PluginFlag::DisplayInAllDialogs as u32 | PluginFlag::FutureCompatible as u32;

    let mut _x = fmx__fmxcpt::new();

    if version >= SDKVersion::V160 as i16 {
        let mut name: Text = Text::new();
        name.assign("RUST_ConvertToBase");

        let mut desc = Text::new();
        desc.assign("Converts the number into a string using the specified base");

        let mut def = Text::new();
        def.assign("RUST_ConvertToBase( number ; base )");

        if unsafe {
            FM_ExprEnv_RegisterExternalFunctionEx(
                plugin_id.ptr,
                100,
                name.ptr,
                def.ptr,
                desc.ptr,
                2,
                2,
                flags,
                Some(rust_convert_to_base),
                &mut _x,
            )
        } != 0
        {
            return sdk_version;
        }

        _x.check();

        let mut name: Text = Text::new();
        name.assign("RUST_ExecuteSQL");

        let mut desc = Text::new();
        desc.assign("Performs SQL Query");

        let mut def = Text::new();
        def.assign("RUST_ExecuteSQL( fileName ; sqlQuery ; fieldSeparator ; rowSeparator { ; arguments... } )");

        if unsafe {
            FM_ExprEnv_RegisterExternalFunctionEx(
                plugin_id.ptr,
                200,
                name.ptr,
                def.ptr,
                desc.ptr,
                4,
                -1,
                flags,
                Some(rust_execute_sql),
                &mut _x,
            )
        } != 0
        {
            return sdk_version;
        }

        _x.check();
        sdk_version = SDKVersion::V190 as u64;
    }

    sdk_version
}

fn plugin_idle(idle_level: FMX_IdleLevel, _session_id: fmx_ptrtype) {
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
    use GetStringType::*;
    let string = match GetStringType::from(which_string) {
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
