use super::*;
use std::cmp::{Eq, PartialEq, PartialOrd};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_ExternCallStruct {
    pub extnVersion: ExternVersion,
    pub unusedID: fmx_unusedid,
    pub entryPoint: fmx_ExternCallProc,
    pub cfmCalls: fmx_boolean,
    pub whichCall: FMExternCallType,
    pub unsafeCalls: fmx_boolean,
    pub parm1: u8,
    pub parm2: fmx_ptrtype,
    pub parm3: fmx_ptrtype,
    pub instanceID: fmx_ptrtype,
    pub result: fmx_ptrtype,
    pub unused: fmx_ptrtype,
    pub cStartScript: fmx_StartScriptCall,
    pub cCurrentEnv: fmx_CurrentEnvCall,
}

#[repr(i16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum ExternVersion {
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

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum FMExternCallType {
    Init = 0,
    Idle = 1,
    Shutdown = 4,
    AppPrefs = 5,
    GetString = 7,
    SessionShutdown = 8,
    FileShutdown = 9,
}

#[repr(u8)]
pub enum IdleType {
    Idle = 0,
    NotIdle = 1,
    ScriptPaused = 2,
    ScriptRunning = 3,
    Unsafe = 4,
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

#[repr(u8)]
pub enum ExternStringType {
    Name = 128,
    AppConfig = 129,
    Options = 131,
    HelpUrl = 132,
    Blank = 0,
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

impl From<u8> for ExternStringType {
    fn from(num: u8) -> Self {
        match num {
            128 => Self::Name,
            129 => Self::AppConfig,
            131 => Self::Options,
            132 => Self::HelpUrl,
            _ => Self::Blank,
        }
    }
}

impl fmx_ExternCallStruct {
    pub fn execute_filemaker_script<F: ToText, S: ToText>(
        &self,
        file_name: F,
        script_name: S,
        control: ScriptControl,
        parameter: Option<Data>,
    ) -> FMError {
        let fn_txt = file_name.to_text();
        let sn_txt = script_name.to_text();
        let parameter = parameter.unwrap_or(Data::default());
        unsafe { self.cStartScript.unwrap()(fn_txt.ptr, sn_txt.ptr, control, parameter.ptr) }
    }
}

#[repr(u8)]
pub enum ScriptControl {
    Halt = 0,
    Exit = 1,
    Resume = 2,
    Pause = 3,
}

#[repr(u8)]
pub enum ApplicationName {
    Developer = 0, // FileMaker Pro Advanced
    Pro = 1,       // FileMaker Pro
    Runtime = 2,   // FileMaker Runtime
    Server = 3,    // This process no longer loads plug-ins
    Web = 4,       // Web Publishing process
    Mobile = 5,    // This iOS process is not allowed to load plug-ins
    XDBC = 6,      // This process does not currently load plug-ins
    SASE = 7,      // Server scripting process
    IWP = 8,       // This process no longer exists
}

impl From<u8> for ApplicationName {
    fn from(num: u8) -> Self {
        match num {
            0 => Self::Developer,
            1 => Self::Pro,
            2 => Self::Runtime,
            3 => Self::Server,
            4 => Self::Web,
            5 => Self::Mobile,
            6 => Self::XDBC,
            7 => Self::SASE,
            8 => Self::IWP,
            _ => unreachable!(),
        }
    }
}
