#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
pub trait incer: std::fmt::Display + Any {
    fn __go_clone_box_incer(&self) -> Box<dyn incer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_incer(&self, other: &(dyn incer + Send + Sync)) -> bool;
    fn inc(&mut self);
}

impl Clone for Box<dyn incer + Send + Sync> {
    fn clone(&self) -> Self {
        incer::__go_clone_box_incer(self.as_ref())
    }
}

pub trait namer: std::fmt::Display + Any {
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn namer + Send + Sync> {
    fn clone(&self) -> Self {
        namer::__go_clone_box_namer(self.as_ref())
    }
}

pub trait speaker: std::fmt::Display + Any {
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
    fn sound(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn speaker + Send + Sync> {
    fn clone(&self) -> Self {
        speaker::__go_clone_box_speaker(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct animal {
    pub name: Arc<Mutex<Option<String>>>,
}

impl animal {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for animal {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for animal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for animal {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct dog {
    pub animal: Arc<Mutex<Option<animal>>>,
    pub breed: Arc<Mutex<Option<String>>>,
}

impl dog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.animal.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.breed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            breed: __go_clone_1_0,
        }
    }
}


impl Default for dog {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(animal::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            animal: __go_default_0_0,
            breed: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.animal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.breed.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct cat {
    pub animal: Arc<Mutex<Option<animal>>>,
    pub lives: Arc<Mutex<Option<i64>>>,
}

impl cat {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.animal.clone();
        let __go_clone_1_0 = { let __guard = self.lives.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            lives: __go_clone_1_0,
        }
    }
}


impl Default for cat {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            animal: __go_default_0_0,
            lives: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.animal.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.lives.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct counter {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for counter {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.n.lock().unwrap().as_ref().cloned() }; let __right = { other.n.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


impl animal {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }
}

impl namer for animal {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        animal::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<animal>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct animalPtr(pub Arc<Mutex<Option<animal>>>);

impl std::fmt::Display for animalPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for animalPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        animal::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<animalPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl dog {
    pub fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("woof".to_string())))
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl namer for dog {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        dog::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dog>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct dogPtr(pub Arc<Mutex<Option<dog>>>);

impl std::fmt::Display for dogPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for dogPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dogPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl speaker for dog {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        dog::name(self)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        dog::sound(self)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dog>() {
            false
        } else {
            false
        }
    }
}

impl speaker for dogPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::name(__recv)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::sound(__recv)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dogPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cat {
    pub fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("meow".to_string())))
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl namer for cat {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        cat::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<cat>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct catPtr(pub Arc<Mutex<Option<cat>>>);

impl std::fmt::Display for catPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for catPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_namer(&self, other: &(dyn namer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<catPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl speaker for cat {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        cat::name(self)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        cat::sound(self)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<cat>() {
            false
        } else {
            false
        }
    }
}

impl speaker for catPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::name(__recv)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::sound(__recv)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn speaker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_speaker(&self, other: &(dyn speaker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<catPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl counter {
    pub fn inc(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl incer for counter {
    fn inc(&mut self) {
        counter::inc(self)
    }
    fn __go_clone_box_incer(&self) -> Box<dyn incer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn incer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_incer(&self, other: &(dyn incer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counter>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct counterPtr(pub Arc<Mutex<Option<counter>>>);

impl std::fmt::Display for counterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl incer for counterPtr {
    fn inc(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        counter::inc(__recv)
    }
    fn __go_clone_box_incer(&self) -> Box<dyn incer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn incer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_incer(&self, other: &(dyn incer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    started.recv().unwrap_or_default();
    
    let mut all: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn speaker + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = all.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(dog { animal: Arc::new(Mutex::new(Some(animal { name: Arc::new(Mutex::new(Some("rex".to_string()))), ..Default::default() }))), breed: Arc::new(Mutex::new(Some("lab".to_string()))), ..Default::default() }) as Box<dyn speaker + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn speaker + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *all.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = all.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(catPtr({
        let __owner = Arc::new(Mutex::new(Some(cat {
            animal: Arc::new(Mutex::new(Some(animal { name: Arc::new(Mutex::new(Some("tom".to_string()))), ..Default::default() }))).clone(),
            lives: Arc::new(Mutex::new(Some(9))),
            ..Default::default()
        })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().animal.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    }.clone())) as Box<dyn speaker + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn speaker + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *all.lock().unwrap() = __slice_header; };
    { let __range_holder = all.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        println!("{} {} {}", format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", "says".to_string()), format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).sound().lock().unwrap().as_ref().unwrap())));
    } }
    let mut names = Arc::new(Mutex::new(Some(Vec::<String>::new())));
    { let __range_holder = all.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn speaker + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<dog>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<dog>()).unwrap().clone())));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = format!("{}{}", "dog:".to_string(), (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).breed.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<catPtr>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<catPtr>()).unwrap().0.clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = (*Arc::new(Mutex::new(Some(format!("{}{}", format!("{}", "cat:".to_string()), format!("{}", (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).lives.clone(); __field }.lock().unwrap().as_ref().unwrap())))))).lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };;
    }
    }
    } }
    sort::strings(names.clone());
    println!("{}", format!("{}", (*strings::join(names.clone(), Arc::new(Mutex::new(Some(",".to_string())))).lock().unwrap().as_ref().unwrap())));
    
    let mut c = Arc::new(Mutex::new(Some(counter { n: Arc::new(Mutex::new(Some(0))) })));
    let mut i = Arc::new(Mutex::new(Some(Box::new(counterPtr(c.clone())) as Box<dyn incer + Send + Sync>)));
    (*i.lock().unwrap().as_mut().unwrap()).inc();
    (*i.lock().unwrap().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", "count".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    
    let mut n = Arc::new(Mutex::new(Some(Box::new(dog { animal: Arc::new(Mutex::new(Some(animal { name: Arc::new(Mutex::new(Some("fido".to_string()))), ..Default::default() }))), breed: Arc::new(Mutex::new(Some("pug".to_string()))), ..Default::default() }) as Box<dyn namer + Send + Sync>)));
    {
        let (mut sp, mut ok) = ({
        let val = n.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn namer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<catPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn speaker + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn namer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<dog>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn speaker + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn speaker + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn speaker + Send + Sync>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "also a speaker:".to_string()), format!("{}", (*(*sp.lock().unwrap().as_ref().unwrap()).sound().lock().unwrap().as_ref().unwrap())));;
        }
    }
    print!("{}\n", (*(*n.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
}

impl GoValueClone for animal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for dog {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cat {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for counter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
