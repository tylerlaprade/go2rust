package main

import (
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"sort"
	"strconv"
	"strings"
)

// A format written as a literal is turned into a Rust format string when the
// program is translated. This file is for the other case: the format is a
// parameter, a variable or the result of a call, so it is only known when the
// program runs. Then the generated program calls go_sprintf, which reads the
// format the way fmt does and formats each argument for its verb.
//
// go_sprintf is a hand-written copy of what fmt does, so it is bridge debt:
// row fmt-runtime-format in docs/bridge_debt.md. It goes when fmt is
// translated from its Go source, or moves when fmt is given to the
// hand-written runtime.

// translationTimeFormat returns the format of a formatted print call as Go
// source text when its value is known at translation time: a string literal
// or any other constant string expression, such as a named constant.
func translationTimeFormat(format ast.Expr) (string, bool) {
	if lit, ok := format.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		return lit.Value, true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return "", false
	}
	tv, ok := typeInfo.info.Types[format]
	if !ok || tv.Value == nil || tv.Value.Kind() != constant.String {
		return "", false
	}
	return strconv.Quote(constant.StringVal(tv.Value)), true
}

// expandedFormat returns the format of a formatted print call that can be
// expanded into a Rust format macro while translating: the format is known
// then, and the arguments are written out one by one. A call that passes its
// arguments on as a list, f(format, args...), cannot be: how many arguments
// there are, and of which types, is only known when it runs.
func expandedFormat(call *ast.CallExpr, formatIndex int) (string, bool) {
	if call.Ellipsis.IsValid() {
		return "", false
	}
	format, ok := translationTimeFormat(call.Args[formatIndex])
	if !ok || !formatExpandsToRust(format) {
		return "", false
	}
	if !namedScalarArgsUseDisplay(format, call.Args[formatIndex+1:]) {
		return "", false
	}
	return format, true
}

// namedScalarArgsUseDisplay reports whether every argument of a type the
// package defines over a basic type is formatted with %v, %s or %T. The
// newtype's Display is its String method, or its value without the verb's
// precision, and Go does not call String for "%.2f".
func namedScalarArgsUseDisplay(quoted string, args []ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return true
	}
	hasNamedScalar := false
	for _, arg := range args {
		if anyFormatTypeIsLocalNamedBasic(typeInfo.GetType(arg)) {
			hasNamedScalar = true
			break
		}
	}
	if !hasNamedScalar {
		return true
	}
	format, err := strconv.Unquote(quoted)
	if err != nil || strings.Contains(format, "[") {
		return false
	}
	var verbs []byte
	for i := 0; i < len(format); i++ {
		if format[i] != '%' {
			continue
		}
		i++
		for i < len(format) && strings.IndexByte("+-# 0123456789.", format[i]) >= 0 {
			i++
		}
		if i >= len(format) {
			return false
		}
		if format[i] != '%' {
			verbs = append(verbs, format[i])
		}
	}
	for index, arg := range args {
		typ := typeInfo.GetType(arg)
		if !anyFormatTypeIsLocalNamedBasic(typ) {
			continue
		}
		// A named integer or bool without a String method is formatted
		// as the value it holds, %x included, by the Rust format.
		if !typeHasGoStringMethod(typ) {
			if basic, ok := typ.Underlying().(*types.Basic); ok && basic.Info()&(types.IsInteger|types.IsBoolean) != 0 {
				continue
			}
		}
		if index >= len(verbs) || strings.IndexByte("vsT", verbs[index]) < 0 {
			return false
		}
	}
	return true
}

