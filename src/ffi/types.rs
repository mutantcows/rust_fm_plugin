use super::*;

#[cfg(target_family = "windows")]
pub type fmx_int64 = i64;
#[cfg(target_family = "unix")]
pub type fmx_int64 = i32;
#[cfg(target_family = "windows")]
pub type fmx_uint64 = u64;
#[cfg(target_family = "unix")]
pub type fmx_uint64 = u32;
#[cfg(target_family = "windows")]
pub type fmx_unusedid = i16;
#[cfg(target_family = "unix")]
pub type fmx_unusedid = i32;

pub type fmx_ptrtype = u64;
pub type fmx_ExternCallSwitch = u8;
pub type fmx_ScriptControl = u8;
pub type fmx_IdleLevel = u8;
pub type fmx_ExternCallProc = Option<unsafe extern "C" fn(arg1: *mut fmx_ExternCallStruct)>;
pub type fmx_CurrentEnvCall = Option<unsafe extern "C" fn(env: *mut fmx_ExprEnv) -> FMError>;
pub type fmx_fontface = u16;
pub type fmx_fontid = u16;
pub type fmx_fontsize = u16;
pub type fmx_colorchannel = u8;
pub type fmx_colorchannel16 = u16;
pub type fmx_fontscript = u16;
pub type fmx_ExtPluginType = Option<
    extern "C" fn(
        functionId: i16,
        env: *const fmx_ExprEnv,
        parms: *const fmx_DataVect,
        result: *mut fmx_Data,
    ) -> FMError,
>;
pub type fmx_StartScriptCall = Option<
    unsafe extern "C" fn(
        fileName: *const fmx_Text,
        scriptName: *const fmx_Text,
        control: ScriptControl,
        parameter: *const fmx_Data,
    ) -> FMError,
>;

