use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Data {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {
    pub fn FM_Data_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_Data;
    pub fn FM_Data_GetLocale(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_Locale;
    pub fn FM_Data_GetAsNumber(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_FixPt;
    pub fn FM_Data_GetAsText(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_Text;
    pub fn FM_Data_Delete(_self: *mut fmx_Data, _x: *mut fmx__fmxcpt);
    pub fn FM_Data_SetAsText(
        _self: *mut fmx_Data,
        textData: *const fmx_Text,
        sourceLocale: *const fmx_Locale,
        nativeType: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;
    pub fn FM_Data_GetFontID(
        _self: *const fmx_Data,
        fontDisplayName: *const fmx_Text,
        fontScript: fmx_CharacterStyle_FontScript,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_CharacterStyle_FontID;
    pub fn FM_Data_GetPostscriptFontID(
        _self: *const fmx_Data,
        fontPostscriptName: *const fmx_Text,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_CharacterStyle_FontID;
    pub fn FM_Data_GetFontInfo(
        _self: *const fmx_Data,
        font: fmx_CharacterStyle_FontID,
        fontDisplayName: *mut fmx_Text,
        fontScript: *mut fmx_CharacterStyle_FontScript,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> bool;
    pub fn FM_Data_GetPostscriptFontInfo(
        _self: *const fmx_Data,
        font: fmx_CharacterStyle_FontID,
        fontPostscriptName: *mut fmx_Text,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> bool;
    pub fn FM_Data_IsEmpty(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> bool;
    pub fn FM_Data_IsValid(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> bool;
    pub fn FM_Data_IsFindRequest(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> bool;
    pub fn FM_Data_Clear(_self: *mut fmx_Data, newNativeType: fmx_int32, _x: *mut fmx__fmxcpt);
    pub fn FM_Data_GetAsDate(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_DateTime;
    pub fn FM_Data_GetAsTime(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_DateTime;
    pub fn FM_Data_GetAsTimeStamp(
        _self: *const fmx_Data,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_DateTime;
    pub fn FM_Data_GetAsBoolean(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> bool;
    pub fn FM_Data_GetBinaryData(
        _self: *const fmx_Data,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_BinaryData;
}

pub(crate) struct Data {
    pub(crate) ptr: *mut fmx_Data,
    drop: bool,
}

impl Data {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_Data) -> Self {
        Self {
            ptr: ptr as *mut fmx_Data,
            drop: false,
        }
    }

    pub(crate) fn get_as_number(&self) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetAsNumber(self.ptr, &mut _x) };
        _x.check();
        FixPt::from_ptr(ptr)
    }

    pub(crate) fn get_as_text(&self) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetAsText(self.ptr, &mut _x) };
        _x.check();
        Text::from_ptr(ptr)
    }

    pub(crate) fn get_locale(&self) -> Locale {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetLocale(self.ptr, &mut _x) };
        _x.check();
        Locale::from_ptr(ptr)
    }

    pub(crate) fn set_as_text(&mut self, text: Text, locale: Locale) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsText(self.ptr, text.ptr, locale.ptr, 1, &mut _x) };
        _x.check();
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_Data_Delete(self.ptr as *mut fmx_Data, &mut _x) };
            _x.check();
        }
    }
}

impl From<Data> for i32 {
    fn from(data: Data) -> i32 {
        i32::from(data.get_as_number())
    }
}

impl From<&Data> for i32 {
    fn from(data: &Data) -> i32 {
        i32::from(data.get_as_number())
    }
}
