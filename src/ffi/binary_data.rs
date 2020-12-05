use super::*;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_BinaryData {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {
    pub fn FM_BinaryData_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_BinaryData;

    pub fn FM_BinaryData_Constructor2(
        sourceData: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    pub fn FM_BinaryData_Constructor3(
        name: *const fmx_Text,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    pub fn FM_BinaryData_Constructor4(
        name: *const fmx_Text,
        context: *mut fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    pub fn FM_BinaryData_operatorAS(
        _self: *mut fmx_BinaryData,
        source: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    pub fn FM_BinaryData_operatorEQ(
        _self: *const fmx_BinaryData,
        compareData: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_BinaryData_operatorNE(
        _self: *const fmx_BinaryData,
        compareData: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_BinaryData_GetCount(_self: *const fmx_BinaryData, _x: *mut fmx__fmxcpt) -> fmx_int32;

    pub fn FM_BinaryData_GetIndex(
        _self: *const fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_int32;

    pub fn FM_BinaryData_GetTotalSize(
        _self: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_uint32;

    pub fn FM_BinaryData_GetType(
        _self: *const fmx_BinaryData,
        index: fmx_int32,
        dataType: *mut fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_BinaryData_GetSize(
        _self: *const fmx_BinaryData,
        index: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_uint32;

    pub fn FM_BinaryData_GetData(
        _self: *const fmx_BinaryData,
        index: fmx_int32,
        offset: fmx_uint32,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_Add(
        _self: *mut fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_Remove(
        _self: *mut fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_BinaryData_RemoveAll(_self: *mut fmx_BinaryData, _x: *mut fmx__fmxcpt);
    pub fn FM_BinaryData_Delete(_self: *mut fmx_BinaryData, _x: *mut fmx__fmxcpt);

    pub fn FM_BinaryData_GetFNAMData(
        _self: *const fmx_BinaryData,
        filepathlist: *mut fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_AddFNAMData(
        _self: *mut fmx_BinaryData,
        filepathlist: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_GetSIZEData(
        _self: *const fmx_BinaryData,
        width: *mut c_short,
        height: *mut c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_AddSIZEData(
        _self: *mut fmx_BinaryData,
        width: c_short,
        height: c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_AddBegin(
        _self: *mut fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        context: *mut fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_AddAppend(
        _self: *mut fmx_BinaryData,
        context: fmx_uint32,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_BinaryData_AddFinish(
        _self: *mut fmx_BinaryData,
        context: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;
}
