use super::*;
use std::fmt;
use widestring::U16CString;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_DateTime {
    _address: u8,
}

#[cfg_attr(target_os = "macos", link(kind = "framework", name = "FMWrapper"))]
#[cfg_attr(target_os = "windows", link(kind = "static", name = "FMWrapper"))]
#[cfg_attr(target_os = "linux", link(kind = "dylib", name = "FMWrapper"))]
extern "C" {

    fn FM_DateTime_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_DateTime;

    fn FM_DateTime_Constructor2(
        dateString: *const u16,
        dateLength: u32,
        intl: *const fmx_Locale,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_DateTime;

    fn FM_DateTime_Constructor3(
        dateText: *const fmx_Text,
        intl: *const fmx_Locale,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_DateTime;

    fn FM_DateTime_operatorEQ(
        _self: *const fmx_DateTime,
        that: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_DateTime_operatorNE(
        _self: *const fmx_DateTime,
        that: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_DateTime_IsLeapYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> bool;

    fn FM_DateTime_DayOfWeek(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_DayOfYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_WeekOfYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_Now(_self: *mut fmx_DateTime, _x: *mut fmx__fmxcpt);

    fn FM_DateTime_SetDate(
        _self: *mut fmx_DateTime,
        datetime: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_SetNormalizedDate1(
        _self: *mut fmx_DateTime,
        month: i16,
        day: i16,
        year: i16,
        _x: *mut fmx__fmxcpt,
    ) -> FMError;

    fn FM_DateTime_SetNormalizedDate2(
        _self: *mut fmx_DateTime,
        year: *const fmx_FixPt,
        month: *const fmx_FixPt,
        day: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> FMError;

    fn FM_DateTime_SetDaysSinceEpoch(
        _self: *mut fmx_DateTime,
        days: fmx_int64,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_SetTime(
        _self: *mut fmx_DateTime,
        datetime: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_SetNormalizedTime1(
        _self: *mut fmx_DateTime,
        hour: fmx_int64,
        minute: i16,
        sec: i16,
        usec: i32,
        _x: *mut fmx__fmxcpt,
    ) -> FMError;

    fn FM_DateTime_SetNormalizedTime2(
        _self: *mut fmx_DateTime,
        hour: *const fmx_FixPt,
        minute: *const fmx_FixPt,
        sec: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> FMError;

    fn FM_DateTime_SetSecsSinceMidnight(
        _self: *mut fmx_DateTime,
        secs: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_SetSecondsSinceEpoch(
        _self: *mut fmx_DateTime,
        seconds: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_GetYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_GetMonth(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_GetDay(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_GetDaysSinceEpoch(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i32;

    fn FM_DateTime_GetHour(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i32;

    fn FM_DateTime_GetMinute(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_GetSec(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i16;

    fn FM_DateTime_GetUSec(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> i32;

    fn FM_DateTime_GetSeconds(
        _self: *const fmx_DateTime,
        results: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_GetSecsSinceMidnight(
        _self: *const fmx_DateTime,
        results: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_GetSecondsSinceEpoch(
        _self: *const fmx_DateTime,
        results: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_Delete(_self: *mut fmx_DateTime, _x: *mut fmx__fmxcpt);
}

#[derive(Debug)]
pub struct DateTime {
    pub(crate) ptr: *mut fmx_DateTime,
    drop: bool,
}

impl DateTime {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DateTime_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_str(date: &str) -> Self {
        Self::from_str_with_locale(date, Locale::default())
    }

    pub fn from_str_with_locale(date: &str, locale: Locale) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let date_len = date.len() as u32;
        let date = U16CString::from_str(date).unwrap();
        let date_ptr = date.as_ptr();
        let ptr = unsafe { FM_DateTime_Constructor2(date_ptr, date_len, locale.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_text(text: Text) -> Self {
        Self::from_text_with_locale(text, Locale::default())
    }

    pub fn from_text_with_locale(text: Text, locale: Locale) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DateTime_Constructor3(text.ptr, locale.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn normalize_date_fixed_point(&mut self, year: FixPt, month: FixPt, day: FixPt) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe {
            FM_DateTime_SetNormalizedDate2(self.ptr, year.ptr, month.ptr, day.ptr, &mut _x)
        };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
    }

    pub fn normalize_date_i16(&mut self, year: i16, month: i16, day: i16) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe { FM_DateTime_SetNormalizedDate1(self.ptr, month, day, year, &mut _x) };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
    }

    pub fn normalize_time_fixed_point(&mut self, hours: FixPt, minutes: FixPt, seconds: FixPt) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe {
            FM_DateTime_SetNormalizedTime2(self.ptr, hours.ptr, minutes.ptr, seconds.ptr, &mut _x)
        };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
    }

    pub fn normalize_time_i64(
        &mut self,
        hours: i64,
        minutes: i16,
        seconds: i16,
        milliseconds: i32,
    ) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe {
            FM_DateTime_SetNormalizedTime1(self.ptr, hours, minutes, seconds, milliseconds, &mut _x)
        };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
    }

    pub fn get_days_since_epoch(&mut self) -> i32 {
        let mut _x = fmx__fmxcpt::new();
        let days = unsafe { FM_DateTime_GetDaysSinceEpoch(self.ptr, &mut _x) };
        _x.check();
        days
    }

    pub fn get_seconds_since_epoch(&mut self) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let results = FixPt::default();
        unsafe { FM_DateTime_GetSecondsSinceEpoch(self.ptr, results.ptr, &mut _x) };
        _x.check();
        results
    }

    pub fn set_days_since_epoch(&mut self, days: i64) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_SetDaysSinceEpoch(self.ptr, days, &mut _x) };
        _x.check();
    }

    pub fn set_seconds_since_epoch(&mut self, seconds: FixPt) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_SetSecondsSinceEpoch(self.ptr, seconds.ptr, &mut _x) };
        _x.check();
    }

    pub fn set_seconds_since_midnight(&mut self, seconds: FixPt) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_SetSecsSinceMidnight(self.ptr, seconds.ptr, &mut _x) };
        _x.check();
    }

    pub fn get_seconds_since_midnight(&mut self) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let results = FixPt::default();
        unsafe { FM_DateTime_GetSecsSinceMidnight(self.ptr, results.ptr, &mut _x) };
        _x.check();
        results
    }

    pub fn from_ptr(ptr: *const fmx_DateTime) -> Self {
        Self {
            ptr: ptr as *mut fmx_DateTime,
            drop: false,
        }
    }

    pub fn is_leap_year(&self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_DateTime_IsLeapYear(self.ptr, &mut _x) };
        _x.check();
        result
    }

    pub fn day_of_week(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let day = unsafe { FM_DateTime_DayOfWeek(self.ptr, &mut _x) };
        _x.check();
        day
    }

    pub fn day_of_year(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let day = unsafe { FM_DateTime_DayOfYear(self.ptr, &mut _x) };
        _x.check();
        day
    }

    pub fn week_of_year(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let day = unsafe { FM_DateTime_WeekOfYear(self.ptr, &mut _x) };
        _x.check();
        day
    }

    pub fn now() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ts = Self::new();
        unsafe { FM_DateTime_Now(ts.ptr, &mut _x) };
        _x.check();
        ts
    }

    pub fn hours(&self) -> i32 {
        let mut _x = fmx__fmxcpt::new();
        let hours = unsafe { FM_DateTime_GetHour(self.ptr, &mut _x) };
        _x.check();
        hours
    }

    pub fn minutes(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let minutes = unsafe { FM_DateTime_GetMinute(self.ptr, &mut _x) };
        _x.check();
        minutes
    }

    pub fn seconds(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let seconds = unsafe { FM_DateTime_GetSec(self.ptr, &mut _x) };
        _x.check();
        seconds
    }

    pub fn milliseconds(&self) -> i32 {
        let mut _x = fmx__fmxcpt::new();
        let milliseconds = unsafe { FM_DateTime_GetUSec(self.ptr, &mut _x) };
        _x.check();
        milliseconds
    }

    pub fn seconds_fixed_point(&self) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let results = FixPt::default();
        unsafe { FM_DateTime_GetSeconds(self.ptr, results.ptr, &mut _x) };
        _x.check();
        results
    }

    pub fn day(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let day = unsafe { FM_DateTime_GetDay(self.ptr, &mut _x) };
        _x.check();
        day
    }

    pub fn month(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let month = unsafe { FM_DateTime_GetMonth(self.ptr, &mut _x) };
        _x.check();
        month
    }

    pub fn year(&self) -> i16 {
        let mut _x = fmx__fmxcpt::new();
        let year = unsafe { FM_DateTime_GetYear(self.ptr, &mut _x) };
        _x.check();
        year
    }

    pub fn set_date<D: ToDateTime>(&mut self, datetime: D) {
        let mut _x = fmx__fmxcpt::new();
        let dt = datetime.to_datetime();
        unsafe { FM_DateTime_SetDate(self.ptr, dt.ptr, &mut _x) };
        _x.check();
    }

    pub fn set_time<D: ToDateTime>(&mut self, datetime: D) {
        let mut _x = fmx__fmxcpt::new();
        let dt = datetime.to_datetime();
        unsafe { FM_DateTime_SetTime(self.ptr, dt.ptr, &mut _x) };
        _x.check();
    }
}

impl Drop for DateTime {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_DateTime_Delete(self.ptr as *mut fmx_DateTime, &mut _x) };
            _x.check();
        }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &DateTime) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_DateTime_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &DateTime) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_DateTime_operatorNE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl From<String> for DateTime {
    fn from(string: String) -> Self {
        DateTime::from_str(&string)
    }
}

impl From<&str> for DateTime {
    fn from(string: &str) -> Self {
        DateTime::from_str(string)
    }
}

impl From<Text> for DateTime {
    fn from(text: Text) -> Self {
        DateTime::from_text(text)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}/{:02}/{:04} {:02}:{:02}:{:02}.{:02}",
            self.month(),
            self.day(),
            self.year(),
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.milliseconds()
        )
    }
}

pub trait ToDateTime {
    fn to_datetime(self) -> DateTime;
}

impl ToDateTime for String {
    fn to_datetime(self) -> DateTime {
        DateTime::from_str(&self)
    }
}

impl ToDateTime for &str {
    fn to_datetime(self) -> DateTime {
        DateTime::from_str(self)
    }
}

impl ToDateTime for DateTime {
    fn to_datetime(self) -> DateTime {
        self
    }
}
