use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
use crate::call::*;
use crate::chan::*;
use crate::check::*;
use crate::r#const::*;
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
use crate::format::*;
use crate::gccgosizes::*;
use crate::gcsizes::*;
use crate::index::*;
use crate::infer::*;
use crate::initorder::*;
use crate::instantiate::*;
use crate::interface::*;
use crate::iter::*;
use crate::labels::*;
use crate::literals::*;
use crate::lookup::*;
use crate::map::*;
use crate::methodset::*;
use crate::mono::*;
use crate::named::*;
use crate::object::*;
use crate::objset::*;
use crate::operand::*;
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
use crate::resolver::*;
use crate::r#return::*;
use crate::scope::*;
use crate::scope2::*;
use crate::selection::*;
use crate::signature::*;
use crate::slice::*;
use crate::stmt::*;
use crate::r#struct::*;
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typelists::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// Sizes defines the sizing functions for package unsafe.
pub trait Sizes: std::fmt::Display + Any {
    fn __go_clone_box_sizes(&self) -> Box<dyn Sizes + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_sizes(&self, other: &(dyn Sizes + Send + Sync)) -> bool;
    fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64;
    fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>>;
    fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64;
}

impl Clone for Box<dyn Sizes + Send + Sync> {
    fn clone(&self) -> Self {
        Sizes::__go_clone_box_sizes(self.as_ref())
    }
}

/// StdSizes is a convenience type for creating commonly used Sizes.
/// It makes the following simplifying assumptions:
///
///   - The size of explicitly sized basic types (int16, etc.) is the
///     specified size.
///   - The size of strings and interfaces is 2*WordSize.
///   - The size of slices is 3*WordSize.
///   - The size of an array of n elements corresponds to the size of
///     a struct of n consecutive fields of the array's element type.
///   - The size of a struct is the offset of the last field plus that
///     field's size. As with all element types, if the struct is used
///     in an array its size must first be aligned to a multiple of the
///     struct's alignment.
///   - All other types have size WordSize.
///   - Arrays and structs are aligned per spec definition; all other
///     types are naturally aligned with a maximum alignment MaxAlign.
///
/// *StdSizes implements Sizes.
#[derive(Debug, Clone)]
pub struct StdSizes {
    pub word_size: Arc<Mutex<Option<i64>>>,
    pub max_align: Arc<Mutex<Option<i64>>>,
}

