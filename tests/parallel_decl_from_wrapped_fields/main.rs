use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct P {
    pub pos: Rc<RefCell<Option<i32>>>,
    pub tok: Rc<RefCell<Option<i32>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, tok: { let __guard = self.tok.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for P {
    fn default() -> Self {
        Self { pos: Rc::new(RefCell::new(Some(0))), tok: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.pos.borrow().as_ref().unwrap()), (*self.tok.borrow().as_ref().unwrap()))
    }
}


impl P {
    /// A parallel short declaration whose RHS values are wrapped struct fields
    /// (`a, b := p.pos, p.tok`) must copy each field's value, not re-wrap the
    /// already-wrapped field handle. go/parser's `pos, op := p.pos, p.tok` hit a
    /// double-wrap (Arc<Mutex<Option<Arc<Mutex<Option<Pos>>>>>>) before this.
    pub fn pair(&self) -> (i32, i32) {
        let (mut a, mut b) = (Rc::new(RefCell::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        return ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()));
    }
}

fn main() {
    let (mut x, mut y) = { let __recv = (Rc::new(RefCell::new(Some(P { pos: Rc::new(RefCell::new(Some(1 as i32))), tok: Rc::new(RefCell::new(Some(2 as i32))), ..Default::default() })))); let __result = (*__recv.borrow().as_ref().unwrap()).pair(); __result };
    println!("{} {}", format!("{}", x), format!("{}", y));
}