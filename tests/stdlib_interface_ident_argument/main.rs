use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Tuple;

impl std::fmt::Display for types_Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Tuple>")
    }
}


impl types_Tuple {
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


impl From<types_Tuple> for types_Type {
    fn from(_value: types_Tuple) -> Self {
        Self::__go_from(_value)
    }
}


pub mod types {
    use super::*;
    pub fn new_tuple<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Tuple>>> {
        Arc::new(Mutex::new(Some::<types_Tuple>(Default::default())))
    }
}


pub fn pass_tuple(t: Arc<Mutex<Option<types_Tuple>>>) -> Arc<Mutex<Option<bool>>> {

    return has({ let __arg = t.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<types_Type> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) });
}

pub fn has(t: Arc<Mutex<Option<types_Type>>>) -> Arc<Mutex<Option<bool>>> {

    return Arc::new(Mutex::new(Some(true)));
}

fn main() {
    let mut tuple = types::new_tuple(((),));
    println!("{}", format!("{}", (*pass_tuple(tuple.clone()).lock().unwrap().as_ref().unwrap())));
}