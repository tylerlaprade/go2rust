use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn l_e_uint64(b: Arc<Mutex<Option<Vec<u8>>>>) -> u64 {
    let _ = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(7) as usize].clone() };
    return {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = 8;
            let __go_binary_3 = __go_binary_1 << __go_binary_2;
            let __go_binary_4 = __go_binary_0 | __go_binary_3;
            let __go_binary_5 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_6 = 16;
            let __go_binary_7 = __go_binary_5 << __go_binary_6;
            let __go_binary_8 = __go_binary_4 | __go_binary_7;
            let __go_binary_9 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_10 = 24;
            let __go_binary_11 = __go_binary_9 << __go_binary_10;
            let __go_binary_12 = __go_binary_8 | __go_binary_11;
            let __go_binary_13 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_14 = 32;
            let __go_binary_15 = __go_binary_13 << __go_binary_14;
            let __go_binary_16 = __go_binary_12 | __go_binary_15;
            let __go_binary_17 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_18 = 40;
            let __go_binary_19 = __go_binary_17 << __go_binary_18;
            let __go_binary_20 = __go_binary_16 | __go_binary_19;
            let __go_binary_21 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_22 = 48;
            let __go_binary_23 = __go_binary_21 << __go_binary_22;
            let __go_binary_24 = __go_binary_20 | __go_binary_23;
            let __go_binary_25 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_26 = 56;
            let __go_binary_27 = __go_binary_25 << __go_binary_26;
            let __go_binary_28 = __go_binary_24 | __go_binary_27;
            __go_binary_28
        };
}