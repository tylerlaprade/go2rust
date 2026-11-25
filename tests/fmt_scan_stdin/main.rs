#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

#[derive(Debug, Clone, PartialEq)]
enum GoScanError {
    Eof,
    UnexpectedEof,
    Msg(String),
}

// Where the scanner reads runes from: fmt's io.RuneScanner.
trait GoScanRunes {
    fn go_read_rune(&mut self) -> Option<char>;
    fn go_unread_rune(&mut self);
}

// fmt's readRune: UTF-8 from a byte source that cannot unread, with one
// rune of lookahead that is lost when the scan ends.
struct GoScanReadRune<F: FnMut() -> Option<u8>> {
    next_byte: F,
    pending: std::vec::Vec<u8>,
    peek: Option<char>,
    last: Option<char>,
}

impl<F: FnMut() -> Option<u8>> GoScanReadRune<F> {
    fn new(next_byte: F) -> Self {
        GoScanReadRune { next_byte, pending: std::vec::Vec::new(), peek: None, last: None }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if !self.pending.is_empty() {
            return Some(self.pending.remove(0));
        }
        (self.next_byte)()
    }
}

impl<F: FnMut() -> Option<u8>> GoScanRunes for GoScanReadRune<F> {
    fn go_read_rune(&mut self) -> Option<char> {
        if let Some(r) = self.peek.take() {
            self.last = Some(r);
            return Some(r);
        }
        let first = self.read_byte()?;
        if first < 0x80 {
            let r = first as char;
            self.last = Some(r);
            return Some(r);
        }
        let want = if first >= 0xF0 { 4 } else if first >= 0xE0 { 3 } else if first >= 0xC0 { 2 } else { 1 };
        let mut bytes: std::vec::Vec<u8> = std::vec![first];
        while bytes.len() < want {
            match self.read_byte() {
                Some(b) => bytes.push(b),
                None => break,
            }
        }
        let r = match std::str::from_utf8(&bytes) {
            Ok(text) => text.chars().next().unwrap_or('\u{FFFD}'),
            Err(_) => {
                // Bad UTF-8 is one U+FFFD for its first byte; the rest
                // is read again.
                self.pending.extend_from_slice(&bytes[1..]);
                '\u{FFFD}'
            }
        };
        self.last = Some(r);
        Some(r)
    }

    fn go_unread_rune(&mut self) {
        if let Some(r) = self.last.take() {
            self.peek = Some(r);
        }
    }
}

// An io.RuneScanner of the program, as *bufio.Reader or *strings.Reader:
// its own ReadRune and UnreadRune.
struct GoScanRuneFns<'a> {
    read: Box<dyn FnMut() -> Option<char> + 'a>,
    unread: Box<dyn FnMut() + 'a>,
}

impl<'a> GoScanRunes for GoScanRuneFns<'a> {
    fn go_read_rune(&mut self) -> Option<char> {
        (self.read)()
    }

    fn go_unread_rune(&mut self) {
        (self.unread)()
    }
}

fn go_scan_string_source(text: String) -> GoScanReadRune<impl FnMut() -> Option<u8>> {
    let bytes = text.into_bytes();
    let mut at = 0usize;
    GoScanReadRune::new(move || {
        let b = bytes.get(at).copied();
        if b.is_some() {
            at += 1;
        }
        b
    })
}

fn go_scan_stdin_source() -> GoScanReadRune<impl FnMut() -> Option<u8>> {
    GoScanReadRune::new(|| {
        use std::io::Read;
        let mut byte = [0u8; 1];
        match std::io::stdin().lock().read(&mut byte) {
            Ok(1) => Some(byte[0]),
            _ => None,
        }
    })
}

