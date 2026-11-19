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

#[derive(Debug, Clone)]
pub struct P {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for P {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct A(pub Rc<RefCell<Option<[P; 1]>>>);

impl Default for A {
    fn default() -> Self {
        A(Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default())))))
    }
}

impl A {
    pub fn __go_value_clone(&self) -> Self {
        A(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))))
    }
}


#[derive(Debug, Clone)]
pub struct Grid(pub Rc<RefCell<Option<[A; 2]>>>);

impl Default for Grid {
    fn default() -> Self {
        Grid(Rc::new(RefCell::new(Some(std::array::from_fn(|_| A(Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))))))))))
    }
}

impl Grid {
    pub fn __go_value_clone(&self) -> Self {
        Grid(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))))
    }
}


#[derive(Debug, Clone)]
pub struct Nums(pub Rc<RefCell<Option<[i32; 2]>>>);

impl Default for Nums {
    fn default() -> Self {
        Nums(Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Nums {
    pub fn __go_value_clone(&self) -> Self {
        Nums(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.clone()))))
    }
}

impl Display for Nums {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


pub fn change(a: Rc<RefCell<Option<A>>>) {
    { let new_val = 50; *{ let __seq_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
}

/// A named array type is a value: assigning it or passing it copies every
/// element.
fn main() {
    let mut a: Rc<RefCell<Option<A>>> = Rc::new(RefCell::new(Some(A(Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))))))));
    { let new_val = 1; *{ let __seq_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
    let mut b = { let __owned = a.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 9; *{ let __seq_holder = { let __named_array = (*b.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "named array of structs".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())), format!("{}", (*{ let __seq_holder = { let __named_array = (*b.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())));

    let mut c: Rc<RefCell<Option<A>>> = Rc::new(RefCell::new(Some(A(Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))))))));
    { let new_val = a.borrow().as_ref().unwrap().__go_value_clone(); *c.borrow_mut() = Some(new_val); };
    { let new_val = 8; *{ let __seq_holder = { let __named_array = (*c.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())), format!("{}", (*{ let __seq_holder = { let __named_array = (*c.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())));

    change(Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {}", format!("{}", "parameter".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())));

    let mut n: Rc<RefCell<Option<Nums>>> = Rc::new(RefCell::new(Some(Nums(Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))))))));
    (*{ let __named_array = (*n.borrow().as_ref().unwrap()).0.clone(); __named_array }.borrow_mut().as_mut().unwrap())[(0) as usize] = 1;
    (*{ let __named_array = (*n.borrow().as_ref().unwrap()).0.clone(); __named_array }.borrow_mut().as_mut().unwrap())[(1) as usize] = 2;
    let mut m = { let __owned = n.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    (*{ let __named_array = (*m.borrow().as_ref().unwrap()).0.clone(); __named_array }.borrow_mut().as_mut().unwrap())[(0) as usize] = 9;
    println!("{} {} {}", format!("{}", "named array of ints".to_string()), format!("{}", { let __seq_holder = { let __named_array = (*n.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }), format!("{}", { let __seq_holder = { let __named_array = (*m.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }));

    let mut g: Rc<RefCell<Option<Grid>>> = Rc::new(RefCell::new(Some(Grid(Rc::new(RefCell::new(Some(std::array::from_fn(|_| A(Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default())))))))))))));
    { let new_val = 1; *{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*g.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
    { let new_val = 2; *{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*g.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
    let mut h = { let __owned = g.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 7; *{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*h.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "nested named arrays".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*g.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())), format!("{}", (*{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*h.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())));

    { let __range_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 99; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over a named array".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.borrow().as_ref().unwrap())));
}