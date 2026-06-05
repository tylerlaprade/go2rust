
fn go_strconv_format_int(value: i64, base: i32) -> String {
    if base == 10 {
        return value.to_string();
    }
    if !(2..=36).contains(&base) {
        return value.to_string();
    }

    let negative = value < 0;
    let mut n = if negative {
        value.wrapping_neg() as u64
    } else {
        value as u64
    };
    let base = base as u64;
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    if n == 0 {
        out.push(b'0');
    }
    while n > 0 {
        out.push(digits[(n % base) as usize]);
        n /= base;
    }
    if negative {
        out.push(b'-');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn go_strconv_format_float(value: f64, fmt: char, precision: i32) -> String {
    let precision = if precision < 0 { 6 } else { precision as usize };
    match fmt {
        'e' => format!("{:.*e}", precision, value),
        'E' => format!("{:.*E}", precision, value),
        'f' => format!("{:.*}", precision, value),
        'g' | 'G' => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.*}", precision, value)
            }
        }
        _ => value.to_string(),
    }
}