// An operand: where the scanned value goes.
enum GoScanTarget<'a> {
    Int(u32, Box<dyn FnMut(i64) + 'a>),
    Uint(u32, Box<dyn FnMut(u64) + 'a>),
    Float(u32, Box<dyn FnMut(f64) + 'a>),
    Str(Box<dyn FnMut(String) + 'a>),
    Bytes(Box<dyn FnMut(std::vec::Vec<u8>) + 'a>),
    Bool(Box<dyn FnMut(bool) + 'a>),
}

const GO_SCAN_HUGE_WID: usize = 1 << 30;

struct GoScanState<'a> {
    rs: &'a mut dyn GoScanRunes,
    buf: String,
    count: usize,
    at_eof: bool,
    nl_is_end: bool,
    nl_is_space: bool,
    arg_limit: usize,
    limit: usize,
    max_wid: usize,
}

fn go_scan_is_space(r: char) -> bool {
    let r = r as u32;
    if r >= 1 << 16 {
        return false;
    }
    const SPACE: [(u32, u32); 10] = [
        (0x0009, 0x000d),
        (0x0020, 0x0020),
        (0x0085, 0x0085),
        (0x00a0, 0x00a0),
        (0x1680, 0x1680),
        (0x2000, 0x200a),
        (0x2028, 0x2029),
        (0x202f, 0x202f),
        (0x205f, 0x205f),
        (0x3000, 0x3000),
    ];
    for (lo, hi) in SPACE {
        if r < lo {
            return false;
        }
        if r <= hi {
            return true;
        }
    }
    false
}

fn go_scan_quote(tok: &str) -> String {
    let mut out = String::from("\"");
    for c in tok.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

// strconv's underscoreOK: underscores only between digits, or between a
// base prefix and a digit.
fn go_scan_underscore_ok(s: &str) -> bool {
    let b = s.as_bytes();
    let mut saw = b'^';
    let mut i = 0;
    let b = if !b.is_empty() && (b[0] == b'-' || b[0] == b'+') { &b[1..] } else { b };
    let mut hex = false;
    if b.len() >= 2 && b[0] == b'0' && matches!(b[1] | 0x20, b'b' | b'o' | b'x') {
        i = 2;
        saw = b'0';
        hex = (b[1] | 0x20) == b'x';
    }
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_digit() || (hex && (c | 0x20) >= b'a' && (c | 0x20) <= b'f') {
            saw = b'0';
        } else if c == b'_' {
            if saw != b'0' {
                return false;
            }
            saw = b'_';
        } else {
            if saw == b'_' {
                return false;
            }
            saw = b'!';
        }
        i += 1;
    }
    saw != b'_'
}

// strconv.ParseUint's digits: base 0 reads a prefix and allows
// underscores. Ok(None) is a value out of range.
fn go_scan_parse_digits(s: &str, base: u32) -> Result<Option<u64>, ()> {
    if s.is_empty() {
        return Err(());
    }
    let mut base = base;
    let mut digits = s;
    let base0 = base == 0;
    if base0 {
        base = 10;
        let b = s.as_bytes();
        if b[0] == b'0' {
            if b.len() >= 3 && (b[1] | 0x20) == b'b' {
                base = 2;
                digits = &s[2..];
            } else if b.len() >= 3 && (b[1] | 0x20) == b'o' {
                base = 8;
                digits = &s[2..];
            } else if b.len() >= 3 && (b[1] | 0x20) == b'x' {
                base = 16;
                digits = &s[2..];
            } else {
                base = 8;
                digits = &s[1..];
            }
        }
    }
    let mut value: u64 = 0;
    let mut overflow = false;
    let mut underscores = false;
    let mut any = false;
    for c in digits.chars() {
        if c == '_' && base0 {
            underscores = true;
            continue;
        }
        let d = match c.to_digit(36) {
            Some(d) if d < base => d as u64,
            _ => return Err(()),
        };
        any = true;
        match value.checked_mul(base as u64).and_then(|v| v.checked_add(d)) {
            Some(v) => value = v,
            None => overflow = true,
        }
    }
    if !any && !(base0 && base == 8 && digits.is_empty()) {
        return Err(());
    }
    if underscores && !go_scan_underscore_ok(s) {
        return Err(());
    }
    if overflow {
        return Ok(None);
    }
    Ok(Some(value))
}

impl<'a> GoScanState<'a> {
    fn new(rs: &'a mut dyn GoScanRunes, nl_is_space: bool, nl_is_end: bool) -> Self {
        GoScanState {
            rs,
            buf: String::new(),
            count: 0,
            at_eof: false,
            nl_is_end,
            nl_is_space,
            arg_limit: GO_SCAN_HUGE_WID,
            limit: GO_SCAN_HUGE_WID,
            max_wid: GO_SCAN_HUGE_WID,
        }
    }

    fn read_rune(&mut self) -> Option<char> {
        if self.at_eof || self.count >= self.arg_limit {
            return None;
        }
        match self.rs.go_read_rune() {
            Some(r) => {
                self.count += 1;
                if self.nl_is_end && r == '\n' {
                    self.at_eof = true;
                }
                Some(r)
            }
            None => {
                self.at_eof = true;
                None
            }
        }
    }

    fn get_rune(&mut self) -> Option<char> {
        self.read_rune()
    }

    fn must_read_rune(&mut self) -> Result<char, GoScanError> {
        self.get_rune().ok_or(GoScanError::UnexpectedEof)
    }

    fn unread_rune(&mut self) {
        self.rs.go_unread_rune();
        self.at_eof = false;
        self.count -= 1;
    }

    fn error_string(&self, text: &str) -> GoScanError {
        GoScanError::Msg(text.to_string())
    }

    fn skip_space(&mut self) -> Result<(), GoScanError> {
        loop {
            let r = match self.get_rune() {
                None => return Ok(()),
                Some(r) => r,
            };
            if r == '\r' && self.peek("\n") {
                continue;
            }
            if r == '\n' {
                if self.nl_is_space {
                    continue;
                }
                return Err(self.error_string("unexpected newline"));
            }
            if !go_scan_is_space(r) {
                self.unread_rune();
                return Ok(());
            }
        }
    }

    fn token(&mut self, skip_space: bool) -> Result<String, GoScanError> {
        if skip_space {
            self.skip_space()?;
        }
        loop {
            let r = match self.get_rune() {
                None => break,
                Some(r) => r,
            };
            if go_scan_is_space(r) {
                self.unread_rune();
                break;
            }
            self.buf.push(r);
        }
        Ok(self.buf.clone())
    }

    fn consume(&mut self, ok: &str, accept: bool) -> bool {
        let r = match self.get_rune() {
            None => return false,
            Some(r) => r,
        };
        if ok.contains(r) {
            if accept {
                self.buf.push(r);
            }
            return true;
        }
        if accept {
            self.unread_rune();
        }
        false
    }

    fn peek(&mut self, ok: &str) -> bool {
        match self.get_rune() {
            None => false,
            Some(r) => {
                self.unread_rune();
                ok.contains(r)
            }
        }
    }

    fn not_eof(&mut self) -> Result<(), GoScanError> {
        match self.get_rune() {
            None => Err(GoScanError::Eof),
            Some(_) => {
                self.unread_rune();
                Ok(())
            }
        }
    }

    fn accept(&mut self, ok: &str) -> bool {
        self.consume(ok, true)
    }

    fn ok_verb(&self, verb: char, ok_verbs: &str, typ: &str) -> Result<(), GoScanError> {
        if ok_verbs.contains(verb) {
            Ok(())
        } else {
            Err(GoScanError::Msg(format!("bad verb '%{}' for {}", verb, typ)))
        }
    }

    fn scan_bool(&mut self, verb: char) -> Result<bool, GoScanError> {
        self.skip_space()?;
        self.not_eof()?;
        self.ok_verb(verb, "tv", "boolean")?;
        let bool_error = || GoScanError::Msg("syntax error scanning boolean".to_string());
        match self.get_rune() {
            Some('0') => Ok(false),
            Some('1') => Ok(true),
            Some('t') | Some('T') => {
                if self.accept("rR") && (!self.accept("uU") || !self.accept("eE")) {
                    return Err(bool_error());
                }
                Ok(true)
            }
            Some('f') | Some('F') => {
                if self.accept("aA") && (!self.accept("lL") || !self.accept("sS") || !self.accept("eE")) {
                    return Err(bool_error());
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn get_base(&self, verb: char) -> Result<(u32, &'static str), GoScanError> {
        self.ok_verb(verb, "bdoUxXv", "integer")?;
        Ok(match verb {
            'b' => (2, "01"),
            'o' => (8, "01234567"),
            'x' | 'X' | 'U' => (16, "0123456789aAbBcCdDeEfF"),
            _ => (10, "0123456789"),
        })
    }

    fn scan_number(&mut self, digits: &str, have_digits: bool) -> Result<String, GoScanError> {
        if !have_digits {
            self.not_eof()?;
            if !self.accept(digits) {
                return Err(self.error_string("expected integer"));
            }
        }
        while self.accept(digits) {}
        Ok(self.buf.clone())
    }

    fn scan_rune(&mut self, bit_size: u32) -> Result<i64, GoScanError> {
        self.not_eof()?;
        let r = self.get_rune().map(|r| r as i64).unwrap_or(-1);
        let shift = 64 - bit_size;
        if (r << shift) >> shift != r {
            let text = char::from_u32(r as u32).map(|c| c.to_string()).unwrap_or_default();
            return Err(GoScanError::Msg(format!("overflow on character value {}", text)));
        }
        Ok(r)
    }

    fn scan_base_prefix(&mut self) -> (u32, &'static str, bool) {
        if !self.peek("0") {
            return (0, "0123456789_", false);
        }
        self.accept("0");
        if self.peek("bB") {
            self.consume("bB", true);
            (0, "01_", true)
        } else if self.peek("oO") {
            self.consume("oO", true);
            (0, "01234567_", true)
        } else if self.peek("xX") {
            self.consume("xX", true);
            (0, "0123456789aAbBcCdDeEfF_", true)
        } else {
            (0, "01234567_", true)
        }
    }

    fn scan_int(&mut self, verb: char, bit_size: u32) -> Result<i64, GoScanError> {
        if verb == 'c' {
            return self.scan_rune(bit_size);
        }
        self.skip_space()?;
        self.not_eof()?;
        let (mut base, mut digits) = self.get_base(verb)?;
        let mut have_digits = false;
        if verb == 'U' {
            if !self.consume("U", false) || !self.consume("+", false) {
                return Err(self.error_string("bad unicode format "));
            }
        } else {
            self.accept("+-");
            if verb == 'v' {
                let (b, d, h) = self.scan_base_prefix();
                base = b;
                digits = d;
                have_digits = h;
            }
        }
        let tok = self.scan_number(digits, have_digits)?;
        let (negative, unsigned) = match tok.as_bytes().first() {
            Some(b'-') => (true, &tok[1..]),
            Some(b'+') => (false, &tok[1..]),
            _ => (false, &tok[..]),
        };
        let syntax = || GoScanError::Msg(format!("strconv.ParseInt: parsing {}: invalid syntax", go_scan_quote(&tok)));
        let range = || GoScanError::Msg(format!("strconv.ParseInt: parsing {}: value out of range", go_scan_quote(&tok)));
        let magnitude = match go_scan_parse_digits(unsigned, base) {
            Err(()) => return Err(syntax()),
            Ok(None) => return Err(range()),
            Ok(Some(m)) => m,
        };
        let value = if negative {
            if magnitude > 1u64 << 63 {
                return Err(range());
            }
            (magnitude as i64).wrapping_neg()
        } else {
            if magnitude > i64::MAX as u64 {
                return Err(range());
            }
            magnitude as i64
        };
        let shift = 64 - bit_size;
        if (value << shift) >> shift != value {
            return Err(GoScanError::Msg(format!("integer overflow on token {}", tok)));
        }
        Ok(value)
    }

    fn scan_uint(&mut self, verb: char, bit_size: u32) -> Result<u64, GoScanError> {
        if verb == 'c' {
            return self.scan_rune(bit_size).map(|r| r as u64);
        }
        self.skip_space()?;
        self.not_eof()?;
        let (mut base, mut digits) = self.get_base(verb)?;
        let mut have_digits = false;
        if verb == 'U' {
            if !self.consume("U", false) || !self.consume("+", false) {
                return Err(self.error_string("bad unicode format "));
            }
        } else if verb == 'v' {
            let (b, d, h) = self.scan_base_prefix();
            base = b;
            digits = d;
            have_digits = h;
        }
        let tok = self.scan_number(digits, have_digits)?;
        let value = match go_scan_parse_digits(&tok, base) {
            Err(()) => return Err(GoScanError::Msg(format!("strconv.ParseUint: parsing {}: invalid syntax", go_scan_quote(&tok)))),
            Ok(None) => return Err(GoScanError::Msg(format!("strconv.ParseUint: parsing {}: value out of range", go_scan_quote(&tok)))),
            Ok(Some(v)) => v,
        };
        let shift = 64 - bit_size;
        if (value << shift) >> shift != value {
            return Err(GoScanError::Msg(format!("unsigned integer overflow on token {}", tok)));
        }
        Ok(value)
    }

    fn float_token(&mut self) -> String {
        self.buf.clear();
        if self.accept("nN") && self.accept("aA") && self.accept("nN") {
            return self.buf.clone();
        }
        self.accept("+-");
        if self.accept("iI") && self.accept("nN") && self.accept("fF") {
            return self.buf.clone();
        }
        let mut digits = "0123456789_";
        let mut exp = "eEpP";
        if self.accept("0") && self.accept("xX") {
            digits = "0123456789aAbBcCdDeEfF_";
            exp = "pP";
        }
        while self.accept(digits) {}
        if self.accept(".") {
            while self.accept(digits) {}
        }
        if self.accept(exp) {
            self.accept("+-");
            while self.accept("0123456789_") {}
        }
        self.buf.clone()
    }

    // strconv.ParseFloat for the decimal forms floatToken reads.
    fn parse_float(text: &str, bit_size: u32) -> Result<f64, GoScanError> {
        let syntax = || GoScanError::Msg(format!("strconv.ParseFloat: parsing {}: invalid syntax", go_scan_quote(text)));
        let lower = text.to_ascii_lowercase();
        let body = lower.trim_start_matches(|c| c == '+' || c == '-');
        if body == "inf" || body == "infinity" || body == "nan" {
            let value: f64 = lower.parse().map_err(|_| syntax())?;
            return Ok(value);
        }
        if body.starts_with("0x") || !body.chars().any(|c| c.is_ascii_digit()) {
            return Err(syntax());
        }
        let plain: String;
        let text_ref = if text.contains('_') {
            if !go_scan_underscore_ok(text) {
                return Err(syntax());
            }
            plain = text.replace('_', "");
            &plain
        } else {
            text
        };
        if text_ref.ends_with(|c: char| c == 'e' || c == 'E' || c == '+' || c == '-') {
            return Err(syntax());
        }
        let value = if bit_size == 32 {
            text_ref.parse::<f32>().map(|v| v as f64).map_err(|_| syntax())?
        } else {
            text_ref.parse::<f64>().map_err(|_| syntax())?
        };
        if value.is_infinite() {
            return Err(GoScanError::Msg(format!("strconv.ParseFloat: parsing {}: value out of range", go_scan_quote(text))));
        }
        Ok(value)
    }

    fn convert_float(&self, text: &str, bit_size: u32) -> Result<f64, GoScanError> {
        if let Some(p) = text.find('p') {
            if !text.contains(|c| c == 'x' || c == 'X') {
                let mantissa = Self::parse_float(&text[..p], bit_size)?;
                let exp: i32 = text[p + 1..].parse().map_err(|_| GoScanError::Msg(format!("strconv.Atoi: parsing {}: invalid syntax", go_scan_quote(text))))?;
                return Ok(mantissa * 2f64.powi(exp));
            }
        }
        Self::parse_float(text, bit_size)
    }

    fn scan_float(&mut self, verb: char, bit_size: u32) -> Result<f64, GoScanError> {
        let typ = if bit_size == 32 { "float32" } else { "float64" };
        self.ok_verb(verb, "beEfFgGv", typ)?;
        self.skip_space()?;
        self.not_eof()?;
        let tok = self.float_token();
        self.convert_float(&tok, bit_size)
    }

    fn quoted_string(&mut self) -> Result<String, GoScanError> {
        self.not_eof()?;
        let quote = self.get_rune();
        match quote {
            Some('`') => {
                loop {
                    let r = self.must_read_rune()?;
                    if r == '`' {
                        break;
                    }
                    self.buf.push(r);
                }
                Ok(self.buf.clone())
            }
            Some('"') => {
                self.buf.push('"');
                loop {
                    let r = self.must_read_rune()?;
                    self.buf.push(r);
                    if r == '\\' {
                        let next = self.must_read_rune()?;
                        self.buf.push(next);
                    } else if r == '"' {
                        break;
                    }
                }
                go_scan_unquote(&self.buf).ok_or_else(|| GoScanError::Msg("invalid syntax".to_string()))
            }
            _ => Err(self.error_string("expected quoted string")),
        }
    }

    fn hex_byte(&mut self) -> Result<Option<u8>, GoScanError> {
        let first = match self.get_rune() {
            None => return Ok(None),
            Some(r) => r,
        };
        let high = match first.to_digit(16) {
            Some(d) => d,
            None => {
                self.unread_rune();
                return Ok(None);
            }
        };
        let second = self.must_read_rune()?;
        match second.to_digit(16) {
            Some(low) => Ok(Some((high << 4 | low) as u8)),
            None => Err(self.error_string("illegal hex digit")),
        }
    }

    fn hex_bytes(&mut self) -> Result<std::vec::Vec<u8>, GoScanError> {
        self.not_eof()?;
        let mut bytes: std::vec::Vec<u8> = std::vec::Vec::new();
        while let Some(b) = self.hex_byte()? {
            bytes.push(b);
        }
        if bytes.is_empty() {
            return Err(self.error_string("no hex data for %x string"));
        }
        Ok(bytes)
    }

    fn convert_bytes(&mut self, verb: char) -> Result<std::vec::Vec<u8>, GoScanError> {
        self.ok_verb(verb, "svqxX", "string")?;
        self.skip_space()?;
        self.not_eof()?;
        match verb {
            'q' => self.quoted_string().map(|s| s.into_bytes()),
            'x' | 'X' => self.hex_bytes(),
            _ => self.token(true).map(|s| s.into_bytes()),
        }
    }

    fn scan_percent(&mut self) -> Result<(), GoScanError> {
        self.skip_space()?;
        self.not_eof()?;
        if !self.accept("%") {
            return Err(self.error_string("missing literal %"));
        }
        Ok(())
    }

    fn scan_one(&mut self, verb: char, target: &mut GoScanTarget) -> Result<(), GoScanError> {
        self.buf.clear();
        match target {
            GoScanTarget::Bool(set) => {
                let v = self.scan_bool(verb)?;
                set(v);
            }
            GoScanTarget::Int(bits, set) => {
                let v = self.scan_int(verb, *bits)?;
                set(v);
            }
            GoScanTarget::Uint(bits, set) => {
                let v = self.scan_uint(verb, *bits)?;
                set(v);
            }
            GoScanTarget::Float(bits, set) => {
                let v = self.scan_float(verb, *bits)?;
                set(v);
            }
            GoScanTarget::Str(set) => {
                let v = self.convert_bytes(verb)?;
                set(String::from_utf8_lossy(&v).into_owned());
            }
            GoScanTarget::Bytes(set) => {
                let v = self.convert_bytes(verb)?;
                set(v);
            }
        }
        Ok(())
    }

    fn do_scan(&mut self, targets: &mut [GoScanTarget]) -> (i64, Option<GoScanError>) {
        let mut n = 0i64;
        for target in targets.iter_mut() {
            if let Err(e) = self.scan_one('v', target) {
                return (n, Some(e));
            }
            n += 1;
        }
        if self.nl_is_end {
            loop {
                match self.get_rune() {
                    None | Some('\n') => break,
                    Some(r) if !go_scan_is_space(r) => return (n, Some(self.error_string("expected newline"))),
                    _ => {}
                }
            }
        }
        (n, None)
    }

    // advance matches the format's literal text and spaces against the
    // input, and returns how many bytes of the format it used, or -1 when
    // the input does not match.
    fn advance(&mut self, format: &str) -> Result<i64, GoScanError> {
        let mut i = 0usize;
        while i < format.len() {
            let mut fmtc = format[i..].chars().next().unwrap();
            let mut w = fmtc.len_utf8();
            if go_scan_is_space(fmtc) {
                let mut newlines = 0;
                let mut trailing_space = false;
                while go_scan_is_space(fmtc) && i < format.len() {
                    if fmtc == '\n' {
                        newlines += 1;
                        trailing_space = false;
                    } else {
                        trailing_space = true;
                    }
                    i += w;
                    match format[i..].chars().next() {
                        Some(c) => {
                            fmtc = c;
                            w = c.len_utf8();
                        }
                        None => break,
                    }
                }
                for _ in 0..newlines {
                    let mut inputc = self.get_rune();
                    while matches!(inputc, Some(c) if go_scan_is_space(c) && c != '\n') {
                        inputc = self.get_rune();
                    }
                    if inputc != Some('\n') && inputc.is_some() {
                        return Err(self.error_string("newline in format does not match input"));
                    }
                }
                if trailing_space {
                    let mut inputc = self.get_rune();
                    if newlines == 0 {
                        if matches!(inputc, Some(c) if !go_scan_is_space(c)) {
                            return Err(self.error_string("expected space in input to match format"));
                        }
                        if inputc == Some('\n') {
                            return Err(self.error_string("newline in input does not match format"));
                        }
                    }
                    while matches!(inputc, Some(c) if go_scan_is_space(c) && c != '\n') {
                        inputc = self.get_rune();
                    }
                    if inputc.is_some() {
                        self.unread_rune();
                    }
                }
                continue;
            }
            if fmtc == '%' {
                if i + w == format.len() {
                    return Err(self.error_string("missing verb: % at end of format string"));
                }
                let nextc = format[i + w..].chars().next();
                if nextc != Some('%') {
                    return Ok(i as i64);
                }
                i += w;
            }
            let inputc = self.must_read_rune()?;
            if fmtc != inputc {
                self.unread_rune();
                return Ok(-1);
            }
            i += w;
        }
        Ok(i as i64)
    }

    fn do_scanf(&mut self, format: &str, targets: &mut [GoScanTarget]) -> (i64, Option<GoScanError>) {
        let mut n = 0usize;
        let result = (|| -> Result<(), GoScanError> {
            let bytes = format.as_bytes();
            let end = format.len() as i64 - 1;
            let mut i = 0usize;
            while (i as i64) <= end {
                let w = self.advance(&format[i..])?;
                if w > 0 {
                    i += w as usize;
                    continue;
                }
                if bytes[i] != b'%' {
                    if w < 0 {
                        return Err(self.error_string("input does not match format"));
                    }
                    break;
                }
                i += 1;
                // The width, as fmt's parsenum reads it.
                let mut width = 0usize;
                let mut wid_present = false;
                if (i as i64) < end {
                    while (i as i64) < end && bytes[i].is_ascii_digit() {
                        width = width * 10 + (bytes[i] - b'0') as usize;
                        wid_present = true;
                        i += 1;
                    }
                } else {
                    i = end as usize;
                }
                self.max_wid = if wid_present { width } else { GO_SCAN_HUGE_WID };
                let c = format[i..].chars().next().unwrap_or('%');
                let cw = c.len_utf8();
                i += cw;
                if c != 'c' {
                    self.skip_space()?;
                }
                if c == '%' {
                    self.scan_percent()?;
                    continue;
                }
                self.arg_limit = self.limit;
                if self.count + self.max_wid < self.arg_limit {
                    self.arg_limit = self.count + self.max_wid;
                }
                if n >= targets.len() {
                    return Err(GoScanError::Msg(format!("too few operands for format '%{}'", &format[i - cw..])));
                }
                self.scan_one(c, &mut targets[n])?;
                n += 1;
                self.arg_limit = self.limit;
            }
            if n < targets.len() {
                return Err(self.error_string("too many operands"));
            }
            Ok(())
        })();
        (n as i64, result.err())
    }
}

// strconv.Unquote of a double-quoted string.
fn go_scan_unquote(text: &str) -> Option<String> {
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '"' || c == '\n' {
            return None;
        }
        if c != '\\' {
            out.push(c);
            continue;
        }
        let escape = chars.next()?;
        match escape {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            'a' => out.push('\u{7}'),
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            'v' => out.push('\u{b}'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            '\'' => return None,
            'x' | 'u' | 'U' => {
                let len = match escape { 'x' => 2, 'u' => 4, _ => 8 };
                let hex: String = (0..len).filter_map(|_| chars.next()).collect();
                if hex.len() != len {
                    return None;
                }
                let value = u32::from_str_radix(&hex, 16).ok()?;
                out.push(char::from_u32(value)?);
            }
            '0'..='7' => {
                let mut value = escape.to_digit(8)?;
                for _ in 0..2 {
                    value = value * 8 + chars.next()?.to_digit(8)?;
                }
                out.push(char::from_u32(value)?);
            }
            _ => return None,
        }
    }
    Some(out)
}

// go_fmt_scan scans runes into targets as fmt.Fscan (mode 's'),
// fmt.Fscanln ('l') or fmt.Fscanf ('f', with format) does, and returns the
// number of operands filled and the error.
fn go_fmt_scan(rs: &mut dyn GoScanRunes, mode: char, format: &str, targets: &mut [GoScanTarget]) -> (i64, Option<GoScanError>) {
    let (nl_is_space, nl_is_end) = match mode {
        's' => (true, false),
        'l' => (false, true),
        _ => (false, false),
    };
    let mut state = GoScanState::new(rs, nl_is_space, nl_is_end);
    if mode == 'f' {
        state.do_scanf(format, targets)
    } else {
        state.do_scan(targets)
    }
}

#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize>: Send + Sync {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.lock().unwrap().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    outer: Arc<Mutex<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.lock().unwrap().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
    pub fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        // &s[i] panics, as indexing does, when i is not below len(s). A
        // slice already borrowed for the expression is not checked.
        if let Ok(__guard) = slice.try_lock() {
            let __len = __guard.as_ref().map_or(0, |__values| __values.len());
            if index >= __len {
                drop(__guard);
                panic!("index out of bounds: the len is {} but the index is {}", __len, index);
            }
            // Hold the backing array the slice has now, not the variable.
            let __header = (*__guard).clone();
            drop(__guard);
            return GoSliceElemPtr { slice: std::sync::Arc::new(std::sync::Mutex::new(__header)), index };
        }
        GoSliceElemPtr { slice, index }
    }

    /// The array and element the pointer names, for == between pointers.
    pub fn identity(&self) -> (usize, usize) {
        if let Ok(__guard) = self.slice.try_lock() {
            if let Some(__slice) = __guard.as_ref() {
                let __base = __slice.as_slice().as_ptr() as usize;
                return (__base.wrapping_add(self.index.wrapping_mul(std::mem::size_of::<T>())), 0);
            }
        }
        (Arc::as_ptr(&self.slice) as *const () as usize, self.index)
    }

    pub fn slice_handle(&self) -> Arc<Mutex<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Arc<Mutex<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Arc<Mutex<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    /// The element as the guard of a cell, for code written against a
    /// Mutex-held value: sync/atomic's Load and Store on &a[i].
    #[allow(dead_code)]
    pub fn lock(&self) -> Result<GoArrayElemMutRef<T, N>, std::convert::Infallible> {
        Ok(self.borrow_mut())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: Send + Sync + 'static>: Send + Sync {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: Send + Sync + 'static> {
    borrow: Arc<dyn Fn() -> Option<T> + Send + Sync>,
    assign: Arc<dyn Fn(Option<T>) + Send + Sync>,
    with_mut: Arc<dyn Fn(&mut dyn FnMut(&mut T)) + Send + Sync>,
    identity: Arc<dyn Fn() -> (*const (), usize) + Send + Sync>,
}

impl<T: Send + Sync + 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
    fn borrow_dyn(&self) -> Option<T> {
        (self.borrow)()
    }

    fn assign_dyn(&self, value: Option<T>) {
        (self.assign)(value)
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        (self.with_mut)(f)
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        (self.identity)()
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: Send + Sync + 'static> {
    Nil,
    Raw(usize),
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Arc<dyn GoArrayElemPtrDyn<T> + Send + Sync>),
}

impl<T: Send + Sync + 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: Send + Sync + 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Arc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Arc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Arc<dyn Fn() -> Option<T> + Send + Sync>,
        assign: Arc<dyn Fn(Option<T>) + Send + Sync>,
        with_mut: Arc<dyn Fn(&mut dyn FnMut(&mut T)) + Send + Sync>,
        identity: Arc<dyn Fn() -> (*const (), usize) + Send + Sync>,
    ) -> Self {
        GoPtr::ArrayElem(Arc::new(GoForeignArrayElemPtrDyn {
            borrow,
            assign,
            with_mut,
            identity,
        }))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.lock().unwrap().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.lock().unwrap();
                guard.as_ref().and_then(|values| values.get(value.index)).is_none()
            }
            GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.lock().unwrap();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.lock().unwrap();
                let values = guard.as_mut().expect("nil pointer dereference");
                f(values.get_mut(slot.index).expect("nil pointer dereference"))
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Arc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                left.identity() == right.identity()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Arc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => {
                let (base, index) = value.identity();
                base.wrapping_add(index)
            }
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> GoPtr<T> {
    /// The pointee, read the way the generated code reads a cell in a
    /// program with goroutines, ptr.lock().unwrap(), as borrow() is in one
    /// without: a []*T or map[K]*T element read through the cell path.
    pub fn lock(&self) -> Result<Option<T>, std::convert::Infallible> {
        Ok(self.borrow())
    }

    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.lock().unwrap()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.lock().unwrap() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: Send + Sync + 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

fn main() {
    ::bufio::__go_init_all();
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    { let mut __scan_src = go_scan_stdin_source(); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = n.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    let mut nums = Arc::new(Mutex::new(Some(vec![0i64; ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    for i in 0..(({ let __range_holder = nums.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let mut __scan_src = go_scan_stdin_source(); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = GoSliceElemPtr::new(nums.clone(), (i) as usize); Box::new(move |__scan_v: i64| { *__scan_h.borrow_mut() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    }
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut score: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let (mut cnt, mut err) = { let mut __scan_src = go_scan_stdin_source(); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Str({ let __scan_h = name.clone(); Box::new(move |__scan_v: String| { *__scan_h.lock().unwrap() = Some(__scan_v); }) }), GoScanTarget::Float(64, { let __scan_h = score.clone(); Box::new(move |__scan_v: f64| { *__scan_h.lock().unwrap() = Some(__scan_v as f64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'l', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    println!("{} {} {} {} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&nums)), format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }), { let __text = go_float_string(({ let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text }, format!("{}", cnt), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut a: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut b: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let (mut k, __tmp_1) = { let mut __scan_src = go_scan_stdin_source(); let __scan_format: String = "%d,%d\n".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = a.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) }), GoScanTarget::Int(64, { let __scan_h = b.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    println!("{} {} {}", format!("{}", { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }), format!("{}", k), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut r#in = bufio::new_reader(Arc::new(Mutex::new(Some(Box::new((*os::Stdin().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Reader + Send + Sync>))));
    let mut w1: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut w2: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    { let mut __scan_src = GoScanRuneFns { read: Box::new(|| { let (__scan_r, _, __scan_e) = { let __recv = r#in.clone(); let __recv_ptr: *mut bufio::r#mod::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Reader }; let __result = unsafe { &mut *__recv_ptr }.read_rune(); __result }; if (*__scan_e.lock().unwrap()).is_none() { char::from_u32(__scan_r as u32) } else { None } }), unread: Box::new(|| { let _ = { let __recv = r#in.clone(); let __recv_ptr: *mut bufio::r#mod::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Reader }; let __result = unsafe { &mut *__recv_ptr }.unread_rune(); __result }; }) }; let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Str({ let __scan_h = w1.clone(); Box::new(move |__scan_v: String| { *__scan_h.lock().unwrap() = Some(__scan_v); }) }), GoScanTarget::Str({ let __scan_h = w2.clone(); Box::new(move |__scan_v: String| { *__scan_h.lock().unwrap() = Some(__scan_v); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    let mut x: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = GoScanRuneFns { read: Box::new(|| { let (__scan_r, _, __scan_e) = { let __recv = r#in.clone(); let __recv_ptr: *mut bufio::r#mod::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Reader }; let __result = unsafe { &mut *__recv_ptr }.read_rune(); __result }; if (*__scan_e.lock().unwrap()).is_none() { char::from_u32(__scan_r as u32) } else { None } }), unread: Box::new(|| { let _ = { let __recv = r#in.clone(); let __recv_ptr: *mut bufio::r#mod::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Reader }; let __result = unsafe { &mut *__recv_ptr }.unread_rune(); __result }; }) }; let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = x.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; k = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {} {} {}", format!("{}", { let __v = (*w1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*w2.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", k), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_stdin_source(); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = x.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; k = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", k), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}