use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx__fmxcpt {
    pub m_vers: fmx_int32,
    pub m_code: fmx_int32,
}

impl fmx__fmxcpt {
    pub(crate) fn check(&self) {
        if self.m_code != 0 {
            panic!();
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            m_code: 0,
            m_vers: 1,
        }
    }
}
