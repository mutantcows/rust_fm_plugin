//! Various helper functions, including a logging function.
use crate::config::read_config;
use std::ffi::CString;
use widestring::U16CString;

pub(crate) fn write_to_file(content: &str) -> Result<(), String> {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let config = read_config().map_err(|e| e.to_string())?;
    let path = config.log.path.ok_or("No path configured")?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
        .map_err(|err| err.to_string())?;

    file.write_all(content.as_bytes())
        .map_err(|err| err.to_string())?;
    Ok(())
}

/// Appends to log file specified in `config.toml`.
pub fn log(content: &str) {
    write_to_file(content).unwrap_or(());
}

/// # Safety
/// Uses slice from raw parts to fill the buffer
/// Asserts that bytes len is <= buffer_size
pub unsafe fn write_to_u16_buff(buffer: *mut u16, buffer_size: u32, s: &str) {
    let c_string = U16CString::from_str(s).unwrap();
    let bytes = c_string.as_slice();

    bytes_to_buff(buffer, buffer_size, bytes);
}

#[allow(dead_code)]
pub(crate) unsafe fn write_to_i8_buff(buffer: *mut i8, buffer_size: u32, s: &str) {
    let c_string = CString::new(s).unwrap();
    let bytes = c_string.as_bytes_with_nul();
    let bytes = &*(bytes as *const [u8] as *const [i8]);

    bytes_to_buff(buffer, buffer_size, bytes);
}

unsafe fn bytes_to_buff<T: Copy>(buffer: *mut T, buffer_size: u32, bytes: &[T]) {
    assert!(bytes.len() <= buffer_size as usize);
    let string_bytes = std::slice::from_raw_parts_mut(buffer, buffer_size as usize);
    string_bytes[..bytes.len()].copy_from_slice(bytes);
}
