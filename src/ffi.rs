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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_RowVect {
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
    pub fn FM_Text_AssignUnicodeWithLength(
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

    pub fn FM_DataVect_AtAsText(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_Text;

    pub fn FM_DataVect_AtAsNumber(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_FixPt;

    pub fn FM_DataVect_PushBack(
        _self: *mut fmx_DataVect,
        data: *const fmx_Data,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_Data_GetAsNumber(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_FixPt;
    pub fn FM_Data_GetAsText(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_Text;

    pub fn FM_FixPt_AsLong(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> fmx_int32;
    pub fn FM_FixPt_Delete(_self: *mut fmx_FixPt, _x: *mut fmx__fmxcpt);
    pub fn FM_Data_Delete(_self: *mut fmx_Data, _x: *mut fmx__fmxcpt);

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
    pub fn FM_Text_Delete(_self: *mut fmx_Text, _x: *mut fmx__fmxcpt);

    pub fn FM_QuadChar_Constructor2(
        c0: c_char,
        c1: c_char,
        c2: c_char,
        c3: c_char,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_QuadChar;

    pub fn FM_ExprEnv_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_ExprEnv;
    pub fn FM_RowVect_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_RowVect;
    pub fn FM_DataVect_Delete(_self: *mut fmx_DataVect, _x: *mut fmx__fmxcpt);
    pub fn FM_RowVect_Delete(_self: *mut fmx_RowVect, _x: *mut fmx__fmxcpt);
    pub fn FM_QuadChar_Delete(_self: *mut fmx_QuadChar, _x: *mut fmx__fmxcpt);
    pub fn FM_Locale_Delete(_self: *mut fmx_Locale, _x: *mut fmx__fmxcpt);
    pub fn FM_Locale_Constructor1(inputType: fmx_int32, _x: *mut fmx__fmxcpt) -> *mut fmx_Locale;
    pub fn FM_FixPt_Constructor1(
        val: fmx_int32,
        precision: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_FixPt;

    pub fn FM_ExprEnv_ExecuteFileSQLTextResult(
        _self: *const fmx_ExprEnv,
        expression: *const fmx_Text,
        filename: *const fmx_Text,
        parameters: *const fmx_DataVect,
        result: *mut fmx_Data,
        colSep: fmx_uint16,
        rowSep: fmx_uint16,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_ExprEnv_ExecuteFileSQL(
        _self: *const fmx_ExprEnv,
        expression: *const fmx_Text,
        filename: *const fmx_Text,
        parameters: *const fmx_DataVect,
        result: *mut fmx_RowVect,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_ExprEnv_Delete(_self: *mut fmx_ExprEnv, _x: *mut fmx__fmxcpt);

    pub fn FM_RowVect_Size(_self: *const fmx_RowVect, _x: *mut fmx__fmxcpt) -> fmx_uint32;
    pub fn FM_RowVect_IsEmpty(_self: *const fmx_RowVect, _x: *mut fmx__fmxcpt) -> bool;
    pub fn FM_RowVect_At(
        _self: *const fmx_RowVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_DataVect;

    pub fn FM_ExprEnv_Evaluate(
        _self: *const fmx_ExprEnv,
        expression: *const fmx_Text,
        result: *mut fmx_Data,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_ExprEnv_EvaluateGetFunction(
        _self: *const fmx_ExprEnv,
        functionValue: c_short,
        result: *mut fmx_Data,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

}
