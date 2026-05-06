use std::cell::{RefCell};
use std::rc::{Rc};

pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static ProcessData: GoGlobal<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>> = GoGlobal::new();

pub(crate) static CombineStrings: GoGlobal<Box<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>> = GoGlobal::new();

pub(crate) static ApplyTwice: GoGlobal<Box<dyn Fn(Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>> = GoGlobal::new();

pub(crate) static GetGreeting: GoGlobal<Box<dyn Fn() -> Rc<RefCell<Option<String>>>>> = GoGlobal::new();

pub(crate) static DivMod: GoGlobal<Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>)>> = GoGlobal::new();

pub(crate) static DynamicFunc: GoGlobal<Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *ProcessData.borrow_mut() = Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>);
    *CombineStrings.borrow_mut() = Some(Box::new(move |a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", format!("{}{}", (*a.borrow().as_ref().unwrap()), " ".to_string()), (*b.borrow().as_ref().unwrap())))));
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>);
    *ApplyTwice.borrow_mut() = Some(Box::new(move |f: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>, x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return { let __f_guard = f.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)({ let __f_guard = f.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(x.clone()) }) };
    }) as Box<dyn Fn(Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>);
    *GetGreeting.borrow_mut() = Some(Box::new(move || -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some("Hello from function variable!".to_string())));
    }) as Box<dyn Fn() -> Rc<RefCell<Option<String>>>>);
    *DivMod.borrow_mut() = Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {
        return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x % __tmp_y)))
        });
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>)>);
}


/// Regular function for comparison
pub fn regular_double(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

/// Function that returns a function
pub fn make_multiplier(factor: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>> {

    return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = (*factor.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

pub fn __go_init_0() {
        // Assign function to variable in init
    { let new_val = Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Dynamic: {}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }))));
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *DynamicFunc.borrow_mut() = Some(new_val); };
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
    __go_init_0();
}
