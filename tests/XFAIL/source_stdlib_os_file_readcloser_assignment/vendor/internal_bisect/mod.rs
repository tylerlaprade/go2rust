use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
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
fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_values(inner.as_ref()))
            .collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Clone)]
pub struct GoSliceElemPtr<T: Clone> {
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

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    pub fn slice_handle(&self) -> Arc<Mutex<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }

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

pub trait GoArrayElemPtrDyn<T: Clone + Send + Sync + 'static>: Send + Sync {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn identity_dyn(&self) -> (*const (), usize);
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

#[derive(Clone)]
pub enum GoPtr<T: Clone + Send + Sync + 'static> {
    Nil,
    Raw(usize),
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Arc<dyn GoArrayElemPtrDyn<T> + Send + Sync>),
}

impl<T: Clone + Send + Sync + 'static> GoPtr<T> {
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

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self {
        GoPtr::ArrayElem(Arc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self {
        match value {
            Some(value) => GoPtr::ArrayElem(Arc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.lock().unwrap().is_none(),
            GoPtr::SliceElem(value) => value.borrow().is_none(),
            GoPtr::ArrayElem(value) => value.borrow_dyn().is_none(),
        }
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

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.lock().unwrap();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.borrow_mut();
                f(guard.as_mut().unwrap())
            }
            GoPtr::ArrayElem(slot) => {
                let mut value = slot.borrow_dyn().expect("nil pointer dereference");
                let result = f(&mut value);
                slot.assign_dyn(Some(value));
                result
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Arc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                Arc::ptr_eq(&left.slice_handle(), &right.slice_handle()) && left.index() == right.index()
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
            GoPtr::SliceElem(value) => (Arc::as_ptr(&value.slice_handle()) as usize).wrapping_add(value.index()),
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Clone + Send + Sync + 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

pub(crate) const OFFSET64: u64 = 14695981039346656037;
pub(crate) const PRIME64: u64 = 1099511628211;


/// A Matcher is the parsed, compiled form of a PATTERN string.
/// The nil *Matcher is valid: it has all changes enabled but none reported.
#[derive(Clone)]
pub struct Matcher {
    pub verbose: Arc<Mutex<Option<bool>>>,
    pub quiet: Arc<Mutex<Option<bool>>>,
    pub enable: Arc<Mutex<Option<bool>>>,
    pub list: Arc<Mutex<Option<Vec<cond>>>>,
    pub dedup: Arc<Mutex<Option<sync_atomic::r#type::Pointer<dedup>>>>,
}

impl Matcher {
    pub fn __go_value_clone(&self) -> Self {
        Self { verbose: { let __guard = self.verbose.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, quiet: { let __guard = self.quiet.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enable: { let __guard = self.enable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: self.list.clone(), dedup: { let __guard = self.dedup.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Matcher {
    fn default() -> Self {
        Self { verbose: Arc::new(Mutex::new(Some(false))), quiet: Arc::new(Mutex::new(Some(false))), enable: Arc::new(Mutex::new(Some(false))), list: Arc::new(Mutex::new(None)), dedup: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for Matcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.verbose.lock().unwrap().as_ref().unwrap()), (*self.quiet.lock().unwrap().as_ref().unwrap()), (*self.enable.lock().unwrap().as_ref().unwrap()), format_slice(&self.list), (*self.dedup.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Matcher {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A cond is a single condition in the matcher.
/// Given an input id, if id&mask == bits, return the result.
#[derive(Debug, Clone)]
pub struct cond {
    pub mask: Arc<Mutex<Option<u64>>>,
    pub bits: Arc<Mutex<Option<u64>>>,
    pub result: Arc<Mutex<Option<bool>>>,
}

impl cond {
    pub fn __go_value_clone(&self) -> Self {
        Self { mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bits: { let __guard = self.bits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, result: { let __guard = self.result.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for cond {
    fn default() -> Self {
        Self { mask: Arc::new(Mutex::new(Some(0))), bits: Arc::new(Mutex::new(Some(0))), result: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for cond {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.mask.lock().unwrap().as_ref().unwrap()), (*self.bits.lock().unwrap().as_ref().unwrap()), (*self.result.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for cond {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Writer is the same interface as io.Writer.
/// It is duplicated here to avoid importing io.
pub trait Writer: std::fmt::Display + Any {
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool;
    fn write(&mut self, __arg0: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Writer + Send + Sync> {
    fn clone(&self) -> Self {
        Writer::__go_clone_box_writer(self.as_ref())
    }
}

/// parseError is a trivial error implementation,
/// defined here to avoid importing errors.
#[derive(Debug, Clone)]
pub struct parseError {
    pub text: Arc<Mutex<Option<String>>>,
}

impl parseError {
    pub fn __go_value_clone(&self) -> Self {
        Self { text: { let __guard = self.text.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for parseError {
    fn default() -> Self {
        Self { text: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for parseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for parseError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A dedup is a deduplicator for call stacks, so that we only print
/// a report for new call stacks, not for call stacks we've already
/// reported.
///
/// It has two modes: an approximate but lock-free mode that
/// may still emit some duplicates, and a precise mode that uses
/// a lock and never emits duplicates.
#[derive(Clone)]
pub struct dedup {
    pub recent: Arc<Mutex<Option<[[u64; 4]; 128]>>>,
    pub mu: sync::mutex::Mutex,
    pub m: Arc<Mutex<Option<BTreeMap<u64, Arc<Mutex<Option<bool>>>>>>>,
}

impl dedup {
    pub fn __go_value_clone(&self) -> Self {
        Self { recent: { let __guard = self.recent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mu: self.mu.clone(), m: self.m.clone() }
    }
}


impl Default for dedup {
    fn default() -> Self {
        Self { recent: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| 0))))), mu: Default::default(), m: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for dedup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_nested_slice(&self.recent), format_map(&self.m))
    }
}

impl GoJsonDecode for dedup {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Matcher {
    /// MarkerOnly reports whether it is okay to print only the marker for
    /// a given change, omitting the identifying information.
    /// MarkerOnly returns true when bisect is using the printed reports
    /// only for an intermediate search step, not for showing to users.
    pub fn marker_only(&self) -> bool {
        !((*self.verbose.clone().lock().unwrap().as_ref().unwrap()))
    }

    /// ShouldEnable reports whether the change with the given id should be enabled.
    pub fn should_enable(&self, id: Arc<Mutex<Option<u64>>>) -> bool {
        if false {
        return true;
    }
        return { let __tmp_x = self.match_result(Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = (*self.enable.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
    }

    /// ShouldPrint reports whether to print identifying information about the change with the given id.
    pub fn should_print(&self, id: Arc<Mutex<Option<u64>>>) -> bool {
        if false || (*self.quiet.clone().lock().unwrap().as_ref().unwrap()) {
        return false;
    }
        self.match_result(Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// matchResult returns the result from the first condition that matches id.
    pub fn match_result(&self, id: Arc<Mutex<Option<u64>>>) -> bool {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let mut c: Option<GoSliceElemPtr<cond>> = Some(GoSliceElemPtr::new(self.list.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        if { let __tmp_x = { let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*c.as_ref().unwrap().borrow().as_ref().unwrap()).mask.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = (*{ let __field = (*c.as_ref().unwrap().borrow().as_ref().unwrap()).bits.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return (*(*c.as_ref().unwrap().borrow().as_ref().unwrap()).result.lock().unwrap().as_ref().unwrap());
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        false
    }

    /// FileLine reports whether the change identified by file and line should be enabled.
    /// If the change should be printed, FileLine prints a one-line report to w.
    pub fn file_line(&self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, file: Arc<Mutex<Option<String>>>, line: Arc<Mutex<Option<i32>>>) -> bool {
        if false {
        return true;
    }
        self.file_line_1(w.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// fileLine does the real work for FileLine.
    /// This lets FileLine's body handle m == nil and potentially be inlined.
    pub fn file_line_1(&self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, file: Arc<Mutex<Option<String>>>, line: Arc<Mutex<Option<i32>>>) -> bool {
        let mut h = hash(Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        if self.should_print(Arc::new(Mutex::new(Some(h)))) {
        if self.marker_only() {
        print_marker(w.clone(), Arc::new(Mutex::new(Some(h))));
    } else {
        print_file_line(w.clone(), Arc::new(Mutex::new(Some(h))), Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
        self.should_enable(Arc::new(Mutex::new(Some(h))))
    }

    /// MatchStack assigns the current call stack a change ID.
    /// If the stack should be printed, MatchStack prints it.
    /// Then MatchStack reports whether a change at the current call stack should be enabled.
    pub fn stack(&self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>) -> bool {
        if false {
        return true;
    }
        self.stack_1(w.clone())
    }

    /// stack does the real work for Stack.
    /// This lets stack's body handle m == nil and potentially be inlined.
    pub fn stack_1(&self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>) -> bool {
        const maxStack: i32 = 16;

        let mut stk: Arc<Mutex<Option<[usize; 16]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let mut n = runtime::callers(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some({ let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
                // caller #2 is not for printing; need it to normalize PCs if ASLR.
        if { let __tmp_x = n; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return false;
    }
        let mut base = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
                // normalize PCs
        for i in 0..({ let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (n) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.len()) {
        { let __idx = i as usize; let __rhs = (*base.lock().unwrap().as_ref().unwrap()); let mut __seq_guard = stk.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] - __rhs; };
    }
        let mut h = hash(Arc::new(Mutex::new(Some(vec![Box::new(Arc::new(Mutex::new(Some({ let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (n) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone()) as Box<dyn Any + Send + Sync>]))));
        if self.should_print(Arc::new(Mutex::new(Some(h)))) {
        let mut d: GoPtr<dedup> = GoPtr::nil();
        loop {
        d = { let __go_ptr = (*self.dedup.lock().unwrap().as_mut().unwrap()).load().clone(); match __go_ptr { sync_atomic::GoPtr::Nil => GoPtr::nil(), sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()), sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr), sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if !d.is_nil() {
        break
    }
        d = GoPtr::local(Arc::new(Mutex::new(Some(dedup::default()))));
        if (*self.dedup.lock().unwrap().as_mut().unwrap()).compare_and_swap(sync_atomic::GoPtr::nil(), { let __go_ptr = d.clone(); match __go_ptr { GoPtr::Nil => sync_atomic::GoPtr::nil(), GoPtr::Local(__value) => sync_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => sync_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => sync_atomic::GoPtr::slice_elem(sync_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }) {
        break
    }
    }
        if self.marker_only() {
        if !{ let __recv_value = d.borrow(); let __result = (*__recv_value.as_ref().unwrap()).seen_lossy(Arc::new(Mutex::new(Some(h)))); __result } {
        print_marker(w.clone(), Arc::new(Mutex::new(Some(h))));
    }
    } else {
        if !{ let __result = d.with_mut(|__recv_value| __recv_value.seen(Arc::new(Mutex::new(Some(h))))); __result } {
                // Restore PCs in stack for printing
        for i in 0..({ let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (n) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.len()) {
        { let __idx = i as usize; let __rhs = (*base.lock().unwrap().as_ref().unwrap()); let mut __seq_guard = stk.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    }
        print_stack(w.clone(), Arc::new(Mutex::new(Some(h))), Arc::new(Mutex::new(Some({ let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (1) as usize; let __high = (n) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
    }
    }
    }
                // Restore PCs in stack for printing
        self.should_enable(Arc::new(Mutex::new(Some(h))))
    }
}

impl parseError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return self.text.clone();
    }
}

impl StdError for parseError {}


impl dedup {
    /// seen records that h has now been seen and reports whether it was seen before.
    /// When seen returns false, the caller is expected to print a report for h.
    pub fn seen(&mut self, h: Arc<Mutex<Option<u64>>>) -> bool {
        self.mu.lock();
        if { let __nil_target = self.m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<u64, Arc<Mutex<Option<bool>>>>::new()))); self.m = new_val; };
    }
        let mut seen = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = self.m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) })));
        { let __map_key = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = Arc::new(Mutex::new(Some(true))); (*self.m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        self.mu.unlock();
        return { let __v = (*seen.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// seenLossy is a variant of seen that avoids a lock by using a cache of recently seen hashes.
    /// Each cache entry is N-way set-associative: h can appear in any of the slots.
    /// If h does not appear in any of them, then it is inserted into a random slot,
    /// overwriting whatever was there before.
    pub fn seen_lossy(&self, h: Arc<Mutex<Option<u64>>>) -> bool {
        let mut cache: Option<GoArrayElemPtr<[u64; 4], 128>> = Some(GoArrayElemPtr::new(self.recent.clone(), ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*h.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*self.recent.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x % __tmp_y }) as usize));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 4; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __elem_ptr_0 = Some(GoArrayElemPtr::from_array_elem(cache.as_ref().unwrap().clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = sync_atomic::load_uint64(__arg0.clone()); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result }; let __tmp_y = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Compute index in set to evict as hash of current set.
        let mut ch = Arc::new(Mutex::new(Some(OFFSET64)));
        { let __range_values = { let __seq = cache.as_ref().unwrap().borrow(); __seq.as_ref().unwrap().clone() }; for x in __range_values.iter().copied() {
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x.clone())))); *ch.lock().unwrap() = Some(new_val); };
    } }
        { let __elem_ptr_0 = Some(GoArrayElemPtr::from_array_elem(cache.as_ref().unwrap().clone(), ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*ch.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(4 as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x % __tmp_y }) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = sync_atomic::store_uint64(__arg0.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
        false
    }
}

/// New creates and returns a new Matcher implementing the given pattern.
/// The pattern syntax is defined in the package doc comment.
///
/// In addition to the pattern syntax syntax, New("") returns nil, nil.
/// The nil *Matcher is valid for use: it returns true from ShouldEnable
/// and false from ShouldPrint for all changes. Callers can avoid calling
/// [Hash], [Matcher.ShouldEnable], and [Matcher.ShouldPrint] entirely
/// when they recognize the nil Matcher.
pub fn new(pattern: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Matcher>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __tmp_x = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }

    let mut m = Arc::new(Mutex::new(Some(Matcher::default())));

    let mut p = { let __owned = pattern.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };

        // Special case for leading 'q' so that 'qn' quietly disables, e.g. fmahash=qn to disable fma
        // Any instance of 'v' disables 'q'.
    if { let __tmp_x = ((*p.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('q' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = true; *(*m.lock().unwrap().as_ref().unwrap()).quiet.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
    }

        // Allow multiple v, so that “bisect cmd vPATTERN” can force verbose all the time.
    while { let __tmp_x = ((*p.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('v' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = true; *(*m.lock().unwrap().as_ref().unwrap()).verbose.lock().unwrap() = Some(new_val); };
        { let new_val = false; *(*m.lock().unwrap().as_ref().unwrap()).quiet.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
    }

        // Allow multiple !, each negating the last, so that “bisect cmd !PATTERN” works
        // even when bisect chooses to add its own !.
    { let new_val = true; *(*m.lock().unwrap().as_ref().unwrap()).enable.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = ((*p.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('!' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = !(*{ let __field = (*m.lock().unwrap().as_ref().unwrap()).enable.clone(); __field }.lock().unwrap().as_ref().unwrap()); *(*m.lock().unwrap().as_ref().unwrap()).enable.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
    }

    if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "n".to_string(); __tmp_x == __tmp_y } {
                // n is an alias for !y.
        { let new_val = !(*{ let __field = (*m.lock().unwrap().as_ref().unwrap()).enable.clone(); __field }.lock().unwrap().as_ref().unwrap()); *(*m.lock().unwrap().as_ref().unwrap()).enable.lock().unwrap() = Some(new_val); };
        { let new_val = "y".to_string(); *p.lock().unwrap() = Some(new_val); };
    }

        // n is an alias for !y.
        // Parse actual pattern syntax.
    let mut result = Arc::new(Mutex::new(Some(true)));
    let mut bits = Arc::new(Mutex::new(Some(0 as u64)));
    let mut start = Arc::new(Mutex::new(Some(0)));
    let mut wid = Arc::new(Mutex::new(Some(1)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*p.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x <= __tmp_y } {
                // Imagine a trailing - at the end of the pattern to flush final suffix
        let mut c = Arc::new(Mutex::new(Some(('-' as i32) as u8)));
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*p.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; *c.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*wid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *start.lock().unwrap() = Some(new_val); };
        { let new_val = 4; *wid.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        {
        let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
        }
        if !_matched && (_switch_val == ('2' as i32) as u8 || _switch_val == ('3' as i32) as u8 || _switch_val == ('4' as i32) as u8 || _switch_val == ('5' as i32) as u8 || _switch_val == ('6' as i32) as u8 || _switch_val == ('7' as i32) as u8 || _switch_val == ('8' as i32) as u8 || _switch_val == ('9' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = { let __v = (*wid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
            _fallthrough = true;
        }
        if !_matched && (_switch_val == ('0' as i32) as u8 || _switch_val == ('1' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = (*wid.lock().unwrap().as_ref().unwrap()); let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
            { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        }
        if !_matched && (_switch_val == ('a' as i32) as u8 || _switch_val == ('b' as i32) as u8 || _switch_val == ('c' as i32) as u8 || _switch_val == ('d' as i32) as u8 || _switch_val == ('e' as i32) as u8 || _switch_val == ('f' as i32) as u8 || _switch_val == ('A' as i32) as u8 || _switch_val == ('B' as i32) as u8 || _switch_val == ('C' as i32) as u8 || _switch_val == ('D' as i32) as u8 || _switch_val == ('E' as i32) as u8 || _switch_val == ('F' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = { let __v = (*wid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
            { let __rhs = 4 as u64; let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
            { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x20 as u8; __tmp_x & ! __tmp_y }; let __tmp_y = ('A' as i32) as u8; __tmp_x - __tmp_y }; let __tmp_y = 10 as u8; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        }
        if !_matched && (_switch_val == ('y' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*p.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && ({ let __tmp_x = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('1' as i32) as u8; __tmp_x == __tmp_y }) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
            { let new_val = 0 as u64; *bits.lock().unwrap() = Some(new_val); };
        }
        if !_matched && (_switch_val == ('+' as i32) as u8 || _switch_val == ('-' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = false; __tmp_x == __tmp_y } {
                // Have already seen a -. Should be - from here on.
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax (+ after -): ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
                        // Have already seen a -. Should be - from here on.
            if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*wid.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "pattern bits too long: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(parseError { text: Arc::new(Mutex::new(Some(format!("{}{}", "invalid pattern syntax: ".to_string(), { let __v = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
        if { let __tmp_x = { let __s = &((*p.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('y' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };
    }
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
        { let new_val = { let __append_target = (*m.lock().unwrap().as_ref().unwrap()).list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(cond { mask: Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), bits: Arc::new(Mutex::new(Some({ let __arg_holder = bits.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), result: Arc::new(Mutex::new(Some({ let __arg_holder = result.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }); __append_target.clone() }; (*m.lock().unwrap().as_mut().unwrap()).list = new_val; };
    } else if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = (*m.lock().unwrap().as_ref().unwrap()).list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(cond { mask: Arc::new(Mutex::new(Some(0 as u64))), bits: Arc::new(Mutex::new(Some(0 as u64))), result: Arc::new(Mutex::new(Some(true))), ..Default::default() }); __append_target.clone() }; (*m.lock().unwrap().as_mut().unwrap()).list = new_val; };
    }
                        // leading - subtracts from complete set
            { let new_val = 0 as u64; *bits.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y }; *result.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *start.lock().unwrap() = Some(new_val); };
            { let new_val = 1; *wid.lock().unwrap() = Some(new_val); };
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Imagine a trailing - at the end of the pattern to flush final suffix
        // leading x for hex
        // Have already seen a -. Should be - from here on.
        // leading - subtracts from complete set
    return (m.clone(), Arc::new(Mutex::new(None)));
}

/// printFileLine prints a non-marker-only report for file:line to w.
pub fn print_file_line(w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, h: Arc<Mutex<Option<u64>>>, file: Arc<Mutex<Option<String>>>, line: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    const markerLen: i32 = 40;

    let mut b = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = ({ let __tmp_x = 40; let __tmp_y = ((*file.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 24; __tmp_x + __tmp_y }) as usize))));
    { let new_val = append_marker(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); b = new_val; };
    { let new_val = append_file_line(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); b = new_val; };
    { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\n' as i32) as u8); __append_target.clone() }; b = new_val; };
    let (_, mut err) = (*w.lock().unwrap().as_mut().unwrap()).write(b.clone());
    return err.clone();
}

/// appendFileLine appends file:line to dst, returning the extended slice.
pub fn append_file_line(mut dst: Arc<Mutex<Option<Vec<u8>>>>, file: Arc<Mutex<Option<String>>>, line: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    { let new_val = { let __append_target = dst.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*file.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; dst = new_val; };
    { let new_val = { let __append_target = dst.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((':' as i32) as u8); __append_target.clone() }; dst = new_val; };
    let mut u = Arc::new(Mutex::new(Some((*line.lock().unwrap().as_ref().unwrap()) as u64)));
    if { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = dst.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; dst = new_val; };
        { let new_val = ((*u.lock().unwrap().as_ref().unwrap())).wrapping_neg(); *u.lock().unwrap() = Some(new_val); };
    }
    let mut buf: Arc<Mutex<Option<[u8; 24]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut i = Arc::new(Mutex::new(Some((*buf.lock().unwrap().as_ref().unwrap()).len() as i32)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 24; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = ('0' as i32) as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x % __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
        { let __rhs = 10 as u64; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    }
    { let new_val = { let __append_target = dst.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; dst = new_val; };
    return dst.clone();
}

/// PrintMarker prints to w a one-line report containing only the marker for h.
/// It is appropriate to use when [Matcher.ShouldPrint] and [Matcher.MarkerOnly] both return true.
pub fn print_marker(w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, h: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut buf: Arc<Mutex<Option<[u8; 50]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut b = append_marker(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\n' as i32) as u8); __append_target.clone() }; b = new_val; };
    let (_, mut err) = (*w.lock().unwrap().as_mut().unwrap()).write(b.clone());
    return err.clone();
}

/// printStack prints to w a multi-line report containing a formatting of the call stack stk,
/// with each line preceded by the marker for h.
pub fn print_stack(w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, h: Arc<Mutex<Option<u64>>>, stk: Arc<Mutex<Option<Vec<usize>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut buf = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity((2048) as usize))));

    let mut prefixBuf: Arc<Mutex<Option<[u8; 100]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut prefix = append_marker(Arc::new(Mutex::new(Some({ let __seq_holder = prefixBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    let mut frames = runtime::callers_frames(stk.clone());
    loop {
        let (mut f, mut more) = { let __recv = frames.clone(); let __recv_ptr: *mut runtime::symtab::Frames = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut runtime::symtab::Frames }; let __result = unsafe { &mut *__recv_ptr }.next(); __result };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = prefix.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*(*f.lock().unwrap().as_ref().unwrap()).function.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("()\n".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = prefix.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\t' as i32) as u8); __append_target.clone() }; buf = new_val; };
        { let new_val = append_file_line(buf.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\n' as i32) as u8); __append_target.clone() }; buf = new_val; };
        if !more {
        break
    }
    }
    { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = prefix.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
    { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\n' as i32) as u8); __append_target.clone() }; buf = new_val; };
    let (_, mut err) = (*w.lock().unwrap().as_mut().unwrap()).write(buf.clone());
    return err.clone();
}

/// AppendMarker is like [Marker] but appends the marker to dst.
pub fn append_marker(dst: Arc<Mutex<Option<Vec<u8>>>>, mut id: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    const prefix: &'static str = "[bisect-match 0x";

    let mut buf: Arc<Mutex<Option<[u8; 33]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    { let _dst_start = 0; let _dst_len = (*buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = prefix.as_bytes().to_vec(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x < __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 16; let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y }) as usize] = { let __s = &("0123456789abcdef".to_string()); __s.as_bytes()[({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x >> __tmp_y }) as usize] };
        { let __rhs = 4 as u64; let mut guard = id.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 16; let __tmp_y = 16; __tmp_x + __tmp_y }) as usize] = (']' as i32) as u8;
    return { let __append_target = dst.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
}

/// Hash computes a hash of the data arguments,
/// each of which must be of type string, byte, int, uint, int32, uint32, int64, uint64, uintptr, or a slice of one of those types.
pub fn hash(data: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> u64 {
    let mut h = Arc::new(Mutex::new(Some(OFFSET64)));
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for mut v in __range_values.into_iter() {
        {
    let _ts_ref = &v;
    let _ts_is_nil = false;
    let _ts_val: Option<&(dyn Any + Send + Sync)> = Some(_ts_ref.as_ref() as &(dyn Any + Send + Sync));
    if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        { let new_val = fnv_string(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<u8>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<u8>()).unwrap().clone())));
        { let new_val = fnv(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u64)))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<u64>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<u64>()).unwrap().clone())));
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u64)))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        { let new_val = fnv_uint32(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u32)))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<u32>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<u32>()).unwrap().clone())));
        { let new_val = fnv_uint32(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u64)))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<u64>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<u64>()).unwrap().clone())));
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<usize>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<usize>()).unwrap().clone())));
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u64)))); *h.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<String>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<String>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let new_val = fnv_string(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x.clone())))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<u8>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<u8>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x.clone())))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<i32>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<i32>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x as u64)))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<u64>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<u64>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x as u64)))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<i32>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<i32>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint32(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x as u32)))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<u32>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<u32>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint32(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x.clone())))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<i64>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<i64>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x as u64)))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<u64>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<u64>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x.clone())))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<usize>>()).is_some() {
        let v = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<usize>>()).unwrap().clone())));
        { let __range_holder = v.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = fnv_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(x as u64)))); *h.lock().unwrap() = Some(new_val); };
    } };
    } else {
        let v = _ts_val.unwrap();
        std::panic::panic_any(Box::new("bisect.Hash: unexpected argument type".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    } }
        // Note: Not printing the type, because reflect.ValueOf(v)
        // would make the interfaces prepared by the caller escape
        // and therefore allocate. This way, Hash(file, line) runs
        // without any allocation. It should be clear from the
        // source code calling Hash what the bad argument was.
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn fnv(mut h: Arc<Mutex<Option<u64>>>, x: Arc<Mutex<Option<u8>>>) -> u64 {
    { let __rhs = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
    { let __rhs = PRIME64 as u64; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn fnv_string(mut h: Arc<Mutex<Option<u64>>>, x: Arc<Mutex<Option<String>>>) -> u64 {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let __rhs = PRIME64 as u64; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn fnv_uint64(mut h: Arc<Mutex<Option<u64>>>, mut x: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x < __tmp_y } {
        { let __rhs = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFF as u64; __tmp_x & __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let __rhs = 8 as u64; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = PRIME64 as u64; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn fnv_uint32(mut h: Arc<Mutex<Option<u64>>>, mut x: Arc<Mutex<Option<u32>>>) -> u64 {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFF as u32; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let __rhs = 8 as u32; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = PRIME64 as u64; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

impl GoValueClone for Matcher {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cond {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for parseError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for dedup {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
