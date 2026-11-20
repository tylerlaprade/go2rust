package main

import (
	"go/ast"
	"go/types"
	"strings"
)

func goFloatBits(typ types.Type) (int, bool) {
	if typ == nil {
		return 0, false
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	if !ok {
		return 0, false
	}
	switch basic.Kind() {
	case types.Float32:
		return 32, true
	case types.Float64:
		return 64, true
	default:
		return 0, false
	}
}

func goPrimitiveFloatBits(typ types.Type) (int, bool) {
	if typ == nil {
		return 0, false
	}
	if _, ok := types.Unalias(typ).(*types.Basic); !ok {
		return 0, false
	}
	return goFloatBits(typ)
}

func formatDefaultFloatArgIndices(format string) map[int]bool {
	indices := map[int]bool{}
	argIndex := 0
	for i := 0; i < len(format); i++ {
		if format[i] != '%' {
			continue
		}
		if i+1 < len(format) && format[i+1] == '%' {
			i++
			continue
		}
		j := i + 1
		for j < len(format) && strings.IndexByte("+-# 0123456789.", format[j]) >= 0 {
			j++
		}
		if j >= len(format) {
			break
		}
		// Passing a formatted String through Rust's width, precision, or
		// alternate/debug formatting changes the meaning (for example %#v
		// would add quotes). Restrict this path to Go's unflagged defaults.
		if j == i+1 && (format[j] == 'v' || format[j] == 'g') {
			indices[argIndex] = true
		}
		argIndex++
		i = j
	}
	return indices
}

var suppressGoDefaultFloatFormat bool

func writeGoDefaultFloatFormat(out *strings.Builder, arg ast.Expr, typ types.Type) bool {
	if suppressGoDefaultFloatFormat {
		return false
	}
	bits, ok := goFloatBits(typ)
	if !ok || typeHasGoStringMethod(typ) || isConcreteGoErrorValue(typ) {
		return false
	}
	NeedGoFloatFormat()
	if bits == 32 {
		out.WriteString("go_format_float32(")
	} else {
		out.WriteString("go_format_float64(")
	}
	if _, named := types.Unalias(typ).(*types.Named); named {
		var primitive strings.Builder
		if writeNamedFloatPrimitiveExpression(&primitive, arg) {
			out.WriteString(primitive.String())
			out.WriteString(")")
			return true
		}
	}
	previous := suppressGoDefaultFloatFormat
	suppressGoDefaultFloatFormat = true
	transpilePrintArg(out, arg)
	suppressGoDefaultFloatFormat = previous
	out.WriteString(")")
	return true
}

func generateGoFloatFormatter(out *strings.Builder, sliceFormat bool, mapFormat bool) {
	out.WriteString(`
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
`)
	if !sliceFormat && !mapFormat {
		return
	}
	trackWrapperImports()
	TrackImport("BTreeMap")
	TrackImport("Display")
	TrackImport("Ord")
	if NeedsConcurrentWrapper() {
		out.WriteString(`
fn format_float64_slice<C: AsRef<[f64]>>(slice: &Arc<Mutex<Option<C>>>) -> String {
    let guard = slice.lock().unwrap();
    match guard.as_ref() {
        Some(values) => format_float64_slice_values(values.as_ref()),
        None => "[]".to_string(),
    }
}

fn format_float32_slice<C: AsRef<[f32]>>(slice: &Arc<Mutex<Option<C>>>) -> String {
    let guard = slice.lock().unwrap();
    match guard.as_ref() {
        Some(values) => format_float32_slice_values(values.as_ref()),
        None => "[]".to_string(),
    }
}

fn format_float64_map_values<K: Display + Ord + Clone>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<f64>>>>>>>) -> String {
    let guard = map.lock().unwrap();
    match guard.as_ref() {
        Some(values) => format!("map[{}]", values.iter().map(|(k, v)| {
            let value = v.lock().unwrap();
            format!("{}:{}", k, value.as_ref().map(|v| go_format_float64(*v)).unwrap_or_else(|| "<nil>".to_string()))
        }).collect::<Vec<_>>().join(" ")),
        None => "map[]".to_string(),
    }
}

fn format_float32_map_values<K: Display + Ord + Clone>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<f32>>>>>>>) -> String {
    let guard = map.lock().unwrap();
    match guard.as_ref() {
        Some(values) => format!("map[{}]", values.iter().map(|(k, v)| {
            let value = v.lock().unwrap();
            format!("{}:{}", k, value.as_ref().map(|v| go_format_float32(*v)).unwrap_or_else(|| "<nil>".to_string()))
        }).collect::<Vec<_>>().join(" ")),
        None => "map[]".to_string(),
    }
}
`)
	} else {
		out.WriteString(`
fn format_float64_slice<C: AsRef<[f64]>>(slice: &Rc<RefCell<Option<C>>>) -> String {
    let guard = slice.borrow();
    match guard.as_ref() {
        Some(values) => format_float64_slice_values(values.as_ref()),
        None => "[]".to_string(),
    }
}

fn format_float32_slice<C: AsRef<[f32]>>(slice: &Rc<RefCell<Option<C>>>) -> String {
    let guard = slice.borrow();
    match guard.as_ref() {
        Some(values) => format_float32_slice_values(values.as_ref()),
        None => "[]".to_string(),
    }
}

fn format_float64_map_values<K: Display + Ord + Clone>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<f64>>>>>>>) -> String {
    let guard = map.borrow();
    match guard.as_ref() {
        Some(values) => format!("map[{}]", values.iter().map(|(k, v)| {
            let value = v.borrow();
            format!("{}:{}", k, value.as_ref().map(|v| go_format_float64(*v)).unwrap_or_else(|| "<nil>".to_string()))
        }).collect::<Vec<_>>().join(" ")),
        None => "map[]".to_string(),
    }
}

fn format_float32_map_values<K: Display + Ord + Clone>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<f32>>>>>>>) -> String {
    let guard = map.borrow();
    match guard.as_ref() {
        Some(values) => format!("map[{}]", values.iter().map(|(k, v)| {
            let value = v.borrow();
            format!("{}:{}", k, value.as_ref().map(|v| go_format_float32(*v)).unwrap_or_else(|| "<nil>".to_string()))
        }).collect::<Vec<_>>().join(" ")),
        None => "map[]".to_string(),
    }
}
`)
	}
	out.WriteString(`
fn format_float64_slice_values(values: &[f64]) -> String {
    format!("[{}]", values.iter().map(|v| go_format_float64(*v)).collect::<Vec<_>>().join(" "))
}

fn format_float32_slice_values(values: &[f32]) -> String {
    format!("[{}]", values.iter().map(|v| go_format_float32(*v)).collect::<Vec<_>>().join(" "))
}
`)
}
