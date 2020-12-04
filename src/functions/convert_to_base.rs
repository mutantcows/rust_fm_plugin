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
    let mut error_result: fmx_errcode = 960;
    let data_vect = DataVect::from_ptr(data_vect_ptr);
    let mut results = Data::from_ptr(results_ptr);

    let mut out_text = Text::new();

    let mut out_locale = results.get_locale();
    let mut insert_buffer = Text::new();

    let data_size = data_vect.size();

    if data_size >= 2 {
        let num = data_vect.at(0);
        let number = num.get_as_number();
        let mut number: fmx_int32 = number.get_as_long();

        let base = data_vect.at(1);
        let base = base.get_as_number();
        let base: fmx_int32 = base.get_as_long();

        if base == 2 || base == 3 || base == 8 || base == 12 || base == 16 {
            if number == 0 {
                prepend_character(&mut out_text, &mut insert_buffer, '0');
            } else {
                let neg: bool = number < 0;
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
            out_locale = num.get_locale();
            error_result = 0;
        }
    }

    results.set_as_text(out_text, out_locale);

    error_result
}
