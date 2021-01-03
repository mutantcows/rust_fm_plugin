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
        face: FontFace,
        size: u16,
        color: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle;

    fn FM_CharacterStyle_Constructor3(
        style: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle;

    fn FM_CharacterStyle_EnableFont(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_EnableFace(
        _self: *mut fmx_CharacterStyle,
        face: FontFace,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_CharacterStyle_EnableSize(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_EnableColor(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_DisableFont(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_DisableFace(
        _self: *mut fmx_CharacterStyle,
        face: FontFace,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_CharacterStyle_DisableAllFaces(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_DisableSize(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_DisableColor(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_DisableAll(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_IsAllDisabled(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_IsFontEnabled(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_IsFaceEnabled(
        _self: *const fmx_CharacterStyle,
        face: FontFace,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_IsAnyFaceEnabled(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_IsSizeEnabled(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_IsColorEnabled(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_SetFontInformation(
        _self: *mut fmx_CharacterStyle,
        font: u16,
        face: FontFace,
        size: u16,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_CharacterStyle_GetFontInformation(
        _self: *mut fmx_CharacterStyle,
        font: *mut u16,
        face: *mut FontFace,
        size: *mut u16,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_CharacterStyle_SetFont(_self: *mut fmx_CharacterStyle, font: u16, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_SetFace(
        _self: *mut fmx_CharacterStyle,
        face: FontFace,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_CharacterStyle_SetSize(_self: *mut fmx_CharacterStyle, size: u16, _x: *mut fmx__fmxcpt);

    fn FM_CharacterStyle_GetFont(_self: *const fmx_CharacterStyle, _x: *mut fmx__fmxcpt) -> u16;

    fn FM_CharacterStyle_GetFace(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> FontFace;

    fn FM_CharacterStyle_GetSize(_self: *const fmx_CharacterStyle, _x: *mut fmx__fmxcpt) -> u16;

    fn FM_CharacterStyle_SetColor(
        _self: *mut fmx_CharacterStyle,
        color: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_CharacterStyle_GetColor(
        _self: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_CharacterStyle_Color;

    #[allow(dead_code)]
    fn FM_CharacterStyle_operatorAS(
        _self: *mut fmx_CharacterStyle,
        rhs: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle;

    fn FM_CharacterStyle_operatorEQ(
        _self: *const fmx_CharacterStyle,
        rhs: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_operatorNE(
        _self: *const fmx_CharacterStyle,
        rhs: *const fmx_CharacterStyle,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_CharacterStyle_Delete(_self: *mut fmx_CharacterStyle, _x: *mut fmx__fmxcpt);

    fn FM_Color_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_CharacterStyle_Color;

    fn FM_Color_Constructor2(
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle_Color;

    fn FM_Color_Constructor3(
        color: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle_Color;

    fn FM_Color_SetRed(_self: *mut fmx_CharacterStyle_Color, r: u8, _x: *mut fmx__fmxcpt);

    fn FM_Color_SetGreen(_self: *mut fmx_CharacterStyle_Color, g: u8, _x: *mut fmx__fmxcpt);

    fn FM_Color_SetBlue(_self: *mut fmx_CharacterStyle_Color, b: u8, _x: *mut fmx__fmxcpt);

    fn FM_Color_SetAlpha(_self: *mut fmx_CharacterStyle_Color, a: u8, _x: *mut fmx__fmxcpt);

    fn FM_Color_GetRed(_self: *const fmx_CharacterStyle_Color, _x: *mut fmx__fmxcpt) -> u8;

    fn FM_Color_GetGreen(_self: *const fmx_CharacterStyle_Color, _x: *mut fmx__fmxcpt) -> u8;

    fn FM_Color_GetBlue(_self: *const fmx_CharacterStyle_Color, _x: *mut fmx__fmxcpt) -> u8;

    fn FM_Color_GetAlpha(_self: *const fmx_CharacterStyle_Color, _x: *mut fmx__fmxcpt) -> u8;

    fn FM_Color_GetRedUpsample(_self: *const fmx_CharacterStyle_Color, _x: *mut fmx__fmxcpt)
        -> u16;

    fn FM_Color_GetGreenUpsample(
        _self: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> u16;

    fn FM_Color_GetBlueUpsample(
        _self: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> u16;

    fn FM_Color_GetAlphaUpsample(
        _self: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> u16;

    #[allow(dead_code)]
    fn FM_Color_operatorAS(
        _self: *mut fmx_CharacterStyle_Color,
        rhs: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_CharacterStyle_Color;

    fn FM_Color_operatorEQ(
        _self: *const fmx_CharacterStyle_Color,
        rhs: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Color_operatorNE(
        _self: *const fmx_CharacterStyle_Color,
        rhs: *const fmx_CharacterStyle_Color,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Color_White() -> *const fmx_CharacterStyle_Color;

    fn FM_Color_Black() -> *const fmx_CharacterStyle_Color;

    fn FM_Color_Delete(_self: *mut fmx_CharacterStyle_Color, _x: *mut fmx__fmxcpt);

}

pub struct TextStyle {
    pub(crate) ptr: *mut fmx_CharacterStyle,
    drop: bool,
}

impl TextStyle {
    pub fn empty() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_CharacterStyle_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn new(font_id: u16, font_face: FontFace, size: u16, color: TextColor) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr =
            unsafe { FM_CharacterStyle_Constructor2(font_id, font_face, size, color.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn get_color(&self) -> TextColor {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_CharacterStyle_GetColor(self.ptr, &mut _x) };
        _x.check();
        TextColor {
            ptr: ptr as *mut _,
            drop: true,
        }
    }

    pub fn get_size(&self) -> u16 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_CharacterStyle_GetSize(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub fn get_font_id(&self) -> u16 {
        let mut _x = fmx__fmxcpt::new();
        let font_id = unsafe { FM_CharacterStyle_GetFont(self.ptr, &mut _x) };
        _x.check();
        font_id
    }

    pub fn get_font_face(&self) -> FontFace {
        let mut _x = fmx__fmxcpt::new();
        let font_face = unsafe { FM_CharacterStyle_GetFace(self.ptr, &mut _x) };
        _x.check();
        font_face
    }

    pub fn get_font_info(&self) -> FontInfo {
        let mut _x = fmx__fmxcpt::new();
        let mut id = 0;
        let mut face = FontFace::Plain;
        let mut size = 0;
        unsafe {
            FM_CharacterStyle_GetFontInformation(self.ptr, &mut id, &mut face, &mut size, &mut _x)
        };
        _x.check();
        FontInfo { id, face, size }
    }

    pub fn set_font_info(&mut self, font_info: FontInfo) {
        let mut _x = fmx__fmxcpt::new();
        unsafe {
            FM_CharacterStyle_SetFontInformation(
                self.ptr,
                font_info.id,
                font_info.face,
                font_info.size,
                &mut _x,
            )
        };
        _x.check();
    }

    pub fn is_color_enabled(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_IsColorEnabled(self.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn is_face_enabled(&self, font_face: FontFace) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_IsFaceEnabled(self.ptr, font_face, &mut _x) };
        _x.check();
        result
    }

    pub fn is_font_enabled(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_IsFontEnabled(self.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn is_size_enabled(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_IsSizeEnabled(self.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn is_any_face_enabled(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_IsAnyFaceEnabled(self.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn is_all_disabled(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_IsAllDisabled(self.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn disable_all(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_DisableAll(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn disable_all_faces(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_DisableAllFaces(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn disable_color(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_DisableColor(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn disable_face(&mut self, font_face: FontFace) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_DisableFace(self.ptr, font_face, &mut _x) };
        _x.check();
    }

    pub fn disable_font(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_DisableFont(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn disable_size(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_DisableSize(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn enable_color(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_EnableColor(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn enable_face(&mut self, font_face: FontFace) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_EnableFace(self.ptr, font_face, &mut _x) };
        _x.check();
    }

    pub fn enable_font(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_EnableFont(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn enable_size(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_EnableSize(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn set_color(&mut self, color: TextColor) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_SetColor(self.ptr, color.ptr, &mut _x) };
        _x.check();
    }

    pub fn set_font_face(&mut self, font_face: FontFace) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_SetFace(self.ptr, font_face, &mut _x) };
        _x.check();
    }

    pub fn set_font(&mut self, font_id: u16) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_SetFont(self.ptr, font_id, &mut _x) };
        _x.check();
    }

    pub fn set_size(&mut self, size: u16) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_CharacterStyle_SetSize(self.ptr, size, &mut _x) };
        _x.check();
    }
}

impl Drop for TextStyle {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_CharacterStyle_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

impl Clone for TextStyle {
    fn clone(&self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_CharacterStyle_Constructor3(self.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
}

pub struct FontInfo {
    pub id: u16,
    pub face: FontFace,
    pub size: u16,
}

pub struct TextColor {
    pub(crate) ptr: *mut fmx_CharacterStyle_Color,
    drop: bool,
}

impl TextColor {
    pub fn empty(&self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Color_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn new(&self, r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Color_Constructor2(r, g, b, a, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn set_red(&mut self, red: u8) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Color_SetRed(self.ptr, red, &mut _x) };
        _x.check();
    }

    pub fn set_green(&mut self, green: u8) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Color_SetGreen(self.ptr, green, &mut _x) };
        _x.check();
    }

    pub fn set_blue(&mut self, blue: u8) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Color_SetBlue(self.ptr, blue, &mut _x) };
        _x.check();
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Color_SetAlpha(self.ptr, alpha, &mut _x) };
        _x.check();
    }

    pub fn get_red(&self) -> u8 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetRed(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_green(&self) -> u8 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetGreen(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_blue(&self) -> u8 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetBlue(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_alpha(&self) -> u8 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetAlpha(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_red_upsample(&self) -> u16 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetRedUpsample(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_green_upsample(&self) -> u16 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetGreenUpsample(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_blue_upsample(&self) -> u16 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetBlueUpsample(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn get_alpha_upsample(&self) -> u16 {
        let mut _x = fmx__fmxcpt::new();
        let color = unsafe { FM_Color_GetAlphaUpsample(self.ptr, &mut _x) };
        _x.check();
        color
    }

    pub fn white() -> Self {
        let ptr = unsafe { FM_Color_White() };
        Self {
            ptr: ptr as *mut _,
            drop: true,
        }
    }

    pub fn black() -> Self {
        let ptr = unsafe { FM_Color_Black() };
        Self {
            ptr: ptr as *mut _,
            drop: true,
        }
    }
}

impl Drop for TextColor {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_Color_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

impl Clone for TextColor {
    fn clone(&self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Color_Constructor3(self.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
}

impl PartialEq for TextColor {
    fn eq(&self, other: &TextColor) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_Color_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &TextColor) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_Color_operatorNE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl PartialEq for TextStyle {
    fn eq(&self, other: &TextStyle) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &TextStyle) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_CharacterStyle_operatorNE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}

#[repr(u16)]
pub enum FontFace {
    Plain = 0,
    Bold = 256,
    Italic = 512,
    Underline = 1024,
    Outline = 2048,
    Shadow = 4096,
    Condense = 8192,
    Extend = 16384,
    Strikethrough = 1,
    SmallCaps = 2,
    Superscript = 4,
    Subscript = 8,
    Uppercase = 16,
    Lowercase = 32,
    Titlecase = 48,
    WordUnderline = 64,
    DoubleUnderline = 128,
    AllStyles = 32767,
}
