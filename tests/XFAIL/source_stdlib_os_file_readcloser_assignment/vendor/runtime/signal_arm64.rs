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
    panic::{should_push_sigpanic},
    print::{hex},
    runtime2::{g},
    signal_darwin_arm64::{sigctxt},
    signal_unix::{sigpanic},
};

use std::any::Any;
use std::sync::{Arc, Mutex};

impl crate::signal_darwin_arm64::sigctxt {
    ///go:nosplit
    ///go:nowritebarrierrec
    pub fn sigpc(&self) -> usize {
        (*Arc::new(Mutex::new(Some(self.pc() as usize))).lock().unwrap().as_ref().unwrap())
    }

    pub fn setsigpc(&self, x: Arc<Mutex<Option<u64>>>) {
        self.set_pc(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    pub fn sigsp(&self) -> usize {
        (*Arc::new(Mutex::new(Some(self.sp() as usize))).lock().unwrap().as_ref().unwrap())
    }

    pub fn siglr(&self) -> usize {
        (*Arc::new(Mutex::new(Some(self.lr() as usize))).lock().unwrap().as_ref().unwrap())
    }

    /// preparePanic sets up the stack to look like a call to sigpanic.
    pub fn prepare_panic(&self, sig_local: Arc<Mutex<Option<u32>>>, gp: GoPtr<crate::runtime2::g>) {
                // We arrange lr, and pc to pretend the panicking
                // function calls sigpanic directly.
                // Always save LR to stack so that panics in leaf
                // functions are correctly handled. This smashes
                // the stack frame but we're not going back there
                // anyway.
        let mut sp = Arc::new(Mutex::new(Some({ let __tmp_x = self.sp(); let __tmp_y = internal_runtime_sys::STACK_ALIGN as u64; __tmp_x - __tmp_y })));
        self.set_sp(Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { unimplemented!("unsafe.Pointer dereference assignment"); };
                // Make sure a valid frame pointer is saved on the stack so that the
                // frame pointer checks in adjustframe are happy, if they're enabled.
                // Frame pointer unwinding won't visit the sigpanic frame, since
                // sigpanic will save the same frame pointer before calling into a panic
                // function.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        let mut pc = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sigpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if should_push_sigpanic(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(self.lr() as usize)))) {
                // Make it look the like faulting PC called sigpanic.
        self.set_lr(Arc::new(Mutex::new(Some((*pc.lock().unwrap().as_ref().unwrap()) as u64))));
    }
                // Make it look the like faulting PC called sigpanic.
                // In case we are panicking from external C code
        self.set_r28(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(gp.addr()))).lock().unwrap().as_ref().unwrap()) as usize as u64))));
        self.set_pc(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(sigpanic.clone()) as Box<dyn Any + Send + Sync>)))) as u64))));
    }

    pub fn push_call(&self, targetPC: Arc<Mutex<Option<usize>>>, resumePC: Arc<Mutex<Option<usize>>>) {
                // Push the LR to stack, as we'll clobber it in order to
                // push the call. The function being pushed is responsible
                // for restoring the LR and setting the SP back.
                // This extra space is known to gentraceback.
        let mut sp = Arc::new(Mutex::new(Some({ let __tmp_x = self.sp(); let __tmp_y = 16 as u64; __tmp_x - __tmp_y })));
        self.set_sp(Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { unimplemented!("unsafe.Pointer dereference assignment"); };
                // Make sure a valid frame pointer is saved on the stack so that the
                // frame pointer checks in adjustframe are happy, if they're enabled.
                // This is not actually used for unwinding.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
                // Set up PC and LR to pretend the function being signaled
                // calls targetPC at resumePC.
        self.set_lr(Arc::new(Mutex::new(Some((*resumePC.lock().unwrap().as_ref().unwrap()) as u64))));
        self.set_pc(Arc::new(Mutex::new(Some((*targetPC.lock().unwrap().as_ref().unwrap()) as u64))));
    }
}

pub fn dumpregs(c: Arc<Mutex<Option<sigctxt>>>) {
    {
            let __go_print_arg_0 = format!("{}", "r0      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r0(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r1      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r1(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r2      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r2(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r3      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r3(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r4      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r4(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r5      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r5(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r6      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r6(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r7      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r7(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r8      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r8(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r9      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r9(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r10     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r10(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r11     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r11(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r12     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r12(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r13     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r13(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r14     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r14(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r15     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r15(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r16     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r16(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r17     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r17(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r18     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r18(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r19     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r19(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r20     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r20(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r21     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r21(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r22     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r22(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r23     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r23(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r24     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r24(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r25     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r25(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r26     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r26(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r27     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r27(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r28     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r28(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "r29     ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.r29(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "lr      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.lr(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "sp      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sp(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "pc      ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.pc(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    {
            let __go_print_arg_0 = format!("{}", "fault   ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.fault(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
}