use super::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_FixPt {
    _address: u8,
}

#[cfg_attr(target_os = "macos", link(kind = "framework", name = "FMWrapper"))]
#[cfg_attr(target_os = "windows", link(kind = "static", name = "FMWrapper"))]
#[cfg_attr(target_os = "linux", link(kind = "dylib", name = "FMWrapper"))]
extern "C" {
    fn FM_FixPt_AsLong(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> i32;

    fn FM_FixPt_Delete(_self: *mut fmx_FixPt, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_Constructor1(val: i32, precision: i32, _x: *mut fmx__fmxcpt) -> *mut fmx_FixPt;

    fn FM_FixPt_Constructor2(
        val: i32,
        precisionExample: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_FixPt;

    fn FM_FixPt_AssignInt(_self: *mut fmx_FixPt, that: i32, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_AssignInt64(_self: *mut fmx_FixPt, that: fmx_int64, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_AssignDouble(_self: *mut fmx_FixPt, that: f64, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_AssignFixPt(_self: *mut fmx_FixPt, that: *const fmx_FixPt, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_operatorEQ(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_FixPt_operatorNE(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_FixPt_operatorLT(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_FixPt_operatorLE(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_FixPt_operatorGT(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_FixPt_operatorGE(
        _self: *const fmx_FixPt,
        that: *const fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_FixPt_Increment(_self: *mut fmx_FixPt, n: i32, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_Increment64(_self: *mut fmx_FixPt, n: fmx_int64, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_Decrement(_self: *mut fmx_FixPt, n: i32, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_Decrement64(_self: *mut fmx_FixPt, n: fmx_int64, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_Negate(_self: *mut fmx_FixPt, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_GetPrecision(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> i32;

    fn FM_FixPt_SetPrecision(_self: *mut fmx_FixPt, precision: i32, _x: *mut fmx__fmxcpt);

    fn FM_FixPt_Add(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_FixPt_Subtract(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_FixPt_Multiply(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_FixPt_Divide(
        _self: *const fmx_FixPt,
        arg: *const fmx_FixPt,
        result: *mut fmx_FixPt,
        _x: *mut fmx__fmxcpt,
    ) -> FMError;

    fn FM_FixPt_AsBool(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> bool;

    fn FM_FixPt_AsLong64(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> fmx_int64;

    fn FM_FixPt_AsFloat(_self: *const fmx_FixPt, _x: *mut fmx__fmxcpt) -> f64;
}

#[derive(Eq)]
pub struct FixPt {
    pub(crate) ptr: *mut fmx_FixPt,
    drop: bool,
}

impl FixPt {
    pub fn new(val: i32, precision: i32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_FixPt_Constructor1(val, precision, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
    pub fn from_ptr(ptr: *const fmx_FixPt) -> FixPt {
        Self {
            ptr: ptr as *mut fmx_FixPt,
            drop: false,
        }
    }

    pub fn clone_precision(&self, val: i32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_FixPt_Constructor2(val, self.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn assign<T: ToFixPt>(&mut self, number: T) {
        let mut _x = fmx__fmxcpt::new();
        let fixed_point = number.to_fixed_point();
        unsafe { FM_FixPt_AssignFixPt(self.ptr, fixed_point.ptr, &mut _x) };
        _x.check();
    }

    pub fn get_precision(&self) -> i32 {
        let mut _x = fmx__fmxcpt::new();
        let precision = unsafe { FM_FixPt_GetPrecision(self.ptr, &mut _x) };
        _x.check();
        precision
    }

    ///Valid values between 16 and 400
    pub fn set_precision(&mut self, precision: i32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_SetPrecision(self.ptr, precision, &mut _x) };
        _x.check();
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

impl From<FixPt> for u32 {
    fn from(fix_pt: FixPt) -> u32 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong(fix_pt.ptr, &mut _x) };
        _x.check();
        num as u32
    }
}

impl From<&FixPt> for i32 {
    fn from(fix_pt: &FixPt) -> i32 {
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
        num as i64
    }
}

impl From<&FixPt> for i64 {
    fn from(fix_pt: &FixPt) -> i64 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong64(fix_pt.ptr, &mut _x) };
        _x.check();
        num as i64
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

impl From<&FixPt> for bool {
    fn from(fix_pt: &FixPt) -> bool {
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

impl From<&FixPt> for f64 {
    fn from(fix_pt: &FixPt) -> f64 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsFloat(fix_pt.ptr, &mut _x) };
        _x.check();
        num
    }
}

impl From<f64> for FixPt {
    fn from(num: f64) -> FixPt {
        let fixed_pt = FixPt::default();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_AssignDouble(fixed_pt.ptr, num, &mut _x) };
        _x.check();
        fixed_pt
    }
}

impl From<i32> for FixPt {
    fn from(num: i32) -> FixPt {
        let fixed_pt = FixPt::default();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_AssignInt(fixed_pt.ptr, num, &mut _x) };
        _x.check();
        fixed_pt
    }
}

impl From<u32> for FixPt {
    fn from(num: u32) -> FixPt {
        let fixed_pt = FixPt::default();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_AssignInt(fixed_pt.ptr, num as i32, &mut _x) };
        _x.check();
        fixed_pt
    }
}

impl From<i64> for FixPt {
    fn from(num: i64) -> FixPt {
        let fixed_pt = FixPt::default();
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_AssignInt64(fixed_pt.ptr, num as fmx_int64, &mut _x) };
        _x.check();
        fixed_pt
    }
}

pub trait ToFixPt {
    fn to_fixed_point(self) -> FixPt;
}

impl ToFixPt for FixPt {
    fn to_fixed_point(self) -> Self {
        self
    }
}

impl ToFixPt for i32 {
    fn to_fixed_point(self) -> FixPt {
        FixPt::from(self)
    }
}

impl ToFixPt for u32 {
    fn to_fixed_point(self) -> FixPt {
        FixPt::from(self)
    }
}

impl ToFixPt for f64 {
    fn to_fixed_point(self) -> FixPt {
        FixPt::from(self)
    }
}

impl ToFixPt for i64 {
    fn to_fixed_point(self) -> FixPt {
        FixPt::from(self)
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

impl Add<i32> for FixPt {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Increment(self.ptr, other, &mut _x) };
        _x.check();
        self
    }
}

impl Add<FixPt> for i32 {
    type Output = FixPt;

    fn add(self, other: FixPt) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Increment(other.ptr, self, &mut _x) };
        _x.check();
        other
    }
}

impl Add<i64> for FixPt {
    type Output = Self;

    fn add(self, other: i64) -> Self {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Increment64(self.ptr, other as fmx_int64, &mut _x) };
        _x.check();
        self
    }
}

impl AddAssign for FixPt {
    fn add_assign(&mut self, other: Self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Increment(self.ptr, other.into(), &mut _x) };
        _x.check();
    }
}

impl AddAssign<i32> for FixPt {
    fn add_assign(&mut self, other: i32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Increment(self.ptr, other, &mut _x) };
        _x.check();
    }
}

impl AddAssign<i64> for FixPt {
    fn add_assign(&mut self, other: i64) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Increment64(self.ptr, other as fmx_int64, &mut _x) };
        _x.check();
    }
}

impl SubAssign for FixPt {
    fn sub_assign(&mut self, other: Self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Decrement(self.ptr, other.into(), &mut _x) };
        _x.check();
    }
}

impl SubAssign<i32> for FixPt {
    fn sub_assign(&mut self, other: i32) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Decrement(self.ptr, other, &mut _x) };
        _x.check();
    }
}

impl SubAssign<i64> for FixPt {
    fn sub_assign(&mut self, other: i64) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Decrement64(self.ptr, other as fmx_int64, &mut _x) };
        _x.check();
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

impl Sub<i32> for FixPt {
    type Output = FixPt;

    fn sub(self, other: i32) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Decrement(self.ptr, other, &mut _x) };
        _x.check();
        self
    }
}

impl Sub<i64> for FixPt {
    type Output = FixPt;

    fn sub(self, other: i64) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Decrement64(self.ptr, other as fmx_int64, &mut _x) };
        _x.check();
        self
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

impl Mul<&FixPt> for &FixPt {
    type Output = FixPt;

    fn mul(self, other: &FixPt) -> FixPt {
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
        let error = unsafe { FM_FixPt_Divide(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
        result
    }
}

impl Div<&FixPt> for &FixPt {
    type Output = FixPt;

    fn div(self, other: &FixPt) -> FixPt {
        let mut _x = fmx__fmxcpt::new();
        let result = FixPt::default();
        let error = unsafe { FM_FixPt_Divide(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
        result
    }
}

impl Rem for FixPt {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let result = FixPt::default();
        let error = unsafe { FM_FixPt_Divide(self.ptr, other.ptr, result.ptr, &mut _x) };
        _x.check();
        if error != FMError::NoError {
            panic!();
        }
        other - (self * result)
    }
}

impl Neg for FixPt {
    type Output = Self;

    fn neg(self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_FixPt_Negate(self.ptr, &mut _x) };
        _x.check();
        self
    }
}

impl PartialEq for FixPt {
    fn eq(&self, other: &FixPt) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_FixPt_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &FixPt) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_FixPt_operatorNE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}

impl PartialEq<i32> for FixPt {
    fn eq(&self, other: &i32) -> bool {
        i32::from(self) == *other
    }
}

impl PartialEq<i64> for FixPt {
    fn eq(&self, other: &i64) -> bool {
        i64::from(self) == *other
    }
}

impl PartialOrd for FixPt {
    fn partial_cmp(&self, other: &FixPt) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let lt = unsafe { FM_FixPt_operatorLT(self.ptr, other.ptr, &mut _x) };
        _x.check();
        lt
    }

    fn le(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let le = unsafe { FM_FixPt_operatorLE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        le
    }

    fn gt(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let gt = unsafe { FM_FixPt_operatorGT(self.ptr, other.ptr, &mut _x) };
        _x.check();
        gt
    }

    fn ge(&self, other: &Self) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let ge = unsafe { FM_FixPt_operatorGE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        ge
    }
}

impl Ord for FixPt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self > other {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}

impl Clone for FixPt {
    fn clone(&self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_FixPt_Constructor2(self.into(), self.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }
}

impl fmt::Display for FixPt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = i32::from(self);
        write!(f, "{}", num)
    }
}
