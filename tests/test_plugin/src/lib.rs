use fm_plugin::prelude::*;
use fm_plugin::{
    Data, DataVect, ExprEnv, FixPt, Locale, LocaleType, QuadChar, ScriptControl, Text,
};
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
            name: "TEST_Locale",
            definition: "TEST_Locale",
            description: "Test locale",
            min_args: 0,
            max_args: 0,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(LocaleTest::extern_func),
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
        Registration::Function {
            id: 500,
            name: "TEST_QuadChar",
            definition: "TEST_QuadChar",
            description: "Test quadchar",
            min_args: 0,
            max_args: 0,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(QuadCharTest::extern_func),
        },
        Registration::Function {
            id: 600,
            name: "TEST_Text",
            definition: "TEST_Text ( \"wow\" )",
            description: "Test text",
            min_args: 1,
            max_args: 1,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TextTest::extern_func),
        },
        Registration::Function {
            id: 700,
            name: "TEST_FixedPoint",
            definition: "TEST_FixedPoint ( 12 )",
            description: "Test fixed point",
            min_args: 1,
            max_args: 1,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(FixPtTest::extern_func),
        }]
    }
}

struct QuadCharTest;

impl FileMakerFunction for QuadCharTest {
    fn function(_id: i16, _env: &ExprEnv, _args: &DataVect, result: &mut Data) -> FMError {
        let quadchar = QuadChar::new(b"1234");
        let quad_str = quadchar.to_string();
        assert_eq!(String::from("1234"), quad_str);
        let _ = QuadChar::empty();
        result.set_as_number(1);
        FMError::NoError
    }
}

struct LocaleTest;

impl FileMakerFunction for LocaleTest {
    fn function(_id: i16, _env: &ExprEnv, _args: &DataVect, result: &mut Data) -> FMError {
        let locale = Locale::new(LocaleType::JPN);
        let _ = result.get_locale();
        result.set_as_text("1", locale);
        FMError::NoError
    }
}

struct TextTest;

impl FileMakerFunction for TextTest {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let amazing = String::from("amazing");
        let wow = String::from("wow");
        let locale = result.get_locale();

        let mut text = Text::new();
        let arg = args.at_as_text(0);

        if arg.size() != 3 {
            result.set_as_text("size failed", locale);
            return FMError::NoError;
        }

        if arg.to_string() != wow {
            result.set_as_text("text to string failed", locale);
            return FMError::NoError;
        }

        text.assign(&amazing);
        if text.to_string() != amazing {
            result.set_as_text("text assign failed", locale);
            return FMError::NoError;
        }

        text.assign_unicode_with_length(&wow, 3);
        if text.to_string() != wow {
            result.set_as_text("text assign unicode with length failed", locale);
            return FMError::NoError;
        }

        text.assign_wide(&amazing);
        if text.to_string() != amazing {
            result.set_as_text("text assign wide failed", locale);
            return FMError::NoError;
        }

        text.insert(&wow, 0);
        if text.to_string() != String::from("wowamazing") {
            result.set_as_text("text insert failed", locale);
            return FMError::NoError;
        }

        text.append(&amazing);
        if text.to_string() != String::from("wowamazingamazing") {
            result.set_as_text("text append failed", locale);
            return FMError::NoError;
        }

        result.set_as_number(1);
        FMError::NoError
    }
}

struct FixPtTest;

impl FileMakerFunction for FixPtTest {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let num = FixPt::new(12, 0);
        let mut arg = args.at_as_number(0);
        let locale = result.get_locale();

        if num != arg {
            result.set_as_text("from ptr failed", locale);
            return FMError::NoError;
        }

        let clone = arg.clone_precision(12);
        if clone != arg {
            result.set_as_text("clone precision failed", locale);
            return FMError::NoError;
        }

        let new = FixPt::new(13, 0);
        arg.assign(new);
        if i32::from(&arg) != 13 {
            result.set_as_text("fixed pt assign failed", locale);
            return FMError::NoError;
        }

        arg.assign(14);
        if i32::from(&arg) != 14 {
            result.set_as_text("i32 assign failed", locale);
            return FMError::NoError;
        }

        arg.assign(15.001);
        if f64::from(&arg) != 15.001 {
            result.set_as_text("f64 assign failed", locale);
            return FMError::NoError;
        }

        arg.assign(1);
        if bool::from(&arg) != true {
            result.set_as_text("bool assign failed", locale);
            return FMError::NoError;
        }

        arg.set_precision(20);
        let prec = arg.get_precision();
        if prec != 20 {
            result.set_as_text(format!("get/set precision failed: {}", prec), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let add: FixPt = num + 12;
        if add != 24 {
            result.set_as_text(format!("fixpt add failed: {}", add), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let add = num + 12;

        if add != 24 {
            result.set_as_text(format!("i32 add failed: {}", add), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let add = num + 12i64;

        if add != 24 {
            result.set_as_text(format!("i64 add failed: {}", add), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let sub = num - 6;

        if sub != 6 {
            result.set_as_text(format!("i32 sub failed: {}", sub), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let sub = num - 6i64;

        if sub != 6 {
            result.set_as_text(format!("i64 sub failed: {}", sub), locale);
            return FMError::NoError;
        }

        let mut num = FixPt::new(12, 0);
        num += 12;

        if num != 24 {
            result.set_as_text(format!("i32 add assign failed: {}", sub), locale);
            return FMError::NoError;
        }

        let mut num = FixPt::new(12, 0);
        num += 12i64;

        if num != 24 {
            result.set_as_text(format!("i64 add assign failed: {}", sub), locale);
            return FMError::NoError;
        }

        let mut num = FixPt::new(12, 0);
        num -= 6;

        if num != 6 {
            result.set_as_text(format!("i32 sub assign failed: {}", sub), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(6, 0);
        let num3 = num * num2;

        if num3 != 6 {
            result.set_as_text(format!("multiply failed: {}", num3), locale);
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(6, 0);
        let num3 = &num / &num2;

        if num3 != 1 {
            result.set_as_text(format!("divide failed: {}", num3), locale);
            return FMError::NoError;
        }

        if -num != -6 {
            result.set_as_text(format!("negate failed: {}", num3), locale);
            return FMError::NoError;
        }

        let rem = num % num2;
        if rem != 0 {
            result.set_as_text(format!("negate failed: {}", num3), locale);
            return FMError::NoError;
        }

        result.set_as_number(1);
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
        if let Ok(mut stream) = TcpStream::connect(address) {
            if stream.write(msg.as_bytes()).is_err() {
                return FMError::ConnectionFailed;
            };
        } else {
            return FMError::ConnectionFailed;
        }
        FMError::NoError
    }
}

register_plugin!(TestPlugin);
