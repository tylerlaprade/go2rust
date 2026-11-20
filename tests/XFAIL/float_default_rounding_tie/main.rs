
// Go's default float rendering is strconv.FormatFloat(v, 'g', -1, bitSize):
// use the shortest identifying digits, switching to exponent notation below
// 1e-4 and at 1e6.
fn go_format_float_digits(exponent_form: &str) -> (String, i32) {
    let (mantissa, exponent) = exponent_form.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, exponent.parse().unwrap())
}

fn go_format_float_exponent(digits: &str, exponent: i32) -> String {
    let mut out = String::new();
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push('e');
    out.push(if exponent < 0 { '-' } else { '+' });
    out.push_str(&format!("{:02}", exponent.abs()));
    out
}

fn go_format_float_magnitude(shortest: &str) -> String {
    let (digits, exponent) = go_format_float_digits(shortest);
    if exponent < -4 || exponent >= 6 {
        return go_format_float_exponent(&digits, exponent);
    }
    if exponent < 0 {
        return format!("0.{}{}", "0".repeat((-exponent - 1) as usize), digits);
    }
    let whole = (exponent + 1) as usize;
    if digits.len() <= whole {
        return format!("{}{}", digits, "0".repeat(whole - digits.len()));
    }
    format!("{}.{}", &digits[..whole], &digits[whole..])
}

fn go_format_float64(value: f64) -> String {
    if value.is_nan() { return "NaN".to_string(); }
    if value.is_infinite() {
        return if value.is_sign_negative() { "-Inf" } else { "+Inf" }.to_string();
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let magnitude = value.abs();
    format!("{}{}", sign, go_format_float_magnitude(&format!("{:e}", magnitude)))
}

fn go_format_float32(value: f32) -> String {
    if value.is_nan() { return "NaN".to_string(); }
    if value.is_infinite() {
        return if value.is_sign_negative() { "-Inf" } else { "+Inf" }.to_string();
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let magnitude = value.abs();
    format!("{}{}", sign, go_format_float_magnitude(&format!("{:e}", magnitude)))
}

fn main() {
    println!("{}", format!("{}", go_format_float64(1000000000000000.25)));
}