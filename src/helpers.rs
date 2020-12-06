use crate::ffi::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_uint, c_ushort};
use widestring::WideCString;

pub(crate) fn write_to_file(content: &str) -> Result<(), String> {
    use directories::UserDirs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::Path;

    let user_dirs = UserDirs::new().ok_or("No user dirs")?;
    let dir = user_dirs.desktop_dir().ok_or("No desktop path")?;
    let path = Path::join(&dir, "plugin.log");

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

pub fn log(content: &str) {
    write_to_file(content).unwrap_or(());
}

/// # Safety
/// buffer size must be accurate and include enough room for str
pub unsafe fn write_to_u16_buff(buffer: *mut c_ushort, buffer_size: c_uint, s: &str) {
    let c_string = WideCString::from_str(s).unwrap();
    let bytes = c_string.as_slice();

    bytes_to_buff(buffer, buffer_size, bytes);
}

/// # Safety
/// buffer size must be accurate and include enough room for str
pub unsafe fn write_to_i8_buff(buffer: *mut c_char, buffer_size: c_uint, s: &str) {
    let c_string = CString::new(s).unwrap();
    let bytes = c_string.as_bytes_with_nul();
    let bytes = &*(bytes as *const [u8] as *const [i8]);

    bytes_to_buff(buffer, buffer_size, bytes);
}

unsafe fn bytes_to_buff<T: Copy>(buffer: *mut T, buffer_size: c_uint, bytes: &[T]) {
    let string_bytes = std::slice::from_raw_parts_mut(buffer, buffer_size as usize);
    string_bytes[..bytes.len()].copy_from_slice(bytes);
}

pub fn prepend_character(txt: &mut Text, insert_buffer: &mut Text, ch: char) {
    let mut tmp = [0; 1];
    let s = ch.encode_utf8(&mut tmp);
    insert_buffer.assign_unicode_with_length(s, 1);
    txt.insert(insert_buffer, 0);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn logging() {
        let result = write_to_file("");
        assert_eq!(result, Ok(()));
    }
}
