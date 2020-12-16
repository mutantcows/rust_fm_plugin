use crate::ffi::*;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::Path;

pub(crate) struct InsertFile;

impl FileMakerFunction for InsertFile {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16 {
        let file_path = args.at_as_text(0).to_string();
        let file_path = Path::new(&file_path);

        let mut f = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => return 99,
        };
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
