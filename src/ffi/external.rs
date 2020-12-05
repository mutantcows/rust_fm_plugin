use super::*;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_ExternCallStruct {
    pub extnVersion: fmx_int16,
    pub unusedID: fmx_unusedid,
    pub entryPoint: fmx_ExternCallProc,
    pub cfmCalls: fmx_boolean,
    pub whichCall: fmx_ExternCallSwitch,
    pub unsafeCalls: fmx_boolean,
    pub parm1: fmx_uchar,
    pub parm2: fmx_ptrtype,
    pub parm3: fmx_ptrtype,
    pub instanceID: fmx_ptrtype,
    pub result: fmx_ptrtype,
    pub unused: fmx_ptrtype,
    pub cStartScript: fmx_StartScriptCall,
    pub cCurrentEnv: fmx_CurrentEnvCall,
}

pub(crate) enum ExternVersion {
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

pub(crate) enum ExternStringType {
    Name,
    AppConfig,
    Options,
    HelpUrl,
    Blank,
}

impl From<u32> for ExternStringType {
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
