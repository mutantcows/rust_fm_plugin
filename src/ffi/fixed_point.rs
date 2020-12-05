use super::*;

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

    pub(crate) fn get_as_long(&self) -> fmx_int32 {
        let mut _x = fmx__fmxcpt::new();
        let num = unsafe { FM_FixPt_AsLong(self.ptr, &mut _x) };
        _x.check();
        num
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
        fix_pt.get_as_long()
    }
}
