use crate::{
    fmx_Data, fmx_DataVect, fmx_ExprEnv, fmx_ExtPluginType, fmx__fmxcpt, fmx_ptrtype,
    write_to_u16_buff, AllowedVersions, ApplicationVersion, Data, DataVect, ExprEnv,
    ExternStringType, ExternVersion, FMError, FM_ExprEnv_RegisterExternalFunctionEx,
    FM_ExprEnv_RegisterScriptStep, FM_ExprEnv_UnRegisterExternalFunction,
    FM_ExprEnv_UnRegisterScriptStep, QuadChar, Text,
};
use widestring::U16CStr;

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
///             display_in_dialogs: true,
///             compatibility_flags: Compatibility::Future as u32,
///             min_ext_version: ExternVersion::V160,
///             min_fm_version: "18.0.2",
///             allowed_versions: AllowedVersions {developer: true, pro: true, web: true, sase: true, runtime: true},
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

    fn initialize(
        ext_version: ExternVersion,
        app_version: ApplicationVersion,
        app_version_number: fmx_ptrtype,
    ) -> ExternVersion {
        let plugin_id = QuadChar::new(T::id());
        for f in T::register_functions() {
            if ext_version < f.min_ext_version()
                || !f.is_fm_version_allowed(&app_version)
                || !is_version_high_enough(app_version_number, f.min_fm_version())
            {
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
            if version < f.min_ext_version() {
                continue;
            }
            f.unregister(&plugin_id);
        }
    }
}

fn is_version_high_enough(app_version_number: fmx_ptrtype, min_version: &str) -> bool {
    let string = unsafe { U16CStr::from_ptr_str(app_version_number as *const u16) };
    let string = string.to_string_lossy();
    let version_number = string.split(' ').last().unwrap();

    let (major, minor, patch) = semantic_version(version_number);
    let (min_major, min_minor, min_patch) = semantic_version(min_version);

    match ((major, minor, patch), (min_major, min_minor, min_patch)) {
        ((None, ..), (None, ..)) => false,
        ((Some(major), ..), (Some(min_major), ..)) if major < min_major => false,
        ((Some(major), ..), (Some(min_major), ..)) if major > min_major => true,
        ((Some(major), ..), (Some(min_major), None, ..)) if major == min_major => true,

        ((_, Some(minor), ..), (_, Some(min_minor), ..)) if minor < min_minor => false,
        ((_, Some(minor), ..), (_, Some(min_minor), ..)) if minor > min_minor => true,
        ((_, Some(minor), ..), (_, Some(min_minor), None)) if minor == min_minor => true,

        ((.., Some(patch)), (.., Some(min_patch))) if patch < min_patch => false,
        ((.., Some(patch)), (.., Some(min_patch))) if patch > min_patch => true,
        _ => true,
    }
}