// formatExpandsToRust reports whether every verb of a literal format is one
// that convertFormatStringWithSkips turns into a Rust format with the same
// output. Any other verb, such as "%8.2f", "%e", "%g", "%.3d", "%+5d" or
// "%q", is formatted by go_sprintf when the program runs, as a format that
// is not a literal is. The Rust format for it used to be wrong: "%05.1f"
// was copied through as text and its argument was left over.
func formatExpandsToRust(quoted string) bool {
	format, err := strconv.Unquote(quoted)
	if err != nil {
		return false
	}
	isDigit := func(c byte) bool { return c >= '0' && c <= '9' }
	for i := 0; i < len(format); i++ {
		if format[i] != '%' {
			continue
		}
		i++
		if i >= len(format) {
			return false
		}
		switch format[i] {
		case '%', 'T':
			continue
		case '+':
			i++
			if i < len(format) && format[i] == '.' {
				j := i + 1
				for j < len(format) && isDigit(format[j]) {
					j++
				}
				if j > i+1 && j < len(format) && format[j] == 'f' {
					i = j
					continue
				}
				return false
			}
			if i < len(format) && strings.IndexByte("dfv", format[i]) >= 0 {
				continue
			}
			return false
		case '#':
			i++
			if i < len(format) && strings.IndexByte("xXvU", format[i]) >= 0 {
				continue
			}
			return false
		case '.':
			j := i + 1
			for j < len(format) && isDigit(format[j]) {
				j++
			}
			if j > i+1 && j < len(format) && (format[j] == 'f' || format[j] == 's') {
				i = j
				continue
			}
			return false
		}
		leftAlign := format[i] == '-'
		if leftAlign {
			i++
		}
		zeroPad := i < len(format) && format[i] == '0'
		if zeroPad {
			i++
		}
		widthStart := i
		for i < len(format) && isDigit(format[i]) {
			i++
		}
		if i >= len(format) {
			return false
		}
		if i > widthStart {
			switch format[i] {
			case 's':
				if zeroPad {
					return false
				}
			case 'x', 'X':
				if leftAlign {
					return false
				}
			case 'd', 'f':
				if leftAlign && zeroPad {
					return false
				}
			default:
				return false
			}
			continue
		}
		if leftAlign || zeroPad {
			return false
		}
		if strings.IndexByte("dsvtwpfxXcbU", format[i]) < 0 {
			return false
		}
	}
	return true
}

// writeRuntimeFormatCall writes the call that formats args with a format
// known only at run time. args are the arguments after the format. spread is
// true for f(format, args...), where the one argument is a []interface{}.
// wraps is true for fmt.Errorf, where %w prints as %v.
func writeRuntimeFormatCall(out *strings.Builder, format ast.Expr, args []ast.Expr, spread bool, wraps bool) {
	if spread && len(args) == 1 {
		NeedGoSprintfSpread()
		out.WriteString("go_sprintf_spread(&")
		writeOwnedStringStdlibArg(out, format)
		out.WriteString(", &")
		if ident, ok := args[0].(*ast.Ident); ok {
			// Inside a closure the slice is the closure's own clone.
			out.WriteString(rustIdentForUseWithCapture(ident))
		} else {
			TranspileExpression(out, args[0])
		}
	} else {
		NeedGoSprintf()
		out.WriteString("go_sprintf(&")
		writeOwnedStringStdlibArg(out, format)
		out.WriteString(", &[")
		for i, arg := range args {
			if i > 0 {
				out.WriteString(", ")
			}
			if NeedsConcurrentWrapper() {
				// Each argument reads its variable through a lock. Taken in
				// one expression, the locks were all held until the call, so
				// printing one variable twice waited on itself.
				out.WriteString("{ let __go_format_arg = ")
				writeVariadicPackedElementValue(out, arg, nil, nil, true)
				out.WriteString("; __go_format_arg }")
				continue
			}
			writeVariadicPackedElementValue(out, arg, nil, nil, true)
		}
		out.WriteString("]")
	}
	if wraps {
		out.WriteString(", true)")
	} else {
		out.WriteString(", false)")
	}
}

