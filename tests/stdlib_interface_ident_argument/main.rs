use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



pub trait GoTypesTupleArgs {
    fn __go_tuple_len(self) -> usize;
}

impl GoTypesTupleArgs for () {
    fn __go_tuple_len(self) -> usize { 0 }
}

impl<T0> GoTypesTupleArgs for (T0, ) {
    fn __go_tuple_len(self) -> usize { 1 }
}

impl<T0, T1> GoTypesTupleArgs for (T0, T1, ) {
    fn __go_tuple_len(self) -> usize { 2 }
}

impl<T0, T1, T2> GoTypesTupleArgs for (T0, T1, T2, ) {
    fn __go_tuple_len(self) -> usize { 3 }
}

impl<T0, T1, T2, T3> GoTypesTupleArgs for (T0, T1, T2, T3, ) {
    fn __go_tuple_len(self) -> usize { 4 }
}

impl<T0, T1, T2, T3, T4> GoTypesTupleArgs for (T0, T1, T2, T3, T4, ) {
    fn __go_tuple_len(self) -> usize { 5 }
}

impl<T0, T1, T2, T3, T4, T5> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, ) {
    fn __go_tuple_len(self) -> usize { 6 }
}

impl<T0, T1, T2, T3, T4, T5, T6> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, T6, ) {
    fn __go_tuple_len(self) -> usize { 7 }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, T6, T7, ) {
    fn __go_tuple_len(self) -> usize { 8 }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, T6, T7, T8, ) {
    fn __go_tuple_len(self) -> usize { 9 }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, ) {
    fn __go_tuple_len(self) -> usize { 10 }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, ) {
    fn __go_tuple_len(self) -> usize { 11 }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> GoTypesTupleArgs for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, ) {
    fn __go_tuple_len(self) -> usize { 12 }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Tuple {
    pub __go_len: usize,
}

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
    pub fn new_tuple<T0: GoTypesTupleArgs>(_arg0: T0) -> Arc<Mutex<Option<types_Tuple>>> {
        Arc::new(Mutex::new(Some::<types_Tuple>(types_Tuple { __go_len: _arg0.__go_tuple_len() })))
    }
}


pub fn pass_tuple(t: Arc<Mutex<Option<types_Tuple>>>) -> bool {
    has({ let __arg = t.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<types_Type> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) })
}

pub fn has(t: Arc<Mutex<Option<types_Type>>>) -> bool {
    true
}

fn main() {
    let mut tuple = types::new_tuple(((),));
    println!("{}", format!("{}", pass_tuple(tuple.clone())));
}