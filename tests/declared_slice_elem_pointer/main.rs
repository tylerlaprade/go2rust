use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Clone)]
struct GoSliceElemPtr<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
}

struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
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

#[derive(Debug, Clone)]
pub struct entry {
    pub key: Arc<Mutex<Option<i32>>>,
    pub value: Arc<Mutex<Option<String>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for entry {
    fn default() -> Self {
        Self { key: Arc::new(Mutex::new(Some(0))), value: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.key.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut bucket = Arc::new(Mutex::new(Some(vec![entry { key: Arc::new(Mutex::new(Some(1))), value: Arc::new(Mutex::new(Some("old".to_string()))), ..Default::default() }])));
    let mut hole: Option<GoSliceElemPtr<entry>> = None;

    { let __range_holder = bucket.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, e) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*e.key.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x == __tmp_y } {
        hole = Some(GoSliceElemPtr::new(bucket.clone(), (i) as usize));
    }
    } }

    if hole.is_some() {
        { let new_val = entry { key: Arc::new(Mutex::new(Some(1))), value: Arc::new(Mutex::new(Some("new".to_string()))), ..Default::default() }; *hole.as_ref().unwrap().borrow_mut() = Some(new_val); };
    }

    println!("{}", format!("{}", (*{ let __seq = { let __seq_holder = bucket.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.value.lock().unwrap().as_ref().unwrap())));
}