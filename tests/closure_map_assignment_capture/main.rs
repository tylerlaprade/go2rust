use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut view = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    let mut visit: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> ()>>>> = Rc::new(RefCell::new(None));
    let view_closure_clone = view.clone(); let visit_closure_clone = visit.clone(); { let __func_lit_target = visit_closure_clone.clone(); let new_val = Box::new(move |pkgs: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>| {
        for (id, _) in { let __range_holder = pkgs.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let __map_key = id.clone(); let __map_value = Rc::new(RefCell::new(Some((*pkgs.borrow().as_ref().unwrap()).len() as i32))); (*view_closure_clone.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        if ((*pkgs.borrow().as_ref().unwrap()).len() as i32) > (0 as i32) {
        { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> ()> = { let mut __f_guard = visit_closure_clone.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([]))))) };
    }
    }) as Box<dyn FnMut(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> ()>; *__func_lit_target.borrow_mut() = Some(new_val); };

    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> ()> = { let mut __f_guard = visit.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("pkg".to_string(), Rc::new(RefCell::new(Some(1))))]))))) };
    println!("{}", (*view.borrow().as_ref().unwrap()).len());
    println!("{}", (*view.borrow().as_ref().unwrap()).get(&"pkg".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0));
}