// generateGoSprintf emits go_sprintf and what it needs. spread adds
// go_sprintf_spread, which takes the arguments as the []interface{} they
// arrived in. anyFormatArms are the types the program puts in an interface
// value; the ones with a String or an Error method are listed in
// go_format_is_text.
func generateGoSprintf(out *strings.Builder, spread bool, anyFormatArms map[string]anyFormatArm) {
	TrackImport("Any")
	anyValue, anyBox := "dyn Any", "Box<dyn Any>"
	sliceType, guard := "Rc<RefCell<Option<Vec<Box<dyn Any>>>>>", "args.borrow()"
	if NeedsConcurrentWrapper() {
		anyValue, anyBox = "(dyn Any + Send + Sync)", "Box<dyn Any + Send + Sync>"
		sliceType, guard = "Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>", "args.lock().unwrap()"
	}
	bytesHandle, bytesGuard := "Rc<RefCell<Option<Vec<u8>>>>", ".borrow()"
	if NeedsConcurrentWrapper() {
		bytesHandle, bytesGuard = "Arc<Mutex<Option<Vec<u8>>>>", ".lock().unwrap()"
	}
	trackWrapperImports()
	replace := strings.NewReplacer("GO_ANY_VALUE", anyValue, "GO_ANY_BOX", anyBox, "GO_BYTES_HANDLE", bytesHandle, "GO_BYTES_GUARD", bytesGuard)
	textTypes := make([]string, 0, len(anyFormatArms))
	var namedBasicTypes []string
	for rustType, arm := range anyFormatArms {
		if arm.text {
			textTypes = append(textTypes, rustType)
		}
		if arm.namedBasic {
			namedBasicTypes = append(namedBasicTypes, rustType)
		}
	}
	sort.Strings(textTypes)
	sort.Strings(namedBasicTypes)
	// go_format_is_text names the program's own types, so it stays where
	// they are in scope. Its one name is one Go cannot declare.
	out.WriteString(`
// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &` + anyValue + `) -> bool {
    false`)
	for _, rustType := range textTypes {
		out.WriteString("\n        || __value.is::<" + rustType + ">()")
	}
	out.WriteString("\n}\n")
	// A "type Level int" value is the one basic value it holds, for the
	// verbs its own text does not take: %d, %x, a width on %v.
	out.WriteString(`
// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &` + anyValue + `) -> Option<` + anyBox + `> {`)
	for _, rustType := range namedBasicTypes {
		var inner strings.Builder
		inner.WriteString("(*__v.0")
		WriteBorrowMethod(&inner, false)
		inner.WriteString(".as_ref().unwrap()).clone()")
		out.WriteString("\n    if let Some(__v) = __value.downcast_ref::<" + rustType + ">() {\n        return Some(Box::new(" + inner.String() + "));\n    }")
	}
	out.WriteString("\n    None\n}\n")
	// The rest is in a module of its own. The crate root, where helpers are
	// written, imports every package-level variable and constant of the Go
	// package, and Rust does not let a local name or a parameter be the
	// name of one: a Go package with "var prefix" stopped the crate from
	// compiling. The module imports only what it uses.
	out.WriteString(`
#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
`)
	if goSliceModeEnabled() {
		out.WriteString("    #[allow(unused_imports)]\n    use super::{GoSlice, Vec};\n")
	}
	out.WriteString(replace.Replace(goSprintfFormatting))
	out.WriteString(replace.Replace(strings.Replace(goSprintfEntry, "\nfn go_sprintf(", "\npub(super) fn go_sprintf(", 1)))
	if spread {
		out.WriteString(`
// go_sprintf for f(format, args...), where the arguments are a slice.
pub(super) fn go_sprintf_spread(format: &str, args: &` + sliceType + `, wraps: bool) -> String {
    let guard = ` + guard + `;
    go_sprintf(format, guard.as_deref().unwrap_or(&[]), wraps)
}
`)
	}
	out.WriteString("}\n")
	if spread {
		out.WriteString("use __go_runtime_format::{go_sprintf, go_sprintf_spread};\n")
	} else {
		out.WriteString("use __go_runtime_format::go_sprintf;\n")
	}
}

