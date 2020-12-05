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

pub(crate) fn log(content: &str) {
    write_to_file(content).unwrap_or(());
}

pub(crate) fn write_to_u16_buff(buffer: *mut c_ushort, buffer_size: c_uint, s: &str) {
    let c_string = WideCString::from_str(s).unwrap();
    let bytes = c_string.as_slice();

    let string_bytes = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_size as usize) };
    string_bytes[..bytes.len()].copy_from_slice(bytes);
}

pub(crate) fn write_to_i8_buff(buffer: *mut c_char, buffer_size: c_uint, s: &str) {
    let c_string = CString::new(s).unwrap();
    let bytes = c_string.as_bytes_with_nul();
    let bytes = unsafe { &*(bytes as *const [u8] as *const [i8]) };
    let string_bytes = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_size as usize) };
    string_bytes[..bytes.len()].copy_from_slice(bytes);
}

pub(crate) fn prepend_character(txt: &mut Text, insert_buffer: &mut Text, ch: char) {
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
        let result = write_to_file("test");
        assert_eq!(result, Ok(()));
    }
}
