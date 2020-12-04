use crate::ffi::*;
use crate::wrappers::*;
use pdf_forms::{FieldState, Form};
use serde_json::{to_string_pretty, Map, Value};
use std::os::raw::c_short;

#[no_mangle]
pub(crate) unsafe extern "C" fn rust_pdf_to_json(
    _func_id: c_short,
    _env_ptr: *const fmx_ExprEnv,
    data_vect_ptr: *const fmx_DataVect,
    results_ptr: *mut fmx_Data,
) -> fmx_errcode {
    let data_vect = DataVect::from_ptr(data_vect_ptr);
    let mut results = Data::from_ptr(results_ptr);

    let mut out_text = Text::new();
    let path = data_vect.at_as_text(0);

    let form = match Form::load(path.to_string()) {
        Ok(f) => f,
        Err(_) => {
            return 802;
        }
    };
    let mut pdf: Map<String, Value> = Map::new();

    for (i, field) in form.get_all_names().iter().enumerate() {
        if let Some(name) = field {
            let value = match form.get_state(i) {
                FieldState::Text { text, .. } => Value::String(text),
                FieldState::CheckBox { is_checked, .. } => Value::Bool(is_checked),
                FieldState::Radio { selected, .. } => Value::String(selected),
                val => Value::String(format!("{:#?}", val)),
            };
            pdf.insert(name.clone(), value);
        }
    }
    let json_str = to_string_pretty(&pdf).unwrap();

    out_text.assign(&json_str);
    let out_locale = results.get_locale();
    results.set_as_text(out_text, out_locale);

    0
}
