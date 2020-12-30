use fm_plugin::prelude::*;
use fm_plugin::{Data, DataVect, ExprEnv, ScriptControl, Text};
use std::io::prelude::*;
use std::net::TcpStream;

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
        Registration::Function {
            id: 300,
            name: "TEST_Version",
            definition: "TEST_Version",
            description: "Test plugin version",
            min_args: 0,
            max_args: 0,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestVersion::extern_func),
        },
        Registration::Function {
            id: 400,
            name: "TEST_Socket",
            definition: "TEST_Socket ( address ; message )",
            description: "Send message over socket.",
            min_args: 2,
            max_args: 2,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestSocket::extern_func),
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

struct TestVersion;

impl FileMakerFunction for TestVersion {
    fn function(_id: i16, _env: &ExprEnv, _args: &DataVect, result: &mut Data) -> FMError {
        result.set_as_number(1);
        FMError::NoError
    }
}

struct TestSocket;

impl FileMakerFunction for TestSocket {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, _result: &mut Data) -> FMError {
        let address = args.at_as_string(0);
        let msg = args.at_as_string(1);
        let mut stream = TcpStream::connect(address).unwrap();

        stream.write(msg.as_bytes()).unwrap();
        FMError::NoError
    }
}

register_plugin!(TestPlugin);
