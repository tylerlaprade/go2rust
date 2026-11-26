#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct Logger {
    pub out: Arc<Mutex<Option<os_File>>>,
    pub w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
}

impl Logger {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.out.clone();
        let __go_clone_1_0 = self.w.clone();
        Self {
            out: __go_clone_0_0,
            w: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.out.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn pick(verbose: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<os_File>>> {
    if { let __v = (*verbose.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return os::Stdout().clone();
    }
    os::Stderr().clone()
}

fn main() {
    ::io::__go_init_all();

    let mut out: Arc<Mutex<Option<os_File>>> = os::Stdout().clone();
    { let __s = format!("{}\n", format!("{}", "to stdout".to_string())); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    println!("{} {} {} {}", format!("{}", {
        let __left = os::Stdout().clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        !__eq
    }));
    let mut l = Arc::new(Mutex::new(Some(Logger {
        out: pick(Arc::new(Mutex::new(Some(true)))).clone(),
        w: Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))),
        ..Default::default()
    })));
    { let __s = format!("logger {}\n", 1); (*(*l.lock().unwrap().as_ref().unwrap()).out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "writer".to_string())); (*(*l.lock().unwrap().as_ref().unwrap()).w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __go_print_arg_1 = Arc::new(Mutex::new(Some({
    let __left = pick(Arc::new(Mutex::new(Some(false))));
    let __right = os::Stderr().clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}))); let __go_print_arg_2 = Arc::new(Mutex::new(Some({
    let __left = pick(Arc::new(Mutex::new(Some(false))));
    let __right = out.clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}))); let __go_print_value = println!("{} {} {}", format!("{}", {
    let __left = (*l.lock().unwrap().as_ref().unwrap()).out.clone();
    let __right = os::Stdout().clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}), format!("{}", { let __v = (*__go_print_arg_1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_2.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut errOut: Arc<Mutex<Option<os_File>>> = Arc::new(Mutex::new(None));
    { let new_val = os::Stderr().clone(); errOut = new_val; };
    println!("{} {}", format!("{}", {
        let __left = errOut.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", { let __nil_result = (*errOut.lock().unwrap()).is_some(); __nil_result }));
    { let __std_stream = (*os::Stdout().lock().unwrap().as_ref().unwrap()).clone(); __std_stream }.write_string(Arc::new(Mutex::new(Some(format!("{}{}", (*{ let __std_stream = (*os::Stdout().lock().unwrap().as_ref().unwrap()).clone(); __std_stream }.name().lock().unwrap().as_ref().unwrap()), "\n".to_string())))));
    {
        let (mut f, mut err) = os::open("/dev/null".to_string());;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            println!("{}", format!("{}", (*{ let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));;
            { let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };;
        }
    }
    let mut files = Arc::new(Mutex::new(Some(vec![{ let __field = os::Stdout().clone(); __field }, { let __field = os::Stderr().clone(); __field }])));
    { let __range_holder = files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        { let __go_print_arg_3 = { let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }; let __go_print_value = println!("{} {} {}", format!("{}", i), format!("{}", {
            let __left = f.clone();
            let __right = os::Stdout().clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            __eq
        }), format!("{}", { let __v = (*__go_print_arg_3.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    } }
}

impl GoValueClone for Logger {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
