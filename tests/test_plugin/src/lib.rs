use chrono::{self, Datelike, Timelike};
use fm_plugin::prelude::*;
use fm_plugin::{
    Data, DataVect, DateTime, ExprEnv, FixPt, Locale, LocaleType, QuadChar, ScriptControl, Text,
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
        },
        Registration::Function {
            id: 800,
            name: "TEST_ExecuteScript",
            definition: "TEST_ExecuteScript ( fileName ; scriptName ; parameter )",
            description: "Test execute script",
            min_args: 3,
            max_args: 3,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestExecuteScript::extern_func),
        },
        Registration::Function {
            id: 900,
            name: "TEST_Date",
            definition: "TEST_Date ( Get ( CurrentHostTimestamp ) )",
            description: "Test date",
            min_args: 1,
            max_args: 1,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestDate::extern_func),
        },
        Registration::Function {
            id: 1000,
            name: "TEST_Time",
            definition: "TEST_Time ( Get ( CurrentHostTimestamp ) )",
            description: "Test time",
            min_args: 1,
            max_args: 1,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestTime::extern_func),
        },
        Registration::Function {
            id: 1100,
            name: "TEST_Timestamp",
            definition: "TEST_Timestamp ( Get ( CurrentHostTimestamp ) )",
            description: "Test timestamp",
            min_args: 1,
            max_args: 1,
            display_in_dialogs: true,
            compatibility_flags: Compatibility::Future as u32,
            min_version: ExternVersion::V160,
            function_ptr: Some(TestTimestamp::extern_func),
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
        result.set_as_text_with_locale("1", locale);
        FMError::NoError
    }
}

struct TextTest;

impl FileMakerFunction for TextTest {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let amazing = String::from("amazing");
        let wow = String::from("wow");

        let mut text = Text::new();
        let arg = args.at_as_text(0);

        if arg.size() != 3 {
            result.set_as_text("size failed");
            return FMError::NoError;
        }

        if arg.to_string() != wow {
            result.set_as_text("text to string failed");
            return FMError::NoError;
        }

        text.assign(&amazing);
        if text.to_string() != amazing {
            result.set_as_text("text assign failed");
            return FMError::NoError;
        }

        text.assign_unicode_with_length(&wow, 3);
        if text.to_string() != wow {
            result.set_as_text("text assign unicode with length failed");
            return FMError::NoError;
        }

        text.assign_wide(&amazing);
        if text.to_string() != amazing {
            result.set_as_text("text assign wide failed");
            return FMError::NoError;
        }

        text.insert(&wow, 0);
        if text.to_string() != String::from("wowamazing") {
            result.set_as_text("text insert failed");
            return FMError::NoError;
        }

