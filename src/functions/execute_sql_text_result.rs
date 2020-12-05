use crate::ffi::*;
use crate::wrappers::*;
use std::os::raw::c_short;

#[no_mangle]
pub(crate) unsafe extern "C" fn rust_execute_sql_text_result(
    _func_id: c_short,
    env_ptr: *const fmx_ExprEnv,
    data_vect_ptr: *const fmx_DataVect,
    results_ptr: *mut fmx_Data,
) -> fmx_errcode {
    let env = ExprEnv::from_ptr(env_ptr);
    let data_vect = DataVect::from_ptr(data_vect_ptr);
    let mut results = Data::from_ptr(results_ptr);

    let file_name = data_vect.at_as_text(0);
    let expression = data_vect.at_as_text(1);

    let col_sep = data_vect.at_as_text(2);
    let col_sep = match col_sep.size() {
        0 => ',' as u16,
        _ => u16::from(col_sep),
    };

    let row_sep = data_vect.at_as_text(3);
    let row_sep = match row_sep.size() {
        0 => '\n' as u16,
        _ => u16::from(row_sep),
    };

    let mut parameters = DataVect::new();
    let param_count = data_vect.size();

    if param_count > 4 {
        for i in 4..param_count {
            let param = data_vect.at(i);
            parameters.push(param);
        }
    }

    let mut result = Data::new();

    env.execute_file_sql_text_result(
        expression,
        file_name,
        parameters,
        &mut result,
        col_sep,
        row_sep,
    );

    let out_locale = result.get_locale();
    let result = result.get_as_text();
    results.set_as_text(result, out_locale);
    0
}
