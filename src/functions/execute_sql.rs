use crate::ffi::*;

pub struct ExecuteSQL {}

impl FileMakerFunction for ExecuteSQL {
    fn function(_id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16 {
        let file_name = args.at_as_text(0);
        let expression = args.at_as_text(1);

        let mut parameters = DataVect::new();
        let param_count = args.size();

        if param_count > 2 {
            for i in 2..param_count {
                let param = args.at(i);
                parameters.push(param);
            }
        }

        let mut sql_result = RowVect::new();

        env.execute_file_sql(expression, file_name, parameters, &mut sql_result);

        if sql_result.is_empty() {
            return 0;
        }

        let mut out_text = Text::new();
        let mut col_sep = Text::new();
        col_sep.assign(",");
        let mut row_sep = Text::new();
        row_sep.assign("\n");

        let record_count = sql_result.size();
        let first_row = sql_result.at(0);
        let field_count = first_row.size();
        let out_locale = first_row.at(0).get_locale();

        for i in 0..record_count {
            let record = sql_result.at(i);
            for j in 0..field_count {
                let field = record.at(j);
                let text = field.get_as_text();
                out_text.append(&text);
                out_text.append(&col_sep);
            }
            out_text.append(&row_sep);
        }

        result.set_as_text(out_text, out_locale);
        0
    }
}
