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
