#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_short;
use std::ptr::null_mut;

mod ffi;
mod helpers;
mod wrappers;
use ffi::*;
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

fn prepend_character(txt: &mut Text, insert_buffer: &mut Text, ch: char) {
    let mut tmp = [0; 1];
    let s = ch.encode_utf8(&mut tmp);
    insert_buffer.assign_unicode_with_length(s, 1);
    txt.insert_text(insert_buffer, 0);
}

#[no_mangle]
unsafe extern "C" fn rust_convert_to_base(
    _func_id: c_short,
    _environment: *const fmx_ExprEnv,
    data_vect: *const fmx_DataVect,
    results: *mut fmx_Data,
) -> fmx_errcode {
    let mut error_result: fmx_errcode = 960;
    let mut out_text = Text::new();

    let mut out_locale = (*results).get_locale();
    let mut insert_buffer = Text::new();

    let data_size = (*data_vect).size();

    if data_size >= 2 {
        let data = (*data_vect).at(0);
        let number = data.get_as_number();
        let mut number: fmx_int32 = number.get_as_long();

        let base = (*data_vect).at(1);
        let base = base.get_as_number();
        let base: fmx_int32 = base.get_as_long();

        if base == 2 || base == 3 || base == 8 || base == 12 || base == 16 {
            if number == 0 {
                prepend_character(&mut out_text, &mut insert_buffer, '0');
            } else {
                let neg: bool = number < 0;
                if neg {
                    number = -number;
                }
                while number > 0 {
                    let digit = (number % base) as u8;
                    let ch = if digit < 10 { b'0' } else { b'A' - 10 } + digit;
                    prepend_character(&mut out_text, &mut insert_buffer, ch as char);

                    number /= base;
                }
                if neg {
                    prepend_character(&mut out_text, &mut insert_buffer, '-');
                }
            }
            out_locale = (*data.ptr).get_locale();
            error_result = 0;
        }
    }

    (*results).set_as_text(out_text, out_locale);

    error_result
}

#[no_mangle]
unsafe extern "C" fn rust_execute_sql(
    _func_id: c_short,
    environment: *const fmx_ExprEnv,
    data_vect: *const fmx_DataVect,
    results: *mut fmx_Data,
) -> fmx_errcode {
    let error_result: fmx_errcode = 0;

    let file_name = (*data_vect).at_as_text(0);
    let expression = (*data_vect).at_as_text(1);

    let col_sep = (*data_vect).at_as_text(2);
    let col_sep = match col_sep.size() {
        0 => ',' as u16,
        _ => *col_sep.get_unicode(0, 1).as_ptr(),
    };

    let row_sep = (*data_vect).at_as_text(3);
    let row_sep = match row_sep.size() {
        0 => '\n' as u16,
        _ => *row_sep.get_unicode(0, 1).as_ptr(),
    };

    let mut parameters = DataVect::new();
    let param_count = (*data_vect).size();

    if param_count > 4 {
        for i in 4..param_count {
            let param = (*data_vect).at(i);
            parameters.push(param);
        }
    }

    let mut result = Data::new();

    (*environment).execute_file_sql_text_result(
        expression,
        file_name,
        parameters,
        &mut result,
        col_sep,
        row_sep,
    );

    let r = result.get_as_text();

    let out_locale = (*result.ptr).get_locale();
    (*results).set_as_text(r, out_locale);

    error_result
}

// Do_PluginInit ===========================================================================

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

// Do_PluginShutdown =======================================================================
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

// Do_GetString ============================================================================
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

// FMExternCallProc ========================================================================
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

// Do_PluginPrefs ==========================================================================
fn plugin_prefs() {}

// Do_SessionNotifications =================================================================
fn session_notifications(_session_id: fmx_ptrtype) {}

// Do_FileNotifications ====================================================================
fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype) {}

// Do_PluginIdle ===========================================================================

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
