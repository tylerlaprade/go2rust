use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic {
    pub __go_kind: types_BasicKind,
    pub __go_info: types_BasicInfo,
    pub __go_name: String,
}

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_name)
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicInfo(pub i32);

impl PartialEq<i32> for types_BasicInfo {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicInfo> for i32 {
    fn eq(&self, other: &types_BasicInfo) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicInfo {
    type Output = types_BasicInfo;
    fn bitand(self, other: Self) -> types_BasicInfo {
        types_BasicInfo(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicInfo {
    type Output = types_BasicInfo;
    fn bitor(self, other: Self) -> types_BasicInfo {
        types_BasicInfo(self.0 | other.0)
    }
}

impl std::fmt::Display for types_BasicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicInfo>")
    }
}


impl types_BasicInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicKind(pub i32);

impl PartialEq<i32> for types_BasicKind {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicKind> for i32 {
    fn eq(&self, other: &types_BasicKind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicKind {
    type Output = types_BasicKind;
    fn bitand(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicKind {
    type Output = types_BasicKind;
    fn bitor(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 | other.0)
    }
}

impl std::fmt::Display for types_BasicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicKind>")
    }
}


impl types_BasicKind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Pointer;

impl std::fmt::Display for types_Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Pointer>")
    }
}


impl types_Pointer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl types_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


impl From<types_Pointer> for types_Type {
    fn from(_value: types_Pointer) -> Self {
        Self::__go_from(_value)
    }
}


pub mod types {
    use super::*;
    pub fn Typ() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<types_Basic>>>>>>> {
        Arc::new(Mutex::new(Some::<Vec<Arc<Mutex<Option<types_Basic>>>>>(Default::default())))
    }

    pub fn new_pointer<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Pointer>>> {
        panic!("new_pointer bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub fn make_type() -> Arc<Mutex<Option<types_Type>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<types_Type>>> = Arc::new(Mutex::new(Some(Default::default())));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        { let new_val = { let __arg = types::new_pointer({ let __seq = { let __seq_holder = types::Typ().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[2usize].clone() }); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<types_Type> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
}

fn main() {
    if false {
        println!("{}", format!("{}", (*make_type().lock().unwrap()).is_some()));
    }
    println!("{}", format!("{}", "ok".to_string()));
}