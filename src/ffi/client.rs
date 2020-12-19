#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct fmx__fmxcpt {
    m_vers: i32,
    m_code: i32,
}

impl fmx__fmxcpt {
    pub(crate) fn check(&self) {
        match self.m_code {
            0 => {}
            1 => panic!("BadAlloc"),
            2 => panic!("Unknown"),
            _ => panic!(),
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            m_code: 0,
            m_vers: 1,
        }
    }
}
