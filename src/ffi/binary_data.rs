use super::*;
use std::convert::TryFrom;
use std::mem::ManuallyDrop;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmx_BinaryData {
    _address: u8,
}

#[link(kind = "static", name = "FMWrapper")]
extern "C" {
    fn FM_BinaryData_Constructor1(_x: *mut fmx__fmxcpt) -> *mut fmx_BinaryData;

    fn FM_BinaryData_Constructor2(
        sourceData: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    fn FM_BinaryData_Constructor3(
        name: *const fmx_Text,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    #[allow(dead_code)]
    fn FM_BinaryData_Constructor4(
        name: *const fmx_Text,
        context: *mut fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    #[allow(dead_code)]
    fn FM_BinaryData_operatorAS(
        _self: *mut fmx_BinaryData,
        source: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> *mut fmx_BinaryData;

    fn FM_BinaryData_operatorEQ(
        _self: *const fmx_BinaryData,
        compareData: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_BinaryData_operatorNE(
        _self: *const fmx_BinaryData,
        compareData: *const fmx_BinaryData,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_BinaryData_GetCount(_self: *const fmx_BinaryData, _x: *mut fmx__fmxcpt) -> fmx_int32;

    fn FM_BinaryData_GetIndex(
        _self: *const fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_int32;

    fn FM_BinaryData_GetTotalSize(_self: *const fmx_BinaryData, _x: *mut fmx__fmxcpt)
        -> fmx_uint32;

    fn FM_BinaryData_GetType(
        _self: *const fmx_BinaryData,
        index: fmx_int32,
        dataType: *mut fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    );

    fn FM_BinaryData_GetSize(
        _self: *const fmx_BinaryData,
        index: fmx_int32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_uint32;

    fn FM_BinaryData_GetData(
        _self: *const fmx_BinaryData,
        index: fmx_int32,
        offset: fmx_uint32,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_Add(
        _self: *mut fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_Remove(
        _self: *mut fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        _x: *mut fmx__fmxcpt,
    ) -> bool;

    fn FM_BinaryData_RemoveAll(_self: *mut fmx_BinaryData, _x: *mut fmx__fmxcpt);
    fn FM_BinaryData_Delete(_self: *mut fmx_BinaryData, _x: *mut fmx__fmxcpt);

    fn FM_BinaryData_GetFNAMData(
        _self: *const fmx_BinaryData,
        filepathlist: *mut fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_AddFNAMData(
        _self: *mut fmx_BinaryData,
        filepathlist: *const fmx_Text,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_GetSIZEData(
        _self: *const fmx_BinaryData,
        width: *mut c_short,
        height: *mut c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_AddSIZEData(
        _self: *mut fmx_BinaryData,
        width: c_short,
        height: c_short,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_AddBegin(
        _self: *mut fmx_BinaryData,
        dataType: *const fmx_QuadChar,
        context: *mut fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_AddAppend(
        _self: *mut fmx_BinaryData,
        context: fmx_uint32,
        amount: fmx_uint32,
        buffer: *mut c_char,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;

    fn FM_BinaryData_AddFinish(
        _self: *mut fmx_BinaryData,
        context: fmx_uint32,
        _x: *mut fmx__fmxcpt,
    ) -> fmx_errcode;
}

#[derive(Eq)]
pub struct BinaryData {
    pub(crate) ptr: *mut fmx_BinaryData,
    drop: bool,
}

impl BinaryData {
    pub fn new() -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_BinaryData_Constructor1(&mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn to_owned(&self) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let ptr = unsafe { FM_BinaryData_Constructor2(self.ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_buffer(name: Text, buffer: Vec<i8>) -> Self {
        let mut _x = fmx__fmxcpt::new();
        let buffer_size = buffer.len() as u32;
        let mut buffer = ManuallyDrop::new(buffer);
        let ptr = buffer.as_mut_ptr();

        let ptr = unsafe { FM_BinaryData_Constructor3(name.ptr, buffer_size as u32, ptr, &mut _x) };
        _x.check();
        Self { ptr, drop: true }
    }

    pub fn from_ptr(ptr: *const fmx_BinaryData) -> Self {
        Self {
            ptr: ptr as *mut fmx_BinaryData,
            drop: false,
        }
    }

    pub fn get_stream_count(&self) -> fmx_int32 {
        let mut _x = fmx__fmxcpt::new();
        let count = unsafe { FM_BinaryData_GetCount(self.ptr, &mut _x) };
        _x.check();
        count
    }

    pub fn get_stream_index(&self, stream_type: BinaryStreamType) -> fmx_int32 {
        let mut _x = fmx__fmxcpt::new();
        let quad = QuadChar::from(stream_type);
        let index = unsafe { FM_BinaryData_GetIndex(self.ptr, quad.ptr, &mut _x) };
        _x.check();
        index
    }

    pub fn total_size(&self) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_BinaryData_GetTotalSize(self.ptr, &mut _x) };
        _x.check();
        size
    }

    pub fn get_size(&self, index: fmx_int32) -> fmx_uint32 {
        let mut _x = fmx__fmxcpt::new();
        let size = unsafe { FM_BinaryData_GetSize(self.ptr, index, &mut _x) };
        _x.check();
        size
    }

    /// # Safety
    /// not proven safe yet
    /// use at own risk
    pub unsafe fn get_data(&self, index: i32, offset: u32, amount: usize) -> Vec<i8> {
        let buffer = Vec::with_capacity(amount);
        let mut buffer = ManuallyDrop::new(buffer);
        let ptr = buffer.as_mut_ptr();

        let mut _x = fmx__fmxcpt::new();
        let error = FM_BinaryData_GetData(self.ptr, index, offset, amount as u32, ptr, &mut _x);
        _x.check();
        if error != 0 {
            panic!();
        }
        Vec::from_raw_parts(ptr, amount, amount)
    }

    pub fn get_type(&self, index: fmx_int32) -> BinaryStreamType {
        let mut _x = fmx__fmxcpt::new();
        let quad = QuadChar::empty();
        unsafe { FM_BinaryData_GetType(self.ptr, index, quad.ptr, &mut _x) };
        _x.check();
        BinaryStreamType::from(quad)
    }

    pub fn add_stream(&self, stream_type: BinaryStreamType, buffer: Vec<i8>) {
        let mut _x = fmx__fmxcpt::new();
        let quad = QuadChar::from(stream_type);
        let size = buffer.len() as u32;
        let mut buffer = ManuallyDrop::new(buffer);
        let buffer_ptr = buffer.as_mut_ptr();
        let error = unsafe { FM_BinaryData_Add(self.ptr, quad.ptr, size, buffer_ptr, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }

    pub fn remove_stream(&self, stream_type: BinaryStreamType) {
        let mut _x = fmx__fmxcpt::new();
        let quad = QuadChar::from(stream_type);
        let success = unsafe { FM_BinaryData_Remove(self.ptr, quad.ptr, &mut _x) };
        _x.check();
        if !success {
            panic!();
        }
    }

    pub fn remove_all(&self) {
        let mut _x = fmx__fmxcpt::new();
        unsafe { FM_BinaryData_RemoveAll(self.ptr, &mut _x) };
        _x.check();
    }

    pub fn add_header(&self, stream_type: BinaryStreamType, context: &mut u32) {
        let mut _x = fmx__fmxcpt::new();
        let quad = QuadChar::from(stream_type);
        let error = unsafe { FM_BinaryData_AddBegin(self.ptr, quad.ptr, context, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }

    pub fn append_stream(&self, context: u32, buffer: Vec<i8>) {
        let mut _x = fmx__fmxcpt::new();
        let size = buffer.len() as u32;
        let mut buffer = ManuallyDrop::new(buffer);
        let buffer_ptr = buffer.as_mut_ptr();
        let error =
            unsafe { FM_BinaryData_AddAppend(self.ptr, context, size, buffer_ptr, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }

    pub fn add_footer(&self, context: u32) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe { FM_BinaryData_AddFinish(self.ptr, context, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }

    pub fn get_file_names(&self) -> Text {
        let mut _x = fmx__fmxcpt::new();
        let file_paths = Text::new();
        let error = unsafe { FM_BinaryData_GetFNAMData(self.ptr, file_paths.ptr, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
        file_paths
    }

    pub fn add_file_paths(&self, file_paths: Text) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe { FM_BinaryData_AddFNAMData(self.ptr, file_paths.ptr, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }

    pub fn get_dimensions(&self) -> (i16, i16) {
        let mut _x = fmx__fmxcpt::new();
        let mut height = 0;
        let mut width = 0;
        let error =
            unsafe { FM_BinaryData_GetSIZEData(self.ptr, &mut width, &mut height, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
        (width, height)
    }

    pub fn set_dimensions(&self, height: i16, width: i16) {
        let mut _x = fmx__fmxcpt::new();
        let error = unsafe { FM_BinaryData_AddSIZEData(self.ptr, width, height, &mut _x) };
        _x.check();
        if error != 0 {
            panic!();
        }
    }
}

impl Drop for BinaryData {
    fn drop(&mut self) {
        if self.drop {
            let mut _x = fmx__fmxcpt::new();
            unsafe { FM_BinaryData_Delete(self.ptr, &mut _x) };
            _x.check();
        }
    }
}

impl Default for BinaryData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum BinaryStreamType {
    FNAM,
    JPEG,
    GIF,
    EPS,
    META,
    PNG,
    BMP,
    PDF,
    FILE,
    ZLIB,
    FORK,
    SND,
    MAIN,
    SIZE,
    Other(String),
}

impl From<BinaryStreamType> for QuadChar {
    fn from(stream_type: BinaryStreamType) -> QuadChar {
        use BinaryStreamType::*;
        match stream_type {
            FNAM => QuadChar::new(b"FNAM"),
            JPEG => QuadChar::new(b"JPEG"),
            GIF => QuadChar::new(b"GIFf"),
            EPS => QuadChar::new(b"EPSf"),
            META => QuadChar::new(b"META"),
            PNG => QuadChar::new(b"PNGf"),
            BMP => QuadChar::new(b"BMPf"),
            PDF => QuadChar::new(b"PDF "),
            SIZE => QuadChar::new(b"SIZE"),
            FILE => QuadChar::new(b"FILE"),
            ZLIB => QuadChar::new(b"ZLIB"),
            FORK => QuadChar::new(b"FORK"),
            SND => QuadChar::new(b"snd "),
            MAIN => QuadChar::new(b"MAIN"),
            Other(txt) => {
                let slice = txt.as_bytes();
                let bytes = <&[u8; 4]>::try_from(&slice[..4]).unwrap();
                QuadChar::new(bytes)
            }
        }
    }
}

impl From<QuadChar> for BinaryStreamType {
    fn from(quad: QuadChar) -> BinaryStreamType {
        let txt = quad.to_string();
        use BinaryStreamType::*;
        match txt.as_ref() {
            "FNAM" => FNAM,
            "JPEG" => JPEG,
            "GIF" => GIF,
            "EPS" => EPS,
            "META" => META,
            "PNG" => PNG,
            "BMP" => BMP,
            "PDF" => PDF,
            "SIZE" => SIZE,
            "FILE" => FILE,
            "ZLIB" => ZLIB,
            "FORK" => FORK,
            "SND" => SND,
            "MAIN" => MAIN,
            t => Other(t.to_string()),
        }
    }
}

impl PartialEq for BinaryData {
    fn eq(&self, other: &BinaryData) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_BinaryData_operatorEQ(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &BinaryData) -> bool {
        let mut _x = fmx__fmxcpt::new();
        let result = unsafe { FM_BinaryData_operatorNE(self.ptr, other.ptr, &mut _x) };
        _x.check();
        result
    }
}
