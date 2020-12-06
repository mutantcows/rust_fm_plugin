use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_DateTime {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {

    pub fn FM_DateTime_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_DateTime;

    pub fn FM_DateTime_Constructor2(
        dateString: *const fmx_unichar16,
        dateLength: fmx_uint32,
        intl: *const fmx_Locale,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_DateTime;

    pub fn FM_DateTime_Constructor3(
        dateText: *const fmx_Text,
        intl: *const fmx_Locale,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_DateTime;

    pub fn FM_DateTime_operatorEQ(
        _self: *const fmx_DateTime,
        that: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_DateTime_operatorNE(
        _self: *const fmx_DateTime,
        that: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_DateTime_IsLeapYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> bool;

    pub fn FM_DateTime_DayOfWeek(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_DayOfYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_WeekOfYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_Now(_self: *mut fmx_DateTime, _x: *mut fmx__fmxcpt);

    pub fn FM_DateTime_SetDate(
        _self: *mut fmx_DateTime,
        datetime: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_SetNormalizedDate1(
        _self: *mut fmx_DateTime,
        month: fmx_int16,
        day: fmx_int16,
        year: fmx_int16,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_DateTime_SetNormalizedDate2(
        _self: *mut fmx_DateTime,
        year: *const fmx_FixPt,
        month: *const fmx_FixPt,
        day: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_DateTime_SetDaysSinceEpoch(
        _self: *mut fmx_DateTime,
        days: fmx_int64,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_SetTime(
        _self: *mut fmx_DateTime,
        datetime: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_SetNormalizedTime1(
        _self: *mut fmx_DateTime,
        hour: fmx_int64,
        minute: fmx_int16,
        sec: fmx_int16,
        usec: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_DateTime_SetNormalizedTime2(
        _self: *mut fmx_DateTime,
        hour: *const fmx_FixPt,
        minute: *const fmx_FixPt,
        sec: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_DateTime_SetSecsSinceMidnight(
        _self: *mut fmx_DateTime,
        secs: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_SetSecondsSinceEpoch(
        _self: *mut fmx_DateTime,
        seconds: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_GetYear(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_GetMonth(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_GetDay(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_GetDaysSinceEpoch(
        _self: *const fmx_DateTime,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_int32;

    pub fn FM_DateTime_GetHour(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int32;

    pub fn FM_DateTime_GetMinute(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_GetSec(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int16;

    pub fn FM_DateTime_GetUSec(_self: *const fmx_DateTime, _x: *mut fmx__fmxcpt) -> fmx_int32;

    pub fn FM_DateTime_GetSeconds(
        _self: *const fmx_DateTime,
        results: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_GetSecsSinceMidnight(
        _self: *const fmx_DateTime,
        results: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_GetSecondsSinceEpoch(
        _self: *const fmx_DateTime,
        results: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_DateTime_Delete(_self: *mut fmx_DateTime, _x: *mut fmx__fmxcpt);
}

pub(crate) struct DateTime {
    pub(crate) ptr: *mut fmx_DateTime,
    drop: bool,
}

impl DateTime {
    pub(crate) fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_DateTime_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub(crate) fn from_ptr(ptr: *const fmx_DateTime) -> Self {
        Self {
            ptr: ptr as *mut fmx_DateTime,
            drop: false,
        }
    }
}
