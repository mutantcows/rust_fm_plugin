use crate::ffi::*;
use pdf_forms::{FieldState, Form};
use serde_json::{to_string_pretty, Map, Value};

pub struct PDFToJSON {}

impl FileMakerFunction for PDFToJSON {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16 {
        let mut out_text = Text::new();
        let path = args.at_as_text(0);

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
        let out_locale = result.get_locale();
        result.set_as_text(out_text, out_locale);

        0
    }
}
