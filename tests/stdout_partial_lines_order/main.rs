#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    go_panic_value_text(__payload)
}

fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    // Go prints a float in a panic as +1.500000e+000; a float32 is widened
    // to float64 first, as Go's print does.
    let __float = __payload
        .downcast_ref::<f64>()
        .copied()
        .or_else(|| __payload.downcast_ref::<f32>().map(|__value| *__value as f64));
    if let Some(__value) = __float {
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if __value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}

#[derive(Debug, Clone, PartialEq)]
enum GoScanError {
    Eof,
    UnexpectedEof,
    Msg(String),
}

// Where the scanner reads runes from: fmt's io.RuneScanner. Ok(None) is
// the end of the input; Err is a read error, which the scan returns.
trait GoScanRunes {
    fn go_read_rune(&mut self) -> Result<Option<char>, String>;
    fn go_unread_rune(&mut self);
}

// fmt's readRune: UTF-8 from a byte source that cannot unread, with one
// rune of lookahead that is lost when the scan ends.
struct GoScanReadRune<F: FnMut() -> Result<Option<u8>, String>> {
    next_byte: F,
    pending: std::vec::Vec<u8>,
    peek: Option<char>,
    last: Option<char>,
}

impl<F: FnMut() -> Result<Option<u8>, String>> GoScanReadRune<F> {
    fn new(next_byte: F) -> Self {
        GoScanReadRune { next_byte, pending: std::vec::Vec::new(), peek: None, last: None }
    }

    fn read_byte(&mut self) -> Result<Option<u8>, String> {
        if !self.pending.is_empty() {
            return Ok(Some(self.pending.remove(0)));
        }
        (self.next_byte)()
    }
}

// utf8.FullRune of a prefix: whether more bytes can still make it one rune.
fn go_scan_wants_more(bytes: &[u8]) -> bool {
    let first = bytes[0];
    let want = match first {
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => 1,
    };
    if bytes.len() >= want {
        return false;
    }
    // A byte that cannot continue the rune ends it now.
    let (lo, hi) = match (first, bytes.len()) {
        (0xE0, 1) => (0xA0, 0xBF),
        (0xED, 1) => (0x80, 0x9F),
        (0xF0, 1) => (0x90, 0xBF),
        (0xF4, 1) => (0x80, 0x8F),
        _ => (0x80, 0xBF),
    };
    bytes[1..].iter().enumerate().all(|(k, &b)| if k == 0 { b >= lo && b <= hi } else { (0x80..=0xBF).contains(&b) })
}

impl<F: FnMut() -> Result<Option<u8>, String>> GoScanRunes for GoScanReadRune<F> {
    fn go_read_rune(&mut self) -> Result<Option<char>, String> {
        if let Some(r) = self.peek.take() {
            self.last = Some(r);
            return Ok(Some(r));
        }
        let first = match self.read_byte()? {
            None => return Ok(None),
            Some(b) => b,
        };
        if first < 0x80 {
            let r = first as char;
            self.last = Some(r);
            return Ok(Some(r));
        }
        // As readRune: read until the bytes are a full rune, or cannot be.
        let mut bytes: std::vec::Vec<u8> = std::vec![first];
        while go_scan_wants_more(&bytes) {
            match self.read_byte()? {
                Some(b) => bytes.push(b),
                None => break,
            }
        }
        let r = match std::str::from_utf8(&bytes) {
            Ok(text) => text.chars().next().unwrap_or('\u{FFFD}'),
            Err(_) => {
                // Bad UTF-8 is one U+FFFD for its first byte; the bytes
                // after it are read again, before any pending ones.
                let mut rest = bytes[1..].to_vec();
                rest.extend_from_slice(&self.pending);
                self.pending = rest;
                '\u{FFFD}'
            }
        };
        self.last = Some(r);
        Ok(Some(r))
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
    read: Box<dyn FnMut() -> Result<Option<char>, String> + 'a>,
    unread: Box<dyn FnMut() + 'a>,
}

impl<'a> GoScanRunes for GoScanRuneFns<'a> {
    fn go_read_rune(&mut self) -> Result<Option<char>, String> {
        (self.read)()
    }

