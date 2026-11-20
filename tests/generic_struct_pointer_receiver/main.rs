include!("__go2rust_helpers.rs");
mod p;
use p::*;

use std::fmt::{Formatter};

pub struct C<T: Any + 'static> {
    pub p: Rc<RefCell<Option<P>>>,
    pub items: Rc<RefCell<Option<[P; 2]>>>,
    pub v: Rc<RefCell<Option<T>>>,
    pub n: Rc<RefCell<Option<i32>>>,
}

impl<T: Any + GoTypeParamFieldClone + 'static> C<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.items.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        let __go_clone_2_0 = <T as GoTypeParamFieldClone>::go_type_param_field_clone(&self.v);
        let __go_clone_3_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
            items: __go_clone_1_0,
            v: __go_clone_2_0,
            n: __go_clone_3_0,
        }
    }
}

impl<T: Any + 'static> Clone for C<T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p.clone(),
            items: self.items.clone(),
            v: self.v.clone(),
            n: self.n.clone(),
        }
    }
}


impl<T: Any + 'static> Default for C<T> {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(P::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default()))));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            p: __go_default_0_0,
            items: __go_default_1_0,
            v: __go_default_2_0,
            n: __go_default_3_0,
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for C<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.items));
        let __go_fmt_2 = format!("{}", (*self.v.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


pub struct Outer<T: Any + 'static> {
    pub r#box: Rc<RefCell<Option<C<T>>>>,
}

impl<T: Any + GoTypeParamFieldClone + 'static> Outer<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#box.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            r#box: __go_clone_0_0,
        }
    }
}

impl<T: Any + 'static> Clone for Outer<T> {
    fn clone(&self) -> Self {
        Self {
            r#box: self.r#box.clone(),
        }
    }
}


impl<T: Any + 'static> Default for Outer<T> {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Default::default())));
        Self {
            r#box: __go_default_0_0,
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for Outer<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#box.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl<T: Any + 'static> C<T> {
    pub fn later_nested(&mut self) -> Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> {
        let mut c_closure_clone = (*self).clone(); Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = (*c_closure_clone.p.borrow().as_ref().unwrap()).x.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>)))
    }

    pub fn later_direct(&mut self) -> Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> {
        let mut c_closure_clone = (*self).clone(); Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = c_closure_clone.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>)))
    }

    pub fn later_items(&mut self) -> Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> {
        let mut c_closure_clone = (*self).clone(); Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = (*c_closure_clone.items.borrow().as_ref().unwrap())[(1) as usize].clone().x.clone(); let __rhs = 5; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>)))
    }

    pub fn self_(&self) -> Rc<RefCell<Option<C<T>>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }
}

