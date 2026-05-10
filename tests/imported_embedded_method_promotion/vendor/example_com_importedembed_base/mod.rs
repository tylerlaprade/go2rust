use go2rust_stdlib_stubs::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct Decoder {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct PkgDecoder {
    pub base: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for PkgDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.base.lock().unwrap().as_ref().unwrap()))
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
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()
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
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()
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
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*n.lock().unwrap().as_ref().unwrap())); };
    }

    pub fn label(&self, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}:{}", { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.value.lock().unwrap().as_ref().unwrap())))));
    }

    pub fn snapshot(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }

    pub fn clone(&self) -> Arc<Mutex<Option<Decoder>>> {
        return Arc::new(Mutex::new(Some(Decoder { value: self.value.clone(), ..Default::default() })));
    }
}

impl PkgDecoder {
    pub fn new_decoder(&self, delta: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Decoder>>> {
        return Arc::new(Mutex::new(Some(Decoder { value: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.base.clone().lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), ..Default::default() })));
    }

    pub fn retire_decoder(&self, d: Arc<Mutex<Option<Decoder>>>) {
        let _ = self;
        let _ = (*d.lock().unwrap().as_ref().unwrap());
    }
}