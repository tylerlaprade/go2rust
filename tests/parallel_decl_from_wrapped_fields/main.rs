use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct P {
    pub pos: Rc<RefCell<Option<i64>>>,
    pub tok: Rc<RefCell<Option<i64>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.tok.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            pos: __go_clone_0_0,
            tok: __go_clone_1_0,
        }
    }
}


impl Default for P {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            pos: __go_default_0_0,
            tok: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.pos.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.tok.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl P {
    /// A parallel short declaration whose RHS values are wrapped struct fields
    /// (`a, b := p.pos, p.tok`) must copy each field's value, not re-wrap the
    /// already-wrapped field handle. go/parser's `pos, op := p.pos, p.tok` hit a
    /// double-wrap (Arc<Mutex<Option<Arc<Mutex<Option<Pos>>>>>>) before this.
    pub fn pair(&self) -> (i64, i64) {
        let (mut a, mut b) = (Rc::new(RefCell::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        return ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()));
    }
}

fn main() {
    let (mut x, mut y) = { let __recv = (Rc::new(RefCell::new(Some(P { pos: Rc::new(RefCell::new(Some(1))), tok: Rc::new(RefCell::new(Some(2))), ..Default::default() })))); let __result = (*__recv.borrow().as_ref().unwrap()).pair(); __result };
    println!("{} {}", format!("{}", x), format!("{}", y));
}