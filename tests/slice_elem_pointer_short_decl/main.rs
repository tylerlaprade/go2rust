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
pub struct info {
    pub name: Arc<Mutex<Option<String>>>,
    pub count: Arc<Mutex<Option<i32>>>,
}

impl info {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, count: { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for info {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), count: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.count.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut infos = Arc::new(Mutex::new(Some(vec![info { name: Arc::new(Mutex::new(Some("foo".to_string()))), count: Arc::new(Mutex::new(Some(1))), ..Default::default() }, info { name: Arc::new(Mutex::new(Some("bar".to_string()))), count: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));

    let mut alt: Option<GoSliceElemPtr<info>> = Some(GoSliceElemPtr::new(infos.clone(), (1) as usize));
    println!("{}", format!("{}", (*{ let __field = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*{ let __field = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).count.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    { let new_val = 42; *(*alt.as_ref().unwrap().borrow().as_ref().unwrap()).count.lock().unwrap() = Some(new_val); };
    println!("{}", format!("{}", (*{ let __seq = { let __seq_holder = infos.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.count.lock().unwrap().as_ref().unwrap())));
}