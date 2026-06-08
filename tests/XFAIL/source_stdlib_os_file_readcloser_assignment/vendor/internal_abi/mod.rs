use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::{abi_arm64::{FLOAT_ARG_REGS, INT_ARG_REGS}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// RegArgs is a struct that has space for each argument
/// and return value register on the current architecture.
///
/// Assembly code knows the layout of the first two fields
/// of RegArgs.
///
/// RegArgs also contains additional space to hold pointers
/// when it may not be safe to keep them only in the integer
/// register space otherwise.
#[derive(Debug, Clone)]
pub struct RegArgs {
    pub ints: Arc<Mutex<Option<[usize; 16]>>>,
    pub floats: Arc<Mutex<Option<[u64; 16]>>>,
    pub ptrs: Arc<Mutex<Option<[usize; 16]>>>,
    pub return_is_ptr: Arc<Mutex<Option<IntArgRegBitmap>>>,
}

impl RegArgs {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.ints.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.floats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.ptrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.return_is_ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            ints: __go_clone_0_0,
            floats: __go_clone_1_0,
            ptrs: __go_clone_2_0,
            return_is_ptr: __go_clone_3_0,
        }
    }
}


impl Default for RegArgs {
    fn default() -> Self {
        Self { ints: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), floats: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), ptrs: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), return_is_ptr: Arc::new(Mutex::new(Some(IntArgRegBitmap(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))) }
    }
}

impl std::fmt::Display for RegArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.ints));
        let __go_fmt_1 = format!("{}", format_slice(&self.floats));
        let __go_fmt_2 = format!("{}", format_slice(&self.ptrs));
        let __go_fmt_3 = format!("{}", (*self.return_is_ptr.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for RegArgs {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Ints") {
            out.ints = <Arc<Mutex<Option<[usize; 16]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Floats") {
            out.floats = <Arc<Mutex<Option<[u64; 16]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// IntArgRegBitmap is a bitmap large enough to hold one bit per
/// integer argument/return register.
#[derive(Debug, Clone)]
pub struct IntArgRegBitmap(pub Arc<Mutex<Option<[u8; 2]>>>);

impl Default for IntArgRegBitmap {
    fn default() -> Self {
        IntArgRegBitmap(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Display for IntArgRegBitmap {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl RegArgs {
    pub fn dump(&self) {
        eprint!("{}", format!("{}", "Ints:".to_string()));
        { let __range_holder = self.ints.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        eprint!("{}{}", format!("{}", " ".to_string()), format!("{}", x));
    } }
        eprintln!();
        eprint!("{}", format!("{}", "Floats:".to_string()));
        { let __range_holder = self.floats.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        eprint!("{}{}", format!("{}", " ".to_string()), format!("{}", x));
    } }
        eprintln!();
        eprint!("{}", format!("{}", "Ptrs:".to_string()));
        { let __range_holder = self.ptrs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        eprint!("{}{}", format!("{}", " ".to_string()), format!("{}", x));
    } }
        eprintln!();
    }

    /// IntRegArgAddr returns a pointer inside of r.Ints[reg] that is appropriately
    /// offset for an argument of size argSize.
    ///
    /// argSize must be non-zero, fit in a register, and a power-of-two.
    ///
    /// This method is a helper for dealing with the endianness of different CPU
    /// architectures, since sub-word-sized arguments in big endian architectures
    /// need to be "aligned" to the upper edge of the register to be interpreted
    /// by the CPU correctly.
    pub fn int_reg_arg_addr(&self, reg: Arc<Mutex<Option<i32>>>, argSize: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
        if { let __tmp_x = { let __v = (*argSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*argSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*argSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*argSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("invalid argSize".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut offset = Arc::new(Mutex::new(Some(0 as usize)));
        if internal_goarch::BIG_ENDIAN {
        { let new_val = { let __tmp_x = internal_goarch::PTR_SIZE as usize; let __tmp_y = { let __v = (*argSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *offset.lock().unwrap() = Some(new_val); };
    }
        return Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.ints.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __v = (*reg.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    }
}

impl IntArgRegBitmap {
    /// Set sets the i'th bit of the bitmap to 1.
    pub fn set(&mut self, i: Arc<Mutex<Option<i32>>>) {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
    }

    /// Get returns whether the i'th bit of the bitmap is set.
    ///
    /// nosplit because it's called in extremely sensitive contexts, like
    /// on the reflectcall return path.
    ///
    ///go:nosplit
    pub fn get(&self, i: Arc<Mutex<Option<i32>>>) -> bool {
        return { let __tmp_x = { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }
}

impl GoValueClone for RegArgs {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
