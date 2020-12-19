#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::str::from_utf8;

pub mod config;
pub mod ffi;
pub mod helpers;
pub mod post_build;
pub use config::kill_filemaker;
pub use ffi::*;
pub use helpers::log;
use helpers::*;

pub trait Plugin {
    fn id() -> &'static [u8; 4];
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn url() -> &'static str;
    fn enable_configure_button() -> bool {
        false
    }
    fn enable_init_and_shutdown() -> bool {
        true
    }
    fn enable_idle() -> bool {
        false
    }
    fn enable_shutdown() -> bool {
        false
    }
    fn register_functions() -> Vec<ExternalFunction>;

    /// # Safety
    /// talks to C
    unsafe fn get_string(
        which_string: ExternStringType,
        _win_lang_id: u32,
        out_buffer_size: u32,
        out_buffer: *mut u16,
    ) {
        use ExternStringType::*;
        let string = match which_string {
            Name => Self::name().to_string(),
            AppConfig => Self::description().to_string(),
            Options => {
                let mut options: String = from_utf8(Self::id()).unwrap().to_string();
                options.push('1');
                options.push(if Self::enable_configure_button() {
                    'Y'
                } else {
                    'n'
                });
                options.push('n');
                options.push(if Self::enable_init_and_shutdown() {
                    'Y'
                } else {
                    'n'
                });
                options.push(if Self::enable_idle() { 'Y' } else { 'n' });
                options.push(if Self::enable_shutdown() { 'Y' } else { 'n' });
                options.push('n');
                options
            }
            HelpUrl => Self::url().to_string(),
            Blank => "".to_string(),
        };
        write_to_u16_buff(out_buffer, out_buffer_size, &string)
    }

    fn initialize(version: ExternVersion) -> u64 {
        let plugin_id = QuadChar::new(Self::id());

        if version < ExternVersion::V160 {
            return ExternVersion::DoNotEnable as u64;
        }

        for f in Self::register_functions() {
            if f.register(&plugin_id) != FMError::NoError {
                return ExternVersion::DoNotEnable as u64;
            }
        }
        ExternVersion::V190 as u64
    }

    fn session_notifications(_session_id: fmx_ptrtype);

    fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype);

    fn preferences();

    fn shutdown(version: ExternVersion) {
        let plugin_id = QuadChar::new(Self::id());
        if version >= ExternVersion::V160 {
            for f in Self::register_functions() {
                f.unregister(&plugin_id);
            }
        }
    }

    fn idle_callback(idle_level: fmx_IdleLevel, _session_id: fmx_ptrtype) {
        use IdleType::*;
        match IdleType::from(idle_level) {
            Idle => Self::idle(),
            NotIdle => {}
            ScriptPaused => {}
            ScriptRunning => {}
            Unsafe => {}
        }
    }

    fn idle();
    fn not_idle();
    fn script_paused();
    fn script_running();
    fn un_safe();
}

#[macro_export]
macro_rules! register_plugin {
    ($x:ident) => {
        #[no_mangle]
        pub static mut gfmx_ExternCallPtr: *mut fmx_ExternCallStruct = std::ptr::null_mut();

        #[no_mangle]
        unsafe extern "C" fn FMExternCallProc(pb: *mut fmx_ExternCallStruct) {
            // Setup global defined in fmxExtern.h (this will be obsoleted in a later header file)
            gfmx_ExternCallPtr = pb;
            use FMExternCallType::*;

            // Message dispatcher
            match FMExternCallType::from((*pb).whichCall) {
                Init => (*pb).result = $x::initialize((*pb).extnVersion),
                Idle => $x::idle_callback((*pb).parm1, (*pb).parm2),
                Shutdown => $x::shutdown((*pb).extnVersion),
                AppPrefs => $x::preferences(),
                GetString => $x::get_string(
                    (*pb).parm1.into(),
                    (*pb).parm2 as u32,
                    (*pb).parm3 as u32,
                    (*pb).result as *mut u16,
                ),
                SessionShutdown => $x::session_notifications((*pb).parm2),
                FileShutdown => $x::file_notifications((*pb).parm2, (*pb).parm3),
            }
        }
    };
}