const goSprintfFormatting = `
// A verb with its flags, as fmt reads it from a format.
#[derive(Default, Clone, Copy)]
struct GoFormatSpec {
    plus: bool,
    minus: bool,
    sharp: bool,
    space: bool,
    zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
}

// Pads text to the width of spec. A number passes the length of its sign so
// that zero padding goes between the sign and the digits.
fn go_format_pad(spec: &GoFormatSpec, text: String, sign_len: usize, zero_allowed: bool) -> String {
    let width = match spec.width {
        Some(width) => width,
        None => return text,
    };
    let len = text.chars().count();
    if len >= width {
        return text;
    }
    let fill = width - len;
    if spec.minus {
        return text + &" ".repeat(fill);
    }
    if spec.zero && zero_allowed {
        let (sign, digits) = text.split_at(sign_len);
        return format!("{}{}{}", sign, "0".repeat(fill), digits);
    }
    " ".repeat(fill) + &text
}

fn go_format_sign(spec: &GoFormatSpec, negative: bool) -> &'static str {
    if negative {
        "-"
    } else if spec.plus {
        "+"
    } else if spec.space {
        " "
    } else {
        ""
    }
}

// Whether strconv.IsPrint prints c as it is: a letter, mark, number,
// punctuation, symbol or the ASCII space. Rust's standard library has no
// table of Unicode categories, so this knows the control characters, the
// other spaces, the format characters in common use (the soft hyphen, the
// zero-width and direction marks, the byte order mark) and the private-use
// areas. It does not know which code points are unassigned.
fn go_format_is_print(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    if c.is_control() || c.is_whitespace() {
        return false;
    }
    !matches!(c as u32, 0xad | 0x600..=0x605 | 0x61c | 0x6dd | 0x70f | 0x8e2 | 0x180e | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff | 0xfff9..=0xfffb | 0xe000..=0xf8ff | 0xe0001 | 0xe0020..=0xe007f | 0xf0000..=0xffffd | 0x100000..=0x10fffd)
}

// Writes text the way strconv.Quote writes the inside of a quoted string,
// with quote as the delimiter. ascii is the + flag: every character outside
// ASCII is written as an escape.
fn go_format_escape(out: &mut String, text: &str, quote: char, ascii: bool) {
    for c in text.chars() {
        match c {
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            '\\' => out.push_str("\\\\"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if !go_format_is_print(c) || (ascii && !c.is_ascii()) => {
                if (c as u32) < 0x10000 {
                    out.push_str(&format!("\\u{:04x}", c as u32));
                } else {
                    out.push_str(&format!("\\U{:08x}", c as u32));
                }
            }
            c => out.push(c),
        }
    }
}

fn go_format_quote(text: &str, quote: char, ascii: bool) -> String {
    let mut out = String::new();
    out.push(quote);
    go_format_escape(&mut out, text, quote, ascii);
    out.push(quote);
    out
}

fn go_format_integer(spec: &GoFormatSpec, verb: char, value: i128) -> Option<String> {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let mut digits = match verb {
        'd' | 'v' => magnitude.to_string(),
        'b' => format!("{:b}", magnitude),
        'o' | 'O' => format!("{:o}", magnitude),
        'x' => format!("{:x}", magnitude),
        'X' => format!("{:X}", magnitude),
        'c' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, c.to_string(), 0, true));
        }
        'q' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, go_format_quote(&c.to_string(), '\'', spec.plus), 0, true));
        }
        'U' => {
            // A precision above four asks for more digits.
            let digits = spec.precision.filter(|precision| *precision > 4).unwrap_or(4);
            let mut text = format!("U+{:0digits$X}", value as u64, digits = digits);
            if spec.sharp {
                if let Some(c) = u32::try_from(value).ok().and_then(char::from_u32).filter(|c| go_format_is_print(*c)) {
                    text.push_str(&format!(" '{}'", c));
                }
            }
            return Some(go_format_pad(spec, text, 0, false));
        }
        _ => return None,
    };
    let sign = go_format_sign(spec, negative);
    // A precision and zero padding both ask for a least number of digits.
    // The sign is counted in the width of zero padding; 0x, 0b and 0o are
    // not, they come on top.
    let least_digits = match spec.precision {
        // Zero with a precision of zero prints nothing but the padding.
        Some(0) if magnitude == 0 => return Some(go_format_pad(&GoFormatSpec { zero: false, ..*spec }, String::new(), 0, false)),
        Some(precision) => precision,
        None if spec.zero && !spec.minus => spec.width.unwrap_or(0).saturating_sub(sign.len()),
        None => 0,
    };
    if digits.len() < least_digits {
        digits = "0".repeat(least_digits - digits.len()) + &digits;
    }
    // The 0 that # puts before an octal number is only added to one that
    // does not start with a 0 already. %#O has it as well, after its 0o.
    let octal_zero = if spec.sharp && matches!(verb, 'o' | 'O') && !digits.starts_with('0') { "0" } else { "" };
    let prefix = match verb {
        'b' if spec.sharp => "0b",
        'O' => "0o",
        'x' if spec.sharp => "0x",
        'X' if spec.sharp => "0X",
        _ => "",
    };
    Some(go_format_pad(spec, format!("{}{}{}{}", sign, prefix, octal_zero, digits), 0, false))
}

// The digits of a float and the power of ten of the first one, from Rust's
// {:e} form, which rounds as strconv does.
fn go_format_float_digits(exponent_form: &str) -> (String, i32) {
    let (mantissa, exponent) = exponent_form.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, exponent.parse().unwrap())
}

fn go_format_float_exponent(digits: &str, exponent: i32, upper: bool) -> String {
    let mut out = String::new();
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push(if upper { 'E' } else { 'e' });
    out.push(if exponent < 0 { '-' } else { '+' });
    out.push_str(&format!("{:02}", exponent.abs()));
    out
}

// Formats the magnitude of a float as strconv.FormatFloat does for 'e', 'f'
// and 'g'. shortest is the float printed with as few digits as identify it.
fn go_format_float_magnitude(verb: char, precision: Option<usize>, shortest: &str, with_precision: impl Fn(usize) -> String, fixed: impl Fn(usize) -> String) -> String {
    let upper = verb.is_ascii_uppercase();
    match verb.to_ascii_lowercase() {
        'e' => {
            let (digits, exponent) = go_format_float_digits(&with_precision(precision.unwrap_or(6)));
            go_format_float_exponent(&digits, exponent, upper)
        }
        'f' => fixed(precision.unwrap_or(6)),
        _ => {
            let (digits, exponent, exponent_limit) = match precision {
                None => {
                    let (digits, exponent) = go_format_float_digits(shortest);
                    (digits, exponent, 6)
                }
                Some(precision) => {
                    let precision = precision.max(1);
                    let (digits, exponent) = go_format_float_digits(&with_precision(precision - 1));
                    let digits = digits.trim_end_matches('0');
                    let digits = if digits.is_empty() { "0".to_string() } else { digits.to_string() };
                    let mut limit = precision as i32;
                    if limit > digits.len() as i32 && digits.len() as i32 >= exponent + 1 {
                        limit = digits.len() as i32;
                    }
                    (digits, exponent, limit)
                }
            };
            if exponent < -4 || exponent >= exponent_limit {
                return go_format_float_exponent(&digits, exponent, upper);
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
    }
}

fn go_format_float(spec: &GoFormatSpec, verb: char, value: f64, single: bool) -> Option<String> {
    // fmt also writes a float in binary (%b) and in hex (%x). Neither is
    // done here.
    if matches!(verb, 'b' | 'x' | 'X') {
        panic!("go2rust: %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if !matches!(verb, 'v' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G') {
        return None;
    }
    if spec.sharp {
        panic!("go2rust: the # flag on %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if value.is_nan() {
        let sign = if spec.plus { "+" } else if spec.space { " " } else { "" };
        return Some(go_format_pad(spec, format!("{}NaN", sign), 0, false));
    }
    let negative = value.is_sign_negative();
    if value.is_infinite() {
        // +Inf keeps its sign, unless the space flag asks for a space there.
        let sign = if negative { "-" } else if spec.space && !spec.plus { " " } else { "+" };
        return Some(go_format_pad(spec, format!("{}Inf", sign), 0, false));
    }
    let magnitude = value.abs();
    // A float32 keeps the digits of the float32, not of the float64 it was
    // widened to.
    let narrow = magnitude as f32;
    let shortest = if single { format!("{:e}", narrow) } else { format!("{:e}", magnitude) };
    let with_precision = |precision: usize| if single { format!("{:.*e}", precision, narrow) } else { format!("{:.*e}", precision, magnitude) };
    let fixed = |precision: usize| if single { format!("{:.*}", precision, narrow) } else { format!("{:.*}", precision, magnitude) };
    let verb = if verb == 'v' { 'g' } else { verb };
    let digits = go_format_float_magnitude(verb, spec.precision, &shortest, with_precision, fixed);
    let sign = go_format_sign(spec, negative);
    Some(go_format_pad(spec, format!("{}{}", sign, digits), sign.len(), true))
}

// %x and %X of a string or a []byte: two digits for each byte. A precision
// counts bytes here, not characters.
fn go_format_hex(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> String {
    let bytes = match spec.precision {
        Some(precision) if precision < bytes.len() => &bytes[..precision],
        _ => bytes,
    };
    let prefix = if !spec.sharp { "" } else if verb == 'x' { "0x" } else { "0X" };
    let digits: Vec<String> = bytes.iter().map(|b| if verb == 'x' { format!("{:02x}", b) } else { format!("{:02X}", b) }).collect();
    let text = if spec.space {
        digits.iter().map(|d| format!("{}{}", prefix, d)).collect::<Vec<_>>().join(" ")
    } else if digits.is_empty() {
        String::new()
    } else {
        format!("{}{}", prefix, digits.concat())
    };
    go_format_pad(spec, text, 0, true)
}

fn go_format_string(spec: &GoFormatSpec, verb: char, text: &str) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, text.as_bytes()));
    }
    let truncated: String = match spec.precision {
        Some(precision) => text.chars().take(precision).collect(),
        None => text.to_string(),
    };
    let formatted = match verb {
        's' | 'v' => truncated,
        'q' => {
            // strconv.CanBackquote: no backquote, no ASCII control character
            // but tab, and no byte order mark.
            let raw = truncated.chars().all(|c| if c.is_ascii() { (c >= ' ' || c == '\t') && c != '\u{60}' && c != '\u{7f}' } else { c != '\u{feff}' });
            if spec.sharp && raw {
                format!("\u{60}{}\u{60}", truncated)
            } else {
                go_format_quote(&truncated, '"', spec.plus)
            }
        }
        _ => return None,
    };
    Some(go_format_pad(spec, formatted, 0, true))
}

// A []byte prints as text for the verbs that print a string.
fn go_format_bytes(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, bytes));
    }
    if !matches!(verb, 's' | 'q') {
        return None;
    }
    // A precision counts characters, and a byte that is not UTF-8 counts as
    // one. It is applied first: what is cut off has no say in how the rest
    // is written.
    let mut kept = bytes.len();
    if let Some(precision) = spec.precision {
        let (mut count, mut end) = (0, 0);
        'chunks: for chunk in bytes.utf8_chunks() {
            for c in chunk.valid().chars() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += c.len_utf8();
            }
            for _ in chunk.invalid() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += 1;
            }
        }
        kept = end;
    }
    let bytes = &bytes[..kept];
    let spec = &GoFormatSpec { precision: None, ..*spec };
    match std::str::from_utf8(bytes) {
        Ok(text) => go_format_string(spec, verb, text),
        // %q writes a byte that is not UTF-8 as an escape.
        Err(_) if verb == 'q' => {
            let mut quoted = String::from("\"");
            for chunk in bytes.utf8_chunks() {
                go_format_escape(&mut quoted, chunk.valid(), '"', spec.plus);
                for byte in chunk.invalid() {
                    quoted.push_str(&format!("\\x{:02x}", byte));
                }
            }
            quoted.push('"');
            Some(go_format_pad(spec, quoted, 0, true))
        }
        // %s would have to write the byte as it is, and the result is a Rust
        // String, which cannot hold it.
        Err(_) => panic!("go2rust: %s of a []byte that is not UTF-8 is not supported in a format that is only known at run time"),
    }
}

// Formats one argument for one verb. None means the verb does not apply to
// the argument's type, which fmt reports in the output.
fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &GO_ANY_VALUE) -> Option<String> {
    if verb == 'T' {
        return Some(go_format_pad(spec, __go_type_name(value).to_string(), 0, true));
    }
    if verb == 'p' {
        panic!("go2rust: %p is not supported in a format that is only known at run time");
    }
    // %+v and %#v ask for field names and Go syntax. A number or a bool has
    // neither, so it prints as %v prints it, with two exceptions for %#v: an
    // unsigned integer is written in hex and a string in quotes.
    let (plus_v, sharp_v) = (verb == 'v' && spec.plus, verb == 'v' && spec.sharp);
    let basic = GoFormatSpec { plus: spec.plus && !plus_v, sharp: spec.sharp && !sharp_v, ..*spec };
    macro_rules! signed {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    signed!(i8, i16, i32, i64, isize);
    // A rune taken from a range over a string can be a Rust char; it is a
    // Go rune, an int32.
    if let Some(v) = value.downcast_ref::<char>() {
        return go_format_integer(&basic, verb, *v as i128);
    }
    macro_rules! unsigned {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                if sharp_v {
                    return go_format_integer(&GoFormatSpec { sharp: true, ..basic }, 'x', *v as i128);
                }
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    unsigned!(u8, u16, u32, u64, usize);
    if let Some(v) = value.downcast_ref::<f64>() {
        return go_format_float(&basic, verb, *v, false);
    }
    if let Some(v) = value.downcast_ref::<f32>() {
        return go_format_float(&basic, verb, *v as f64, true);
    }
    if let Some(v) = value.downcast_ref::<bool>() {
        return if matches!(verb, 't' | 'v') { Some(go_format_pad(&basic, v.to_string(), 0, true)) } else { None };
    }
    let string_verb = if sharp_v { 'q' } else { verb };
    if let Some(v) = value.downcast_ref::<String>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<&str>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<GO_BYTES_HANDLE>() {
        let guard = vGO_BYTES_GUARD;
        let bytes: &[u8] = guard.as_deref().unwrap_or(&[]);
        if let Some(text) = go_format_bytes(spec, verb, bytes) {
            return Some(text);
        }
        // Any other verb is applied to each byte, between brackets. A verb
        // that does not fit a number is reported for each byte. %#v writes
        // the slice as Go source, which is not done here.
        if !sharp_v {
            let items: Vec<String> = bytes.iter().map(|byte| go_format_integer(&basic, verb, *byte as i128).unwrap_or_else(|| format!("%!{}(uint8={})", verb, go_format_integer(&basic, 'd', *byte as i128).unwrap_or_default()))).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A value with a String or an Error method is the text that method
    // returns, for the verbs that print a string.
    if go_format_is_text(value) && !sharp_v && matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') {
        return go_format_string(&basic, verb, &format_any(value));
    }
    if let Some(inner) = go_format_named_basic_value(value) {
        return go_format_argument(spec, verb, inner.as_ref());
    }
    // Any other value prints as Println prints it, for a plain %v. fmt
    // applies a width, a precision, a flag or another verb to each field or
    // element, which is not done here. nil is one text.
    if verb == 'v' && !plus_v && !sharp_v {
        let text = format_any(value);
        if text == "<nil>" {
            return Some(go_format_pad(spec, text, 0, true));
        }
        if spec.width.is_none() && spec.precision.is_none() && !spec.space {
            return Some(text);
        }
    }
    panic!("go2rust: %{}{}{} of a {} is not supported in a format that is only known at run time", if spec.plus { "+" } else { "" }, if spec.sharp { "#" } else { "" }, verb, __go_type_name(value));
}
`

