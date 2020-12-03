use std::ffi::CString;
use widestring::{U16CString, WideCString};

use crate::ffi::*;

impl fmx__fmxcpt {
    pub(crate) fn check(&self) {
        if self.m_code != 0 {
            panic!();
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            m_code: 0,
            m_vers: 1,
        }
    }
}

pub(crate) struct QuadChar {
    pub(crate) ptr: *mut fmx_QuadChar,
}

impl QuadChar {
    pub(crate) fn new(bytes: &[u8; 4]) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let b: &[i8; 4] = unsafe { &*(bytes as *const [u8; 4] as *const [i8; 4]) };
        let ptr = unsafe { FM_QuadChar_Constructor2(b[0], b[1], b[2], b[3], &mut _x) };
        _x.check();
        Self { ptr }
    }
}

pub(crate) struct Text {
    pub(crate) ptr: *mut fmx_Text,
}

impl Text {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Text_Constructor1(&mut _x) };
        _x.check();
        Self { ptr }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_Text) -> Self {
        Self {
            ptr: ptr as *mut fmx_Text,
        }
    }

    pub(crate) fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_Text_GetSize(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub(crate) fn assign(&mut self, s: &str) {
        let c_string: CString = CString::new(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_Assign(self.ptr, c_string.as_ptr(), 1, &mut _x) };
        _x.check();
    }

    pub(crate) fn assign_unicode_with_length(&mut self, s: &str, len: u32) {
        let c_string = WideCString::from_str(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AssignUnicodeWithLength(self.ptr, c_string.as_ptr(), len, &mut _x) };
        _x.check();
    }

    pub(crate) fn assign_wide(&mut self, s: &str) {
        let c_string = WideCString::from_str(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AssignWide(self.ptr, c_string.as_ptr(), &mut _x) };
        _x.check();
    }

    pub(crate) fn insert_text(&mut self, s: &Text, pos: u32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_InsertText(self.ptr, s.ptr, pos, &mut _x) };
        _x.check();
    }

    pub(crate) fn get_unicode(&self, position: fmx_uint32, size: fmx_uint32) -> U16CString {
        let mut _x = fmx__fmxcpt::new();
        let out_vec: Vec<u16> = vec![1; size as usize];
        let out_buffer = U16CString::new(out_vec).unwrap();

        unsafe {
            FM_Text_GetUnicode(
                self.ptr,
                out_buffer.as_ptr() as *mut u16,
                position,
                size,
                &mut _x,
            )
        };
        _x.check();
        out_buffer
    }
}

pub(crate) struct FixPt {
    pub(crate) ptr: *const fmx_FixPt,
}

impl FixPt {
    pub(crate) fn from_ptr(ptr: *const fmx_FixPt) -> Self {
        Self { ptr }
    }

    pub(crate) fn get_as_long(&self) -> fmx_int32 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong(self.ptr, &mut _x) };
        _x.check();
        num
    }
}

pub(crate) struct Locale {
    pub(crate) ptr: *const fmx_Locale,
}

impl Locale {
    pub(crate) fn from_ptr(ptr: *const fmx_Locale) -> Self {
        Self { ptr }
    }
}
pub(crate) struct Data {
    pub(crate) ptr: *mut fmx_Data,
}

impl Data {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_Constructor1(&mut _x) };
        _x.check();
        Self { ptr }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_Data) -> Self {
        Self {
            ptr: ptr as *mut fmx_Data,
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
}

impl fmx_Data {
    pub(crate) fn get_locale(&self) -> Locale {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetLocale(self, &mut _x) };
        _x.check();
        Locale::from_ptr(ptr)
    }

    pub(crate) fn set_as_text(&mut self, text: Text, locale: Locale) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsText(self, text.ptr, locale.ptr, 1, &mut _x) };
        _x.check();
    }
}

pub(crate) struct DataVect {
    pub(crate) ptr: *mut fmx_DataVect,
}

impl DataVect {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_Constructor1(&mut _x) };
        _x.check();
        Self { ptr }
    }

    pub(crate) fn push(&mut self, data: Data) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DataVect_PushBack(self.ptr, data.ptr, &mut _x) };
        _x.check();
    }
}

impl fmx_DataVect {
    pub(crate) fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_DataVect_Size(self, &mut _x) };
        _x.check();
        size
    }

    pub(crate) fn at(&self, position: fmx_uint32) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let data_ptr = unsafe { FM_DataVect_At(self, position, &mut _x) };
        _x.check();
        Data::from_ptr(data_ptr)
    }

    pub(crate) fn at_as_text(&self, position: fmx_uint32) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsText(self, position, &mut _x) };
        _x.check();
        Text::from_ptr(ptr)
    }

    pub(crate) fn at_as_number(&self, position: fmx_uint32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsNumber(self, position, &mut _x) };
        _x.check();
        FixPt::from_ptr(ptr)
    }
}
