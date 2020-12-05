use crate::ffi::*;
use crate::helpers::*;
use crate::wrappers::*;
use std::os::raw::c_short;

#[no_mangle]
pub(crate) unsafe extern "C" fn rust_convert_to_base(
    _func_id: c_short,
    _env_ptr: *const fmx_ExprEnv,
    data_vect_ptr: *const fmx_DataVect,
    results_ptr: *mut fmx_Data,
) -> fmx_errcode {
    let data_vect = DataVect::from_ptr(data_vect_ptr);

    let number = data_vect.at(0);
    let out_locale = number.get_locale();
    let mut number: fmx_int32 = i32::from(number);

    let base = data_vect.at_as_number(1);
    let base: fmx_int32 = i32::from(base);

    match base {
        2 | 3 | 8 | 12 | 16 => {}
        _ => return 960,
    }

    let mut out_text = Text::new();
    let mut insert_buffer = Text::new();

    if number == 0 {
        out_text.assign("0");
    } else {
        let neg = number < 0;
        if neg {
            number = -number;
        }
        while number > 0 {
            let digit = (number % base) as u8;
            let ch = if digit < 10 { b'0' } else { b'A' - 10 } + digit;
            prepend_character(&mut out_text, &mut insert_buffer, ch as char);
            number /= base;
        }
        if neg {
            prepend_character(&mut out_text, &mut insert_buffer, '-');
        }
    }

    let mut results = Data::from_ptr(results_ptr);
    results.set_as_text(out_text, out_locale);
    0
}
