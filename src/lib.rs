#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_longlong, c_short, c_uchar, c_uint, c_ulonglong, c_ushort};
use std::ptr::null_mut;
use widestring::WideCString;

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

pub type fmx_uint16 = c_ushort;
pub type fmx_int16 = c_short;
pub type fmx_uchar = c_uchar;
pub type fmx_int32 = c_int;
pub type fmx_uint32 = c_uint;
pub type fmx_int64 = c_longlong;
pub type fmx_uint64 = c_ulonglong;
pub type fmx_ptrtype = fmx_uint64;
pub type fmx_unusedid = fmx_int16;
pub type fmx_errcode = c_short;
pub type fmx_unichar16 = c_ushort;
pub type fmx_unichar = fmx_unichar16;
pub type fmx_boolean = c_char;
pub type FMX_ExternCallSwitch = fmx_uchar;
pub type FMX_ScriptControl = fmx_uchar;
pub type FMX_IdleLevel = fmx_uchar;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_ExprEnv {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Locale {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_DataVect {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Data {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Text {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_FixPt {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx__fmxcpt {
    pub m_vers: fmx_int32,
    pub m_code: fmx_int32,
}

impl fmx__fmxcpt {
    fn check(&self) {
        if self.m_code != 0 {
            panic!();
        }
    }

    fn new() -> Self {
        Self {
            m_code: 0,
            m_vers: 1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_unique_ptr {
    pub _Mypair: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_TextUniquePtr {
    pub _base: std_unique_ptr,
}

#[repr(C)]
#[derive(Debug)]
pub struct fmx_QuadCharUniquePtr {
    pub _base: std_unique_ptr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_QuadChar {
    pub _address: u8,
}

pub type fmx_ExtPluginType = Option<
    unsafe extern "C" fn(
        functionId: c_short,
        env: *const fmx_ExprEnv,
        parms: *const fmx_DataVect,
        result: *mut fmx_Data,
    ) -> fmx_errcode,
>;

pub type FMX_StartScriptCall = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const fmx_Text,
        scriptName: *const fmx_Text,
        control: FMX_ScriptControl,
        parameter: *const fmx_Data,
    ) -> fmx_errcode,
>;

pub type FMX_ExternCallPtr = *mut FMX_ExternCallStruct;
pub type FMX_ExternCallProc = Option<unsafe extern "C" fn(arg1: FMX_ExternCallPtr)>;
pub type FMX_CurrentEnvCall = Option<unsafe extern "C" fn(env: *mut fmx_ExprEnv) -> fmx_errcode>;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FMX_ExternCallStruct {
    pub extnVersion: fmx_int16,
    pub unusedID: fmx_unusedid,
    pub entryPoint: FMX_ExternCallProc,
    pub cfmCalls: fmx_boolean,
    pub whichCall: FMX_ExternCallSwitch,
    pub unsafeCalls: fmx_boolean,
    pub parm1: fmx_uchar,
    pub parm2: fmx_ptrtype,
    pub parm3: fmx_ptrtype,
    pub instanceID: fmx_ptrtype,
    pub result: fmx_ptrtype,
    pub unused: fmx_ptrtype,
    pub cStartScript: FMX_StartScriptCall,
    pub cCurrentEnv: FMX_CurrentEnvCall,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {
    fn FM_Text_AssignUnicodeWithLength(
        _self: *mut fmx_Text,
        s: *const fmx_uint16,
        strlength: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_Text_InsertText(
        _self: *mut fmx_Text,
        other: *const fmx_Text,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_Data_GetLocale(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_Locale;

    pub fn FM_DataVect_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_DataVect;

    pub fn FM_DataVect_Size(_self: *const fmx_DataVect, _x: *mut fmx__fmxcpt) -> fmx_uint32;

    pub fn FM_DataVect_At(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_Data;

    pub fn FM_Data_GetAsNumber(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_FixPt;

    pub fn FM_FixPt_AsLong(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> fmx_int32;

    pub fn FM_Text_AssignWide(_self: *mut fmx_Text, s: *const u16, _x: *mut fmx__fmxcpt);

    pub fn FM_Data_SetAsText(
        _self: *mut fmx_Data,
        textData: *const fmx_Text,
        sourceLocale: *const fmx_Locale,
        nativeType: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_Text_Assign(
        _self: *mut fmx_Text,
        s: *const c_char,
        encoding: fmx_int32,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_Text_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_Text;
    pub fn FM_Data_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_Data;

    pub fn FM_ExprEnv_RegisterScriptStep(
        pluginId: *const fmx_QuadChar,
        scriptStepId: c_short,
        scriptStepName: *const fmx_Text,
        scriptStepDefinition: *const fmx_Text,
        scriptStepDescription: *const fmx_Text,
        compatibleOnFlags: fmx_uint32,
        funcPtr: fmx_ExtPluginType,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_ExprEnv_RegisterExternalFunctionEx(
        pluginId: *const fmx_QuadChar,
        functionId: c_short,
        functionName: *const fmx_Text,
        functionPrototype: *const fmx_Text,
        functionDescription: *const fmx_Text,
        minArgs: c_short,
        maxArgs: c_short,
        compatibleOnFlags: fmx_uint32,
        funcPtr: fmx_ExtPluginType,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_ExprEnv_UnRegisterExternalFunction(
        pluginId: *const fmx_QuadChar,
        functionId: c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_Text_GetSize(_self: *const fmx_Text, _x: *mut fmx__fmxcpt) -> fmx_uint32;

    pub fn FM_Text_GetUnicode(
        _self: *const fmx_Text,
        s: *mut fmx_uint16,
        position: fmx_uint32,
        size: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_QuadChar_Constructor2(
        c0: c_char,
        c1: c_char,
        c2: c_char,
        c3: c_char,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_QuadChar;

}

struct QuadChar {
    ptr: *mut fmx_QuadChar,
}

impl QuadChar {
    fn new(bytes: &[u8; 4]) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let b: &[i8; 4] = unsafe { &*(bytes as *const [u8; 4] as *const [i8; 4]) };
        let ptr = unsafe { FM_QuadChar_Constructor2(b[0], b[1], b[2], b[3], &mut _x) };
        _x.check();
        Self { ptr }
    }
}

struct Text {
    ptr: *mut fmx_Text,
}

impl Text {
    fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Text_Constructor1(&mut _x) };
        _x.check();
        Self { ptr }
    }

    fn assign(&mut self, s: &str) {
        let c_string: CString = CString::new(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_Assign(self.ptr, c_string.as_ptr(), 1, &mut _x) };
        _x.check();
    }

    fn assign_unicode_with_length(&mut self, s: &str, len: u32) {
        let c_string = WideCString::from_str(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AssignUnicodeWithLength(self.ptr, c_string.as_ptr(), len, &mut _x) };
        _x.check();
    }

    fn assign_wide(&mut self, s: &str) {
        let c_string = WideCString::from_str(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AssignWide(self.ptr, c_string.as_ptr(), &mut _x) };
        _x.check();
    }

    fn insert_text(&mut self, s: &Text, pos: u32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_InsertText(self.ptr, s.ptr, pos, &mut _x) };
        _x.check();
    }
}

struct FixPt {
    ptr: *const fmx_FixPt,
}

impl FixPt {
    fn from_ptr(ptr: *const fmx_FixPt) -> Self {
        Self { ptr }
    }

    fn get_as_long(&self) -> fmx_int32 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong(self.ptr, &mut _x) };
        _x.check();
        num
    }
}

struct Locale {
    ptr: *const fmx_Locale,
}

impl Locale {
    fn from_ptr(ptr: *const fmx_Locale) -> Self {
        Self { ptr }
    }
}
struct Data {
    ptr: *mut fmx_Data,
}

impl Data {
    fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_Constructor1(&mut _x) };
        _x.check();
        Self { ptr }
    }

    fn from_ptr(ptr: *const fmx_Data) -> Self {
        Self {
            ptr: ptr as *mut fmx_Data,
        }
    }

    fn get_as_number(&mut self) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetAsNumber(self.ptr, &mut _x) };
        _x.check();
        FixPt::from_ptr(ptr)
    }
}

impl fmx_Data {
    fn get_locale(&self) -> Locale {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetLocale(self, &mut _x) };
        _x.check();
        Locale::from_ptr(ptr)
    }

    fn set_as_text(&mut self, text: Text, locale: Locale) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsText(self, text.ptr, locale.ptr, 1, &mut _x) };
        _x.check();
    }
}

impl fmx_DataVect {
    fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_DataVect_Size(self, &mut _x) };
        _x.check();
        size
    }

    fn at(&self, position: fmx_uint32) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let data_ptr = unsafe { FM_DataVect_At(self, position, &mut _x) };
        _x.check();
        Data::from_ptr(data_ptr)
    }
}

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
    let mut error_result: fmx_errcode = 1552;
    let mut out_text = Text::new();

    let mut out_locale = (*results).get_locale();
    let mut insert_buffer = Text::new();

    let data_size = (*data_vect).size();

    if data_size >= 2 {
        let mut data: Data = (*data_vect).at(0);
        let number: FixPt = data.get_as_number();
        let mut number: fmx_int32 = number.get_as_long();

        let mut base: Data = (*data_vect).at(1);
        let base: FixPt = base.get_as_number();
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

enum PluginFlag {
    DisplayInAllDialogs = 0b1111111100000000,
    MacCompatible = 0b0000000000000010,
    WinCompatible = 0b0000000000000100,
    ServerCompatible = 0b0000000000001000,
    IOSCompatible = 0b0000000000010000,
    CustomWebCompatible = 0b0000000000100000,
    WebDirectCompatible = 0b0000000001000000,
    AllDeviceCompatible = 0b0000000001111110,
    FutureCompatible = 0b111111110000000000000000,
}

enum SDKVersion {
    BadExtn = -1,
    DoNotEnable = -2,
    V40 = 11,
    V41 = 12,
    V50 = 14,
    V60 = 17,
    V70 = 50, // Jumping to 50
    V80 = 51,
    V110 = 52,
    V120 = 53, // Support for 64-bit plugins
    V130 = 54,
    V140 = 55,
    V150 = 56,
    V160 = 57,
    V170 = 59,
    V180 = 60,
    V190 = 62,
    Min = 4,
    Max = 255,
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
        } == 0
        {
            _x.check();
            sdk_version = SDKVersion::V190 as u64;
        }
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
    }
}

enum GetStringType {
    Name,
    AppConfig,
    Options,
    HelpUrl,
    Blank,
}

impl From<u32> for GetStringType {
    fn from(num: u32) -> Self {
        match num {
            128 => Self::Name,
            129 => Self::AppConfig,
            131 => Self::Options,
            132 => Self::HelpUrl,
            _ => Self::Blank,
        }
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

impl From<u8> for FMExternCallType {
    fn from(num: u8) -> Self {
        match num {
            0 => Self::Init,
            1 => Self::Idle,
            4 => Self::Shutdown,
            5 => Self::AppPrefs,
            7 => Self::GetString,
            8 => Self::SessionShutdown,
            9 => Self::FileShutdown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum FMExternCallType {
    Init,
    Idle,
    Shutdown,
    AppPrefs,
    GetString,
    SessionShutdown,
    FileShutdown,
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
enum IdleType {
    Idle,
    NotIdle,
    ScriptPaused,
    ScriptRunning,
    Unsafe,
}

impl From<u8> for IdleType {
    fn from(num: u8) -> Self {
        match num {
            0 => Self::Idle,
            1 => Self::NotIdle,
            2 => Self::ScriptPaused,
            3 => Self::ScriptRunning,
            4 => Self::Unsafe,
            _ => unreachable!(),
        }
    }
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

macro_rules! ok_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}

macro_rules! some_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

fn write_to_file(content: &str) {
    use directories::UserDirs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::Path;

    let user_dirs = some_or_return!(UserDirs::new());
    let dir = some_or_return!(user_dirs.desktop_dir());
    let path = Path::join(&dir, "plugin.log");

    let mut file = ok_or_return!(OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path));

    ok_or_return!(file.write_all(content.as_bytes()));
}

fn write_to_u16_buff(buffer: *mut c_ushort, buffer_size: c_uint, s: &str) {
    let c_string = WideCString::from_str(s).unwrap();
    let bytes = c_string.as_slice();

    let string_bytes = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_size as usize) };
    string_bytes[..bytes.len()].copy_from_slice(bytes);
}

fn write_to_i8_buff(buffer: *mut c_char, buffer_size: c_uint, s: &str) {
    let c_string = CString::new(s).unwrap();
    let bytes = c_string.as_bytes_with_nul();
    let bytes = unsafe { &*(bytes as *const [u8] as *const [i8]) };
    let string_bytes = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_size as usize) };
    string_bytes[..bytes.len()].copy_from_slice(bytes);
}
