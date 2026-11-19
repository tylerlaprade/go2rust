use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i32>>>,
    pub y: Rc<RefCell<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Line {
    pub a: Rc<RefCell<Option<Point>>>,
    pub b: Rc<RefCell<Option<Point>>>,
}

impl Line {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_0_1 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for Line {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Point::default())));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(Point::default())));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> i32;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub w: Rc<RefCell<Option<i32>>>,
    pub h: Rc<RefCell<Option<i32>>>,
}

impl Rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Rect {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            w: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.w.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.h.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Rect {
    pub fn area(&self) -> i32 {
        return (*self.w.borrow().as_ref().unwrap()) * (*self.h.borrow().as_ref().unwrap());
    }
}

impl Shape for Rect {
    fn area(&self) -> i32 {
        Rect::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rect>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectPtr(pub Rc<RefCell<Option<Rect>>>);

impl std::fmt::Display for RectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for RectPtr {
    fn area(&self) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// Storing a struct somewhere copies it, and so does reading it back. Each
/// line changes the variable after the store, or the value after the read,
/// and prints both sides.
fn main() {
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() })));

    let mut s = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(0))), y: Rc::new(RefCell::new(Some(0))), ..Default::default() }])));
    (*s.borrow_mut().as_mut().unwrap())[(0) as usize] = (*p.borrow().as_ref().unwrap()).__go_value_clone();
    { let new_val = 2; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "stored in a slice".to_string()), format!("{}", (*s.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*p.borrow().as_ref().unwrap())));

    { let new_val = { let __append_target = s.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*p.borrow().as_ref().unwrap()).__go_value_clone()); __append_target.clone() }; s = new_val; };
    { let new_val = 3; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "appended".to_string()), format!("{}", (*s.borrow().as_ref().unwrap())[(1) as usize].clone()), format!("{}", (*p.borrow().as_ref().unwrap())));

    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([]))));
    { let __map_key = "a".to_string(); let __map_value = { let __owned = p.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }; (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let new_val = 4; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "stored in a map".to_string()), format!("{}", { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }), format!("{}", (*p.borrow().as_ref().unwrap())));

    let mut got = Rc::new(RefCell::new(Some({ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
    { let new_val = 40; *(*got.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "read from a map".to_string()), format!("{}", { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }), format!("{}", (*got.borrow().as_ref().unwrap())));

    let mut l = Rc::new(RefCell::new(Some(Line { a: Rc::new(RefCell::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), b: Rc::new(RefCell::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));
    { let new_val = 5; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "in a struct literal".to_string()), format!("{}", (*l.borrow().as_ref().unwrap())), format!("{}", (*p.borrow().as_ref().unwrap())));

    { let new_val = p.borrow().as_ref().unwrap().__go_value_clone(); *(*l.borrow().as_ref().unwrap()).b.borrow_mut() = Some(new_val); };
    { let new_val = 6; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned to a field".to_string()), format!("{}", (*l.borrow().as_ref().unwrap())), format!("{}", (*p.borrow().as_ref().unwrap())));

    let mut arr = Rc::new(RefCell::new(Some([Default::default(), Default::default()])));
    (*arr.borrow_mut().as_mut().unwrap())[(0) as usize] = (*p.borrow().as_ref().unwrap()).__go_value_clone();
    { let new_val = 7; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "stored in an array".to_string()), format!("{}", (*arr.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*p.borrow().as_ref().unwrap())));

    let mut r = Rc::new(RefCell::new(Some(Rect { w: Rc::new(RefCell::new(Some(2))), h: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    let mut shape: Rc<RefCell<Option<Box<dyn Shape>>>> = Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape>)));
    { let new_val = 10; *(*r.borrow().as_ref().unwrap()).w.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "in an interface".to_string()), format!("{}", (*shape.borrow().as_ref().unwrap()).area()), format!("{}", (*r.borrow().as_ref().unwrap()).area()));

    let mut q = p.clone();
    let mut c = Rc::new(RefCell::new(Some({ let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    { let new_val = 100; *(*c.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "read through a pointer".to_string()), format!("{}", (*p.borrow().as_ref().unwrap())), format!("{}", (*c.borrow().as_ref().unwrap())));

    let mut d: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone(); *d.borrow_mut() = Some(new_val); };
    { let new_val = 200; *(*d.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned through a pointer".to_string()), format!("{}", (*p.borrow().as_ref().unwrap())), format!("{}", (*d.borrow().as_ref().unwrap())));

    let mut e: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = (*s.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone(); *e.borrow_mut() = Some(new_val); };
    { let new_val = 300; *(*e.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned from an element".to_string()), format!("{}", (*s.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*e.borrow().as_ref().unwrap())));

    let mut list = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut spread = { let __append_target = Rc::new(RefCell::new(Some(Vec::<Point>::new()))).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend({ let __slice_holder = list.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().map(|__go_elem| __go_elem.__go_value_clone())); __append_target.clone() };
    { let new_val = 400; *(*spread.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "slice copied with append".to_string()), format!("{}", (*list.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*spread.borrow().as_ref().unwrap())[(0) as usize].clone()));

    let mut copied: Rc<RefCell<Option<Vec<Point>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); ((*list.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    let mut n = {
        let _src = { let __copy_src_holder = list.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
        let _n = std::cmp::min((*copied.borrow().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*copied.borrow_mut().as_mut().unwrap())[_i] = _src[_i].__go_value_clone();
        }
        Rc::new(RefCell::new(Some(_n as i32)))
    };
    { let new_val = 500; *(*copied.borrow().as_ref().unwrap())[(1) as usize].clone().y.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "slice copied with copy".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*list.borrow().as_ref().unwrap())[(1) as usize].clone()), format!("{}", (*copied.borrow().as_ref().unwrap())[(1) as usize].clone()));

    let mut grown = { let __append_target = spread.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend({ let __slice_holder = list.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().map(|__go_elem| __go_elem.__go_value_clone())); __append_target.clone() };
    { let new_val = 600; *(*grown.borrow().as_ref().unwrap())[(2) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "slice appended to another".to_string()), format!("{}", (*list.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*grown.borrow().as_ref().unwrap())[(2) as usize].clone()), format!("{}", (*grown.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}