    fn go_unread_rune(&mut self) {
        (self.unread)()
    }
}

fn go_scan_string_source(text: String) -> GoScanReadRune<impl FnMut() -> Result<Option<u8>, String>> {
    let bytes = text.into_bytes();
    let mut at = 0usize;
    GoScanReadRune::new(move || {
        let b = bytes.get(at).copied();
        if b.is_some() {
            at += 1;
        }
        Ok(b)
    })
}

// The process's standard input, which os.Stdin reads too.
fn go_scan_stdin_source() -> GoScanReadRune<impl FnMut() -> Result<Option<u8>, String>> {
    GoScanReadRune::new(|| {
        use std::io::Read;
        let mut byte = [0u8; 1];
        loop {
            match std::io::stdin().lock().read(&mut byte) {
                Ok(0) => return Ok(None),
                Ok(_) => return Ok(Some(byte[0])),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(format!("read /dev/stdin: {}", e)),
            }
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
    // A read error ends the input; the scan returns it instead of the
    // operand it was reading, as fmt's getRune does.
    read_err: Option<String>,
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
            // strconv reports the range at the digit that overflows.
            None => return Ok(None),
        }
    }
    if !any && !(base0 && base == 8 && digits.is_empty()) {
        return Err(());
    }
    if underscores && !go_scan_underscore_ok(s) {
        return Err(());
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
            read_err: None,
        }
    }

    fn check_read(&mut self) -> Result<(), GoScanError> {
        match self.read_err.take() {
            Some(e) => Err(GoScanError::Msg(e)),
            None => Ok(()),
        }
    }

    fn read_rune(&mut self) -> Option<char> {
        if self.at_eof || self.count >= self.arg_limit {
            return None;
        }
        match self.rs.go_read_rune().unwrap_or_else(|e| {
            self.read_err = Some(e);
            None
        }) {
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
        if body.starts_with("0x") {
            return go_scan_parse_hex_float(&lower, bit_size).ok_or_else(syntax).and_then(|v| {
                if v.is_infinite() {
                    Err(GoScanError::Msg(format!("strconv.ParseFloat: parsing {}: value out of range", go_scan_quote(text))))
                } else {
                    Ok(v)
                }
            });
        }
        if !body.chars().any(|c| c.is_ascii_digit()) {
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
                // fmt's own decimal mantissa with a binary exponent, 1.5p3:
                // math.Ldexp of the two, with the exponent an int.
                let mantissa = Self::parse_float(&text[..p], bit_size).map_err(|e| match e {
                    GoScanError::Msg(m) => GoScanError::Msg(m.replacen(&go_scan_quote(&text[..p]), &go_scan_quote(text), 1)),
                    other => other,
                })?;
                let exp_text = &text[p + 1..];
                let exp: i64 = match exp_text.parse::<i64>() {
                    Ok(e) => e,
                    Err(_) if !exp_text.is_empty() && exp_text.trim_start_matches(|c| c == '+' || c == '-').chars().all(|c| c.is_ascii_digit()) && !exp_text.trim_start_matches(|c| c == '+' || c == '-').is_empty() => {
                        return Err(GoScanError::Msg(format!("strconv.Atoi: parsing {}: value out of range", go_scan_quote(text))));
                    }
                    Err(_) => return Err(GoScanError::Msg(format!("strconv.Atoi: parsing {}: invalid syntax", go_scan_quote(text)))),
                };
                return Ok(go_scan_ldexp(mantissa, exp));
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

    fn quoted_string(&mut self) -> Result<std::vec::Vec<u8>, GoScanError> {
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
                Ok(self.buf.clone().into_bytes())
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
            'q' => self.quoted_string(),
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
        // Each value is stored only if no read failed while it was read.
        match target {
            GoScanTarget::Bool(set) => {
                let v = self.scan_bool(verb);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Int(bits, set) => {
                let v = self.scan_int(verb, *bits);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Uint(bits, set) => {
                let v = self.scan_uint(verb, *bits);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Float(bits, set) => {
                let v = self.scan_float(verb, *bits);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Str(set) => {
                let v = self.convert_bytes(verb);
                self.check_read()?;
                match String::from_utf8(v?) {
                    Ok(text) => set(text),
                    // A Go string holds any bytes, a Rust String only UTF-8.
                    Err(_) => panic!("fmt scan: a string that is not UTF-8, as %x or %q can give, is not supported; see docs/bridge_debt.md fmt-scan"),
                }
            }
            GoScanTarget::Bytes(set) => {
                let v = self.convert_bytes(verb);
                self.check_read()?;
                set(v?);
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
        if let Err(e) = self.check_read() {
            return (n, Some(e));
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
                let w = self.advance(&format[i..]);
                self.check_read()?;
                let w = w?;
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
                        // parsenum's tooLarge: a width over a million is
                        // dropped, and the verb is the format's last byte.
                        if width > 1_000_000 {
                            width = 0;
                            wid_present = false;
                            i = end as usize;
                            break;
                        }
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

// strconv.Unquote of a double-quoted string, as bytes: \xNN and octal
// escapes are bytes, \u and \U escapes and other text are UTF-8.
fn go_scan_unquote(text: &str) -> Option<std::vec::Vec<u8>> {
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    let mut out: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut chars = inner.chars();
    let mut utf8 = [0u8; 4];
    while let Some(c) = chars.next() {
        if c == '"' || c == '\n' {
            return None;
        }
        if c != '\\' {
            out.extend_from_slice(c.encode_utf8(&mut utf8).as_bytes());
            continue;
        }
        let escape = chars.next()?;
        let simple = match escape {
            'n' => Some('\n'),
            't' => Some('\t'),
            'r' => Some('\r'),
            'a' => Some('\u{7}'),
            'b' => Some('\u{8}'),
            'f' => Some('\u{c}'),
            'v' => Some('\u{b}'),
            '\\' => Some('\\'),
            '"' => Some('"'),
            _ => None,
        };
        if let Some(c) = simple {
            out.push(c as u8);
            continue;
        }
        match escape {
            'x' => {
                let hex: String = (0..2).filter_map(|_| chars.next()).collect();
                if hex.len() != 2 {
                    return None;
                }
                out.push(u8::from_str_radix(&hex, 16).ok()?);
            }
            'u' | 'U' => {
                let len = if escape == 'u' { 4 } else { 8 };
                let hex: String = (0..len).filter_map(|_| chars.next()).collect();
                if hex.len() != len {
                    return None;
                }
                let value = u32::from_str_radix(&hex, 16).ok()?;
                out.extend_from_slice(char::from_u32(value)?.encode_utf8(&mut utf8).as_bytes());
            }
            '0'..='7' => {
                let mut value = escape.to_digit(8)?;
                for _ in 0..2 {
                    value = value * 8 + chars.next()?.to_digit(8)?;
                }
                if value > 255 {
                    return None;
                }
                out.push(value as u8);
            }
            _ => return None,
        }
    }
    Some(out)
}

// math.Ldexp: frac times two to the exp, without overflowing on the way.
fn go_scan_ldexp(frac: f64, exp: i64) -> f64 {
    if frac == 0.0 || !frac.is_finite() {
        return frac;
    }
    let mut value = frac;
    let mut exp = exp.clamp(-2200, 2200) as i32;
    while exp > 0 {
        let step = exp.min(1000);
        value *= 2f64.powi(step);
        exp -= step;
    }
    while exp < 0 {
        let step = exp.max(-1000);
        value *= 2f64.powi(step);
        exp -= step;
    }
    value
}

// strconv.ParseFloat of a hexadecimal float: 0x, hex digits with an
// optional point, and a binary exponent p.
fn go_scan_parse_hex_float(lower: &str, bit_size: u32) -> Option<f64> {
    let (negative, rest) = match lower.as_bytes().first() {
        Some(b'-') => (true, &lower[1..]),
        Some(b'+') => (false, &lower[1..]),
        _ => (false, lower),
    };
    let rest = rest.strip_prefix("0x")?;
    let p = rest.find('p')?;
    let (digits, exp_text) = (&rest[..p], &rest[p + 1..]);
    let digits: String = digits.chars().filter(|&c| c != '_').collect();
    let mut mantissa: u128 = 0;
    let mut scale: i64 = 0;
    let mut seen_point = false;
    let mut any = false;
    for c in digits.chars() {
        if c == '.' {
            if seen_point {
                return None;
            }
            seen_point = true;
            continue;
        }
        let d = c.to_digit(16)? as u128;
        any = true;
        if mantissa < (1u128 << 120) {
            mantissa = mantissa * 16 + d;
            if seen_point {
                scale -= 4;
            }
        } else if !seen_point {
            scale += 4;
        }
    }
    if !any {
        return None;
    }
    let exp: i64 = exp_text.parse().ok()?;
    let value = go_scan_ldexp(mantissa as f64, exp + scale);
    let value = if bit_size == 32 { value as f32 as f64 } else { value };
    Some(if negative { -value } else { value })
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

fn main() {
    go_install_panic_exit();
    ::bufio::__go_init_all();
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    print!("{}", format!("{}", "Name: ".to_string()));
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    { let mut __scan_src = go_scan_stdin_source(); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Str({ let __scan_h = name.clone(); Box::new(move |__scan_v: String| { *__scan_h.lock().unwrap() = Some(__scan_v); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'l', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("EOF".to_string()))))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unexpected EOF".to_string()))))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    print!("Hello, {}! Age? ", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v });
    let mut r#in = bufio::new_scanner(Arc::new(Mutex::new(Some({ let __arg_value = Box::new((*os::Stdin().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Reader + Send + Sync>; __arg_value }))));
    { let __recv = r#in.clone(); let __recv_ptr: *mut bufio::scan::Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::scan::Scanner }; let __result = unsafe { &mut *__recv_ptr }.scan(); __result };
    print!("{}{}", format!("{}", "got ".to_string()), format!("{}", (*{ let __recv = r#in.clone(); let __recv_ptr: *const bufio::scan::Scanner = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const bufio::scan::Scanner }; let __result = unsafe { &*__recv_ptr }.text(); __result }.lock().unwrap().as_ref().unwrap())));
    eprintln!("{}", format!("{}", " [stderr]".to_string()));
    print!("{}", format!("{}", "partial ".to_string()));
    eprint!("{}", format!("{}", "then stderr".to_string()));
    println!("{}", format!("{}", " done".to_string()));
}