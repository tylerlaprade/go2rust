use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

#[derive(Clone)]
pub struct GoSliceElemPtr<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone, const N: usize> {
    array: Rc<RefCell<Option<[T; N]>>>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone, const N: usize> {
    array: Rc<RefCell<Option<[T; N]>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn new(slice: Rc<RefCell<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    pub fn slice_handle(&self) -> Rc<RefCell<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Rc<RefCell<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr { array, index }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        let guard = self.array.borrow();
        GoArrayElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        let guard = self.array.borrow();
        GoArrayElemMutRef {
            array: self.array.clone(),
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
            if let Some(values) = self.slice.borrow_mut().as_mut() {
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

impl<T: Clone, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.array.borrow_mut().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

#[derive(Clone)]
pub enum GoPtr<T: Clone> {
    Nil,
    Local(Rc<RefCell<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
}

impl<T: Clone> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn local(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
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

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Local(value) => value.borrow().is_none(),
            GoPtr::SliceElem(value) => value.borrow().is_none(),
        }
    }

    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Local(value) => (*value.borrow()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
        }
    }
}

impl<T: Clone> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Clone> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Point {
    fn default() -> Self {
        Self { x: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(10))), ..Default::default() })));
    let mut px = (*p.borrow().as_ref().unwrap()).x.clone();
    { let new_val = 20; *px.borrow_mut() = Some(new_val); };

    let mut nums = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    let mut first: Option<GoSliceElemPtr<i32>> = Some(GoSliceElemPtr::new(nums.clone(), (0) as usize));
    { let new_val = 9; *first.as_ref().unwrap().borrow_mut() = Some(new_val); };

    println!("{} {}", format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", format_slice(&nums)));
}