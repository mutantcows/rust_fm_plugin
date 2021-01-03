use super::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ffi::{CString, OsStr};
use std::fmt;
use std::mem::ManuallyDrop;
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

    fn FM_Text_SetText(
        _self: *mut fmx_Text,
        other: *const fmx_Text,
        position: u32,
        size: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_AssignWide(_self: *mut fmx_Text, s: *const u16, _x: *mut fmx__fmxcpt);

    fn FM_Text_Assign(_self: *mut fmx_Text, s: *const i8, encoding: i32, _x: *mut fmx__fmxcpt);

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

    fn FM_Text_DeleteText(
        _self: *mut fmx_Text,
        positionToDelete: u32,
        sizeToDelete: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_Find(
        _self: *const fmx_Text,
        other: *const fmx_Text,
        position: u32,
        _x: *mut fmx__fmxcpt,
    ) -> u32;

    fn FM_Text_FindPrev(
        _self: *const fmx_Text,
        other: *const fmx_Text,
        position: u32,
        _x: *mut fmx__fmxcpt,
    ) -> u32;

    fn FM_Text_FindIgnoringCase(
        _self: *const fmx_Text,
        other: *const fmx_Text,
        position: u32,
        _x: *mut fmx__fmxcpt,
    ) -> u32;

    fn FM_Text_FindPrevIgnoringCase(
        _self: *const fmx_Text,
        other: *const fmx_Text,
        position: u32,
        _x: *mut fmx__fmxcpt,
    ) -> u32;

    fn FM_Text_Uppercase(_self: *const fmx_Text, _x: *mut fmx__fmxcpt);

    fn FM_Text_Lowercase(_self: *const fmx_Text, _x: *mut fmx__fmxcpt);

    fn FM_Text_GetStyle(
        _self: *const fmx_Text,
        style: *mut fmx_CharacterStyle,
        position: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_GetDefaultStyle(
        _self: *const fmx_Text,
        style: *mut fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_SetStyle(
        _self: *mut fmx_Text,
        style: *const fmx_CharacterStyle,
        position: u32,
        size: u32,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_RemoveStyle(
        _self: *mut fmx_Text,
        style: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Text_ResetAllStyleBuffers(_self: *mut fmx_Text, _x: *mut fmx__fmxcpt);

    fn FM_Text_operatorEQ(
        _self: *const fmx_Text,
        that: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Text_operatorNE(
        _self: *const fmx_Text,
        that: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Text_operatorLT(
        _self: *const fmx_Text,
        that: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Text_operatorLE(
        _self: *const fmx_Text,
        that: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Text_operatorGT(
        _self: *const fmx_Text,
        that: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Text_operatorGE(
        _self: *const fmx_Text,
        that: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Text_GetBytesEx(
        _self: *const fmx_Text,
        buffer: *mut i8,
        buffersize: u32,
        position: u32,
        size: u32,
        encoding: i32,
        _x: *mut fmx__fmxcpt,
    ) -> u32;

}

#[derive(Eq)]
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

    pub fn set_text<T: ToText>(&mut self, other: T, position: u32, size: u32) {
        let mut _x = fmx__fmxcpt::new();
        let text = other.to_text();
        unsafe { FM_Text_SetText(self.ptr, text.ptr, position, size, &mut _x) };
        _x.check();
    }

    pub fn delete_text(&mut self, position: u32, size: u32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_DeleteText(self.ptr, position, size, &mut _x) };
        _x.check();
    }

    pub fn find<T: ToText>(&self, other: T, position: u32) {
        let mut _x = fmx__fmxcpt::new();
        let text = other.to_text();
        unsafe { FM_Text_Find(self.ptr, text.ptr, position, &mut _x) };
        _x.check();
    }

    pub fn find_previous<T: ToText>(&self, other: T, position: u32) {
        let mut _x = fmx__fmxcpt::new();
        let text = other.to_text();
        unsafe { FM_Text_FindPrev(self.ptr, text.ptr, position, &mut _x) };
        _x.check();
    }

    pub fn find_case_insensitive<T: ToText>(&self, other: T, position: u32) {
        let mut _x = fmx__fmxcpt::new();
        let text = other.to_text();
        unsafe { FM_Text_FindIgnoringCase(self.ptr, text.ptr, position, &mut _x) };
        _x.check();
    }

    pub fn find_previous_case_insensitive<T: ToText>(&self, other: T, position: u32) {
        let mut _x = fmx__fmxcpt::new();
        let text = other.to_text();
        unsafe { FM_Text_FindPrevIgnoringCase(self.ptr, text.ptr, position, &mut _x) };
        _x.check();
    }

    pub fn uppercase<T: ToText>(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_Uppercase(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn lowercase<T: ToText>(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_Lowercase(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn get_style(&self, position: u32) -> TextStyle {
        let mut _x = fmx__fmxcpt::new();
        let style = TextStyle::empty();
        unsafe { FM_Text_GetStyle(self.ptr, style.ptr, position, &mut _x) };
        _x.check();
        style
    }

    pub fn get_default_style(&self) -> TextStyle {
        let mut _x = fmx__fmxcpt::new();
        let style = TextStyle::empty();
        unsafe { FM_Text_GetDefaultStyle(self.ptr, style.ptr, &mut _x) };
        _x.check();
        style
    }

    pub fn set_style(&mut self, style: TextStyle, position: u32, size: u32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_SetStyle(self.ptr, style.ptr, position, size, &mut _x) };
        _x.check();
    }

    pub fn remove_style(&mut self, style: TextStyle) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_RemoveStyle(self.ptr, style.ptr, &mut _x) };
        _x.check();
    }

    pub fn remove_all_styles(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Text_ResetAllStyleBuffers(self.ptr, &mut _x) };
        _x.check();
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

    pub fn insert<T: ToText>(&mut self, s: T, pos: u32) {
        let mut _x = fmx__fmxcpt::new();
        let s = s.to_text();
        unsafe { FM_Text_InsertText(self.ptr, s.ptr, pos, &mut _x) };
        _x.check();
    }

    pub fn append<T: ToText>(&mut self, s: T) {
        let mut _x = fmx__fmxcpt::new();
        let s = s.to_text();
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

    pub fn get_bytes(&self, position: u32, size: u32) -> Vec<u8> {
        self.get_bytes_with_encoding(position, size, TextEncoding::UTF8)
    }

    pub fn get_bytes_with_encoding(
        &self,
        position: u32,
        size: u32,
        encoding: TextEncoding,
    ) -> Vec<u8> {
        let mut _x = fmx__fmxcpt::new();
        let buffer: Vec<i8> = Vec::with_capacity(size as usize);

        let mut buffer = ManuallyDrop::new(buffer);
        let buffer_ptr = buffer.as_mut_ptr();

        let bytes_written = unsafe {
            FM_Text_GetBytesEx(
                self.ptr,
                buffer_ptr,
                size as u32,
                position,
                size as u32,
                encoding as i32,
                &mut _x,
            )
        };
        _x.check();

        unsafe { Vec::from_raw_parts(buffer_ptr as *mut u8, bytes_written as usize, size as usize) }
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

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self.get_unicode(0, self.size());
        write!(f, "{}", str.to_string_lossy())
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

impl ToText for &String {
    fn to_text(self) -> Text {
        let mut txt = Text::new();
        txt.assign(&self);
        txt
    }
}

impl ToText for &str {
    fn to_text(self) -> Text {
        let mut txt = Text::new();
        txt.assign(self);
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

impl From<String> for Text {
    fn from(txt: String) -> Text {
        let mut text = Text::new();
        text.assign(&txt);
        text
    }
}

impl From<&str> for Text {
    fn from(txt: &str) -> Text {
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

impl PartialEq for Text {
    fn eq(&self, other: &Text) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_Text_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Text) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_Text_operatorNE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Text) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let lt = unsafe { FM_Text_operatorLT(self.ptr, other.ptr, &mut _x) };
        _x.check();
        lt
    }

    fn le(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let le = unsafe { FM_Text_operatorLE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        le
    }

    fn gt(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let gt = unsafe { FM_Text_operatorGT(self.ptr, other.ptr, &mut _x) };
        _x.check();
        gt
    }

    fn ge(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let ge = unsafe { FM_Text_operatorGE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        ge
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self > other {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}

impl PartialEq<&str> for Text {
    #[allow(clippy::clippy::cmp_owned)]
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for Text {
    #[allow(clippy::clippy::cmp_owned)]
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

#[repr(i32)]
pub enum TextEncoding {
    Native = 0,
    UTF8 = 1,
    ASCIIDOS = 2,
    ASCIIWindows = 3,
    ASCIIMac = 4,
    ISO8859_1 = 5,
    ShiftJISMac = 6,
    ShiftJISWin = 7,
    KoreanMac = 8,
    KoreanWin = 9,
    KoreanJohab = 10,
    ChineseTradMac = 11,
    ChineseTradWin = 12,
    ChineseSimpMac = 13,
    ChineseSimpWin = 14,
    CyrillicMac = 15,
    CyrillicWin = 16,
    ISO8859_5 = 17,
    CentralEuropeMac = 18,
    EasternEuropeWin = 19,
    ISO8859_2 = 20,
    TurkishMac = 21,
    TurkishWin = 22,
    ISO8859_3 = 23,
    ISO8859_9 = 24,
    BalticWin = 25,
    ISO8859_4 = 26,
    ArabicMac = 27,
    ArabicWin = 28,
    ISO8859_6 = 29,
    GreekMac = 30,
    GreekWin = 31,
    ISO88597 = 32,
    HebrewMac = 33,
    HebrewWin = 34,
    ISO8859_8 = 35,
    ISO8859_15 = 36,
}
