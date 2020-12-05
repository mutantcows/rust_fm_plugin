use super::*;
use std::os::raw::{c_char, c_int, c_longlong, c_short, c_uchar, c_uint, c_ulonglong, c_ushort};

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
pub type fmx_ExternCallSwitch = fmx_uchar;
pub type fmx_ScriptControl = fmx_uchar;
pub type fmx_IdleLevel = fmx_uchar;
pub type fmx_ExternCallProc = Option<unsafe extern "C" fn(arg1: *mut fmx_ExternCallStruct)>;
pub type fmx_CurrentEnvCall = Option<unsafe extern "C" fn(env: *mut fmx_ExprEnv) -> fmx_errcode>;
pub type fmx_CharacterStyle_Face = c_ushort;
pub type fmx_CharacterStyle_FontID = c_ushort;
pub type fmx_CharacterStyle_FontSize = c_ushort;
pub type fmx_CharacterStyle_ColorChannel = c_uchar;
pub type fmx_CharacterStyle_ColorChannel16 = c_ushort;
pub type fmx_CharacterStyle_FontScript = c_ushort;
pub type fmx_ExtPluginType = Option<
    unsafe extern "C" fn(
        functionId: c_short,
        env: *const fmx_ExprEnv,
        parms: *const fmx_DataVect,
        result: *mut fmx_Data,
    ) -> fmx_errcode,
>;
pub type fmx_StartScriptCall = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const fmx_Text,
        scriptName: *const fmx_Text,
        control: fmx_ScriptControl,
        parameter: *const fmx_Data,
    ) -> fmx_errcode,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Locale {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_QuadChar {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {

    pub fn FM_QuadChar_Constructor2(
        c0: c_char,
        c1: c_char,
        c2: c_char,
        c3: c_char,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_QuadChar;

    pub fn FM_QuadChar_Delete(_self: *mut fmx_QuadChar, _x: *mut fmx__fmxcpt);

    pub fn FM_Locale_Delete(_self: *mut fmx_Locale, _x: *mut fmx__fmxcpt);
    pub fn FM_Locale_Constructor1(inputType: fmx_int32, _x: *mut fmx__fmxcpt) -> *mut fmx_Locale;

}

pub(crate) struct Locale {
    pub(crate) ptr: *mut fmx_Locale,
    drop: bool,
}

impl Locale {
    pub(crate) fn new(input_type: fmx_int32) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Locale_Constructor1(input_type, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_Locale) -> Self {
        Self {
            ptr: ptr as *mut fmx_Locale,
            drop: false,
        }
    }
}

impl Drop for Locale {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_Locale_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

pub(crate) struct QuadChar {
    pub(crate) ptr: *mut fmx_QuadChar,
    drop: bool,
}

impl QuadChar {
    pub(crate) fn new(bytes: &[u8; 4]) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let b: &[i8; 4] = unsafe { &*(bytes as *const [u8; 4] as *const [i8; 4]) };
        let ptr = unsafe { FM_QuadChar_Constructor2(b[0], b[1], b[2], b[3], &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
}

impl Drop for QuadChar {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_QuadChar_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}
