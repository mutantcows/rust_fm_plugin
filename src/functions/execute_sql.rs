use crate::ffi::*;
use std::os::raw::c_short;

#[no_mangle]
pub(crate) unsafe extern "C" fn rust_execute_sql(
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

    let mut parameters = DataVect::new();
    let param_count = data_vect.size();

    if param_count > 2 {
        for i in 2..param_count {
            let param = data_vect.at(i);
            parameters.push(param);
        }
    }

    let mut result = RowVect::new();

    env.execute_file_sql(expression, file_name, parameters, &mut result);

    if result.is_empty() {
        return 0;
    }

    let mut out_text = Text::new();
    let mut col_sep = Text::new();
    col_sep.assign(",");
    let mut row_sep = Text::new();
    row_sep.assign("\n");

    let record_count = result.size();
    let first_row = result.at(0);
    let field_count = first_row.size();
    let out_locale = first_row.at(0).get_locale();

    for i in 0..record_count {
        let record = result.at(i);
        for j in 0..field_count {
            let field = record.at(j);
            let text = field.get_as_text();
            out_text.append(&text);
            out_text.append(&col_sep);
        }
        out_text.append(&row_sep);
    }

    results.set_as_text(out_text, out_locale);
    0
}
