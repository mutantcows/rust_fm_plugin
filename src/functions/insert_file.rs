use crate::ffi::*;
use std::fs::{metadata, File};
use std::io::Read;
use std::os::raw::c_short;
use std::path::Path;

#[no_mangle]
pub(crate) unsafe extern "C" fn rust_insert_file(
    _func_id: c_short,
    _env_ptr: *const fmx_ExprEnv,
    data_vect_ptr: *const fmx_DataVect,
    results_ptr: *mut fmx_Data,
) -> fmx_errcode {
    let data_vect = DataVect::from_ptr(data_vect_ptr);
    let mut results = Data::from_ptr(results_ptr);

    let file_path = data_vect.at_as_text(0).to_string();
    let file_path = Path::new(&file_path);

    let mut f = File::open(&file_path).expect("no file found");
    let file_info = metadata(&file_path).expect("unable to read metadata");
    let mut buffer = vec![0; file_info.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");
    let file_name = file_path.file_name().unwrap();
    let buffer = buffer.into_iter().map(|e| e as i8).collect();

    let binary_data = BinaryData::from_buffer(file_name, buffer);

    results.set_binarydata(binary_data, false);
    0
}

pub trait FileMakerFunction {
    fn function(id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16;

    #[no_mangle]
    unsafe extern "C" fn external(
        id: c_short,
        env_ptr: *const fmx_ExprEnv,
        args_ptr: *const fmx_DataVect,
        result_ptr: *mut fmx_Data,
    ) -> i16 {
        let arguments = DataVect::from_ptr(args_ptr);
        let env = ExprEnv::from_ptr(env_ptr);
        let mut results = Data::from_ptr(result_ptr);

        Self::function(id, &env, &arguments, &mut results)
    }
}

pub(crate) struct InsertFile;

impl FileMakerFunction for InsertFile {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16 {
        let file_path = args.at_as_text(0).to_string();
        let file_path = Path::new(&file_path);

        let mut f = File::open(&file_path).expect("no file found");
        let file_info = metadata(&file_path).expect("unable to read metadata");
        let mut buffer = vec![0; file_info.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");
        let file_name = file_path.file_name().unwrap();
        let buffer = buffer.into_iter().map(|e| e as i8).collect();

        let binary_data = BinaryData::from_buffer(file_name, buffer);

        result.set_binarydata(binary_data, false);
        0
    }
}
