use super::*;
use std::cmp::{Eq, PartialEq, PartialOrd};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_ExternCallStruct {
    pub extnVersion: ExternVersion,
    pub unusedID: fmx_unusedid,
    pub entryPoint: fmx_ExternCallProc,
    pub cfmCalls: fmx_boolean,
    pub whichCall: fmx_ExternCallSwitch,
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
pub enum FMExternCallType {
    Init,
    Idle,
    Shutdown,
    AppPrefs,
    GetString,
    SessionShutdown,
    FileShutdown,
}

pub enum IdleType {
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

#[repr(i32)]
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
    pub fn execute_filemaker_script(
        &self,
        file_name: Text,
        script_name: Text,
        control: ScriptControl,
        parameter: Data,
    ) -> FMError {
        unsafe {
            self.cStartScript.unwrap()(file_name.ptr, script_name.ptr, control, parameter.ptr)
        }
    }
}

#[repr(u8)]
pub enum ScriptControl {
    Halt = 0,
    Exit = 1,
    Resume = 2,
    Pause = 3,
}
