#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use unicode_utf8::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

#[derive(Debug, Clone)]
pub struct Utf8Map {
    pub r: Rc<RefCell<Option<i32>>>,
    pub str: Rc<RefCell<Option<String>>>,
}

impl Utf8Map {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.str.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
            str: __go_clone_1_0,
        }
    }
}


impl Default for Utf8Map {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            r: __go_default_0_0,
            str: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Utf8Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.str.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct RuneLenTest {
    pub r: Rc<RefCell<Option<i32>>>,
    pub size: Rc<RefCell<Option<i64>>>,
}

impl RuneLenTest {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.size.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for RuneLenTest {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            r: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for RuneLenTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.size.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct ValidRuneTest {
    pub r: Rc<RefCell<Option<i32>>>,
    pub ok: Rc<RefCell<Option<bool>>>,
}

impl ValidRuneTest {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ok.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
            ok: __go_clone_1_0,
        }
    }
}


impl Default for ValidRuneTest {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            r: __go_default_0_0,
            ok: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for ValidRuneTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.ok.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static utf8map: GoGlobal<Vec<Utf8Map>> = GoGlobal::new();

pub(crate) static runelentests: GoGlobal<Vec<RuneLenTest>> = GoGlobal::new();

pub(crate) static validrunetests: GoGlobal<Vec<ValidRuneTest>> = GoGlobal::new();


fn __go_init_globals() {
    *utf8map.borrow_mut() = Some(vec![]);
    *runelentests.borrow_mut() = Some(vec![]);
    *validrunetests.borrow_mut() = Some(vec![]);
    {
        let mut __go_slice = Vec::<Utf8Map>::with_capacity(32);
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0000 as i32))), str: Rc::new(RefCell::new(Some("\u{0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0001 as i32))), str: Rc::new(RefCell::new(Some("\u{1}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x007e as i32))), str: Rc::new(RefCell::new(Some("~".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x007f as i32))), str: Rc::new(RefCell::new(Some("\u{7f}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0080 as i32))), str: Rc::new(RefCell::new(Some("\u{80}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0081 as i32))), str: Rc::new(RefCell::new(Some("\u{81}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00bf as i32))), str: Rc::new(RefCell::new(Some("\u{bf}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00c0 as i32))), str: Rc::new(RefCell::new(Some("\u{c0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00c1 as i32))), str: Rc::new(RefCell::new(Some("\u{c1}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00c8 as i32))), str: Rc::new(RefCell::new(Some("\u{c8}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00d0 as i32))), str: Rc::new(RefCell::new(Some("\u{d0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00e0 as i32))), str: Rc::new(RefCell::new(Some("\u{e0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00f0 as i32))), str: Rc::new(RefCell::new(Some("\u{f0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00f8 as i32))), str: Rc::new(RefCell::new(Some("\u{f8}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x00ff as i32))), str: Rc::new(RefCell::new(Some("\u{ff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0100 as i32))), str: Rc::new(RefCell::new(Some("\u{100}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x07ff as i32))), str: Rc::new(RefCell::new(Some("\u{7ff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0400 as i32))), str: Rc::new(RefCell::new(Some("\u{400}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0800 as i32))), str: Rc::new(RefCell::new(Some("\u{800}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x0801 as i32))), str: Rc::new(RefCell::new(Some("\u{801}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x1000 as i32))), str: Rc::new(RefCell::new(Some("\u{1000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0xd000 as i32))), str: Rc::new(RefCell::new(Some("\u{d000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0xd7ff as i32))), str: Rc::new(RefCell::new(Some("\u{d7ff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0xe000 as i32))), str: Rc::new(RefCell::new(Some("\u{e000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0xfffe as i32))), str: Rc::new(RefCell::new(Some("\u{fffe}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0xffff as i32))), str: Rc::new(RefCell::new(Some("\u{ffff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x10000 as i32))), str: Rc::new(RefCell::new(Some("\u{10000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x10001 as i32))), str: Rc::new(RefCell::new(Some("\u{10001}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x40000 as i32))), str: Rc::new(RefCell::new(Some("\u{40000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x10fffe as i32))), str: Rc::new(RefCell::new(Some("\u{10fffe}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0x10ffff as i32))), str: Rc::new(RefCell::new(Some("\u{10ffff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Rc::new(RefCell::new(Some(0xFFFD as i32))), str: Rc::new(RefCell::new(Some("\u{fffd}".to_string()))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *utf8map.borrow_mut() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<RuneLenTest>::with_capacity(10);
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(0 as i32))), size: Rc::new(RefCell::new(Some(1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(('e' as i32)))), size: Rc::new(RefCell::new(Some(1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(('\u{e9}' as i32)))), size: Rc::new(RefCell::new(Some(2))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(('\u{263a}' as i32)))), size: Rc::new(RefCell::new(Some(3))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(RUNE_ERROR as i32))), size: Rc::new(RefCell::new(Some(3))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(MAX_RUNE as i32))), size: Rc::new(RefCell::new(Some(4))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(0xD800 as i32))), size: Rc::new(RefCell::new(Some(-1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(0xDFFF as i32))), size: Rc::new(RefCell::new(Some(-1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(MAX_RUNE + 1 as i32))), size: Rc::new(RefCell::new(Some(-1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Rc::new(RefCell::new(Some(-1 as i32))), size: Rc::new(RefCell::new(Some(-1))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *runelentests.borrow_mut() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<ValidRuneTest>::with_capacity(12);
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(0 as i32))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(('e' as i32)))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(('\u{e9}' as i32)))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(('\u{263a}' as i32)))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(RUNE_ERROR as i32))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(MAX_RUNE as i32))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(0xD7FF as i32))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(0xD800 as i32))), ok: Rc::new(RefCell::new(Some(false))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(0xDFFF as i32))), ok: Rc::new(RefCell::new(Some(false))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(0xE000 as i32))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(MAX_RUNE + 1 as i32))), ok: Rc::new(RefCell::new(Some(false))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Rc::new(RefCell::new(Some(-1 as i32))), ok: Rc::new(RefCell::new(Some(false))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *validrunetests.borrow_mut() = Some(__go_slice);
    }
}


/// TestConstants is copied verbatim from unicode/utf8's own utf8_test.go.
/// Keeping the package test body unchanged makes this fixture an oracle for the
/// source-transpiled package rather than a hand-written approximation of it.
pub fn test_constants(t: Rc<RefCell<Option<oracle_testing::r#mod::T>>>) {
    if MAX_RUNE != unicode::MAX_RUNE {
        (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("utf8.MaxRune is wrong: %x should be %x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1114111i32) as Box<dyn Any>, Box::new(1114111i32) as Box<dyn Any>]))));
    }
    if RUNE_ERROR != unicode::REPLACEMENT_CHAR {
        (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("utf8.RuneError is wrong: %x should be %x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(65533i32) as Box<dyn Any>, Box::new(65533i32) as Box<dyn Any>]))));
    }
}

pub fn test_append_rune(t: Rc<RefCell<Option<oracle_testing::r#mod::T>>>) {
    { let __range_holder = utf8map.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        let m = &m.__go_value_clone();
        {
        let mut buf = append_rune(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
let mut buf = { let __slice_header = GoSliceHeader::go_slice_header(&buf); Rc::new(RefCell::new(__slice_header)) };;
        if (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*buf.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap()).clone() != { let __selector_holder = m.str.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } {
            (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("AppendRune(nil, %#04x) = %s, want %s".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>, Box::new(buf.clone()) as Box<dyn Any>, Box::new({ let __selector_holder = m.str.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>]))));;
        }
    }
        {
        let mut buf = append_rune(Rc::new(RefCell::new(Some(("init".to_string()).as_bytes().to_go_slice()))), Rc::new(RefCell::new(Some({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
let mut buf = { let __slice_header = GoSliceHeader::go_slice_header(&buf); Rc::new(RefCell::new(__slice_header)) };;
        if (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*buf.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap()).clone() != format!("{}{}", "init".to_string(), (*m.str.borrow().as_ref().unwrap()).clone()) {
            (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("AppendRune(init, %#04x) = %s, want %s".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>, Box::new(buf.clone()) as Box<dyn Any>, Box::new(format!("{}{}", "init".to_string(), (*m.str.borrow().as_ref().unwrap()).clone())) as Box<dyn Any>]))));;
        }
    }
    } }
}

pub fn test_rune_len(t: Rc<RefCell<Option<oracle_testing::r#mod::T>>>) {
    { let __range_holder = runelentests.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tt in __range_values.iter() {
        let tt = &tt.__go_value_clone();
        {
        let mut size = rune_len(Rc::new(RefCell::new(Some({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if size != (*tt.size.borrow().as_ref().unwrap()) {
            (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("RuneLen(%#U) = %d, want %d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>, Box::new(size) as Box<dyn Any>, Box::new({ let __selector_holder = tt.size.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>]))));;
        }
    }
    } }
}

pub fn test_valid_rune(t: Rc<RefCell<Option<oracle_testing::r#mod::T>>>) {
    { let __range_holder = validrunetests.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tt in __range_values.iter() {
        let tt = &tt.__go_value_clone();
        {
        let mut ok = valid_rune(Rc::new(RefCell::new(Some({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if ok != (*tt.ok.borrow().as_ref().unwrap()) {
            (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("ValidRune(%#U) = %t, want %t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>, Box::new(ok) as Box<dyn Any>, Box::new({ let __selector_holder = tt.ok.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any>]))));;
        }
    }
    } }
}

pub fn run(t: Rc<RefCell<Option<oracle_testing::r#mod::T>>>, name: Rc<RefCell<Option<String>>>, test: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()>>>>) -> bool {
    (*t.borrow_mut().as_mut().unwrap()).reset();
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()> = { let mut __f_guard = test.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t.clone()) };
    if (*t.borrow().as_ref().unwrap()).failed() {
        {
            let __go_print_arg_0 = format!("{}", format!("{}{}", "FAIL ".to_string(), (*name.borrow().as_ref().unwrap())));
            eprintln!("{}", __go_print_arg_0)
        };
        return false;
    }
    {
            let __go_print_arg_0 = format!("{}", format!("{}{}", "PASS ".to_string(), (*name.borrow().as_ref().unwrap())));
            eprintln!("{}", __go_print_arg_0)
        };
    true
}

fn main() {
    ::oracle_testing::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    let mut t = Rc::new(RefCell::new(Some(oracle_testing::r#mod::T { ..Default::default() })));
    let mut passed = Rc::new(RefCell::new(Some(0i64)));
    let mut failed = Rc::new(RefCell::new(Some(0i64)));
    for tt in &vec![AnonymousStruct1 { name: Rc::new(RefCell::new(Some("TestConstants".to_string()))), test: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<oracle_testing::r#mod::T>>>| { test_constants(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()>))), ..Default::default() }, AnonymousStruct1 { name: Rc::new(RefCell::new(Some("TestAppendRune".to_string()))), test: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<oracle_testing::r#mod::T>>>| { test_append_rune(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()>))), ..Default::default() }, AnonymousStruct1 { name: Rc::new(RefCell::new(Some("TestRuneLen".to_string()))), test: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<oracle_testing::r#mod::T>>>| { test_rune_len(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()>))), ..Default::default() }, AnonymousStruct1 { name: Rc::new(RefCell::new(Some("TestValidRune".to_string()))), test: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<oracle_testing::r#mod::T>>>| { test_valid_rune(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()>))), ..Default::default() }] {
        let tt = &tt.__go_value_clone();
        if run(t.clone(), Rc::new(RefCell::new(Some({ let __selector_holder = tt.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), tt.test.clone()) {
        { let mut guard = passed.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else {
        { let mut guard = failed.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    {
            let __go_print_arg_0 = format!("{}", "summary".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*passed.borrow().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "passed,".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*failed.borrow().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", "failed".to_string());
            eprintln!("{} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub name: Rc<RefCell<Option<String>>>,
    pub test: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<oracle_testing::r#mod::T>>>) -> ()>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.test.clone();
        Self {
            name: __go_clone_0_0,
            test: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            test: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Utf8Map {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RuneLenTest {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ValidRuneTest {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
