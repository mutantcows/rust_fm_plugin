use super::*;
use std::ffi::CString;
use widestring::{U16CString, WideCString};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_DateTime {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {

    fn FM_DateTime_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_DateTime;

    fn FM_DateTime_Constructor2(
        dateString: *const fmx_unichar16,
        dateLength: fmx_uint32,
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

    fn FM_DateTime_DayOfWeek(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_DayOfYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_WeekOfYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_Now(_self: *mut fmx_DateTime, _x: *mut fmx__fmxcpt);

    fn FM_DateTime_SetDate(
        _self: *mut fmx_DateTime,
        datetime: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_DateTime_SetNormalizedDate1(
        _self: *mut fmx_DateTime,
        month: fmx_int16,
        day: fmx_int16,
        year: fmx_int16,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_DateTime_SetNormalizedDate2(
        _self: *mut fmx_DateTime,
        year: *const fmx_FixPt,
        month: *const fmx_FixPt,
        day: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

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
        minute: fmx_int16,
        sec: fmx_int16,
        usec: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_DateTime_SetNormalizedTime2(
        _self: *mut fmx_DateTime,
        hour: *const fmx_FixPt,
        minute: *const fmx_FixPt,
        sec: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

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

    fn FM_DateTime_GetYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_GetMonth(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_GetDay(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_GetDaysSinceEpoch(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt)
        -> fmx_int32;

    fn FM_DateTime_GetHour(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int32;

    fn FM_DateTime_GetMinute(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_GetSec(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    fn FM_DateTime_GetUSec(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int32;

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

    pub fn from_str(date: &str, locale: Locale) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let date_len = date.len() as u32;
        let date = U16CString::from_str(date).unwrap();
        let date_ptr = date.as_ptr();
        let ptr = unsafe { FM_DateTime_Constructor2(date_ptr, date_len, locale.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_text(text: Text, locale: Locale) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DateTime_Constructor3(text.ptr, locale.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn normalize_fixed_point(&mut self, year: FixPt, month: FixPt, day: FixPt) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe {
            FM_DateTime_SetNormalizedDate2(self.ptr, year.ptr, month.ptr, day.ptr, &mut _x)
        };
        _x.check();
        if error != 0 {
            panic!();
        }
    }
    pub fn normalize_i32(&mut self, year: i16, month: i16, day: i16) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe { FM_DateTime_SetNormalizedDate1(self.ptr, year, month, day, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }

    pub fn days_since_epoch(&mut self, days: i64) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_SetDaysSinceEpoch(self.ptr, days, &mut _x) };
        _x.check();
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

    pub fn now(&mut self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_Now(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn set_date(&mut self, datetime: DateTime) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_SetDate(self.ptr, datetime.ptr, &mut _x) };
        _x.check();
    }

    pub fn set_time(&mut self, datetime: DateTime) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_DateTime_SetTime(self.ptr, datetime.ptr, &mut _x) };
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
