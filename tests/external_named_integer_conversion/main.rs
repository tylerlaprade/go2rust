use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn kind() -> Arc<Mutex<Option<types_BasicKind>>> {
    Arc::new(Mutex::new(Some(types::INT)))
}

pub fn dir() -> Arc<Mutex<Option<types_ChanDir>>> {
    Arc::new(Mutex::new(Some(types::SEND_RECV)))
}

pub fn zero_kind() -> Arc<Mutex<Option<types_BasicKind>>> {
    Arc::new(Mutex::new(Some(types_BasicKind(0 as i32))))
}

pub fn pos_from_int(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
    Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32)))))))
}

pub fn int_value() -> i32 {
    1
}

pub fn kind_name() -> Arc<Mutex<Option<String>>> {
    { let _switch_val = { let __v = kind(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (types::INT) {
            return Arc::new(Mutex::new(Some("int".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("other".to_string())));
        }
    }
}

pub fn int_name() -> Arc<Mutex<Option<String>>> {
    { let _switch_val = int_value();
    if _switch_val == (1) {
            return Arc::new(Mutex::new(Some("one".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("other".to_string())));
        }
    }
}

fn main() {
    go_token::__go_init_all();

    if false {
        println!("{} {} {} {} {} {}", format!("{}", (*Arc::new(Mutex::new(Some((*kind().lock().unwrap().as_ref().unwrap()).0 as u32))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some((*dir().lock().unwrap().as_ref().unwrap()).0 as u32))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some((*zero_kind().lock().unwrap().as_ref().unwrap()).0 as u32))).lock().unwrap().as_ref().unwrap())), format!("{}", (*pos_from_int(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*kind_name().lock().unwrap().as_ref().unwrap())), format!("{}", (*int_name().lock().unwrap().as_ref().unwrap())));
    }
    println!("{}", format!("{}", "ok".to_string()));
}