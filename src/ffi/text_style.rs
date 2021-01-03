use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_CharacterStyle {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_CharacterStyle_Color {
    pub _address: u8,
}

#[cfg_attr(target_os = "macos", link(kind = "framework", name = "FMWrapper"))]
#[cfg_attr(target_os = "windows", link(kind = "static", name = "FMWrapper"))]
#[cfg_attr(target_os = "linux", link(kind = "dylib", name = "FMWrapper"))]
extern "C" {
    fn FM_CharacterStyle_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_CharacterStyle;
    fn FM_CharacterStyle_Constructor2(
        font: u16,
        face: u16,
        size: u16,
        color: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle;
    fn FM_CharacterStyle_Constructor3(
        style: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle;
}

pub struct CharacterStyle {
    pub(crate) ptr: *mut fmx_CharacterStyle,
    drop: bool,
}

impl CharacterStyle {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_CharacterStyle_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
}
