use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_Data {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {
    fn FM_Data_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_Data;

    fn FM_Data_GetLocale(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_Locale;

    fn FM_Data_GetAsNumber(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_FixPt;

    fn FM_Data_GetAsText(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> *const fmx_Text;

    fn FM_Data_Delete(_self: *mut fmx_Data, _x: *mut fmx__fmxcpt);

    fn FM_Data_SetAsText(
        _self: *mut fmx_Data,
        textData: *const fmx_Text,
        sourceLocale: *const fmx_Locale,
        nativeType: DataType,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_Data_GetFontID(
        _self: *const fmx_Data,
        fontDisplayName: *const fmx_Text,
        fontScript: fmx_CharacterStyle_FontScript,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_CharacterStyle_FontID;

    fn FM_Data_GetPostscriptFontID(
        _self: *const fmx_Data,
        fontPostscriptName: *const fmx_Text,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_CharacterStyle_FontID;

    fn FM_Data_GetFontInfo(
        _self: *const fmx_Data,
        font: fmx_CharacterStyle_FontID,
        fontDisplayName: *mut fmx_Text,
        fontScript: *mut fmx_CharacterStyle_FontScript,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Data_GetPostscriptFontInfo(
        _self: *const fmx_Data,
        font: fmx_CharacterStyle_FontID,
        fontPostscriptName: *mut fmx_Text,
        env: *const fmx_ExprEnv,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_Data_GetNativeType(_self: *const fmx_Data, _x: *mut fmx__fmxcpt) -> DataType;

    fn FM_Data_ConvertData(_self: *mut fmx_Data, nativeType: DataType, _x: *mut fmx__fmxcpt);

    fn FM_Data_SetAsNumber(
        _self: *mut fmx_Data,
        numericData: *const fmx_FixPt,
        nativeType: DataType,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Data_SetAsDate(
        _self: *mut fmx_Data,
        dateData: *const fmx_DateTime,
        nativeType: DataType,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Data_SetAsTime(
        _self: *mut fmx_Data,
        timeData: *const fmx_DateTime,
        nativeType: DataType,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Data_SetAsTimeStamp(
        _self: *mut fmx_Data,
        timeStampData: *const fmx_DateTime,
        nativeType: DataType,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Data_SetDateTime(
        _self: *mut fmx_Data,
        dateTimeData: *const fmx_DateTime,
        dateTimeType: fmx_int32,
        nativeType: DataType,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_Data_SetBinaryData(
        _self: *mut fmx_Data,
        binaryData: *const fmx_BinaryData,
        forceBinaryNativeType: bool,
        _x: *mut fmx__fmxcpt,
    );
}

pub struct Data {
    pub(crate) ptr: *mut fmx_Data,
    drop: bool,
}

impl Data {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_ptr(ptr: *const fmx_Data) -> Self {
        Self {
            ptr: ptr as *mut fmx_Data,
            drop: false,
        }
    }

    pub fn get_as_number(&self) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetAsNumber(self.ptr, &mut _x) };
        _x.check();
        FixPt::from_ptr(ptr)
    }

    pub fn get_as_text(&self) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetAsText(self.ptr, &mut _x) };
        _x.check();
        Text::from_ptr(ptr)
    }

    pub fn get_locale(&self) -> Locale {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Data_GetLocale(self.ptr, &mut _x) };
        _x.check();
        Locale::from_ptr(ptr)
    }

    pub fn set_as_text(&mut self, text: Text, locale: Locale) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsText(self.ptr, text.ptr, locale.ptr, DataType::Text, &mut _x) };
        _x.check();
    }

    pub fn set_as_number(&mut self, number: FixPt) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsNumber(self.ptr, number.ptr, DataType::Number, &mut _x) };
        _x.check();
    }

    pub fn set_as_date(&mut self, datetime: DateTime) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsDate(self.ptr, datetime.ptr, DataType::Date, &mut _x) };
        _x.check();
    }

    pub fn set_as_time(&mut self, datetime: DateTime) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsTime(self.ptr, datetime.ptr, DataType::Time, &mut _x) };
        _x.check();
    }

    pub fn set_as_timestamp(&mut self, datetime: DateTime) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_Data_SetAsTimeStamp(self.ptr, datetime.ptr, DataType::TimeStamp, &mut _x) };
        _x.check();
    }

    pub fn get_data_type(&self) -> DataType {
        let mut _x = fmx__fmxcpt::new();
        let data_type = unsafe { FM_Data_GetNativeType(self.ptr, &mut _x) };
        _x.check();
        data_type
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

impl Default for Data {
    fn default() -> Self {
        Self::new()
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

impl From<Data> for String {
    fn from(data: Data) -> String {
        data.get_as_text().to_string()
    }
}

#[repr(C)]
pub enum DataType {
    Invalid = 0,
    Text = 1,
    Number = 2,
    Date = 3,
    Time = 4,
    TimeStamp = 5,
    Binary = 6,
    Boolean = 7,
}
