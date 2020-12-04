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
    drop: bool,
}

impl QuadChar {
    pub(crate) fn new(bytes: &[u8; 4]) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let b: &[i8; 4] = unsafe { &*(bytes as *const [u8; 4] as *const [i8; 4]) };
        let ptr = unsafe { FM_QuadChar_Constructor2(b[0], b[1], b[2], b[3], &mut _x) };
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

pub(crate) struct Text {
    pub(crate) ptr: *mut fmx_Text,
    drop: bool,
}

impl Text {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Text_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_Text) -> Self {
        Self {
            ptr: ptr as *mut fmx_Text,
            drop: false,
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

impl Drop for Text {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_Text_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        let str = self.get_unicode(0, self.size());
        str.to_string().unwrap()
    }
}

pub(crate) struct FixPt {
    pub(crate) ptr: *mut fmx_FixPt,
    drop: bool,
}

impl FixPt {
    pub(crate) fn new(val: fmx_int32, precision: fmx_int32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_FixPt_Constructor1(val, precision, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
    pub(crate) fn from_ptr(ptr: *const fmx_FixPt) -> FixPt {
        Self {
            ptr: ptr as *mut fmx_FixPt,
            drop: false,
        }
    }

    pub(crate) fn get_as_long(&self) -> fmx_int32 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong(self.ptr, &mut _x) };
        _x.check();
        num
    }
}

impl Drop for FixPt {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_FixPt_Delete(self.ptr as *mut fmx_FixPt, &mut _x) };
            _x.check();
        }
    }
}

pub(crate) struct Locale {
    pub(crate) ptr: *mut fmx_Locale,
    drop: bool,
}

impl Locale {
    pub(crate) fn new(input_type: fmx_int32) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_Locale_Constructor1(input_type, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_Locale) -> Self {
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

pub(crate) struct DataVect {
    pub(crate) ptr: *mut fmx_DataVect,
    drop: bool,
}

impl DataVect {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_DataVect) -> Self {
        Self {
            ptr: ptr as *mut fmx_DataVect,
            drop: false,
        }
    }

    pub(crate) fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_DataVect_Size(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub(crate) fn at(&self, position: fmx_uint32) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let data_ptr = unsafe { FM_DataVect_At(self.ptr, position, &mut _x) };
        _x.check();
        Data::from_ptr(data_ptr)
    }

    pub(crate) fn at_as_text(&self, position: fmx_uint32) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsText(self.ptr, position, &mut _x) };
        _x.check();
        Text::from_ptr(ptr)
    }

    pub(crate) fn at_as_number(&self, position: fmx_uint32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsNumber(self.ptr, position, &mut _x) };
        _x.check();
        FixPt::from_ptr(ptr)
    }

    pub(crate) fn push(&mut self, data: Data) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DataVect_PushBack(self.ptr, data.ptr, &mut _x) };
        _x.check();
    }
}

impl Drop for DataVect {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_DataVect_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

pub(crate) struct RowVect {
    pub(crate) ptr: *mut fmx_RowVect,
    drop: bool,
}

impl RowVect {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_RowVect_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_RowVect_Size(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub(crate) fn at(&self, position: fmx_uint32) -> DataVect {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_RowVect_At(self.ptr, position, &mut _x) };
        _x.check();
        DataVect::from_ptr(ptr)
    }

