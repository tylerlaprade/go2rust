use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

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

struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    // A select case's receive: Some(Some(v)) for a value, Some(None) when
    // the channel is closed and drained, None when nothing is ready yet.
    fn try_recv_select(&self) -> Option<Option<T>> {
        if self.is_nil() {
            return None;
        }
        match self.rx.lock().unwrap().try_recv() {
            Ok(value) => {
                if self.capacity > 0 {
                    let _ = self.len.fetch_update(
                        std::sync::atomic::Ordering::SeqCst,
                        std::sync::atomic::Ordering::SeqCst,
                        |__go_current| __go_current.checked_sub(1),
                    );
                }
                Some(Some(value))
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(None),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
        }
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Arc<Mutex<Option<i32>>>,
    pub next: Arc<Mutex<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            value: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            value: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Tree {
    pub left: Arc<Mutex<Option<Tree>>>,
    pub right: Arc<Mutex<Option<Tree>>>,
    pub val: Arc<Mutex<Option<i32>>>,
}

impl Tree {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.left.clone();
        let __go_clone_0_1 = self.right.clone();
        let __go_clone_1_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            left: __go_clone_0_0,
            right: __go_clone_0_1,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Tree {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_0_1 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            left: __go_default_0_0,
            right: __go_default_0_1,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.left.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.right.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct List {
    pub head: Arc<Mutex<Option<Elem>>>,
    pub size: Arc<Mutex<Option<i32>>>,
}

impl List {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_1_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            head: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for List {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            head: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Elem {
    pub val: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Elem>>>,
}

impl Elem {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            val: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Elem {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            val: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


pub trait Stringer: std::fmt::Display + Any {
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Stringer + Send + Sync> {
    fn clone(&self) -> Self {
        Stringer::__go_clone_box_stringer(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Counter {
    pub hits: Arc<Mutex<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.hits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            hits: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            hits: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.hits.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Node {
    pub fn len(&self) -> i32 {
        if false {
        return 0;
    }
        return { let __tmp_x = 1; let __tmp_y = __go_nil_recv_node_len({ let __field = self.next.clone(); __field }); __tmp_x + __tmp_y };
    }

    pub fn sum(&self) -> i32 {
        if false {
        return 0;
    }
        return {
            let __tmp_x = (*self.value.lock().unwrap().as_ref().unwrap());
            let __tmp_y = __go_nil_recv_node_sum({ let __field = self.next.clone(); __field });
            __tmp_x + __tmp_y
        };
    }
}

impl Tree {
    pub fn insert(&mut self, v: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Tree>>> {
        if false {
        return Arc::new(Mutex::new(Some(Tree { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.val.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __go_nil_recv_tree_insert({ let __field = self.left.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.left = new_val; };
    } else {
        { let new_val = __go_nil_recv_tree_insert({ let __field = self.right.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.right = new_val; };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn in_order(&self, visit: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>>>) {
        if false {
        return;
    }
        __go_nil_recv_tree_in_order({ let __field = self.left.clone(); __field }, visit.clone());
        {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = {
                let mut __f_guard = visit.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some({ let __selector_holder = self.val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))
            )
        };
        __go_nil_recv_tree_in_order({ let __field = self.right.clone(); __field }, visit.clone());
    }

    pub fn height(&self) -> i32 {
        if false {
        return 0;
    }
        let (mut l, mut r) = (__go_nil_recv_tree_height({ let __field = self.left.clone(); __field }), __go_nil_recv_tree_height({ let __field = self.right.clone(); __field }));
        if { let __tmp_x = l; let __tmp_y = r; __tmp_x > __tmp_y } {
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
        return { let __tmp_x = r; let __tmp_y = 1; __tmp_x + __tmp_y };
    }

    pub fn contains(&mut self, v: Arc<Mutex<Option<i32>>>) -> bool {
        let mut __self = Arc::new(Mutex::new(Some(self.clone())));
        while { let __self_guard = __self.lock().unwrap(); __self_guard.is_some() } {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*__self.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*__self.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = (*__self.lock().unwrap().as_ref().unwrap()).left.clone(); __self = new_val; };
    } else {
        { let new_val = (*__self.lock().unwrap().as_ref().unwrap()).right.clone(); __self = new_val; };
    }
    }
        false
    }
}

impl Elem {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if false {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*self.val.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", "->".to_string()));
            __s.push_str(&format!("{}", (*__go_nil_recv_elem_string({ let __field = self.next.clone(); __field }).lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }

    pub fn append(&mut self, v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Elem>>> {
        if false {
        return Arc::new(Mutex::new(Some(Elem { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
        { let new_val = __go_nil_recv_elem_append({ let __field = self.next.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.next = new_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }
}

impl Stringer for Elem {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Elem::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Elem>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ElemPtr(pub Arc<Mutex<Option<Elem>>>);

impl std::fmt::Display for ElemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Stringer for ElemPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Elem::string(__recv)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ElemPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl List {
    pub fn push(&mut self, v: Arc<Mutex<Option<String>>>) {
        { let new_val = __go_nil_recv_elem_append({ let __field = self.head.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); self.head = new_val; };
        { let __target = self.size.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl Counter {
    pub fn hit(&mut self) -> i32 {
        if false {
        return -(1);
    }
        { let __target = self.hits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.hits.lock().unwrap().as_ref().unwrap());
    }
}

pub fn describe(s: Arc<Mutex<Option<Box<dyn Stringer + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "[".to_string()));
        __s.push_str(&format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
        __s.push_str(&format!("{}", "]".to_string()));
        __s
    })));
}

pub fn more() {
    let mut l: Arc<Mutex<Option<List>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = "a b c".to_string(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        (*l.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some((*w).clone()))));
    } }
    println!("{} {}", format!("{}", (*__go_nil_recv_elem_string({ let __field = (*l.lock().unwrap().as_ref().unwrap()).head.clone(); __field }).lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut none: Arc<Mutex<Option<Elem>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", (*__go_nil_recv_elem_string(none.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(ElemPtr((*l.lock().unwrap().as_ref().unwrap()).head.clone())) as Box<dyn Stringer + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut c = Arc::new(Mutex::new(Some(Counter { hits: Arc::new(Mutex::new(Some(0))) })));
    __go_nil_recv_counter_hit(c.clone());
    __go_nil_recv_counter_hit(c.clone());
    let mut missing: Arc<Mutex<Option<Counter>>> = Arc::new(Mutex::new(None));
    println!("{} {} {}", format!("{}", __go_nil_recv_counter_hit(c.clone())), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).hits.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", __go_nil_recv_counter_hit(missing.clone())));
    let mut cs = Arc::new(Mutex::new(Some(vec![c.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Counter { ..Default::default() })))])));
    { let __range_holder = cs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        print!("{}{}", format!("{}", __go_nil_recv_counter_hit((*x).clone())), format!("{}", " ".to_string()));
    } }
    println!();
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Counter>>>>::from([("a".to_string(), c.clone())]))));
    println!("{} {}", format!("{}", __go_nil_recv_counter_hit({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", __go_nil_recv_counter_hit({ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"zz".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    let mut done = GoChannel::<i32>::new();
    let c_thread = c.clone(); let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(__go_nil_recv_counter_hit(c_thread.clone()));;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut list = Arc::new(Mutex::new(Some(Node {
        value: Arc::new(Mutex::new(Some(1))),
        next: Arc::new(Mutex::new(Some(Node {
            value: Arc::new(Mutex::new(Some(2))),
            next: Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(3))), next: Default::default(), ..Default::default() }))).clone(),
            ..Default::default()
        }))).clone(),
        ..Default::default()
    })));
    println!("{} {}", format!("{}", __go_nil_recv_node_len(list.clone())), format!("{}", __go_nil_recv_node_sum(list.clone())));
    let mut empty: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", __go_nil_recv_node_len(empty.clone())), format!("{}", __go_nil_recv_node_sum(empty.clone())));

    let mut root: Arc<Mutex<Option<Tree>>> = Arc::new(Mutex::new(None));
    for v in vec![5, 3, 8, 1, 4, 9].iter().copied() {
        { let new_val = __go_nil_recv_tree_insert(root.clone(), Arc::new(Mutex::new(Some(v.clone())))).clone(); root = new_val; };
    }
    let mut out: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let mut out_closure_clone = out.clone(); __go_nil_recv_tree_in_order(root.clone(), Arc::new(Mutex::new(Some(Box::new(move |v: Arc<Mutex<Option<i32>>>| {
        { let __append_target = out_closure_clone.clone(); let __append_value = (*v.lock().unwrap().as_ref().unwrap()).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>))));
    println!("{} {} {} {}", format!("{}", format_slice(&out)), format!("{}", __go_nil_recv_tree_height(root.clone())), format!("{}", __go_nil_recv_tree_contains(root.clone(), Arc::new(Mutex::new(Some(4))))), format!("{}", __go_nil_recv_tree_contains(root.clone(), Arc::new(Mutex::new(Some(7))))));
    more();
}

pub fn __go_nil_recv_counter_hit(c: Arc<Mutex<Option<Counter>>>) -> i32 {
    if { let __nil_result = (*c.lock().unwrap()).is_none(); __nil_result } {
        return -(1);
    }
    { let __target = (*c.lock().unwrap().as_ref().unwrap()).hits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return (*(*c.lock().unwrap().as_ref().unwrap()).hits.lock().unwrap().as_ref().unwrap());
}

pub fn __go_nil_recv_elem_append(e: Arc<Mutex<Option<Elem>>>, v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Elem>>> {
    if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Elem { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    { let new_val = __go_nil_recv_elem_append({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*e.lock().unwrap().as_mut().unwrap()).next = new_val; };
    e.clone()
}

pub fn __go_nil_recv_elem_string(e: Arc<Mutex<Option<Elem>>>) -> Arc<Mutex<Option<String>>> {
    if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
        __s.push_str(&format!("{}", "->".to_string()));
        __s.push_str(&format!("{}", (*__go_nil_recv_elem_string({ let __field = (*e.lock().unwrap().as_ref().unwrap()).next.clone(); __field }).lock().unwrap().as_ref().unwrap())));
        __s
    })));
}

pub fn __go_nil_recv_node_len(n: Arc<Mutex<Option<Node>>>) -> i32 {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    return { let __tmp_x = 1; let __tmp_y = __go_nil_recv_node_len({ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field }); __tmp_x + __tmp_y };
}

pub fn __go_nil_recv_node_sum(n: Arc<Mutex<Option<Node>>>) -> i32 {
    if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    return {
        let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = __go_nil_recv_node_sum({ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field });
        __tmp_x + __tmp_y
    };
}

pub fn __go_nil_recv_tree_contains(mut t: Arc<Mutex<Option<Tree>>>, v: Arc<Mutex<Option<i32>>>) -> bool {
    while { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); t = new_val; };
    } else {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); t = new_val; };
    }
    }
    false
}

pub fn __go_nil_recv_tree_height(t: Arc<Mutex<Option<Tree>>>) -> i32 {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    let (mut l, mut r) = (__go_nil_recv_tree_height({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }), __go_nil_recv_tree_height({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }));
    if { let __tmp_x = l; let __tmp_y = r; __tmp_x > __tmp_y } {
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
    return { let __tmp_x = r; let __tmp_y = 1; __tmp_x + __tmp_y };
}

pub fn __go_nil_recv_tree_in_order(t: Arc<Mutex<Option<Tree>>>, visit: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>>>) {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    __go_nil_recv_tree_in_order({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, visit.clone());
    {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = {
            let mut __f_guard = visit.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))
        )
    };
    __go_nil_recv_tree_in_order({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, visit.clone());
}

pub fn __go_nil_recv_tree_insert(t: Arc<Mutex<Option<Tree>>>, v: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Tree>>> {
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(Tree { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).left = new_val; };
    } else {
        { let new_val = __go_nil_recv_tree_insert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).right.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); (*t.lock().unwrap().as_mut().unwrap()).right = new_val; };
    }
    t.clone()
}