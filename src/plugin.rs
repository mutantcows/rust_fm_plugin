use crate::{
    fmx_ptrtype, write_to_u16_buff, ExternStringType, ExternVersion, FMError, QuadChar,
    Registration,
};

/// Implement this trait for your plugin struct. The different functions are used to give FileMaker information about the plugin. You also need to register all your functions/script steps in the trait implementation.
///
/// # Example
/// ```rust
/// # use fm_plugin::prelude::*;
/// # use fm_plugin::{DataVect, ExprEnv, Data, FMError};
/// # struct MyFunction;
/// # impl FileMakerFunction for MyFunction {
/// # fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
/// #     FMError::NoError
/// # }
/// # }
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn id() -> &'static [u8; 4] { &b"MyPl" }
///     fn name() -> &'static str { "MY PLUGIN" }
///     fn description() -> &'static str { "Does all sorts of great things." }
///     fn url() -> &'static str { "http://myplugin.com" }
///
///     fn register_functions() -> Vec<Registration> {
///         vec![Registration::Function {
///             id: 100,
///             name: "MyPlugin_MyFunction",
///             definition: "MyPlugin_MyFunction( arg1 ; arg2 )",
///             description: "Does some really great stuff.",
///             min_args: 2,
///             max_args: 2,
///             compatible_flags: DisplayInAllDialogs | FutureCompatible,
///             min_version: ExternVersion::V160,
///             function_ptr: Some(MyFunction::extern_func),
///             }
///         ]
///     }
/// }
/// ```
pub trait Plugin {
    /// Unique 4 letter identifier for the plug-in.
    fn id() -> &'static [u8; 4];
    /// Plug-in's name.
    fn name() -> &'static str;
    /// Description of the plug-in.
    fn description() -> &'static str;
    /// Url to send users to from the help in FileMaker. The function's name that the user  will be appended to the url when clicked.
    fn url() -> &'static str;

    /// Register all custom functions/script steps
    fn register_functions() -> Vec<Registration>;

    /// Defaults to false
    fn enable_configure_button() -> bool {
        false
    }
    /// Defaults to true
    fn enable_init_and_shutdown() -> bool {
        true
    }
    /// Defaults to false
    fn enable_idle() -> bool {
        false
    }
    /// Defaults to false
    fn enable_file_and_session_shutdown() -> bool {
        false
    }

    fn session_shutdown(_session_id: fmx_ptrtype) {}
    fn file_shutdown(_session_id: fmx_ptrtype, _file_id: fmx_ptrtype) {}
    fn preferences() {}
    fn idle(_session_id: fmx_ptrtype) {}
    fn not_idle(_session_id: fmx_ptrtype) {}
    fn script_paused(_session_id: fmx_ptrtype) {}
    fn script_running(_session_id: fmx_ptrtype) {}
    fn un_safe(_session_id: fmx_ptrtype) {}
}

pub trait PluginInternal<T>
where
    T: Plugin,
{
    unsafe fn get_string(
        which_string: ExternStringType,
        _win_lang_id: u32,
        out_buffer_size: u32,
        out_buffer: *mut u16,
    ) {
        use ExternStringType::*;
        let string = match which_string {
            Name => T::name().to_string(),
            AppConfig => T::description().to_string(),
            Options => {
                let mut options: String = ::std::str::from_utf8(T::id()).unwrap().to_string();
                options.push('1');
                options.push(if T::enable_configure_button() {
                    'Y'
                } else {
                    'n'
                });
                options.push('n');
                options.push(if T::enable_init_and_shutdown() {
                    'Y'
                } else {
                    'n'
                });
                options.push(if T::enable_idle() { 'Y' } else { 'n' });
                options.push(if T::enable_file_and_session_shutdown() {
                    'Y'
                } else {
                    'n'
                });
                options.push('n');
                options
            }
            HelpUrl => T::url().to_string(),
            Blank => "".to_string(),
        };
        write_to_u16_buff(out_buffer, out_buffer_size, &string)
    }

    fn initialize(version: ExternVersion) -> ExternVersion {
        let plugin_id = QuadChar::new(T::id());
        for f in T::register_functions() {
            if version < f.min_version() {
                continue;
            }
            if f.register(&plugin_id) != FMError::NoError {
                return ExternVersion::DoNotEnable;
            }
        }
        ExternVersion::V190
    }

    fn shutdown(version: ExternVersion) {
        let plugin_id = QuadChar::new(T::id());
        for f in T::register_functions() {
            if version < f.min_version() {
                continue;
            }
            f.unregister(&plugin_id);
        }
    }
}