    pub(crate) fn is_empty(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let empty = unsafe { FM_RowVect_IsEmpty(self.ptr, &mut _x) };
        _x.check();
        empty
    }
}

impl Drop for RowVect {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_RowVect_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

pub(crate) enum PluginFlag {
    DisplayInAllDialogs = 0b1111111100000000,
    MacCompatible = 0b0000000000000010,
    WinCompatible = 0b0000000000000100,
    ServerCompatible = 0b0000000000001000,
    IOSCompatible = 0b0000000000010000,
    CustomWebCompatible = 0b0000000000100000,
    WebDirectCompatible = 0b0000000001000000,
    AllDeviceCompatible = 0b0000000001111110,
    FutureCompatible = 0b111111110000000000000000,
}

pub(crate) enum SDKVersion {
    BadExtn = -1,
    DoNotEnable = -2,
    V40 = 11,
    V41 = 12,
    V50 = 14,
    V60 = 17,
    V70 = 50, // Jumping to 50
    V80 = 51,
    V110 = 52,
    V120 = 53, // Support for 64-bit plugins
    V130 = 54,
    V140 = 55,
    V150 = 56,
    V160 = 57,
    V170 = 59,
    V180 = 60,
    V190 = 62,
    Min = 4,
    Max = 255,
}

pub(crate) enum GetStringType {
    Name,
    AppConfig,
    Options,
    HelpUrl,
    Blank,
}

impl From<u32> for GetStringType {
    fn from(num: u32) -> Self {
        match num {
            128 => Self::Name,
            129 => Self::AppConfig,
            131 => Self::Options,
            132 => Self::HelpUrl,
            _ => Self::Blank,
        }
    }
}

impl From<u8> for FMExternCallType {
    fn from(num: u8) -> Self {
        match num {
            0 => Self::Init,
            1 => Self::Idle,
            4 => Self::Shutdown,
            5 => Self::AppPrefs,
            7 => Self::GetString,
            8 => Self::SessionShutdown,
            9 => Self::FileShutdown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum FMExternCallType {
    Init,
    Idle,
    Shutdown,
    AppPrefs,
    GetString,
    SessionShutdown,
    FileShutdown,
}

pub(crate) enum IdleType {
    Idle,
    NotIdle,
    ScriptPaused,
    ScriptRunning,
    Unsafe,
}

impl From<u8> for IdleType {
    fn from(num: u8) -> Self {
        match num {
            0 => Self::Idle,
            1 => Self::NotIdle,
            2 => Self::ScriptPaused,
            3 => Self::ScriptRunning,
            4 => Self::Unsafe,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct ExprEnv {
    pub(crate) ptr: *mut fmx_ExprEnv,
    drop: bool,
}

impl ExprEnv {
    pub(crate) fn from_ptr(ptr: *const fmx_ExprEnv) -> Self {
        Self {
            ptr: ptr as *mut fmx_ExprEnv,
            drop: false,
        }
    }

    pub(crate) fn execute_file_sql_text_result(
        &self,
        expression: Text,
        file_name: Text,
        parameters: DataVect,
        result: &mut Data,
        col_sep: fmx_uint16,
        row_sep: fmx_uint16,
    ) -> fmx_errcode {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe {
            FM_ExprEnv_ExecuteFileSQLTextResult(
                self.ptr,
                expression.ptr,
                file_name.ptr,
                parameters.ptr,
                result.ptr,
                col_sep,
                row_sep,
                &mut _x,
            )
        };
        _x.check();
        error
    }

    pub(crate) fn execute_file_sql(
        &self,
        expression: Text,
        file_name: Text,
        parameters: DataVect,
        result: &mut RowVect,
    ) -> fmx_errcode {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe {
            FM_ExprEnv_ExecuteFileSQL(
                self.ptr,
                expression.ptr,
                file_name.ptr,
                parameters.ptr,
                result.ptr,
                &mut _x,
            )
        };
        _x.check();
        error
    }

    pub(crate) fn eval_get(&self, func: GetFunction) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let result = Data::new();
        unsafe { FM_ExprEnv_EvaluateGetFunction(self.ptr, func as i16, result.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl Drop for ExprEnv {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_ExprEnv_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

pub(crate) struct ExternalFunction {
    pub(crate) id: fmx_int16,
    pub(crate) name: Text,
    pub(crate) definition: Text,
    pub(crate) description: Text,
    pub(crate) min_args: fmx_int16,
    pub(crate) max_args: fmx_int16,
    pub(crate) compatible_flags: fmx_uint32,
    pub(crate) function_ptr: fmx_ExtPluginType,
}

impl ExternalFunction {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        id: fmx_int16,
        name: &str,
        definition: &str,
        description: &str,
        min_args: fmx_int16,
        max_args: fmx_int16,
        compatible_flags: fmx_uint32,
        function_ptr: fmx_ExtPluginType,
    ) -> Self {
        let mut fm_name = Text::new();
        fm_name.assign(name);

        let mut fm_desc = Text::new();
        fm_desc.assign(description);

        let mut fm_def = Text::new();
        fm_def.assign(definition);
        Self {
            id,
            name: fm_name,
            definition: fm_def,
            description: fm_desc,
            min_args,
            max_args,
            compatible_flags,
            function_ptr,
        }
    }
    pub(crate) fn register(&self, plugin_id: &QuadChar) -> fmx_errcode {
        let mut _x = fmx__fmxcpt::new();

        let error = unsafe {
            FM_ExprEnv_RegisterExternalFunctionEx(
                plugin_id.ptr,
                self.id,
                self.name.ptr,
                self.definition.ptr,
                self.description.ptr,
                self.min_args,
                self.max_args,
                self.compatible_flags,
                self.function_ptr,
                &mut _x,
            )
        };

        _x.check();
        error
    }
}

pub(crate) enum GetFunction {
    ApplicationVersion = 1001,
    CurrentDate = 1002,
    LastError = 1003,
    ActiveFieldName = 1004,
    FileName = 1005,
    FileSize = 1006,
    FoundCount = 1007,
    HostName = 1008,
    LayoutCount = 1009,
    LayoutName = 1010,
    LayoutNumber = 1011,
    SystemLanguage = 1012,
    WindowMode = 1013,
    MultiUserState = 1014,
    PageNumber = 1015,
    SystemPlatform = 1016,
    ActivePortalRowNumber = 1017,
    PrinterName = 1018,
    TotalRecordCount = 1019,
    RecordNumber = 1020,
    ActiveRepetitionNumber = 1021,
    RequestCount = 1022,
    ScreenDepth = 1023,
    ScreenHeight = 1024,
    ScreenWidth = 1025,
    ScriptName = 1026,
    SortState = 1027,
    SystemVersion = 1028,
    CurrentTime = 1029,
    UserCount = 1030,
    UserName = 1031,
    AccountName = 1032,
    LastMessageChoice = 1033,
    CurrentPrivilegeSetName = 1034,
    ActiveModifierKeys = 1035,
    NetworkProtocol = 1036,
    RecordID = 1037,
    RecordModificationCount = 1038,
    ActiveFieldContents = 1039,
    FilePath = 1040,
    LastExternalErrorDetail = 1041,
    LayoutAccess = 1042,
    RecordAccess = 1043,
    HighContrastState = 1044,
    HighContrastColor = 1045,
    StatusAreaState = 1046,
    LayoutViewState = 1047,
    WindowName = 1048,
    WindowHeight = 1049,
    WindowLeft = 1050,
    WindowTop = 1051,
    WindowVisible = 1052,
    WindowWidth = 1053,
    SystemNICAddress = 1054,
    SystemIpAddress = 1055,
    ActiveFieldTableName = 1056,
    ActiveSelectionSize = 1057,
    ActiveSelectionStart = 1058,
    ApplicationLanguage = 1059,
    CurrentHostTimestamp = 1060,
    LayoutTableName = 1061,
    ScriptParameter = 1062,
    CurrentTimeStamp = 1063,
    WindowDesktopHeight = 1064,
    WindowDesktopWidth = 1065,
    WindowContentHeight = 1066,
    WindowContentWidth = 1067,
    CalculationRepetitionNumber = 1068,
    CurrentExtendedPrivileges = 1069,
    AllowAbortState = 1070,
    ErrorCaptureState = 1071,
    DesktopPath = 1072,
    DocumentsPath = 1073,
    FileMakerPath = 1074,
    HostIPAddress = 1075,
    RequestOmitState = 1076,
    PreferencesPath = 1077,
    RecordOpenCount = 1078,
    RecordOpenState = 1079,
    ScriptResult = 1080,
    SystemDrive = 1081,
    TextRulerVisible = 1082,
    AllowFormattingBarState = 1083,
    UseSystemFormatsState = 1084,
    WindowZoomLevel = 1085,
    CustomMenuSetName = 1086,
    ActiveLayoutObjectName = 1088,
    TemporaryPath = 1089,
    HostApplicationVersion = 1090,
    TriggerModifierKeys = 1091,
    TriggerKeystroke = 1092,
    DocumentsPathListing = 1093,
    AccountPrivilegeSet = 1094,
    AccountExtendedPrivileges = 1095,
    QuickFindText = 1096,
    TriggerCurrentPanel = 1097,
    TriggerTargetPanel = 1098,
    WindowStyle = 1099,
    InstalledFMPlugins = 1100,
    UUID = 1101,
    PersistentID = 1102,
    ConnectionState = 1103,
    CurrentTimeUTCMilliseconds = 1104,
    Device = 1105,
    WindowOrientation = 1106,
    TriggerGestureInfo = 1107,
    EncryptionState = 1108,
    ScriptAnimation = 1109,
    ModifiedFields = 1110,
    NetworkType = 1111,
    ConnectionAttributes = 1112,
    ScreenScaleFactor = 1113,
    ApplicationArchitecture = 1115,
    TriggerExternalEvent = 1116,
    TouchKeyboardState = 1117,
    MenubarState = 1118,
    RegionMonitorEvents = 1119,
    AccountGroupName = 1120,
    ActiveRecordNumber = 1121,
    UUIDNumber = 1122,
    OpenDataFileInfo = 1123,
    AccountType = 1124,
    PageCount = 1125,
}
