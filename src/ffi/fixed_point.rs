use super::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Sub};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_FixPt {
    pub _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {
    pub fn FM_FixPt_AsLong(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> fmx_int32;

    pub fn FM_FixPt_Delete(_self: *mut fmx_FixPt, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_Constructor1(
        val: fmx_int32,
        precision: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_FixPt;

    pub fn FM_FixPt_Constructor2(
        val: fmx_int32,
        precisionExample: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_FixPt;

    pub fn FM_FixPt_AssignInt(_self: *mut fmx_FixPt, that: fmx_int32, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_AssignInt64(_self: *mut fmx_FixPt, that: fmx_int64, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_AssignDouble(_self: *mut fmx_FixPt, that: f64, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_AssignFixPt(
        _self: *mut fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_FixPt_operatorEQ(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_FixPt_operatorNE(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_FixPt_operatorLT(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_FixPt_operatorLE(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_FixPt_operatorGT(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_FixPt_operatorGE(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    pub fn FM_FixPt_Increment(_self: *mut fmx_FixPt, n: fmx_int32, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_Increment64(_self: *mut fmx_FixPt, n: fmx_int64, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_Decrement(_self: *mut fmx_FixPt, n: fmx_int32, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_Decrement64(_self: *mut fmx_FixPt, n: fmx_int64, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_Negate(_self: *mut fmx_FixPt, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_GetPrecision(
        _self: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> ::std::os::raw::c_int;

    pub fn FM_FixPt_SetPrecision(_self: *mut fmx_FixPt, precision: fmx_int32, _x: *mut fmx__fmxcpt);

    pub fn FM_FixPt_Add(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_FixPt_Subtract(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_FixPt_Multiply(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    pub fn FM_FixPt_Divide(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    pub fn FM_FixPt_AsBool(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> bool;

    pub fn FM_FixPt_AsLong64(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> fmx_int64;

    pub fn FM_FixPt_AsFloat(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> f64;
}

#[derive(Eq)]
pub(crate) struct FixPt {
    pub(crate) ptr: *mut fmx_FixPt,
    drop: bool,
}

impl FixPt {
    pub(crate) fn new(val: fmx_int32, precision: fmx_int32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_FixPt_Constructor1(val, precision, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
    pub(crate) fn from_ptr(ptr: *const fmx_FixPt) -> FixPt {
        Self {
            ptr: ptr as *mut fmx_FixPt,
            drop: false,
        }
    }
}

impl Default for FixPt {
    fn default() -> Self {
        FixPt::new(0, 16)
    }
}

impl Drop for FixPt {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_FixPt_Delete(self.ptr as *mut fmx_FixPt, &mut _x) };
            _x.check();
        }
    }
}

impl From<FixPt> for i32 {
    fn from(fix_pt: FixPt) -> i32 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong(fix_pt.ptr, &mut _x) };
        _x.check();
        num
    }
}

impl From<FixPt> for i64 {
    fn from(fix_pt: FixPt) -> i64 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong64(fix_pt.ptr, &mut _x) };
        _x.check();
        num
    }
}

impl From<FixPt> for bool {
    fn from(fix_pt: FixPt) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let b = unsafe { FM_FixPt_AsBool(fix_pt.ptr, &mut _x) };
        _x.check();
        b
    }
}

impl From<FixPt> for f64 {
    fn from(fix_pt: FixPt) -> f64 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsFloat(fix_pt.ptr, &mut _x) };
        _x.check();
        num
    }
}

impl Add for FixPt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let result = FixPt::default();
        unsafe { FM_FixPt_Add(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl Sub for FixPt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let result = FixPt::default();
        unsafe { FM_FixPt_Subtract(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl Mul for FixPt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let result = FixPt::default();
        unsafe { FM_FixPt_Multiply(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl Div for FixPt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let result = FixPt::default();
        unsafe { FM_FixPt_Divide(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl PartialEq for FixPt {
    fn eq(&self, other: &FixPt) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_FixPt_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl PartialOrd for FixPt {
    fn partial_cmp(&self, other: &FixPt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FixPt {
    fn cmp(&self, other: &FixPt) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        let mut _x = fmx__fmxcpt::new();
        let gt = unsafe { FM_FixPt_operatorGT(self.ptr, other.ptr, &mut _x) };
        _x.check();
        match gt {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}
