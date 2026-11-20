use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> f64;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub w: Rc<RefCell<Option<f64>>>,
    pub h: Rc<RefCell<Option<f64>>>,
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
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
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


#[derive(Debug, Clone)]
pub struct Square {
    pub rect: Rc<RefCell<Option<Rect>>>,
}

impl Square {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.rect.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            rect: __go_clone_0_0,
        }
    }
}


impl Default for Square {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Rect::default())));
        Self {
            rect: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.rect.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Tri {
    pub b: Rc<RefCell<Option<f64>>>,
    pub h: Rc<RefCell<Option<f64>>>,
}

impl Tri {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            b: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Tri {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            b: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Tri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.b.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.h.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Rect {
    pub fn area(&self) -> f64 {
        return (*self.w.borrow().as_ref().unwrap()) * (*self.h.borrow().as_ref().unwrap());
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
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
    fn area(&self) -> f64 {
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

impl Tri {
    pub fn area(&self) -> f64 {
        return (*self.b.borrow().as_ref().unwrap()) * (*self.h.borrow().as_ref().unwrap()) / 2.0;
    }
}

impl Shape for Tri {
    fn area(&self) -> f64 {
        Tri::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Tri>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TriPtr(pub Rc<RefCell<Option<Tri>>>);

impl std::fmt::Display for TriPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for TriPtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Tri::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TriPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Square {
    pub fn area(&self) -> f64 {
        // Forward to embedded type's method
        let embedded = self.rect.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.area()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        Square::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Square>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SquarePtr(pub Rc<RefCell<Option<Square>>>);

impl std::fmt::Display for SquarePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for SquarePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SquarePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn new_square(s: Rc<RefCell<Option<f64>>>) -> Rc<RefCell<Option<Square>>> {
    Rc::new(RefCell::new(Some(Square { rect: Rc::new(RefCell::new(Some(Rect { w: Rc::new(RefCell::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), h: Rc::new(RefCell::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), ..Default::default() })))
}

fn main() {
    let mut shapes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Rect { w: Rc::new(RefCell::new(Some(3.0))), h: Rc::new(RefCell::new(Some(4.0))), ..Default::default() }) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new((*new_square(Rc::new(RefCell::new(Some(2.0)))).borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new(Tri { b: Rc::new(RefCell::new(Some(4.0))), h: Rc::new(RefCell::new(Some(5.0))), ..Default::default() }) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new(Rect { w: Rc::new(RefCell::new(Some(1.0))), h: Rc::new(RefCell::new(Some(2.0))), ..Default::default() }) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new((*new_square(Rc::new(RefCell::new(Some(1.5)))).borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>)))])));
    let mut total = Rc::new(RefCell::new(Some(0.0)));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, s) in __range_values.iter().enumerate() {
        print!("{} {:.2}\n", i, (*s.borrow().as_ref().unwrap()).area());
        { let __rhs = (*s.borrow().as_ref().unwrap()).area(); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    print!("total {:.2}\n", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v });
}