impl StdSizes {
    pub fn __go_value_clone(&self) -> Self {
        Self { word_size: { let __guard = self.word_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, max_align: { let __guard = self.max_align.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for StdSizes {
    fn default() -> Self {
        Self { word_size: Arc::new(Mutex::new(Some(0))), max_align: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for StdSizes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.word_size.lock().unwrap().as_ref().unwrap()), (*self.max_align.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for StdSizes {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("WordSize") {
            out.word_size = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("MaxAlign") {
            out.max_align = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<go_ast::r#mod::Ident>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), ptr: { let __guard = self.ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: self.recv.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { obj: Arc::new(Mutex::new(None)), ptr: Arc::new(Mutex::new(Some(false))), recv: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.obj.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ptr.lock().unwrap().as_ref().unwrap()), { let __guard = self.recv.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static basicSizes: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 17]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcArchSizes: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<crate::gcsizes::gcSizes>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stdSizes: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Sizes + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *basicSizes.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *gcArchSizes.lock().unwrap() = Some(BTreeMap::new());
    *stdSizes.lock().unwrap() = None;
    *basicSizes.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0, 1 as u8, 0, 1 as u8, 2 as u8, 4 as u8, 8 as u8, 0, 1 as u8, 2 as u8, 4 as u8, 8 as u8, 0, 4 as u8, 8 as u8, 8 as u8, 16 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
    {
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<crate::gcsizes::gcSizes>>>>::new();
        __go_map.insert("386".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("amd64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("amd64p32".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("arm".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("arm64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("loong64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mips".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mipsle".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mips64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mips64le".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("ppc64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("ppc64le".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("riscv64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("s390x".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("sparc64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("wasm".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        *gcArchSizes.lock().unwrap() = Some(__go_map);
    }
    *stdSizes.lock().unwrap() = Some((*sizes_for(Arc::new(Mutex::new(Some("gc".to_string()))), Arc::new(Mutex::new(Some("amd64".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *basicSizes.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *gcArchSizes.lock().unwrap() = Some(BTreeMap::new());
    *stdSizes.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_7() {
    *basicSizes.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0, 1 as u8, 0, 1 as u8, 2 as u8, 4 as u8, 8 as u8, 0, 1 as u8, 2 as u8, 4 as u8, 8 as u8, 0, 4 as u8, 8 as u8, 8 as u8, 16 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_8() {
    {
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<crate::gcsizes::gcSizes>>>>::new();
        __go_map.insert("386".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("amd64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("amd64p32".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("arm".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("arm64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("loong64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mips".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mipsle".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mips64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("mips64le".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("ppc64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("ppc64le".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("riscv64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("s390x".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("sparc64".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        __go_map.insert("wasm".to_string(), Arc::new(Mutex::new(Some(crate::gcsizes::gcSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        *gcArchSizes.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_init_order_9() {
    *stdSizes.lock().unwrap() = Some((*sizes_for(Arc::new(Mutex::new(Some("gc".to_string()))), Arc::new(Mutex::new(Some("amd64".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


impl StdSizes {
    pub fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*result_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x >= __tmp_y }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
                        // For arrays and structs, alignment is defined in terms
                        // of alignment of the elements and fields, respectively.
            {
    let _ts_subject = under(T.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        { let new_val = self.alignof({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        if { let __tmp_x = (({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && __is_sync_atomic_align64(T.clone()) {
        {
        { let new_val = 8 as i64; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
        let mut max = Arc::new(Mutex::new(Some(1 as i64)));;
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        {
        let mut a = self.alignof({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });;
        if { let __tmp_x = a; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = a; *max.lock().unwrap() = Some(new_val); };;
        }
    }
    } };
        {
        { let new_val = max.lock().unwrap().as_ref().unwrap().clone(); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_subject.clone();
        assert(Arc::new(Mutex::new(Some(!is_type_param(T.clone())))));;
        {
        { let new_val = { let __selector_holder = self.word_size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if { let __tmp_x = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(((*(*{ let __recv = t.clone(); let __recv_ptr: *const crate::basic::Basic = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::basic::Basic }; let __result = unsafe { &*__recv_ptr }.info(); __result }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & IS_STRING as i32))))); let __tmp_y = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        {
        { let new_val = { let __selector_holder = self.word_size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
                        // spec: "For a variable x of array type: unsafe.Alignof(x)
                        // is the same as unsafe.Alignof(x[0]), but at least 1."
                        // Special case: sync/atomic.align64 is an
                        // empty struct we recognize as a signal that
                        // the struct it contains must be
                        // 64-bit-aligned.
                        //
                        // This logic is equivalent to the logic in
                        // cmd/compile/internal/types/size.go:calcStructOffset
                        // spec: "For a variable x of struct type: unsafe.Alignof(x)
                        // is the largest of the values unsafe.Alignof(x.f) for each
                        // field f of x, but at least 1."
                        // Multiword data structures are effectively structs
                        // in which each element has size WordSize.
                        // Type parameters lead to variable sizes/alignments;
                        // StdSizes.Alignof won't be called for them.
                        // Strings are like slices and interfaces.
            let mut a = self.sizeof(T.clone());
                        // spec: "For a variable x of any type: unsafe.Alignof(x) is at least 1."
            if { let __tmp_x = a; let __tmp_y = 1 as i64; __tmp_x < __tmp_y } {
        {
        { let new_val = 1 as i64; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                        // complex{64,128} are aligned like [2]float{32,64}.
            if is_complex(T.clone()) {
        { let __rhs = 2 as i64; a = a / __rhs; };
    }
            if { let __tmp_x = a; let __tmp_y = (*self.max_align.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
        { let new_val = { let __selector_holder = self.max_align.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
            {
        { let new_val = a; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                (*result.lock().unwrap().as_ref().unwrap())
            }
        }
    }

    pub fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        let mut offsets = Arc::new(Mutex::new(Some(vec![0; ((*fields.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        let mut offs: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        if { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // all remaining offsets are too large
        (*offsets.lock().unwrap().as_mut().unwrap())[(i) as usize] = -1 as i64;
        continue
    }
                // all remaining offsets are too large
                // offs >= 0
        let mut a = self.alignof({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });
        { let new_val = align(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(a)))); *offs.lock().unwrap() = Some(new_val); };
        (*offsets.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v };
        {
        let mut d = self.sizeof({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });;
        if { let __tmp_x = d; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
            { let __rhs = d; let mut guard = offs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
        } else {
            { let new_val = -1 as i64; *offs.lock().unwrap() = Some(new_val); };;
        }
    }
    } }
                // all remaining offsets are too large
                // offs >= 0
                // possibly < 0 if align overflows
                // ok to overflow to < 0
                // f.typ or offs is too large
        return offsets.clone();
    }

    pub fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        {
    let _ts_subject = under(T.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some(is_typed(T.clone())))));;
        let mut k = Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = ((*Arc::new(Mutex::new(Some((*{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 17; __tmp_x < __tmp_y } {
        {
        let mut s = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = basicSizes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));;
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x > __tmp_y } {
            return (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap());;
        }
    }
    };
        if { let __tmp_x = (*k.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(STRING as i32)))); __tmp_x == __tmp_y } {
        return { let __tmp_x = (*self.word_size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as i64; __tmp_x * __tmp_y };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        let mut n = Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return 0;
    };
        let mut esize = self.sizeof({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });;
        if { let __tmp_x = esize; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    };
        if { let __tmp_x = esize; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        return 0;
    };
        let mut a = self.alignof({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });;
        let mut ea = align(Arc::new(Mutex::new(Some(esize))), Arc::new(Mutex::new(Some(a))));;
        if { let __tmp_x = ea; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    };
        let mut n1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y })));;
        const maxInt64: u64 = (1 << 63) - 1;
;
        if { let __tmp_x = { let __v = (*n1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = ea; let __tmp_y = { let __tmp_x = maxInt64 as i64; let __tmp_y = { let __v = (*n1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; __tmp_x > __tmp_y } {
        return -(1);
    };
        return { let __tmp_x = { let __tmp_x = ea; let __tmp_y = { let __v = (*n1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = esize; __tmp_x + __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        return { let __tmp_x = (*self.word_size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as i64; __tmp_x * __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        let mut n = { let __recv = t.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result };;
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    };
        let mut offsets = self.offsetsof({ let __field = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); __field });;
        let mut offs = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = offsets.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = n; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));;
        let mut size = self.sizeof({ let __field = (*{ let __seq = { let __seq_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = n; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone(); __field });;
        if { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = size; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    };
        return { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = size; __tmp_x + __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some(!is_type_param(T.clone())))));;
        return { let __tmp_x = (*self.word_size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as i64; __tmp_x * __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
                // n > 0
                // element too large
                // 0-size element
                // esize > 0
                // possibly < 0 if align overflows
                // ea >= 1
                // n1 >= 0
                // Final size is ea*n1 + esize; and size must be <= maxInt64.
                // ea*n1 overflows
                // may still overflow to < 0 which is ok
                // type too large
                // may overflow to < 0 which is ok
                // Type parameters lead to variable sizes/alignments;
                // StdSizes.Sizeof won't be called for them.
        return (*self.word_size.lock().unwrap().as_ref().unwrap());
    }
}

impl Sizes for StdSizes {
    fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        StdSizes::alignof(self, T)
    }
    fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        StdSizes::offsetsof(self, fields)
    }
    fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        StdSizes::sizeof(self, T)
    }
    fn __go_clone_box_sizes(&self) -> Box<dyn Sizes + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sizes + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sizes(&self, other: &(dyn Sizes + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StdSizes>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct StdSizesPtr(pub Arc<Mutex<Option<StdSizes>>>);

impl std::fmt::Display for StdSizesPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Sizes for StdSizesPtr {
    fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        StdSizes::alignof(__recv, T)
    }
    fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        StdSizes::offsetsof(__recv, fields)
    }
    fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        StdSizes::sizeof(__recv, T)
    }
    fn __go_clone_box_sizes(&self) -> Box<dyn Sizes + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sizes + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sizes(&self, other: &(dyn Sizes + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StdSizesPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl crate::api::Config {
    pub fn alignof(&self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut f = Arc::new(Mutex::new(Some({ let mut __recv = (*stdSizes.lock().unwrap().as_ref().unwrap()).clone(); Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> i64 { __recv.alignof(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> })));
        if { let __iface_handle = { let __field = self.sizes.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = { let __selector_holder = self.sizes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> i64 { __recv.alignof(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }))); f = new_val; };
    }
        {
        let mut a = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(T.clone()) };;
        if { let __tmp_x = a; let __tmp_y = 1 as i64; __tmp_x >= __tmp_y } {
            return a;;
        }
    }
        std::panic::panic_any(Box::new("implementation of alignof returned an alignment < 1".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn offsetsof(&self, T: Arc<Mutex<Option<Struct>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        let mut offsets: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = { let __recv = T.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
                // compute offsets on demand
        let mut f = Arc::new(Mutex::new(Some({ let mut __recv = (*stdSizes.lock().unwrap().as_ref().unwrap()).clone(); Box::new(move |__arg0: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>| -> Arc<Mutex<Option<Vec<i64>>>> { __recv.offsetsof(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> + Send + Sync> })));
        if { let __iface_handle = { let __field = self.sizes.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = { let __selector_holder = self.sizes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; Box::new(move |__arg0: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>| -> Arc<Mutex<Option<Vec<i64>>>> { __recv.offsetsof(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> + Send + Sync> }))); f = new_val; };
    }
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*T.lock().unwrap().as_ref().unwrap()).fields.clone(); __field }) }; offsets = new_val; };
                // sanity checks
        if { let __tmp_x = ((*offsets.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __recv = T.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result } as i32); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("implementation of offsetsof returned the wrong number of offsets".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }
                // compute offsets on demand
                // sanity checks
        return offsets.clone();
    }

    /// offsetof returns the offset of the field specified via
    /// the index sequence relative to T. All embedded fields
    /// must be structs (rather than pointers to structs).
    /// If the offset is too large (because T is too large),
    /// the result is negative.
    pub fn offsetof(&self, mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, index: Arc<Mutex<Option<Vec<i32>>>>) -> i64 {
        let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(T.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        let mut offs: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = index.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for i in __range_values.iter().copied() {
        let mut s = ({
        let val = under(T.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::r#struct::StructPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        let mut d = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.offsetsof(s.clone()).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })));
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    }
        { let __rhs = (*d.lock().unwrap().as_ref().unwrap()); let mut guard = offs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    }
        { let __iface_handle = { let __field = (*{ let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *T.lock().unwrap() = __iface_value; };
    } }
        return { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// sizeof returns the size of T.
    /// If T is too large, the result is negative.
    pub fn sizeof(&self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut f = Arc::new(Mutex::new(Some({ let mut __recv = (*stdSizes.lock().unwrap().as_ref().unwrap()).clone(); Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> i64 { __recv.sizeof(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> })));
        if { let __iface_handle = { let __field = self.sizes.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = { let __selector_holder = self.sizes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> i64 { __recv.sizeof(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }))); f = new_val; };
    }
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(T.clone()) };
    }
}

pub fn __is_sync_atomic_align64(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let mut named = as_named(T.clone());
    if { let __nil_result = (*named.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
    let mut obj = { let __recv = named.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.obj(); __result };
    return { let __tmp_x = (*{ let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "align64".to_string(); __tmp_x == __tmp_y } && { let __nil_result = (*{ let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pkg(); __result }.lock().unwrap()).is_some(); __nil_result } && ({ let __tmp_x = (*{ let __recv = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pkg(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).path(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "sync/atomic".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __recv = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pkg(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).path(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "internal/runtime/atomic".to_string(); __tmp_x == __tmp_y });
}

/// SizesFor returns the Sizes used by a compiler for an architecture.
/// The result is nil if a compiler/architecture pair is not known.
///
/// Supported architectures for compiler "gc":
/// "386", "amd64", "amd64p32", "arm", "arm64", "loong64", "mips", "mipsle",
/// "mips64", "mips64le", "ppc64", "ppc64le", "riscv64", "s390x", "sparc64", "wasm".
pub fn sizes_for(compiler: Arc<Mutex<Option<String>>>, arch: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Sizes + Send + Sync>>>> {
    { let _switch_val = (*compiler.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("gc".to_string()) {
            {
        let mut s = gc_sizes_for(Arc::new(Mutex::new(Some({ let __arg_holder = compiler.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = arch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
            return Arc::new(Mutex::new(Some(Box::new(crate::gcsizes::gcSizesPtr(s.clone())) as Box<dyn Sizes + Send + Sync>))).clone();;
        }
    }
        } else if _switch_val == ("gccgo".to_string()) {
            {
        let (mut s, mut ok) = { let __map = { let __map_holder = gccgoArchSizes.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*arch.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(StdSizesPtr(s.clone())) as Box<dyn Sizes + Send + Sync>))).clone();;
        }
    }
        }
    }
    return Arc::new(Mutex::new(None));
}

/// align returns the smallest y >= x such that y % a == 0.
/// a must be within 1 and 8 and it must be a power of 2.
/// The result may be negative due to overflow.
pub fn align(x: Arc<Mutex<Option<i64>>>, a: Arc<Mutex<Option<i64>>>) -> i64 {
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = 1 as i64; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as i64; __tmp_x <= __tmp_y } && { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y }))));
    return { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for StdSizes {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