#[derive(Debug, Clone, Copy)]
#[repr(i8)]
pub enum fmx_boolean {
    False = 0,
    True = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Locale {
    _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_QuadChar {
    _address: u8,
}

#[cfg_attr(target_os = "macos", link(kind = "framework", name = "FMWrapper"))]
#[cfg_attr(target_os = "windows", link(kind = "static", name = "FMWrapper"))]
#[cfg_attr(target_os = "linux", link(kind = "dylib", name = "FMWrapper"))]
extern "C" {
    #[allow(dead_code)]
    fn FM_QuadChar_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_QuadChar;

    fn FM_QuadChar_Constructor2(
        c0: i8,
        c1: i8,
        c2: i8,
        c3: i8,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_QuadChar;

    #[allow(dead_code)]
    fn FM_QuadChar_Constructor3(
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_QuadChar;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorAS(
        _self: *mut fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_QuadChar;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorAR(_self: *mut fmx_QuadChar, i: i32, _x: *mut fmx__fmxcpt) -> u8;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorCAR(_self: *const fmx_QuadChar, i: i32, _x: *mut fmx__fmxcpt) -> u8;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorEQ(
        _self: *const fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorNE(
        _self: *const fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorLT(
        _self: *const fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorLE(
        _self: *const fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorGT(
        _self: *const fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    #[allow(dead_code)]
    fn FM_QuadChar_operatorGE(
        _self: *const fmx_QuadChar,
        value: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    #[allow(dead_code)]
    fn FM_QuadChar_GetMacType(_self: *const fmx_QuadChar, _x: *mut fmx__fmxcpt) -> u32;

    #[allow(dead_code)]
    fn FM_QuadChar_SetMacType(_self: *mut fmx_QuadChar, value: u32, _x: *mut fmx__fmxcpt);

    fn FM_QuadChar_Delete(_self: *mut fmx_QuadChar, _x: *mut fmx__fmxcpt);

    fn FM_Locale_Delete(_self: *mut fmx_Locale, _x: *mut fmx__fmxcpt);

    fn FM_Locale_Constructor1(inputType: LocaleType, _x: *mut fmx__fmxcpt) -> *mut fmx_Locale;

    #[allow(dead_code)]
    fn FM_Locale_operatorAS(
        _self: *mut fmx_Locale,
        rhs: *const fmx_Locale,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_Locale;

}

pub struct Locale {
    pub(crate) ptr: *mut fmx_Locale,
    drop: bool,
}

impl Locale {
    pub fn new(input_type: LocaleType) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Locale_Constructor1(input_type, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_ptr(ptr: *const fmx_Locale) -> Self {
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

pub struct QuadChar {
    pub(crate) ptr: *mut fmx_QuadChar,
    drop: bool,
}

impl QuadChar {
    pub fn new(bytes: &[u8; 4]) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let b: &[i8; 4] = unsafe { &*(bytes as *const [u8; 4] as *const [i8; 4]) };
        let ptr = unsafe { FM_QuadChar_Constructor2(b[0], b[1], b[2], b[3], &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn empty() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_QuadChar_Constructor1(&mut _x) };
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

#[repr(i32)]
pub enum LocaleType {
    None = 0,            // Empty
    System = 1,          // Uses system settings
    UnicodeRaw = 2,      // Use raw unicode bytes (like the old ASCII ordering)
    UnicodeStandard = 3, // Standard unicode rules
    Catalog = 4,         // FileMaker list rules
    CAT = 16,            // Catalan
    HRV = 17,            // Croatian
    CES = 18,            // Czech
    DAN = 19,            // Danish
    NLD = 20,            // Dutch
    ENG = 21,            // English
    FIN = 22,            // Finnish
    FINFMI = 23,         // Finnish (FileMaker custom)
    FRA = 24,            // French
    DEU = 25,            // German
    DEUDictionary = 26,  // German (dictionary ordering)
    ELL = 27,            // Greek
    HUN = 28,            // Hungarian
    ISL = 29,            // Icelandic
    ITA = 30,            // Italian
    JPN = 31,            // Japanese
    NOR = 32,            // Norwegian
    POL = 33,            // Polish
    POR = 34,            // Portuguese
    RON = 35,            // Romanian
    RUS = 36,            // Russian
    SLK = 37,            // Slovak
    SLV = 38,            // Slovenian
    SPA = 39,            // Spanish
    SPATraditional = 40, // Spanish (traditional)
    SWE = 41,            // Swedish
    SWEFMI = 42,         // Swedish (FileMaker custom)
    TUR = 43,            // Turkish
    UKR = 44,            // Ukrainian

    // New to FileMaker Pro 8.5
    CHI = 45,       // Chinese (PinYin)
    CHIStroke = 46, // Chinese (Stroke-radical)

    // New to FileMaker Pro 13
    KOR = 76, // Korean

    // For compatibility with WinSoft versions (supported in FMI versions >= 12)
    HE = 47,       // Hebrew
    HI = 48,       // Hindi
    AR = 49,       // Arabic
    ET = 50,       // Estonian
    LT = 51,       // Lithuanian
    LV = 52,       // Latvian
    SR = 53,       // Serbian
    FA = 54,       // Persian
    BG = 55,       // Bulgarian
    VI = 56,       // Vietnamese
    TH = 57,       // Thai
    ELLMixed = 58, // Greek Mixed
    BEN = 59,      // Bengali
    TEL = 60,      // Telugu
    MAR = 61,      // Marathi
    TAM = 62,      // Tamil
    GUJ = 63,      // Gujarati
    KAN = 64,      // Kannada
    MAL = 65,      // Malayalam
    PAN = 67,      // Panjabi

    // Used in versions distributed by WinSoft (not supported in FMI versions)
    ORI = 66, // Oriya
    SIN = 68, // Sinhala
    URD = 69, // Urdu
    DIV = 70, // Divehi (Thanaa)
    BUR = 71, // Burmese (Myanmar)
    SAN = 72, // Sanskrit
    LAO = 73, // Lao
    KHM = 74, // Khmer
    BOD = 75, // Tibetan

    Invalid = 0xFFFF,
}

impl ToString for QuadChar {
    fn to_string(&self) -> String {
        let mut _x = fmx__fmxcpt::new();
        let mut bytes: Vec<u8> = Vec::with_capacity(4);
        for i in 0..4 {
            let c = unsafe { FM_QuadChar_operatorAR(self.ptr, i, &mut _x) };
            bytes.push(c);
            _x.check();
        }
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}
