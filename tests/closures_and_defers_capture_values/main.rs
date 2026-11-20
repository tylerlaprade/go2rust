use std::cell::{RefCell};
use std::rc::{Rc};

pub fn deferred() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut x = Rc::new(RefCell::new(Some(1)));
    let x_defer_captured = Rc::new(RefCell::new((*x.borrow()).clone())); __defer_stack.push(Box::new(move || {
        println!("{} {}", format!("{}", "deferred x:".to_string()), format!("{}", { let __v = (*x_defer_captured.borrow().as_ref().unwrap()).clone(); __v }));
    }));
    { let new_val = 2; *x.borrow_mut() = Some(new_val); };
    let mut i = Rc::new(RefCell::new(Some(3)));
    while (*i.borrow().as_ref().unwrap()) > 0 {
        let i_defer_captured = Rc::new(RefCell::new((*i.borrow()).clone())); __defer_stack.push(Box::new(move || {
        print!("{}{}", format!("{}", { let __v = (*i_defer_captured.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
    }));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut name = Rc::new(RefCell::new(Some("first".to_string())));
    let __defer_arg_0 = Rc::new(RefCell::new(Some((*name.borrow().as_ref().unwrap()).clone()))); __defer_stack.push(Box::new(move || {
        (move |n: Rc<RefCell<Option<String>>>| {
        println!("{} {}", format!("{}", "arg:".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));;
        })(__defer_arg_0);
    }));
    { let new_val = "second".to_string(); *name.borrow_mut() = Some(new_val); };
    let name_defer_captured = name.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{} {}", format!("{}", "closure sees:".to_string()), format!("{}", { let __v = (*name_defer_captured.borrow().as_ref().unwrap()).clone(); __v }));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    println!("{} {}", format!("{}", "x is".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    let mut funcs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn FnMut() -> i32>>>>>>>> = Rc::new(RefCell::new(None));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        let i_closure_clone = Rc::new(RefCell::new((*i.borrow()).clone())); { let new_val = { let __append_target = funcs.clone(); let __append_value = Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        return { let __bin_i = (*i_closure_clone.borrow().as_ref().unwrap()).clone(); __bin_i * __bin_i };
    }) as Box<dyn FnMut() -> i32>))); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; funcs = new_val; };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __range_holder = funcs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter().cloned() {
        print!("{}{}", format!("{}", { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }), format!("{}", " ".to_string()));
    } }
    println!();

    let mut printers: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>>>> = Rc::new(RefCell::new(None));
    let (mut i, mut s) = (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some("a".to_string()))));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        let i_closure_clone = Rc::new(RefCell::new((*i.borrow()).clone())); let s_closure_clone = Rc::new(RefCell::new((*s.borrow()).clone())); { let new_val = { let __append_target = printers.clone(); let __append_value = Rc::new(RefCell::new(Some(Box::new(move || {
        print!("{}{}{}", format!("{}", { let __v = (*i_closure_clone.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*s_closure_clone.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
    }) as Box<dyn FnMut() -> ()>))); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; printers = new_val; };
        {
            let __tmp_0 = (*i.borrow().as_ref().unwrap()) + 1;
            let __tmp_1 = format!("{}{}", (*s.borrow().as_ref().unwrap()), "a".to_string());
            *i.borrow_mut() = Some(__tmp_0);
            *s.borrow_mut() = Some(__tmp_1);
        };
    }
    { let __range_holder = printers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter().cloned() {
        { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = p.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    } }
    println!();

    let mut total = Rc::new(RefCell::new(Some(0)));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 4 {
        let i_closure_clone = Rc::new(RefCell::new((*i.borrow()).clone())); let mut total_closure_clone = total.clone(); let mut add = Rc::new(RefCell::new(Some(Box::new(move || {
        { let __rhs = (*i_closure_clone.borrow().as_ref().unwrap()); let mut guard = total_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>)));
        { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = add.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "total:".to_string()), format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));

    let mut count = Rc::new(RefCell::new(Some(0)));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 10 {
        let mut i_closure_clone = i.clone(); let mut step = Rc::new(RefCell::new(Some(Box::new(move || {
        { let __rhs = 3; let mut guard = i_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>)));
        { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = step.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "steps:".to_string()), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
    deferred();
    println!();
}