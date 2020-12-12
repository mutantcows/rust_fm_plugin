use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_DataVect {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_ExprEnv {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_RowVect {
    pub _address: u8,
}

#[cfg_attr(target_os = "macos", link(kind = "framework", name = "FMWrapper"))]
#[cfg_attr(target_os = "windows", link(kind = "static", name = "FMWrapper"))]
extern "C" {
    fn FM_ExprEnv_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_ExprEnv;

    fn FM_ExprEnv_RegisterScriptStep(
        pluginId: *const fmx_QuadChar,
        scriptStepId: c_short,
        scriptStepName: *const fmx_Text,
        scriptStepDefinition: *const fmx_Text,
        scriptStepDescription: *const fmx_Text,
        compatibleOnFlags: fmx_uint32,
        funcPtr: fmx_ExtPluginType,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_RegisterExternalFunctionEx(
        pluginId: *const fmx_QuadChar,
        functionId: c_short,
        functionName: *const fmx_Text,
        functionPrototype: *const fmx_Text,
        functionDescription: *const fmx_Text,
        minArgs: c_short,
        maxArgs: c_short,
        compatibleOnFlags: fmx_uint32,
        funcPtr: fmx_ExtPluginType,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_UnRegisterExternalFunction(
        pluginId: *const fmx_QuadChar,
        functionId: c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_ExecuteFileSQLTextResult(
        _self: *const fmx_ExprEnv,
        expression: *const fmx_Text,
        filename: *const fmx_Text,
        parameters: *const fmx_DataVect,
        result: *mut fmx_Data,
        colSep: fmx_uint16,
        rowSep: fmx_uint16,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_ExecuteFileSQL(
        _self: *const fmx_ExprEnv,
        expression: *const fmx_Text,
        filename: *const fmx_Text,
        parameters: *const fmx_DataVect,
        result: *mut fmx_RowVect,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_Delete(_self: *mut fmx_ExprEnv, _x: *mut fmx__fmxcpt);

    fn FM_ExprEnv_Evaluate(
        _self: *const fmx_ExprEnv,
        expression: *const fmx_Text,
        result: *mut fmx_Data,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_EvaluateGetFunction(
        _self: *const fmx_ExprEnv,
        functionValue: c_short,
        result: *mut fmx_Data,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_EvaluateConvertToFileMakerPath(
        _self: *const fmx_ExprEnv,
        inPath: *const fmx_Text,
        inFormat: FilePathFormat,
        outFMPath: *mut fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_EvaluateConvertFromFileMakerPath(
        _self: *const fmx_ExprEnv,
        inFMPath: *const fmx_Text,
        inFormat: FilePathFormat,
        outPath: *mut fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    #[deprecated]
    #[allow(dead_code)]
    fn FM_ExprEnv_RegisterExternalFunction(
        pluginId: *const fmx_QuadChar,
        functionId: c_short,
        functionName: *const fmx_Text,
        functionPrototype: *const fmx_Text,
        minArgs: c_short,
        maxArgs: c_short,
        compatibleOnFlags: fmx_uint32,
        funcPtr: fmx_ExtPluginType,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_ExprEnv_SessionID(_self: *const fmx_ExprEnv, _x: *mut fmx__fmxcpt) -> fmx_ptrtype;

    fn FM_ExprEnv_FileID(_self: *const fmx_ExprEnv, _x: *mut fmx__fmxcpt) -> fmx_ptrtype;

    fn FM_ExprEnv_UnRegisterScriptStep(
        pluginId: *const fmx_QuadChar,
        scriptStepId: c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_DataVect_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_DataVect;

    fn FM_DataVect_Size(_self: *const fmx_DataVect, _x: *mut fmx__fmxcpt) -> fmx_uint32;

    fn FM_DataVect_At(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_Data;

    fn FM_DataVect_AtAsText(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_Text;

    fn FM_DataVect_AtAsNumber(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_FixPt;

    fn FM_DataVect_PushBack(_self: *mut fmx_DataVect, data: *const fmx_Data, _x: *mut fmx__fmxcpt);

    fn FM_DataVect_Delete(_self: *mut fmx_DataVect, _x: *mut fmx__fmxcpt);

    fn FM_DataVect_Clear(_self: *mut fmx_DataVect, _x: *mut fmx__fmxcpt);

    fn FM_DataVect_IsEmpty(_self: *const fmx_DataVect, _x: *mut fmx__fmxcpt) -> bool;

    fn FM_DataVect_PopBack(_self: *mut fmx_DataVect, _x: *mut fmx__fmxcpt) -> *mut fmx_Data;

    fn FM_DataVect_AtAsDate(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_DateTime;

    fn FM_DataVect_AtAsTime(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_DateTime;

    fn FM_DataVect_AtAsTimeStamp(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_DateTime;

    fn FM_DataVect_AtAsBoolean(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_DataVect_AtAsBinaryData(
        _self: *const fmx_DataVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_BinaryData;

    fn FM_RowVect_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_RowVect;

    fn FM_RowVect_Delete(_self: *mut fmx_RowVect, _x: *mut fmx__fmxcpt);

    fn FM_RowVect_Size(_self: *const fmx_RowVect, _x: *mut fmx__fmxcpt) -> fmx_uint32;

    fn FM_RowVect_IsEmpty(_self: *const fmx_RowVect, _x: *mut fmx__fmxcpt) -> bool;

    fn FM_RowVect_At(
        _self: *const fmx_RowVect,
        position: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *const fmx_DataVect;

}

pub enum PluginFlag {
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

pub struct ExprEnv {
    pub(crate) ptr: *mut fmx_ExprEnv,
    drop: bool,
}

impl ExprEnv {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_ExprEnv_Constructor1(&mut _x) };
        _x.check();
        ExprEnv { ptr, drop: true }
    }

    pub fn from_ptr(ptr: *const fmx_ExprEnv) -> Self {
        Self {
            ptr: ptr as *mut fmx_ExprEnv,
            drop: false,
        }
    }

    pub fn session_id(&self) -> u64 {
        let mut _x = fmx__fmxcpt::new();
        let session_id = unsafe { FM_ExprEnv_SessionID(self.ptr, &mut _x) };
        _x.check();
        session_id
    }

    pub fn file_id(&self) -> u64 {
        let mut _x = fmx__fmxcpt::new();
        let file_id = unsafe { FM_ExprEnv_FileID(self.ptr, &mut _x) };
        _x.check();
        file_id
    }

    pub fn execute_file_sql_text_result(
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

    pub fn execute_file_sql(
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

    pub fn eval_get(&self, func: GetFunction) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let result = Data::new();
        unsafe { FM_ExprEnv_EvaluateGetFunction(self.ptr, func as i16, result.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn evaluate<T: ToText>(&self, expression: T) -> Data {
        let expr_txt = expression.to_text();
        let mut _x = fmx__fmxcpt::new();
        let result = Data::new();
        unsafe { FM_ExprEnv_Evaluate(self.ptr, expr_txt.ptr, result.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn from_fm_path(&self, path: Text, format: FilePathFormat) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let result = Text::new();
        unsafe {
            FM_ExprEnv_EvaluateConvertFromFileMakerPath(
                self.ptr, path.ptr, format, result.ptr, &mut _x,
            )
        };
        _x.check();
        result
    }

    pub fn to_fm_path(&self, path: Text, format: FilePathFormat) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let result = Text::new();
        unsafe {
            FM_ExprEnv_EvaluateConvertToFileMakerPath(
                self.ptr, path.ptr, format, result.ptr, &mut _x,
            )
        };
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

impl Default for ExprEnv {
    fn default() -> Self {
        Self::new()
    }
}

pub enum GetFunction {
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

pub struct RowVect {
    pub(crate) ptr: *mut fmx_RowVect,
    drop: bool,
}

impl RowVect {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_RowVect_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_RowVect_Size(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub fn at(&self, position: fmx_uint32) -> DataVect {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_RowVect_At(self.ptr, position, &mut _x) };
        _x.check();
        DataVect::from_ptr(ptr)
    }

    pub fn is_empty(&self) -> bool {
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

impl Default for RowVect {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DataVect {
    pub(crate) ptr: *mut fmx_DataVect,
    drop: bool,
}

impl DataVect {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_ptr(ptr: *const fmx_DataVect) -> Self {
        Self {
            ptr: ptr as *mut fmx_DataVect,
            drop: false,
        }
    }

    pub fn size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_DataVect_Size(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub fn at(&self, position: fmx_uint32) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let data_ptr = unsafe { FM_DataVect_At(self.ptr, position, &mut _x) };
        _x.check();
        Data::from_ptr(data_ptr)
    }

    pub fn at_as_text(&self, position: fmx_uint32) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsText(self.ptr, position, &mut _x) };
        _x.check();
        Text::from_ptr(ptr)
    }

    pub fn at_as_number(&self, position: fmx_uint32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsNumber(self.ptr, position, &mut _x) };
        _x.check();
        FixPt::from_ptr(ptr)
    }

    pub fn at_as_date(&self, position: fmx_uint32) -> DateTime {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsDate(self.ptr, position, &mut _x) };
        _x.check();
        DateTime::from_ptr(ptr)
    }

    pub fn at_as_time(&self, position: fmx_uint32) -> DateTime {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsTime(self.ptr, position, &mut _x) };
        _x.check();
        DateTime::from_ptr(ptr)
    }

    pub fn at_as_timestamp(&self, position: fmx_uint32) -> DateTime {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsTimeStamp(self.ptr, position, &mut _x) };
        _x.check();
        DateTime::from_ptr(ptr)
    }

    pub fn at_as_binary(&self, position: fmx_uint32) -> BinaryData {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DataVect_AtAsBinaryData(self.ptr, position, &mut _x) };
        _x.check();
        BinaryData::from_ptr(ptr)
    }

    pub fn at_as_bool(&self, position: fmx_uint32) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_DataVect_AtAsBoolean(self.ptr, position, &mut _x) };
        _x.check();
        result
    }

    pub fn push(&mut self, data: Data) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DataVect_PushBack(self.ptr, data.ptr, &mut _x) };
        _x.check();
    }

    pub fn clear(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DataVect_Clear(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn is_empty(&mut self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let empty = unsafe { FM_DataVect_IsEmpty(self.ptr, &mut _x) };
        _x.check();
        empty
    }

    pub fn pop(&mut self) -> Data {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_DataVect_PopBack(self.ptr, &mut _x) };
        _x.check();
        Data::from_ptr(result)
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

impl Default for DataVect {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct ExternalFunction {
    pub id: fmx_int16,
    pub name: &'static str,
    pub definition: &'static str,
    pub description: &'static str,
    pub min_args: fmx_int16,
    pub max_args: fmx_int16,
    pub compatible_flags: fmx_uint32,
    pub function_ptr: fmx_ExtPluginType,
}

impl ExternalFunction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: fmx_int16,
        name: &'static str,
        definition: &'static str,
        description: &'static str,
        min_args: fmx_int16,
        max_args: fmx_int16,
        compatible_flags: fmx_uint32,
        function_ptr: fmx_ExtPluginType,
    ) -> Self {
        Self {
            id,
            name,
            definition,
            description,
            min_args,
            max_args,
            compatible_flags,
            function_ptr,
        }
    }

    pub fn register(&self, plugin_id: &QuadChar) -> fmx_errcode {
        let mut _x = fmx__fmxcpt::new();

        let mut name = Text::new();
        name.assign(self.name);

        let mut description = Text::new();
        description.assign(self.description);

        let mut definition = Text::new();
        definition.assign(self.definition);

        let error = unsafe {
            FM_ExprEnv_RegisterExternalFunctionEx(
                plugin_id.ptr,
                self.id,
                name.ptr,
                definition.ptr,
                description.ptr,
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

    pub fn unregister(&self, plugin_id: &QuadChar) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_ExprEnv_UnRegisterExternalFunction(plugin_id.ptr, self.id, &mut _x) };
        _x.check();
    }
}

pub struct ExternalScriptStep {
    pub id: fmx_int16,
    pub name: &'static str,
    pub definition: &'static str,
    pub description: &'static str,
    pub compatible_flags: fmx_uint32,
    pub function_ptr: fmx_ExtPluginType,
}

impl ExternalScriptStep {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: fmx_int16,
        name: &'static str,
        definition: &'static str,
        description: &'static str,
        compatible_flags: fmx_uint32,
        function_ptr: fmx_ExtPluginType,
    ) -> Self {
        Self {
            id,
            name,
            definition,
            description,
            compatible_flags,
            function_ptr,
        }
    }

    pub fn register(&self, plugin_id: &QuadChar) -> fmx_errcode {
        let mut _x = fmx__fmxcpt::new();

        let mut name = Text::new();
        name.assign(self.name);

        let mut description = Text::new();
        description.assign(self.description);

        let mut definition = Text::new();
        definition.assign(self.definition);

        let error = unsafe {
            FM_ExprEnv_RegisterScriptStep(
                plugin_id.ptr,
                self.id,
                name.ptr,
                definition.ptr,
                description.ptr,
                self.compatible_flags,
                self.function_ptr,
                &mut _x,
            )
        };

        _x.check();
        error
    }

    pub fn unregister(&self, plugin_id: &QuadChar) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_ExprEnv_UnRegisterScriptStep(plugin_id.ptr, self.id, &mut _x) };
        _x.check();
    }
}

#[repr(i16)]
pub enum PluginError {
    NoError = 0,
    Result1 = 1552,
    Result2 = 1553,
    Result3 = 1554,
    Result4 = 1555,
    Result5 = 1556,
    Result6 = 1557,
    Result7 = 1558,
    Result8 = 1559,
}

#[repr(i32)]
pub enum FilePathFormat {
    PosixPath = 1,
    WinPath = 2,
    URLPath = 3,
}
