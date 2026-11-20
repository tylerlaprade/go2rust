use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct sink {
    pub text: Arc<Mutex<Option<String>>>,
}

impl sink {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.text.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            text: __go_clone_0_0,
        }
    }
}


impl Default for sink {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            text: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for sink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.text.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl sink {
    pub fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        { let __target = self.text.clone(); (*__target.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = Arc::new(Mutex::new(Some(String::from_utf8((*p.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        (
            (*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32,
            Arc::new(Mutex::new(None))
        )
    }
}

fn main() {
    ::flag::__go_init_all();

    let mut set = flag::new_flag_set(Arc::new(Mutex::new(Some("probe".to_string()))), Arc::new(Mutex::new(Some(flag::r#mod::ErrorHandling(Arc::new(Mutex::new(Some(flag::CONTINUE_ON_ERROR as i32))))))));
    let mut out = Arc::new(Mutex::new(Some(sink { text: Arc::new(Mutex::new(Some(String::new()))) })));
    { let __recv = set.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.set_output(Arc::new(Mutex::new(Some(io_Writer::__go_from(out.clone()))))); __result };
    let mut err = { let __recv = set.clone(); let __recv_ptr: *mut flag::r#mod::FlagSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut flag::r#mod::FlagSet }; let __result = unsafe { &mut *__recv_ptr }.parse(Arc::new(Mutex::new(Some(vec!["-missing".to_string()])))); __result };
    println!("{}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result }));
    print!("{}", format!("{}", (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).text.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
}

impl GoValueClone for sink {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
