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

use crate::{env_posix::{gogetenv}, symtab::{Frame, Frames, callers_frames}, traceback::{callers_1}};

use std::sync::{Arc, Mutex};

pub const G_O_O_S: &'static str = "darwin";


pub const G_O_A_R_C_H: &'static str = "arm64";


pub(crate) static defaultGOROOT: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static buildVersion: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *defaultGOROOT.lock().unwrap() = Some(String::new());
    *buildVersion.lock().unwrap() = Some(String::new());
}


pub(crate) fn __go_zero_globals() {
    *defaultGOROOT.lock().unwrap() = Some(String::new());
    *buildVersion.lock().unwrap() = Some(String::new());
}


/// Caller reports file and line number information about function invocations on
/// the calling goroutine's stack. The argument skip is the number of stack frames
/// to ascend, with 0 identifying the caller of Caller. (For historical reasons the
/// meaning of skip differs between Caller and [Callers].) The return values report
/// the program counter, the file name (using forward slashes as path separator, even
/// on Windows), and the line number within the file of the corresponding call.
/// The boolean ok is false if it was not possible to recover the information.
pub fn caller(skip: Arc<Mutex<Option<i32>>>) -> (usize, Arc<Mutex<Option<String>>>, i32, bool) {
    let mut pc: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut rpc = Arc::new(Mutex::new(Some(vec![0; (1) as usize])));
    let mut n = callers_1(
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        rpc.clone()
    );
    if { let __tmp_x = n; let __tmp_y = 1; __tmp_x < __tmp_y } {
        return ((*pc.lock().unwrap().as_ref().unwrap()), file.clone(), (*line.lock().unwrap().as_ref().unwrap()), (*ok.lock().unwrap().as_ref().unwrap()));
    }
    let (mut frame, _) = { let __recv = callers_frames(rpc.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).next(); __result };
    return (
        (*(*frame.lock().unwrap().as_ref().unwrap()).p_c.lock().unwrap().as_ref().unwrap()),
        { let __return_value_1 = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); __return_value_1 },
        (*(*frame.lock().unwrap().as_ref().unwrap()).line.lock().unwrap().as_ref().unwrap()),
        { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).p_c.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y }
    );
}

/// Callers fills the slice pc with the return program counters of function invocations
/// on the calling goroutine's stack. The argument skip is the number of stack frames
/// to skip before recording in pc, with 0 identifying the frame for Callers itself and
/// 1 identifying the caller of Callers.
/// It returns the number of entries written to pc.
///
/// To translate these PCs into symbolic information such as function
/// names and line numbers, use [CallersFrames]. CallersFrames accounts
/// for inlined functions and adjusts the return program counters into
/// call program counters. Iterating over the returned slice of PCs
/// directly is discouraged, as is using [FuncForPC] on any of the
/// returned PCs, since these cannot account for inlining or return
/// program counter adjustment.
pub fn callers(skip: Arc<Mutex<Option<i32>>>, pc: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
        // runtime.callers uses pc.array==nil as a signal
        // to print a stack trace. Pick off 0-length pc here
        // so that we don't let a nil pc slice get to it.
    if { let __tmp_x = ((*pc.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
    callers_1(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pc.clone())
}

/// GOROOT returns the root of the Go tree. It uses the
/// GOROOT environment variable, if set at process start,
/// or else the root used during the Go build.
///
/// Deprecated: The root used during the Go build will not be
/// meaningful if the binary is copied to another machine.
/// Use the system path to locate the “go” binary, and use
/// “go env GOROOT” to find its GOROOT.
pub fn g_o_r_o_o_t() -> Arc<Mutex<Option<String>>> {
    let mut s = gogetenv(Arc::new(Mutex::new(Some("GOROOT".to_string()))));
    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    { let __owned = defaultGOROOT.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
