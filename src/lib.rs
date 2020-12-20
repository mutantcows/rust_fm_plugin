#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod config;
pub mod ffi;
pub mod helpers;
pub mod post_build;
pub use config::kill_filemaker;
pub use ffi::*;
pub use helpers::{log, write_to_u16_buff};

pub mod prelude {
    pub use crate::PluginFlag::*;
    pub use crate::{
        fmx_ExternCallStruct, fmx_ptrtype, register_plugin, write_to_u16_buff, ExternStringType,
        ExternVersion, ExternalFunction, FMError, FMExternCallType, FileMakerFunction, IdleType,
        Plugin, QuadChar,
    };
}

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

    fn session_notifications(_session_id: fmx_ptrtype);

    fn file_notifications(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype);

    fn preferences();

    #[doc(hidden)]
    fn shutdown(version: ExternVersion) {
        let plugin_id = QuadChar::new(Self::id());
        for f in Self::register_functions() {
            if version < f.min_version {
                continue;
            }
            f.unregister(&plugin_id);
        }
    }

    fn idle(session_id: fmx_ptrtype);
    fn not_idle(session_id: fmx_ptrtype);
    fn script_paused(session_id: fmx_ptrtype);
    fn script_running(session_id: fmx_ptrtype);
    fn un_safe(session_id: fmx_ptrtype);
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
                Init => (*pb).result = initialize((*pb).extnVersion) as u64,
                Idle => {
                    use IdleType::*;
                    match IdleType::from((*pb).parm1) {
                        Idle => $x::idle((*pb).parm2),
                        NotIdle => $x::not_idle((*pb).parm2),
                        ScriptPaused => $x::script_paused((*pb).parm2),
                        ScriptRunning => $x::script_running((*pb).parm2),
                        Unsafe => $x::un_safe((*pb).parm2),
                    }
                }
                Shutdown => $x::shutdown((*pb).extnVersion),
                AppPrefs => $x::preferences(),
                GetString => get_string(
                    (*pb).parm1.into(),
                    (*pb).parm2 as u32,
                    (*pb).parm3 as u32,
                    (*pb).result as *mut u16,
                ),
                SessionShutdown => $x::session_notifications((*pb).parm2),
                FileShutdown => $x::file_notifications((*pb).parm2, (*pb).parm3),
            }
        }

        fn get_string(
            which_string: ExternStringType,
            _win_lang_id: u32,
            out_buffer_size: u32,
            out_buffer: *mut u16,
        ) {
            use ExternStringType::*;
            let string = match which_string {
                Name => $x::name().to_string(),
                AppConfig => $x::description().to_string(),
                Options => {
                    let mut options: String = ::std::str::from_utf8($x::id()).unwrap().to_string();
                    options.push('1');
                    options.push(if $x::enable_configure_button() {
                        'Y'
                    } else {
                        'n'
                    });
                    options.push('n');
                    options.push(if $x::enable_init_and_shutdown() {
                        'Y'
                    } else {
                        'n'
                    });
                    options.push(if $x::enable_idle() { 'Y' } else { 'n' });
                    options.push(if $x::enable_shutdown() { 'Y' } else { 'n' });
                    options.push('n');
                    options
                }
                HelpUrl => $x::url().to_string(),
                Blank => "".to_string(),
            };
            unsafe { write_to_u16_buff(out_buffer, out_buffer_size, &string) }
        }

        fn initialize(version: ExternVersion) -> ExternVersion {
            let plugin_id = QuadChar::new($x::id());
            for f in $x::register_functions() {
                if version < f.min_version {
                    continue;
                }
                if f.register(&plugin_id) != FMError::NoError {
                    return ExternVersion::DoNotEnable;
                }
            }
            ExternVersion::V190
        }
    };
}
