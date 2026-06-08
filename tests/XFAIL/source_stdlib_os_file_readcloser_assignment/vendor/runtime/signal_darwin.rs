use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    runtime2::{__SIG_DEFAULT, __SIG_IGN, __SIG_KILL, __SIG_NOTIFY, __SIG_PANIC, __SIG_THROW, __SIG_UNBLOCK},
    signal_unix::{sigTabT},
};

use std::sync::{Arc, Mutex};

pub(crate) static sigtable: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[crate::signal_unix::sigTabT; 32]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *sigtable.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    {
        let mut __go_array = Vec::<crate::signal_unix::sigTabT>::with_capacity(32);
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(0 as i32))), name: Arc::new(Mutex::new(Some("SIGNONE: no trap".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_KILL; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGHUP: terminal line hangup".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_KILL; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGINT: interrupt".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_THROW; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGQUIT: quit".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_THROW; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGILL: illegal instruction".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_THROW; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTRAP: trace trap".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_THROW; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGABRT: abort".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_THROW as i32))), name: Arc::new(Mutex::new(Some("SIGEMT: emulate instruction executed".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_PANIC; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGFPE: floating-point exception".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(0 as i32))), name: Arc::new(Mutex::new(Some("SIGKILL: kill".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_PANIC; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGBUS: bus error".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_PANIC; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGSEGV: segmentation violation".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_THROW as i32))), name: Arc::new(Mutex::new(Some("SIGSYS: bad system call".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGPIPE: write to broken pipe".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGALRM: alarm clock".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_KILL; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTERM: termination".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGURG: urgent condition on socket".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(0 as i32))), name: Arc::new(Mutex::new(Some("SIGSTOP: stop".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTSTP: keyboard stop".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGCONT: continue after stop".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGCHLD: child status has changed".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTTIN: background read from tty".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTTOU: background write to tty".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGIO: i/o now possible".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGXCPU: cpu limit exceeded".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGXFSZ: file size limit exceeded".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGVTALRM: virtual alarm clock".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGPROF: profiling alarm clock".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGWINCH: window size change".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGINFO: status request from keyboard".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGUSR1: user-defined signal 1".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGUSR2: user-defined signal 2".to_string()))), ..Default::default() });
        let __go_array: [crate::signal_unix::sigTabT; 32] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *sigtable.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *sigtable.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
}


pub(crate) fn __go_init_order_67() {
    {
        let mut __go_array = Vec::<crate::signal_unix::sigTabT>::with_capacity(32);
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(0 as i32))), name: Arc::new(Mutex::new(Some("SIGNONE: no trap".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_KILL; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGHUP: terminal line hangup".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_KILL; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGINT: interrupt".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_THROW; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGQUIT: quit".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_THROW; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGILL: illegal instruction".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_THROW; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTRAP: trace trap".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_THROW; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGABRT: abort".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_THROW as i32))), name: Arc::new(Mutex::new(Some("SIGEMT: emulate instruction executed".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_PANIC; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGFPE: floating-point exception".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(0 as i32))), name: Arc::new(Mutex::new(Some("SIGKILL: kill".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_PANIC; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGBUS: bus error".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_PANIC; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGSEGV: segmentation violation".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_THROW as i32))), name: Arc::new(Mutex::new(Some("SIGSYS: bad system call".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGPIPE: write to broken pipe".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGALRM: alarm clock".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_KILL; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTERM: termination".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGURG: urgent condition on socket".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(0 as i32))), name: Arc::new(Mutex::new(Some("SIGSTOP: stop".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTSTP: keyboard stop".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGCONT: continue after stop".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGCHLD: child status has changed".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTTIN: background read from tty".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_DEFAULT; __tmp_x + __tmp_y }; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGTTOU: background write to tty".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGIO: i/o now possible".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGXCPU: cpu limit exceeded".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGXFSZ: file size limit exceeded".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGVTALRM: virtual alarm clock".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_UNBLOCK; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGPROF: profiling alarm clock".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGWINCH: window size change".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some({ let __tmp_x = __SIG_NOTIFY; let __tmp_y = __SIG_IGN; __tmp_x + __tmp_y } as i32))), name: Arc::new(Mutex::new(Some("SIGINFO: status request from keyboard".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGUSR1: user-defined signal 1".to_string()))), ..Default::default() });
        __go_array.push(crate::signal_unix::sigTabT { flags: Arc::new(Mutex::new(Some(__SIG_NOTIFY as i32))), name: Arc::new(Mutex::new(Some("SIGUSR2: user-defined signal 2".to_string()))), ..Default::default() });
        let __go_array: [crate::signal_unix::sigTabT; 32] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *sigtable.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
