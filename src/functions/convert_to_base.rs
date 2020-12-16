use crate::ffi::*;

pub struct ConvertToBase {}

impl FileMakerFunction for ConvertToBase {
    fn function(_id: i16, _env: &ExprEnv, args: &DataVect, result: &mut Data) -> i16 {
        let number = args.at(0);
        let out_locale = number.get_locale();
        let mut number = i32::from(number);

        let base = args.at_as_number(1);
        let base: fmx_int32 = i32::from(base);

        match base {
            2 | 3 | 8 | 12 | 16 => {}
            _ => return 960,
        }

        let mut text = String::new();
        if number == 0 {
            text.push('0');
        } else {
            let neg = number < 0;
            if neg {
                number = -number;
            }
            while number > 0 {
                let digit = (number % base) as u8;
                let ch = if digit < 10 { b'0' } else { b'A' - 10 } + digit;
                text.insert(0, ch as char);
                number /= base;
            }
            if neg {
                text.insert(0, '-');
            }
        }

        let out_text = Text::from(&text);
        result.set_as_text(out_text, out_locale);
        0
    }
}
