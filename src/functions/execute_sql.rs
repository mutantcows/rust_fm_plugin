use crate::ffi::*;
use crate::wrappers::*;
use std::os::raw::c_short;

#[no_mangle]
pub(crate) unsafe extern "C" fn rust_execute_sql(
    _func_id: c_short,
    environment: *const fmx_ExprEnv,
    data_vect: *const fmx_DataVect,
    results: *mut fmx_Data,
) -> fmx_errcode {
    let error_result: fmx_errcode = 0;

    let file_name = (*data_vect).at_as_text(0);
    let expression = (*data_vect).at_as_text(1);

    let mut parameters = DataVect::new();
    let param_count = (*data_vect).size();

    if param_count > 2 {
        for i in 2..param_count {
            let param = (*data_vect).at(i);
            parameters.push(param);
        }
    }

    let mut result = RowVect::new();

    (*environment).execute_file_sql(expression, file_name, parameters, &mut result);

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
    let field_count = (*first_row.ptr).size();
    let out_locale = (*(*first_row.ptr).at(0).ptr).get_locale();

    for i in 0..record_count {
        let record = result.at(i);
        for j in 0..field_count {
            let field = (*record.ptr).at(j);
            let text = field.get_as_text();
            out_text.insert_text(&text, out_text.size());
            out_text.insert_text(&col_sep, out_text.size());
        }
        out_text.insert_text(&row_sep, out_text.size());
    }

    (*results).set_as_text(out_text, out_locale);

    error_result
}
