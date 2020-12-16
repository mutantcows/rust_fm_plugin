use crate::ffi::*;

pub struct ExecuteSQLTextResult {}

impl FileMakerFunction for ExecuteSQLTextResult {
    fn function(_id: i16, env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16 {
        let file_name = args.at_as_text(0);
        let expression = args.at_as_text(1);

        let col_sep = args.at_as_text(2);
        let col_sep = match col_sep.size() {
            0 => ',' as u16,
            _ => u16::from(col_sep),
        };

        let row_sep = args.at_as_text(3);
        let row_sep = match row_sep.size() {
            0 => '\n' as u16,
            _ => u16::from(row_sep),
        };

        let mut parameters = DataVect::new();
        let param_count = args.size();

        if param_count > 4 {
            for i in 4..param_count {
                let param = args.at(i);
                parameters.push(param);
            }
        }

        let mut sql_result = Data::new();

        env.execute_file_sql_text_result(
            expression,
            file_name,
            parameters,
            &mut sql_result,
            col_sep,
            row_sep,
        );

        let out_locale = sql_result.get_locale();
        let sql_result = sql_result.get_as_text();
        result.set_as_text(sql_result, out_locale);
        0
    }
}
