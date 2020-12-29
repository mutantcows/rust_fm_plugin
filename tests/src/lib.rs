use fm_plugin::prelude::*;
use fm_plugin::{Data, DataVect, ExprEnv, ScriptControl, Text};

struct TestPlugin;

impl Plugin for TestPlugin {
    fn id() -> &'static [u8; 4] {
        &b"TEST"
    }

    fn name() -> &'static str {
        "TEST Plugin"
    }

    fn description() -> &'static str {
        "Test plug-in"
    }

    fn url() -> &'static str {
        "http://test.test"
    }

    fn enable_idle() -> bool {
        false
    }

    fn idle(_session_id: fmx_ptrtype) {}

    fn not_idle(_session_id: fmx_ptrtype) {}

    fn register_functions() -> Vec<Registration> {
        vec![Registration::Function {
            id: 100,
            name: "TEST_Function",
            definition: "TEST_Function( number ; text )",
            description: "Test function",
            min_args: 2,
            max_args: 2,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestFunction::extern_func),
        },
        Registration::ScriptStep{
            id: 200,
            name: "TEST_ScriptStep",
            definition:
            "<PluginStep>
                <Parameter Type=\"target\" Label=\"Target\" ShowInLine=\"true\"/>
                <Parameter Type=\"calc\" DataType=\"text\" ShowInline=\"true\" Label=\"Text\" ID=\"0\"/>
                <Parameter Type=\"calc\" DataType=\"number\" ShowInline=\"true\" Label=\"Number\" ID=\"1\"/>
            </PluginStep>",
            description: "Test script step.",
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestScriptStep::extern_func),
        },
]
    }
}

struct TestFunction;

impl FileMakerFunction for TestFunction {
    fn function(_id: i16, _env: &ExprEnv, _args: &DataVect, _result: &mut Data) -> FMError {
        FMError::NoError
    }
}

struct TestScriptStep;

impl FileMakerFunction for TestScriptStep {
    fn function(_id: i16, _env: &ExprEnv, _args: &DataVect, _result: &mut Data) -> FMError {
        FMError::NoError
    }
}

register_plugin!(TestPlugin);
