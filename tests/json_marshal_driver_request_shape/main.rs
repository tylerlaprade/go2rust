use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

fn go_base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }

        i += 3;
    }
    out
}

fn go_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn go_base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 != 0 {
        return Err(format!("illegal base64 data at input byte {}", bytes.len()));
    }

    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c0 = go_base64_value(bytes[i])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i))?;
        let c1 = go_base64_value(bytes[i + 1])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 1))?;
        let pad2 = bytes[i + 2] == b'=';
        let pad3 = bytes[i + 3] == b'=';
        let c2 = if pad2 {
            0
        } else {
            go_base64_value(bytes[i + 2])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 2))?
        };
        let c3 = if pad3 {
            0
        } else {
            go_base64_value(bytes[i + 3])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 3))?
        };

        out.push((c0 << 2) | (c1 >> 4));
        if !pad2 {
            out.push((c1 << 4) | (c2 >> 2));
        }
        if !pad3 {
            out.push((c2 << 6) | c3);
        }

        i += 4;
    }
    Ok(out)
}

fn go_json_escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c < ' ' => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}

#[derive(Debug, Clone, Default)]
pub struct Mode(pub Rc<RefCell<Option<i32>>>);

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Mode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Mode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Mode> for i32 {
    fn eq(&self, other: &Mode) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Mode> for i32 {
    fn partial_cmp(&self, other: &Mode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Mode {
    type Output = Mode;
    fn add(self, other: Self) -> Mode {
        Mode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Mode {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Mode> for i32 {
    type Output = i32;
    fn add(self, other: Mode) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Mode {
    type Output = Mode;
    fn sub(self, other: Self) -> Mode {
        Mode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Mode {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Mode> for i32 {
    type Output = i32;
    fn sub(self, other: Mode) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Mode {
    type Output = Mode;
    fn bitand(self, other: Self) -> Mode {
        Mode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Mode {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Mode> for i32 {
    type Output = i32;
    fn bitand(self, other: Mode) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Mode {
    type Output = Mode;
    fn bitor(self, other: Self) -> Mode {
        Mode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Mode {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Mode> for i32 {
    type Output = i32;
    fn bitor(self, other: Mode) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Mode {
    type Output = Mode;
    fn bitxor(self, other: Self) -> Mode {
        Mode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Mode {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Mode> for i32 {
    type Output = i32;
    fn bitxor(self, other: Mode) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for Mode {}

impl Ord for Mode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Request {
    // tags: `json:"mode"`
    pub mode: Rc<RefCell<Option<Mode>>>,
    // tags: `json:"env"`
    pub env: Rc<RefCell<Option<Vec<String>>>>,
    // tags: `json:"build_flags"`
    pub build_flags: Rc<RefCell<Option<Vec<String>>>>,
    // tags: `json:"tests"`
    pub tests: Rc<RefCell<Option<bool>>>,
    // tags: `json:"overlay"`
    pub overlay: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<u8>>>>>>>>,
}

impl Request {
    pub fn __go_value_clone(&self) -> Self {
        Self { mode: { let __guard = self.mode.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, env: self.env.clone(), build_flags: self.build_flags.clone(), tests: { let __guard = self.tests.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, overlay: self.overlay.clone() }
    }
}


impl Default for Request {
    fn default() -> Self {
        Self { mode: Rc::new(RefCell::new(Some(Mode(Rc::new(RefCell::new(Some(0))))))), env: Rc::new(RefCell::new(None)), build_flags: Rc::new(RefCell::new(None)), tests: Rc::new(RefCell::new(Some(false))), overlay: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.mode.borrow().as_ref().unwrap()), format_slice(&self.env), format_slice(&self.build_flags), (*self.tests.borrow().as_ref().unwrap()), "<map>")
    }
}


fn main() {
    let mut overlay = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<u8>>>>>::new())));
    { let __map_key = "b.go".to_string(); let __map_value = Rc::new(RefCell::new(Some(vec![0, 1, 255]))); (*overlay.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "a.go".to_string(); let __map_value = Rc::new(RefCell::new(Some(("tmp-a".to_string()).as_bytes().to_vec()))); (*overlay.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut req = Rc::new(RefCell::new(Some(Request { mode: Rc::new(RefCell::new(Some(Mode(Rc::new(RefCell::new(Some(3 as i32))))))), env: Rc::new(RefCell::new(Some(vec!["B=2".to_string(), "A=1".to_string()]))), build_flags: Rc::new(RefCell::new(Some(vec!["-tags".to_string(), "dev".to_string()]))), tests: Rc::new(RefCell::new(Some(true))), overlay: overlay.clone(), ..Default::default() })));

    let (mut data, _) = { let __json_value = (*req.borrow().as_ref().unwrap()).clone(); let mut __json_fields: Vec<String> = Vec::new(); __json_fields.push(format!("\"mode\":{}", *__json_value.mode.borrow().as_ref().unwrap().0.borrow().as_ref().unwrap())); { let __slice_guard = __json_value.env.borrow(); if let Some(__slice) = __slice_guard.as_ref() { let __slice_entries = __slice.iter().map(|__v| format!("\"{}\"", go_json_escape(__v))).collect::<Vec<_>>().join(","); __json_fields.push(format!("\"env\":[{}]", __slice_entries)); } else { __json_fields.push("\"env\":null".to_string()); } } { let __slice_guard = __json_value.build_flags.borrow(); if let Some(__slice) = __slice_guard.as_ref() { let __slice_entries = __slice.iter().map(|__v| format!("\"{}\"", go_json_escape(__v))).collect::<Vec<_>>().join(","); __json_fields.push(format!("\"build_flags\":[{}]", __slice_entries)); } else { __json_fields.push("\"build_flags\":null".to_string()); } } __json_fields.push(format!("\"tests\":{}", *__json_value.tests.borrow().as_ref().unwrap())); { let __map_guard = __json_value.overlay.borrow(); if let Some(__map) = __map_guard.as_ref() { let __map_entries = __map.iter().map(|(__k, __v)| { let __v_guard = __v.borrow(); if let Some(__bytes) = __v_guard.as_ref() { format!("\"{}\":\"{}\"", go_json_escape(__k), go_base64_encode(__bytes)) } else { format!("\"{}\":null", go_json_escape(__k)) } }).collect::<Vec<_>>().join(","); __json_fields.push(format!("\"overlay\":{{{}}}", __map_entries)); } else { __json_fields.push("\"overlay\":null".to_string()); } } let __json = format!("{{{}}}", __json_fields.join(",")); (Rc::new(RefCell::new(Some(__json.into_bytes()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))) };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*data.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}