        text.append(&amazing);
        if text.to_string() != String::from("wowamazingamazing") {
            result.set_as_text("text append failed");
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

        if num != arg {
            result.set_as_text("from ptr failed");
            return FMError::NoError;
        }

        let clone = arg.clone_precision(12);
        if clone != arg {
            result.set_as_text("clone precision failed");
            return FMError::NoError;
        }

        let new = FixPt::new(13, 0);
        arg.assign(new);
        if i32::from(&arg) != 13 {
            result.set_as_text("fixed pt assign failed");
            return FMError::NoError;
        }

        arg.assign(14);
        if i32::from(&arg) != 14 {
            result.set_as_text("i32 assign failed");
            return FMError::NoError;
        }

        arg.assign(15.001);
        if f64::from(&arg) != 15.001 {
            result.set_as_text("f64 assign failed");
            return FMError::NoError;
        }

        arg.assign(1);
        if bool::from(&arg) != true {
            result.set_as_text("bool assign failed");
            return FMError::NoError;
        }

        arg.set_precision(20);
        let prec = arg.get_precision();
        if prec != 20 {
            result.set_as_text("get/set precision failed");
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let add: FixPt = num + 12;
        if add != 24 {
            result.set_as_text("fixpt add failed");
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let add = num + 12;

        if add != 24 {
            result.set_as_text("i32 add failed");
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let add = num + 12i64;

        if add != 24 {
            result.set_as_text("i64 add failed");
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let sub = num - 6;

        if sub != 6 {
            result.set_as_text("i32 sub failed");
            return FMError::NoError;
        }

        let num = FixPt::new(12, 0);
        let sub = num - 6i64;

        if sub != 6 {
            result.set_as_text("i64 sub failed");
            return FMError::NoError;
        }

        let mut num = FixPt::new(12, 0);
        num += 12;

        if num != 24 {
            result.set_as_text("i32 add assign failed");
            return FMError::NoError;
        }

        let mut num = FixPt::new(12, 0);
        num += 12i64;

        if num != 24 {
            result.set_as_text("i64 add assign failed");
            return FMError::NoError;
        }

        let mut num = FixPt::new(12, 0);
        num -= 6;

        if num != 6 {
            result.set_as_text("i32 sub assign failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(6, 0);
        let num3 = num * num2;

        if num3 != 36 {
            result.set_as_text("multiply failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(6, 0);
        let num3 = &num / &num2;

        if num3 != 1 {
            result.set_as_text("divide failed");
            return FMError::NoError;
        }

        if -num != -6 {
            result.set_as_text("negate failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let rem = num % num2;
        if rem != 0 {
            result.set_as_text("modulo failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        if num > num {
            result.set_as_text("greater than failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        if num < num {
            result.set_as_text("less than failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(7, 0);
        if num >= num2 {
            result.set_as_text("greater than or equal to failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(7, 0);
        if num2 <= num {
            result.set_as_text("less than or equal to failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = FixPt::new(7, 0);
        if num2 == num {
            result.set_as_text("equal to failed");
            return FMError::NoError;
        }

        let num = FixPt::new(6, 0);
        let num2 = num.clone();
        if num2 != num {
            result.set_as_text("clone failed");
            return FMError::NoError;
        }

        result.set_as_number(1);
        FMError::NoError
    }
}

struct TestExecuteScript;

impl FileMakerFunction for TestExecuteScript {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let file_name = args.at_as_text(0);
        let script_name = args.at_as_text(1);
        let parameter = args.at(2);
        execute_filemaker_script(
            file_name,
            script_name,
            ScriptControl::Pause,
            Some(parameter),
        );

        result.set_as_number(1);
        FMError::NoError
    }
}

struct TestDate;

impl FileMakerFunction for TestDate {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let d = args.at_as_date(0);

        result.set_as_date(d);
        FMError::NoError
    }
}

struct TestTime;

impl FileMakerFunction for TestTime {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let t = args.at_as_time(0);

        result.set_as_time(t);
        FMError::NoError
    }
}

struct TestTimestamp;

impl FileMakerFunction for TestTimestamp {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> FMError {
        let ts = DateTime::now();
        let local: chrono::DateTime<chrono::Local> = chrono::Local::now();

        if ts.day() as u32 != local.day() {
            result.set_as_text("day failed");
            return FMError::NoError;
        }

        if ts.month() as u32 != local.month() {
            result.set_as_text("month failed");
            return FMError::NoError;
        }

        if ts.year() as i32 != local.year() {
            result.set_as_text("year failed");
            return FMError::NoError;
        }

        if ts.hours() as u32 != local.hour() {
            result.set_as_text("hour failed");
            return FMError::NoError;
        }

        if ts.minutes() as u32 != local.minute() {
            result.set_as_text("minutes failed");
            return FMError::NoError;
        }

        if ts.seconds() as u32 != local.second() {
            result.set_as_text("seconds failed");
            return FMError::NoError;
        }

        let dt = DateTime::from_str("1/1/2021", Locale::default());
        if &dt.to_string() != "01/01/2021 00:00:00.00" {
            result.set_as_text("from string failed");
            return FMError::NoError;
        }

        let mut dt = DateTime::from_text(Text::from("1/1/2021"), Locale::default());
        if &dt.to_string() != "01/01/2021 00:00:00.00" {
            result.set_as_text("from text failed");
            return FMError::NoError;
        }

        dt.normalize_date_fixed_point(FixPt::from(2020), FixPt::from(12), FixPt::from(31));
        if &dt.to_string() != "12/31/2020 00:00:00.00" {
            result.set_as_text("normalized date fixed point failed");
            return FMError::NoError;
        }

        dt.normalize_date_i16(2019, 6, 15);
        if &dt.to_string() != "06/15/2019 00:00:00.00" {
            result.set_as_text("normalized date i16 failed");
            return FMError::NoError;
        }

        dt.normalize_time_fixed_point(FixPt::from(12), FixPt::from(30), FixPt::from(30));
        if &dt.to_string() != "06/15/2019 12:30:30.00" {
            result.set_as_text("normalized time fixed point failed");
            return FMError::NoError;
        }

        dt.normalize_time_i64(6, 15, 15, 50);
        if &dt.to_string() != "06/15/2019 06:15:15.50" {
            result.set_as_text("normalized time i64 failed");
            return FMError::NoError;
        }

        let dse = dt.get_days_since_epoch();
        if dse != 737225 {
            result.set_as_text("days since epoch failed");
            return FMError::NoError;
        }

        let sse = dt.get_seconds_since_epoch();
        if sse != 63696176115i64 {
            result.set_as_text("seconds since epoch failed: {}");
            return FMError::NoError;
        }

        dt.set_days_since_epoch(5);
        if &dt.to_string() != "01/05/0001 06:15:15.50" {
            result.set_as_text("set days since epoch failed");
            return FMError::NoError;
        }

        dt.set_seconds_since_epoch(FixPt::from(5));
        if &dt.to_string() != "01/01/0001 00:00:05.00" {
            result.set_as_text("set seconds since epoch failed");
            return FMError::NoError;
        }

        dt.set_seconds_since_midnight(FixPt::from(500));
        if &dt.to_string() != "01/01/0001 00:08:20.00" {
            result.set_as_text("set seconds since midnight failed");
            return FMError::NoError;
        }

        let ssm = dt.get_seconds_since_midnight();
        if ssm != 500 {
            result.set_as_text("get seconds since midnight failed");
            return FMError::NoError;
        }

        if dt.is_leap_year() != false {
            result.set_as_text("is leap year failed");
            return FMError::NoError;
        }

        if dt.day_of_week() != 2 {
            result.set_as_text("day of week failed");
            return FMError::NoError;
        }

        if dt.day_of_year() != 1 {
            result.set_as_text("day of year failed");
            return FMError::NoError;
        }

        if dt.week_of_year() != 1 {
            result.set_as_text("week of year failed");
            return FMError::NoError;
        }

        let secs = dt.seconds_fixed_point();
        if secs != 20 {
            result.set_as_text("seconds fixed point failed");
            return FMError::NoError;
        }

        dt.set_date(DateTime::from_str("1/1/2020", Locale::default()));
        if &dt.to_string() != "01/01/2020 00:08:20.00" {
            result.set_as_text("set date failed");
            return FMError::NoError;
        }

        dt.set_time(DateTime::from_str("12:30:30", Locale::default()));
        if &dt.to_string() != "01/01/2020 12:30:30.00" {
            result.set_as_text("set time failed");
            return FMError::NoError;
        }

        if dt != DateTime::from_str("01/01/2020 12:30:30.00", Locale::default()) {
            result.set_as_text("datetime neq failed");
            return FMError::NoError;
        }

        if dt == DateTime::from_str("02/01/2020 12:30:30.00", Locale::default()) {
            result.set_as_text("datetime eq failed");
            return FMError::NoError;
        }

        result.set_as_timestamp(args.at_as_timestamp(0));
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
