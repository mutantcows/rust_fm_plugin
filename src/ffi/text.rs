use super::*;
use std::ffi::{CString, OsStr};
use widestring::U16CString;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Text {
    _address: u8,
}

#[cfg_attr(target_os = "macos", link(kind = "framework", name = "FMWrapper"))]
#[cfg_attr(target_os = "windows", link(kind = "static", name = "FMWrapper"))]
#[cfg_attr(target_os = "linux", link(kind = "dylib", name = "FMWrapper"))]
extern "C" {
    fn FM_Text_AssignUnicodeWithLength(
        _self: *mut fmx_Text,
        s: *const u16,
        strlength: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_InsertText(
        _self: *mut fmx_Text,
        other: *const fmx_Text,
        position: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_AppendText(
        _self: *mut fmx_Text,
        other: *const fmx_Text,
        position: u32,
        size: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_AssignWide(_self: *mut fmx_Text, s: *const u16, _x: *mut fmx__fmxcpt);

    fn FM_Text_Assign(_self: *mut fmx_Text, s: *const c_char, encoding: i32, _x: *mut fmx__fmxcpt);

    fn FM_Text_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_Text;
    fn FM_Text_GetSize(_self: *const fmx_Text, _x: *mut fmx__fmxcpt) -> u32;

    fn FM_Text_GetUnicode(
        _self: *const fmx_Text,
        s: *mut u16,
        position: u32,
        size: u32,
        _x: *mut fmx__fmxcpt,
    );
    fn FM_Text_Delete(_self: *mut fmx_Text, _x: *mut fmx__fmxcpt);

}

pub struct Text {
    pub(crate) ptr: *mut fmx_Text,
    drop: bool,
}

impl Text {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Text_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_ptr(ptr: *const fmx_Text) -> Self {
        Self {
            ptr: ptr as *mut fmx_Text,
            drop: false,
        }
    }

    pub fn size(&self) -> u32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_Text_GetSize(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub fn assign(&mut self, s: &str) {
        let c_string: CString = CString::new(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_Assign(self.ptr, c_string.as_ptr(), 1, &mut _x) };
        _x.check();
    }

    pub fn assign_unicode_with_length(&mut self, s: &str, len: u32) {
        let c_string = U16CString::from_str(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AssignUnicodeWithLength(self.ptr, c_string.as_ptr(), len, &mut _x) };
        _x.check();
    }

    pub fn assign_wide(&mut self, s: &str) {
        let c_string = U16CString::from_str(s).unwrap();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AssignWide(self.ptr, c_string.as_ptr(), &mut _x) };
        _x.check();
    }

    pub fn insert(&mut self, s: &Text, pos: u32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_InsertText(self.ptr, s.ptr, pos, &mut _x) };
        _x.check();
    }

    pub fn append(&mut self, s: &Text) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_AppendText(self.ptr, s.ptr, 0, s.size(), &mut _x) };
        _x.check();
    }

    pub fn get_unicode(&self, position: u32, size: u32) -> U16CString {
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

impl Drop for Text {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_Text_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        let str = self.get_unicode(0, self.size());
        str.to_string().unwrap()
    }
}

pub trait ToText {
    fn to_text(self) -> Text;
}

impl ToText for Text {
    fn to_text(self) -> Self {
        self
    }
}

impl ToText for String {
    fn to_text(self) -> Text {
        let mut txt = Text::new();
        txt.assign(&self);
        txt
    }
}

impl From<Text> for u16 {
    fn from(txt: Text) -> u16 {
        unsafe { *txt.get_unicode(0, txt.size()).as_ptr() }
    }
}

impl From<&String> for Text {
    fn from(txt: &String) -> Text {
        let mut text = Text::new();
        text.assign(txt);
        text
    }
}

impl From<&OsStr> for Text {
    fn from(txt: &OsStr) -> Text {
        let mut text = Text::new();
        text.assign(&*txt.to_string_lossy());
        text
    }
}

impl ToText for &OsStr {
    fn to_text(self) -> Text {
        let mut txt = Text::new();
        txt.assign(&*self.to_string_lossy());
        txt
    }
}