const goSprintfEntry = `
// The number after % or after the dot: digits, or * for the next argument.
// Ok(None) is no number. Err is a * whose argument is not a usable integer.
fn go_format_number(chars: &mut std::iter::Peekable<std::str::Chars>, args: &[GO_ANY_BOX], next: &mut usize) -> Result<Option<i64>, ()> {
    if chars.peek() == Some(&'*') {
        chars.next();
        let arg: &GO_ANY_VALUE = match args.get(*next) {
            Some(arg) => arg.as_ref(),
            None => return Err(()),
        };
        *next += 1;
        // fmt takes any integer type here.
        let mut value: Option<i128> = None;
        macro_rules! integer {
            ($($t:ty),*) => {
                $(if let Some(v) = arg.downcast_ref::<$t>() {
                    value = Some(*v as i128);
                })*
            };
        }
        integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
        return match value {
            Some(value) if value.abs() <= 1_000_000 => Ok(Some(value as i64)),
            _ => Err(()),
        };
    }
    let mut value: Option<i64> = None;
    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
        value = Some(value.unwrap_or(0).saturating_mul(10).saturating_add(digit as i64));
        chars.next();
    }
    Ok(value.filter(|value| *value <= 1_000_000))
}

// Whether value is of a type that go_format_argument formats itself.
fn go_format_is_basic(value: &GO_ANY_VALUE) -> bool {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str, GO_BYTES_HANDLE)
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &GO_ANY_VALUE) -> String {
    let text = go_format_argument(spec, go_format_plain_verb(value), value).unwrap_or_default();
    format!("{}={}", __go_type_name(value), text)
}

// The verb that writes value as %v does and leaves the flags as they are.
// %v itself would read + and # as %+v and %#v.
fn go_format_plain_verb(value: &GO_ANY_VALUE) -> char {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    if is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize) {
        'd'
    } else if is_one_of!(f32, f64) {
        'g'
    } else if value.is::<bool>() {
        't'
    } else if is_one_of!(String, &str) {
        's'
    } else {
        'v'
    }
}

// fmt.Sprintf for a format that is only known when the program runs. A
// format written as a literal is expanded when the program is translated and
// does not come here. wraps is true for fmt.Errorf, where %w prints as %v.
fn go_sprintf(format: &str, args: &[GO_ANY_BOX], wraps: bool) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let mut spec = GoFormatSpec::default();
        loop {
            match chars.peek() {
                Some('+') => spec.plus = true,
                Some('-') => spec.minus = true,
                Some('#') => spec.sharp = true,
                Some(' ') => spec.space = true,
                Some('0') => spec.zero = true,
                _ => break,
            }
            chars.next();
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        match go_format_number(&mut chars, args, &mut next) {
            // A negative width from an argument means left alignment.
            Ok(Some(width)) if width < 0 => {
                spec.minus = true;
                spec.width = Some((-width) as usize);
            }
            Ok(width) => spec.width = width.map(|width| width as usize),
            Err(()) => out.push_str("%!(BADWIDTH)"),
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            match go_format_number(&mut chars, args, &mut next) {
                Ok(Some(precision)) if precision < 0 => out.push_str("%!(BADPREC)"),
                Ok(precision) => spec.precision = Some(precision.unwrap_or(0) as usize),
                Err(()) => out.push_str("%!(BADPREC)"),
            }
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        let verb = match chars.next() {
            Some(verb) => verb,
            None => {
                out.push_str("%!(NOVERB)");
                break;
            }
        };
        if verb == '%' {
            out.push('%');
            continue;
        }
        let arg = match args.get(next) {
            Some(arg) => arg,
            None => {
                out.push_str(&format!("%!{}(MISSING)", verb));
                continue;
            }
        };
        next += 1;
        // In fmt.Errorf %w prints an error as %v does. A number, a string, a
        // bool or a []byte is not an error, and fmt reports the verb: once
        // for a scalar, and for each byte of a []byte.
        let verb = if verb == 'w' && wraps && !go_format_is_basic(arg.as_ref()) { 'v' } else { verb };
        match go_format_argument(&spec, verb, arg.as_ref()) {
            Some(text) => out.push_str(&text),
            None => out.push_str(&format!("%!{}({})", verb, go_format_typed_value(&spec, arg.as_ref()))),
        }
    }
    if next < args.len() {
        let extra: Vec<String> = args[next..].iter().map(|arg| go_format_typed_value(&GoFormatSpec::default(), arg.as_ref())).collect();
        out.push_str(&format!("%!(EXTRA {})", extra.join(", ")));
    }
    out
}
`