/// Sets up the entry point for every FileMaker call into the plug-in. The function then dispatches the calls to the various trait functions you can implement.
/// Impl [`Plugin`][Plugin] for your plugin struct, and then call the macro on it.
///
/// # Example
/// ```rust
/// use fm_plugin::prelude::*;
///
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
/// # fn id()-> &'static [u8; 4] { b"TEST" }
/// # fn name()-> &'static str { "TEST" }
/// # fn description()-> &'static str { "TEST" }
/// # fn url()-> &'static str { "TEST" }
/// # fn register_functions()-> Vec<Registration> { Vec::new() }
///            // ...
/// }
///
/// register_plugin!(MyPlugin);
/// ```
///
/// # Macro Contents
///```rust
/// # use fm_plugin::prelude::*;
/// # #[macro_export]
/// #    macro_rules! register_plugin {
/// #        ($x:ident) => {
/// #[no_mangle]
/// pub static mut gfmx_ExternCallPtr: *mut fmx_ExternCallStruct = std::ptr::null_mut();
///
/// #[no_mangle]
/// unsafe extern "C" fn FMExternCallProc(pb: *mut fmx_ExternCallStruct) {
///     // Setup global defined in fmxExtern.h (this will be obsoleted in a later header file)
///     gfmx_ExternCallPtr = pb;
///     use FMExternCallType::*;
///
///     // Message dispatcher
///     match FMExternCallType::from((*pb).whichCall) {
///         Init => (*pb).result = $x::initialize((*pb).extnVersion) as u64,
///         Idle => {
///             use IdleType::*;
///             match IdleType::from((*pb).parm1) {
///                 Idle => $x::idle((*pb).parm2),
///                 NotIdle => $x::not_idle((*pb).parm2),
///                 ScriptPaused => $x::script_paused((*pb).parm2),
///                 ScriptRunning => $x::script_running((*pb).parm2),
///                 Unsafe => $x::un_safe((*pb).parm2),
///             }
///         }
///         Shutdown => $x::shutdown((*pb).extnVersion),
///         AppPrefs => $x::preferences(),
///         GetString => $x::get_string(
///             (*pb).parm1.into(),
///             (*pb).parm2 as u32,
///             (*pb).parm3 as u32,
///             (*pb).result as *mut u16,
///         ),
///         SessionShutdown => $x::session_shutdown((*pb).parm2),
///         FileShutdown => $x::file_shutdown((*pb).parm2, (*pb).parm3),
///     }
/// }
///
/// impl PluginInternal<$x> for $x {}
///
/// pub fn execute_filemaker_script(
///     file_name: Text,
///     script_name: Text,
///     control: ScriptControl,
///     parameter: Data,
/// ) -> FMError {
///     unsafe {
///         (*gfmx_ExternCallPtr).execute_filemaker_script(
///             file_name,
///             script_name,
///             control,
///             parameter,
///         )
///     }
/// }
///
/// lazy_static! {
///     static ref GLOBAL_STATE: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
/// }
/// #        };
/// #    }
///
/// ```
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
                Init => (*pb).result = $x::initialize((*pb).extnVersion) as u64,
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
                GetString => $x::get_string(
                    (*pb).parm1.into(),
                    (*pb).parm2 as u32,
                    (*pb).parm3 as u32,
                    (*pb).result as *mut u16,
                ),
                SessionShutdown => $x::session_shutdown((*pb).parm2),
                FileShutdown => $x::file_shutdown((*pb).parm2, (*pb).parm3),
            }
        }

        impl PluginInternal<$x> for $x {}

        pub fn execute_filemaker_script(
            file_name: Text,
            script_name: Text,
            control: ScriptControl,
            parameter: Data,
        ) -> FMError {
            unsafe {
                (*gfmx_ExternCallPtr).execute_filemaker_script(
                    file_name,
                    script_name,
                    control,
                    parameter,
                )
            }
        }

        lazy_static! {
            static ref GLOBAL_STATE: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
        }
    };
}
