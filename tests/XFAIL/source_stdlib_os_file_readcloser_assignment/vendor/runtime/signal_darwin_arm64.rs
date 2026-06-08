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

use crate::{defs_darwin_arm64::{__S_I_G_T_R_A_P, mcontext64, regs64, siginfo, ucontext}, os_darwin::{__S_I__U_S_E_R}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct sigctxt {
    pub info: Arc<Mutex<Option<siginfo>>>,
    pub ctxt: Arc<Mutex<Option<usize>>>,
}

impl sigctxt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.info.clone();
        let __go_clone_1_0 = { let __guard = self.ctxt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            info: __go_clone_0_0,
            ctxt: __go_clone_1_0,
        }
    }
}


impl Default for sigctxt {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            info: __go_default_0_0,
            ctxt: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for sigctxt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.info.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.ctxt.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for sigctxt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl sigctxt {
    ///go:nosplit
    ///go:nowritebarrierrec
    pub fn regs(&self) -> Arc<Mutex<Option<crate::defs_darwin_arm64::regs64>>> {
        (*(*Arc::new(Mutex::new({ let __ptr = self.ctxt.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<ucontext>(unimplemented!("unsafe.Pointer conversion to ucontext")) } })).lock().unwrap().as_ref().unwrap()).uc_mcontext.lock().unwrap().as_ref().unwrap()).ss.clone()
    }

    pub fn r0(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }
    }

    pub fn r1(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }
    }

    pub fn r2(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }
    }

    pub fn r3(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() }
    }

    pub fn r4(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() }
    }

    pub fn r5(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(5) as usize].clone() }
    }

    pub fn r6(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(6) as usize].clone() }
    }

    pub fn r7(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(7) as usize].clone() }
    }

    pub fn r8(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(8) as usize].clone() }
    }

    pub fn r9(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(9) as usize].clone() }
    }

    pub fn r10(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(10) as usize].clone() }
    }

    pub fn r11(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(11) as usize].clone() }
    }

    pub fn r12(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(12) as usize].clone() }
    }

    pub fn r13(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(13) as usize].clone() }
    }

    pub fn r14(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(14) as usize].clone() }
    }

    pub fn r15(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(15) as usize].clone() }
    }

    pub fn r16(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(16) as usize].clone() }
    }

    pub fn r17(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(17) as usize].clone() }
    }

    pub fn r18(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(18) as usize].clone() }
    }

    pub fn r19(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(19) as usize].clone() }
    }

    pub fn r20(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(20) as usize].clone() }
    }

    pub fn r21(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(21) as usize].clone() }
    }

    pub fn r22(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(22) as usize].clone() }
    }

    pub fn r23(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(23) as usize].clone() }
    }

    pub fn r24(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(24) as usize].clone() }
    }

    pub fn r25(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(25) as usize].clone() }
    }

    pub fn r26(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(26) as usize].clone() }
    }

    pub fn r27(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(27) as usize].clone() }
    }

    pub fn r28(&self) -> u64 {
        { let __seq = { let __seq_holder = (*self.regs().lock().unwrap().as_ref().unwrap()).x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(28) as usize].clone() }
    }

    pub fn r29(&self) -> u64 {
        return (*(*self.regs().lock().unwrap().as_ref().unwrap()).fp.lock().unwrap().as_ref().unwrap());
    }

    pub fn lr(&self) -> u64 {
        return (*(*self.regs().lock().unwrap().as_ref().unwrap()).lr.lock().unwrap().as_ref().unwrap());
    }

    pub fn sp(&self) -> u64 {
        return (*(*self.regs().lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap());
    }

    ///go:nosplit
    ///go:nowritebarrierrec
    pub fn pc(&self) -> u64 {
        return (*(*self.regs().lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
    }

    pub fn fault(&self) -> usize {
        (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*self.info.lock().unwrap().as_ref().unwrap()).si_addr.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap())
    }

    pub fn sigcode(&self) -> u64 {
        (*Arc::new(Mutex::new(Some({ let __selector_holder = (*self.info.lock().unwrap().as_ref().unwrap()).si_code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap())
    }

    pub fn sigaddr(&self) -> u64 {
        (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*self.info.lock().unwrap().as_ref().unwrap()).si_addr.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize as u64))).lock().unwrap().as_ref().unwrap())
    }

    pub fn set_pc(&self, x: Arc<Mutex<Option<u64>>>) {
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *(*self.regs().lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    }

    pub fn set_sp(&self, x: Arc<Mutex<Option<u64>>>) {
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *(*self.regs().lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
    }

    pub fn set_lr(&self, x: Arc<Mutex<Option<u64>>>) {
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *(*self.regs().lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    }

    pub fn set_r28(&self, x: Arc<Mutex<Option<u64>>>) {
        (*(*self.regs().lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_mut().unwrap())[(28) as usize] = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    pub fn set_sigcode(&mut self, x: Arc<Mutex<Option<u64>>>) {
        { let new_val = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.info.lock().unwrap().as_ref().unwrap()).si_code.lock().unwrap() = __moved_val; };
    }

    pub fn set_sigaddr(&mut self, x: Arc<Mutex<Option<u64>>>) {
        { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone(); (*self.info.lock().unwrap().as_mut().unwrap()).si_addr = new_val; };
    }

    ///go:nosplit
    pub fn fixsigcode(&mut self, sig_local: Arc<Mutex<Option<u32>>>) {
        { let _switch_val = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__S_I_G_T_R_A_P as u32) {
                        // OS X sets c.sigcode() == TRAP_BRKPT unconditionally for all SIGTRAPs,
                        // leaving no way to distinguish a breakpoint-induced SIGTRAP
                        // from an asynchronous signal SIGTRAP.
                        // They all look breakpoint-induced by default.
                        // Try looking at the code to see if it's a breakpoint.
                        // The assumption is that we're very unlikely to get an
                        // asynchronous SIGTRAP at just the moment that the
                        // PC started to point at unmapped memory.
            let mut pc = Arc::new(Mutex::new(Some(self.pc() as usize)));
                        // OS X will leave the pc just after the instruction.
            let mut code: GoPtr<u32> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as usize; __tmp_x - __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            if { let __tmp_x = { let __ptr_value = code.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = 0xd4200000 as u32; __tmp_x != __tmp_y } {
                // SIGTRAP on something other than breakpoint.
        self.set_sigcode(Arc::new(Mutex::new(Some(__S_I__U_S_E_R as u64))));
    }
        }
    }
    }
}

impl GoValueClone for sigctxt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