impl<T: Any + GoTypeParamFieldClone + 'static> C<T> {
    /// A value receiver is copied even when its body does not otherwise mention
    /// the type-parameter field.
    pub fn bump_value(&self) {
        let mut __self = self.__go_value_clone();
        { let __target = __self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl<T: Any + GoValueClone + GoTypeParamFieldClone + 'static> C<T> {
    /// The implicit receiver copy and the direct type-parameter assignment require
    /// both generated copy traits on T.
    pub fn store_value(&self, value: Rc<RefCell<Option<T>>>) {
        let mut __self = self.__go_value_clone();
        { let new_val = (*value.borrow().as_ref().unwrap()).go_value_clone(); *__self.v.borrow_mut() = Some(new_val); };
    }
}

pub fn copy_c<T: Any + GoTypeParamFieldClone + 'static>(value: Rc<RefCell<Option<C<T>>>>) -> Rc<RefCell<Option<C<T>>>> {
    { let __owned = value.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn store_and_copy<T: Any + GoValueClone + GoTypeParamFieldClone + 'static>(c: Rc<RefCell<Option<C<T>>>>, value: Rc<RefCell<Option<T>>>) -> Rc<RefCell<Option<C<T>>>> {
    { let new_val = (*value.borrow().as_ref().unwrap()).go_value_clone(); *(*c.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    { let __owned = c.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }
}

/// A pointer method of a struct with type parameters can hand on its
/// receiver: a closure over it and a returned receiver are the same struct.
fn main() {
    let mut c = Rc::new(RefCell::new(Some(C::<i32> { ..Default::default() })));
    let mut f = (*c.borrow_mut().as_mut().unwrap()).later_nested();
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", "nested field through a closure".to_string()), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut g = (*c.borrow_mut().as_mut().unwrap()).later_direct();
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = g.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", "direct field through a closure".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut k = (*c.borrow_mut().as_mut().unwrap()).later_items();
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = k.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", "array of structs through a closure".to_string()), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())));

    let mut d = (*c.borrow().as_ref().unwrap()).self_();
    { let new_val = 50; *(*d.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    { let new_val = 60; *(*(*d.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "returned receiver".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut v = Rc::new(RefCell::new(Some({ let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    { let new_val = 7; *(*v.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    { let new_val = 8; *(*(*v.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 9; *(*(*v.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {} {} {} {}", format!("{}", "copy".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*v.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*(*v.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*(*v.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())));

        // The instantiated type parameter is a field value too. Copying the
        // generic struct must copy a concrete struct value rather than sharing its
        // generated field cells.
    let mut originalValuePtr = Rc::new(RefCell::new(Some(C::<P> { v: Rc::new(RefCell::new(Some(P { x: Rc::new(RefCell::new(Some(11))), ..Default::default() }))), ..Default::default() })));
    let mut originalValue = Rc::new(RefCell::new(Some({ let __v = (*originalValuePtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedValue = { let __owned = originalValue.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 12; *(*(*copiedValue.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "type parameter struct copy".to_string()), format!("{}", (*(*(*originalValue.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*(*copiedValue.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

        // A slice is itself a handle in Go: the generic field cell is independent,
        // while the slice's backing storage remains shared by the copied header.
    let mut originalSlicePtr = Rc::new(RefCell::new(Some(C::<Vec<i32>> { v: Rc::new(RefCell::new(Some(vec![13]))), ..Default::default() })));
    let mut originalSlice = Rc::new(RefCell::new(Some({ let __v = (*originalSlicePtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedSlice = { let __owned = originalSlice.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __idx = (0) as usize; let __rhs = 14; (*(*copiedSlice.borrow().as_ref().unwrap()).v.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "type parameter slice elements".to_string()), format!("{}", (*(*originalSlice.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*(*copiedSlice.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()));
    { let new_val = Rc::new(RefCell::new(Some(vec![15]))); (*copiedSlice.borrow_mut().as_mut().unwrap()).v = new_val; };
    println!("{} {} {}", format!("{}", "type parameter slice field".to_string()), format!("{}", (*(*originalSlice.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*(*copiedSlice.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()));

        // Arrays remain Go values when they instantiate the type parameter.
    let mut originalArrayPtr = Rc::new(RefCell::new(Some(C::<[i32; 2]> { v: Rc::new(RefCell::new(Some([16, 17]))), ..Default::default() })));
    let mut originalArray = Rc::new(RefCell::new(Some({ let __v = (*originalArrayPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedArray = { let __owned = originalArray.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __idx = (0) as usize; let __rhs = 18; (*(*copiedArray.borrow().as_ref().unwrap()).v.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "type parameter array copy".to_string()), format!("{}", (*(*originalArray.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*(*copiedArray.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()));

        // Named scalar instantiations implement the generated copy traits, and a
        // generic function returning C[T] propagates the stronger bound.
    let mut originalNumberPtr = Rc::new(RefCell::new(Some(C::<Number> { v: Rc::new(RefCell::new(Some(Number(Rc::new(RefCell::new(Some(19 as i32))))))), ..Default::default() })));
    let mut originalNumber = Rc::new(RefCell::new(Some({ let __v = (*originalNumberPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedNumber = copy_c::<Number>(originalNumber.clone());
    { let new_val = Number(Rc::new(RefCell::new(Some(20 as i32)))); *(*copiedNumber.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "type parameter named scalar".to_string()), format!("{}", (*(*originalNumber.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*copiedNumber.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).clone()));

        // A nested generic struct propagates the field-copy bound through C[T].
    let mut originalOuterPtr = Rc::new(RefCell::new(Some(Outer::<P> { r#box: Rc::new(RefCell::new(Some(C::<P> { v: Rc::new(RefCell::new(Some(P { x: Rc::new(RefCell::new(Some(21))), ..Default::default() }))), ..Default::default() }))), ..Default::default() })));
    let mut originalOuter = Rc::new(RefCell::new(Some({ let __v = (*originalOuterPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedOuter = { let __owned = originalOuter.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 22; *(*(*(*copiedOuter.borrow().as_ref().unwrap()).r#box.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "nested generic struct copy".to_string()), format!("{}", (*(*(*(*originalOuter.borrow().as_ref().unwrap()).r#box.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*(*(*copiedOuter.borrow().as_ref().unwrap()).r#box.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

        // An array containing a slice copies the array value but retains the slice
        // backing storage, as Go does.
    let mut shared = Rc::new(RefCell::new(Some(vec![23])));
    let mut originalSliceArrayPtr = Rc::new(RefCell::new(Some(C::<[Vec<i32>; 1]> { v: Rc::new(RefCell::new(Some([{ let __slice_holder = shared.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))), ..Default::default() })));
    let mut originalSliceArray = Rc::new(RefCell::new(Some({ let __v = (*originalSliceArrayPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedSliceArray = { let __owned = originalSliceArray.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let __rhs = 24; (*(*copiedSliceArray.borrow().as_ref().unwrap()).v.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "type parameter array slice backing".to_string()), format!("{}", (*(*originalSliceArray.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone()), format!("{}", (*(*copiedSliceArray.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone()));

    let mut valueReceiverPtr = Rc::new(RefCell::new(Some(C::<i32> { n: Rc::new(RefCell::new(Some(25))), ..Default::default() })));
    let mut valueReceiver = Rc::new(RefCell::new(Some({ let __v = (*valueReceiverPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    (*valueReceiver.borrow().as_ref().unwrap()).bump_value();
    println!("{} {}", format!("{}", "generic value receiver".to_string()), format!("{}", (*(*valueReceiver.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut combinedBoundPtr = Rc::new(RefCell::new(Some(C::<P> { v: Rc::new(RefCell::new(Some(P { x: Rc::new(RefCell::new(Some(26))), ..Default::default() }))), ..Default::default() })));
    let mut combinedBound = Rc::new(RefCell::new(Some({ let __v = (*combinedBoundPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    (*combinedBound.borrow().as_ref().unwrap()).store_value(Rc::new(RefCell::new(Some(P { x: Rc::new(RefCell::new(Some(27))), ..Default::default() }))));
    println!("{} {}", format!("{}", "combined generic copy bounds".to_string()), format!("{}", (*(*(*combinedBound.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut next = Rc::new(RefCell::new(Some(30)));
    let mut next_closure_clone = next.clone(); let mut originalFuncPtr = Rc::new(RefCell::new(Some(C::<Box<dyn FnMut() -> i32>> { v: Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        { let mut guard = next_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*next_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> i32>))), ..Default::default() })));
    let mut originalFunc = Rc::new(RefCell::new(Some({ let __v = (*originalFuncPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedFunc = { let __owned = originalFunc.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    println!("{} {} {}", format!("{}", "function type argument".to_string()), format!("{}", { let __f_holder = (*originalFunc.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }), format!("{}", { let __f_holder = (*copiedFunc.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }));
    { let new_val = Box::new(move || -> i32 {
        99
    }) as Box<dyn FnMut() -> i32>; *(*copiedFunc.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "function type argument field".to_string()), format!("{}", { let __f_holder = (*originalFunc.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }), format!("{}", { let __f_holder = (*copiedFunc.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }));

    let mut total = Rc::new(RefCell::new(Some(40)));
    let mut total_closure_clone = total.clone(); let mut originalUnaryPtr = Rc::new(RefCell::new(Some(C::<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>> { v: Rc::new(RefCell::new(Some(Box::new(move |value: Rc<RefCell<Option<i32>>>| -> i32 {
        { let __rhs = (*value.borrow().as_ref().unwrap()); let mut guard = total_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return (*total_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>))), ..Default::default() })));
    let mut originalUnary = Rc::new(RefCell::new(Some({ let __v = (*originalUnaryPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut copiedUnary = { let __owned = originalUnary.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    println!("{} {} {}", format!("{}", "function type argument parameter".to_string()), format!("{}", { let __f_holder = (*originalUnary.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(1)))) }), format!("{}", { let __f_holder = (*copiedUnary.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(2)))) }));
    { let new_val = Box::new(move |value: Rc<RefCell<Option<i32>>>| -> i32 {
        (*value.borrow().as_ref().unwrap()) * 10
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>; *(*copiedUnary.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "function type argument parameter field".to_string()), format!("{}", { let __f_holder = (*originalUnary.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(3)))) }), format!("{}", { let __f_holder = (*copiedUnary.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) }));

        // Repeatedly copying the same function field must keep a stable callable
        // target rather than growing a forwarding chain on the source value.
    let mut stableNext = Rc::new(RefCell::new(Some(100)));
    let mut stableNext_closure_clone = stableNext.clone(); let mut stableFuncPtr = Rc::new(RefCell::new(Some(C::<Box<dyn FnMut() -> i32>> { v: Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        { let mut guard = stableNext_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*stableNext_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> i32>))), ..Default::default() })));
    let mut stableFunc = Rc::new(RefCell::new(Some({ let __v = (*stableFuncPtr.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 100000 {
        let mut discarded = { let __owned = stableFunc.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
        let _ = &discarded;
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "function repeated copy".to_string()), format!("{}", { let __f_holder = (*stableFunc.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }));
    { let new_val = Box::new(move || -> i32 {
        200
    }) as Box<dyn FnMut() -> i32>; *(*stableFunc.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    let mut replacementCopy = { let __owned = stableFunc.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    println!("{} {} {}", format!("{}", "function repeated copy replacement".to_string()), format!("{}", { let __f_holder = (*stableFunc.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }), format!("{}", { let __f_holder = (*replacementCopy.borrow().as_ref().unwrap()).v.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }));
}

impl<T: Any + GoTypeParamFieldClone + 'static> GoValueClone for C<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl<T: Any + GoTypeParamFieldClone + 'static> GoTypeParamFieldClone for C<T> {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Rc<RefCell<Option<Self>>>) -> Rc<RefCell<Option<Self>>> {
        let __guard = field.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}


impl<T: Any + GoTypeParamFieldClone + 'static> GoValueClone for Outer<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl<T: Any + GoTypeParamFieldClone + 'static> GoTypeParamFieldClone for Outer<T> {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Rc<RefCell<Option<Self>>>) -> Rc<RefCell<Option<Self>>> {
        let __guard = field.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}
