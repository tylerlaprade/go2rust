use go2rust_stdlib_stubs::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Decoder {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl Decoder {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Decoder {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Decoder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Value") {
            out.value = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct PkgDecoder {
    pub base: Arc<Mutex<Option<i32>>>,
}

impl PkgDecoder {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for PkgDecoder {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for PkgDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.base.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for PkgDecoder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Base") {
            out.base = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Index(pub Arc<Mutex<Option<i32>>>);

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Index {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Index {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Index> for i32 {
    fn eq(&self, other: &Index) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Index> for i32 {
    fn partial_cmp(&self, other: &Index) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Index {
    type Output = Index;
    fn add(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Index {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Index> for i32 {
    type Output = i32;
    fn add(self, other: Index) -> i32 {
        self + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub for Index {
    type Output = Index;
    fn sub(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Index {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Index> for i32 {
    type Output = i32;
    fn sub(self, other: Index) -> i32 {
        self - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Index {
    type Output = Index;
    fn bitand(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Index {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Index> for i32 {
    type Output = i32;
    fn bitand(self, other: Index) -> i32 {
        self & *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Index {
    type Output = Index;
    fn bitor(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Index {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Index> for i32 {
    type Output = i32;
    fn bitor(self, other: Index) -> i32 {
        self | *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Index {
    type Output = Index;
    fn bitxor(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Index {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Index> for i32 {
    type Output = i32;
    fn bitxor(self, other: Index) -> i32 {
        self ^ *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl Eq for Index {}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl Decoder {
    pub fn add(&mut self, n: Arc<Mutex<Option<i32>>>) {
        { let __target = self.value.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    pub fn label(&self, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}:{}", { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.value.lock().unwrap().as_ref().unwrap())))));
    }

    pub fn snapshot(&self) -> i32 {
        return (*self.value.lock().unwrap().as_ref().unwrap());
    }

    pub fn clone(&self) -> Arc<Mutex<Option<Decoder>>> {
        return Arc::new(Mutex::new(Some(Decoder { value: Arc::new(Mutex::new(Some({ let __selector_holder = self.value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
    }
}

impl PkgDecoder {
    pub fn new_decoder(&self, delta: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Decoder>>> {
        return Arc::new(Mutex::new(Some(Decoder { value: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), ..Default::default() })));
    }

    pub fn retire_decoder(&self, d: Arc<Mutex<Option<Decoder>>>) {
        let _ = self;
        let _ = (*d.lock().unwrap().as_ref().unwrap());
    }
}