fn semantic_version(version: &str) -> (Option<u8>, Option<u8>, Option<u8>) {
    let mut str_vec = version.split('.');
    let major = str_vec.next();
    let minor = str_vec.next();
    let patch = str_vec.next();
    (
        match major {
            Some(n) => n.parse::<u8>().ok(),
            None => None,
        },
        match minor {
            Some(n) => n.parse::<u8>().ok(),
            None => None,
        },
        match patch {
            Some(n) => n.parse::<u8>().ok(),
            None => None,
        },
    )
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
///     match (*pb).whichCall {
///         Init => {
///             (*pb).result = $x::initialize(
///                 (*pb).extnVersion,
///                 ApplicationVersion::from((*pb).parm1),
///                 (*pb).parm2,
///             ) as u64
///         }
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
/// pub fn execute_filemaker_script<F, S>(
///     file_name: F,
///     script_name: S,
///     control: ScriptControl,
///     parameter: Option<Data>,
/// ) -> FMError
/// where
///     F: ToText,
///     S: ToText,
/// {
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
///
/// pub fn store_state(key: &str, value: &str) {
///     let mut hmap = GLOBAL_STATE.write().unwrap();
///     (*hmap).insert(String::from(key), String::from(value));
/// }
///
/// pub fn get_state(key: &str) -> Option<String> {
///     let hmap = GLOBAL_STATE.read().unwrap();
///     (*hmap).get(key).cloned()
/// }
/// #    };
/// # }
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
            match (*pb).whichCall {
                Init => {
                    (*pb).result = $x::initialize(
                        (*pb).extnVersion,
                        ApplicationVersion::from((*pb).parm1),
                        (*pb).parm2,
                    ) as u64
                }
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

        pub fn execute_filemaker_script<F, S>(
            file_name: F,
            script_name: S,
            control: ScriptControl,
            parameter: Option<Data>,
        ) -> FMError
        where
            F: ToText,
            S: ToText,
        {
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

        pub fn store_state(key: &str, value: &str) {
            let mut hmap = GLOBAL_STATE.write().unwrap();
            (*hmap).insert(String::from(key), String::from(value));
        }

        pub fn get_state(key: &str) -> Option<String> {
            let hmap = GLOBAL_STATE.read().unwrap();
            (*hmap).get(key).cloned()
        }
    };
}

/// Register [`ScriptSteps`][Registration::ScriptStep] and [`Functions`][Registration::Function] for your plugin.
/// # Function Registration
/// Registration enables the function so that it appears in the calculation dialog in the application.
///
/// * `id` is the unique id that you can use to represent which function was called, it will be passed back to the registered function as the first parameter (see the parameter of the same name in [`fmx_ExtPluginType`][fmx_ExtPluginType]).
/// * `name` is the name of the function as it should appear in the calculation formula.
/// * `definition` is the suggested syntax that will appear in the list of functions in the calculation dialog.
/// * `description` is the text that will display when auto-entered into the calculation dialog. The format is "type ahead word list|description text".
/// * `min_args` is the number of required parameters for the function. `0` is the smallest valid value.
/// * `max_args` is the maximum number of parameters that they user should be able to specify in the calculation dialog and still have correct syntax usage for the function. Use `-1` to allow a variable number of parameters up to the number supported by calculation formulas in the application.
/// * `compatible_flags` see bit flags above.
/// * `function_ptr` is the pointer to the function that must match the signature defined by [`fmx_ExtPluginType`][fmx_ExtPluginType]. If you implement [`FileMakerFunction`][FileMakerFunction] for your function, then you can just reference [`MyFunction.extern_func`][FileMakerFunction::extern_func] here.
///
/// # Script Step Registration
///
/// [`Registration::ScriptStep::definition`][Registration::ScriptStep::definition] must contain XML defining the script step options.  Up to ten script parameters can be specified in addition to the optional target parameter. All the parameters are defined with `<Parameter>` tags in a `<PluginStep>` grouping.
///
/// The attributes for a `<Parameter>` tag include:
///
///   * `Type` - if not one of the following four types, the parameter is ignored
///       1. `Calc` - a standard Specify button that brings up the calculation dialog. When the script step is executed, the calculation will be evaluated and its results passed to the plug-in
///       2. `Bool` - simple check box that returns the value of `0` or `1`
///       3. `List` - a static drop-down or pop-up list in which the id of the item selected is returned. The size limit of this list is limited by the capabilities of the UI widgets used to display it. A `List` type parameter expects to contain `<Value>` tags as specified below
///       4. `Target` - will include a specify button that uses the new  `Insert From Target` field targeting dialog that allows a developer to put the results of a script step into a field (whether or not it is on a layout), into a variable, or insert into the current active field on a layout. If no `Target` is defined then the result `Data` object is ignored. If there are multiple `Target` definitions, only the first one will be honored.
///
///   * `ID` - A value in the range of `0` to `9` which is used as an index into the `DataVect` parms object for the plug-in to retrieve the value of the parameter. Indexes that are not in range or duplicated will cause the parameter to be ignored. A parameter of type `Target` ignores this attribute if specified
///
///   * `Label` - The name of parameter or control that is displayed in the UI
///
///   * `DataType` - only used by the `Calc` and `Target` parameter types. If not specified or not one of the six data types, the type `Text` will be used
///       1. `Text`
///       2. `Number`
///       3. `Date`
///       4. `Time`
///       5. `Timestamp`
///       6. `Container`
///
///   * `ShowInline` - value is either true or false. If defined and true, will cause the parameter to show up inlined with the script step in the Scripting Workspace
///
///   * `Default` - either the numeric index of the default list item or the true/false value for a bool item. Ignored for calc and target parameters
///
/// Parameters of type `List` are expected to contain `<Value>` tags whose values are used to construct the drop-down or pop-up list. The id of a value starts at zero but specific id can be given to a value by defining an `ID` attribute. If later values do not have an `ID` attributes the id will be set to the previous values id plus one.
///
/// Sample XML description:
///```xml
///<PluginStep>
///    <Parameter ID="0" Type="Calc" DataType="text" ShowInline="true" Label="Mood"/>
///    <Parameter ID="1" Type="List" ShowInline="true" Label="Color">
///    <Value ID="0">Red</Value>
///    <Value ID="1">Green</Value>
///    <Value ID="2">Blue</Value>
///    </Parameter>
///    <Parameter ID="2" Type="Bool" Label="Beep when happy"/>
///</PluginStep>
///```
pub enum Registration {
    Function {
        id: i16,
        name: &'static str,
        definition: &'static str,
        description: &'static str,
        min_args: i16,
        max_args: i16,
        display_in_dialogs: bool,
        compatibility_flags: u32,
        min_ext_version: ExternVersion,
        min_fm_version: &'static str,
        allowed_versions: AllowedVersions,
        function_ptr: fmx_ExtPluginType,
    },
    ScriptStep {
        id: i16,
        name: &'static str,
        definition: &'static str,
        description: &'static str,
        display_in_dialogs: bool,
        compatibility_flags: u32,
        min_ext_version: ExternVersion,
        min_fm_version: &'static str,
        allowed_versions: AllowedVersions,
        function_ptr: fmx_ExtPluginType,
    },
}

impl Registration {
    /// Called automatically by [`register_plugin!`][register_plugin].
    pub fn register(&self, plugin_id: &QuadChar) -> FMError {
        let mut _x = fmx__fmxcpt::new();

        let (id, n, desc, def, display, flags, func_ptr) = match *self {
            Registration::Function {
                id,
                name,
                description,
                definition,
                display_in_dialogs,
                compatibility_flags,
                function_ptr,
                ..
            } => (
                id,
                name,
                description,
                definition,
                display_in_dialogs,
                compatibility_flags,
                function_ptr,
            ),
            Registration::ScriptStep {
                id,
                name,
                description,
                definition,
                display_in_dialogs,
                compatibility_flags,
                function_ptr,
                ..
            } => (
                id,
                name,
                description,
                definition,
                display_in_dialogs,
                compatibility_flags,
                function_ptr,
            ),
        };

        let mut name = Text::new();
        name.assign(n);

        let mut description = Text::new();
        description.assign(desc);

        let mut definition = Text::new();
        definition.assign(def);

        let flags = if display { 0x0000FF00 } else { 0 } | flags;

        let error = match self {
            Registration::Function {
                min_args, max_args, ..
            } => unsafe {
                FM_ExprEnv_RegisterExternalFunctionEx(
                    plugin_id.ptr,
                    id,
                    name.ptr,
                    definition.ptr,
                    description.ptr,
                    *min_args,
                    *max_args,
                    flags,
                    func_ptr,
                    &mut _x,
                )
            },
            Registration::ScriptStep { .. } => unsafe {
                FM_ExprEnv_RegisterScriptStep(
                    plugin_id.ptr,
                    id,
                    name.ptr,
                    definition.ptr,
                    description.ptr,
                    flags,
                    func_ptr,
                    &mut _x,
                )
            },
        };

        _x.check();
        error
    }

    /// Returns minimum allowed sdk version for a function/script step.
    pub fn min_ext_version(&self) -> ExternVersion {
        match self {
            Registration::Function {
                min_ext_version, ..
            } => *min_ext_version,
            Registration::ScriptStep {
                min_ext_version, ..
            } => *min_ext_version,
        }
    }

    /// Returns minimum allowed FileMaker version for a function/script step.
    pub fn min_fm_version(&self) -> &str {
        match self {
            Registration::Function { min_fm_version, .. } => *min_fm_version,
            Registration::ScriptStep { min_fm_version, .. } => *min_fm_version,
        }
    }

    pub fn is_fm_version_allowed(&self, version: &ApplicationVersion) -> bool {
        let allowed_versions = match self {
            Registration::Function {
                allowed_versions, ..
            } => allowed_versions,
            Registration::ScriptStep {
                allowed_versions, ..
            } => allowed_versions,
        };
        use ApplicationVersion::*;
        match version {
            Developer => allowed_versions.developer,
            Pro => allowed_versions.pro,
            Runtime => allowed_versions.runtime,
            SASE => allowed_versions.sase,
            Web => allowed_versions.web,
        }
    }

    /// Called automatically by [`register_plugin!`][register_plugin].
    pub fn unregister(&self, plugin_id: &QuadChar) {
        let mut _x = fmx__fmxcpt::new();
        match self {
            Registration::Function { id, .. } => unsafe {
                FM_ExprEnv_UnRegisterExternalFunction(plugin_id.ptr, *id, &mut _x);
            },
            Registration::ScriptStep { id, .. } => unsafe {
                FM_ExprEnv_UnRegisterScriptStep(plugin_id.ptr, *id, &mut _x);
            },
        }
        _x.check();
    }
}

pub trait FileMakerFunction {
    /// Define your custom function here. Set the return value to the result parameter.
    fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError;

    /// Entry point for FileMaker to call your function.
    extern "C" fn extern_func(
        id: i16,
        env_ptr: *const fmx_ExprEnv,
        args_ptr: *const fmx_DataVect,
        result_ptr: *mut fmx_Data,
    ) -> FMError {
        let arguments = DataVect::from_ptr(args_ptr);
        let env = ExprEnv::from_ptr(env_ptr);
        let mut result = Data::from_ptr(result_ptr);

        Self::function(id, &env, &arguments, &mut result)
    }
}
