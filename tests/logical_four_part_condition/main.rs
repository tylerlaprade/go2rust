use std::sync::{Arc, Mutex};
use std::thread;

pub fn compare_and_swap(state: Arc<Mutex<Option<i32>>>, old: Arc<Mutex<Option<i32>>>, next: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as i32; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as i32; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6 as i32; __tmp_x == __tmp_y };
}

pub fn spin(awoke: Arc<Mutex<Option<bool>>>, old: Arc<Mutex<Option<i32>>>) -> bool {
    std::thread::spawn(move || {
        ;
    });
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = !{ let __v = (*awoke.lock().unwrap().as_ref().unwrap()).clone(); __v };
                if __go_cond_2 {
                    let __go_cond_3 = { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y };
                    __go_cond_3
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_4 = { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y };
                __go_cond_4
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_5 = compare_and_swap(
                Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as i32; __tmp_x | __tmp_y })))
            );
            __go_cond_5
        } else {
            false
        }
    } {
        return true;
    }
    false
}

fn main() {
    println!("{}", format!("{}", spin(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(4 as i32))))));
    println!("{}", format!("{}", spin(Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(4 as i32))))));
    println!("{}", format!("{}", spin(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(2 as i32))))));
}