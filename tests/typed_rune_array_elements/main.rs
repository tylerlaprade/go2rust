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

pub(crate) static label: GoGlobal<[u8; 5]> = GoGlobal::new();


fn __go_init_globals() {
    *label.borrow_mut() = Some(std::array::from_fn::<_, 5, _>(|_| 0));
    {
        let mut __go_array = Vec::<u8>::with_capacity(5);
        __go_array.push(('n' as u8));
        __go_array.push(('o' as u8));
        __go_array.push(('n' as u8));
        __go_array.push(('e' as u8));
        __go_array.push(0 as u8);
        let __go_array: [u8; 5] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *label.borrow_mut() = Some(__go_array);
    }
}


pub fn first(values: Rc<RefCell<Option<Vec<u8>>>>) -> u8 {
    (*values.borrow().as_ref().unwrap())[(0) as usize].clone()
}

fn main() {
    __go_init_all();
    let mut path: Rc<RefCell<Option<[i8; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 1, _>(|_| 0))));
    { let __idx = (0) as usize; let __rhs = ('@' as i8); (*path.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    {
            let __go_print_arg_0 = format!("{}", (*label.borrow().as_ref().unwrap())[(0) as usize].clone());
            let __go_print_arg_1 = format!("{}", (*path.borrow().as_ref().unwrap())[(0) as usize].clone());
            let __go_print_arg_2 = format!("{}", first(Rc::new(RefCell::new(Some(vec![('m' as u8)])))));
            eprintln!("{} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
