use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

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
use crate::sizes::*;
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

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Context is an opaque type checking context. It may be used to share
/// identical type instances across type-checked packages or calls to
/// Instantiate. Contexts are safe for concurrent use.
///
/// The use of a shared context does not guarantee that identical instances are
/// deduplicated in all cases.
#[derive(Clone)]
pub struct Context {
    pub mu: GoMutex,
    pub type_map: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<ctxtEntry>>>>>>>>,
    pub next_i_d: Arc<Mutex<Option<i32>>>,
    pub origin_i_ds: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn Type + Send + Sync>>, Arc<Mutex<Option<i32>>>>>>>,
}

impl Context {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: self.mu.clone(), type_map: self.type_map.clone(), next_i_d: { let __guard = self.next_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, origin_i_ds: self.origin_i_ds.clone() }
    }
}


impl Default for Context {
    fn default() -> Self {
        Self { mu: GoMutex::new(), type_map: Arc::new(Mutex::new(None)), next_i_d: Arc::new(Mutex::new(Some(0))), origin_i_ds: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", "<map>", (*self.next_i_d.lock().unwrap().as_ref().unwrap()), format_map(&self.origin_i_ds))
    }
}

impl GoJsonDecode for Context {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct ctxtEntry {
    pub orig: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>,
    pub instance: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl ctxtEntry {
    pub fn __go_value_clone(&self) -> Self {
        Self { orig: self.orig.clone(), targs: self.targs.clone(), instance: self.instance.clone() }
    }
}

impl std::fmt::Display for ctxtEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.orig.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.targs), (*self.instance.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ctxtEntry {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Context {
    /// instanceHash returns a string representation of typ instantiated with targs.
    /// The hash should be a perfect hash, though out of caution the type checker
    /// does not assume this. The result is guaranteed to not contain blanks.
    pub fn instance_hash(&mut self, orig: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<String>>> {
        assert(Arc::new(Mutex::new(Some(true))));
        assert(Arc::new(Mutex::new(Some((*orig.lock().unwrap()).is_some()))));
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut h = new_type_hasher(buf.clone(), Arc::new(Mutex::new(Some(self.clone()))));
        { let __recv = h.clone(); let __recv_ptr: *const crate::typestring::typeWriter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typestring::typeWriter }; let __result = unsafe { &*__recv_ptr }.string(Arc::new(Mutex::new(Some((self.get_i_d(orig.clone())).to_string())))); __result };
                // Because we've already written the unique origin ID this call to h.typ is
                // unnecessary, but we leave it for hash readability. It can be removed later
                // if performance is an issue.
        { let __recv = h.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.typ(orig.clone()); __result };
        if { let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // TODO(rfindley): consider asserting on isGeneric(typ) here, if and when
                // isGeneric handles *Signature types.
        { let __recv = h.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.type_list(targs.clone()); __result };
    }
                // TODO(rfindley): consider asserting on isGeneric(typ) here, if and when
                // isGeneric handles *Signature types.
        return Arc::new(Mutex::new(Some({ let __s = (*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone(); let __old = " ".to_string(); let __new = "#".to_string(); __s.replace(&__old, &__new) })));
    }

    /// lookup returns an existing instantiation of orig with targs, if it exists.
    /// Otherwise, it returns nil.
    pub fn lookup(&self, h: Arc<Mutex<Option<String>>>, orig: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __mutex_guard_source_3125 = self.mu.clone(); let __mutex_guard_3125 = __mutex_guard_source_3125.guard();
        // mu.Unlock() handled by RAII guard
        { let __range_holder = { let __map = { let __map_holder = self.type_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*h.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        if identical_instance(orig.clone(), targs.clone(), e.orig.clone(), { let __field = e.targs.clone(); __field }) {
        return e.instance.clone();
    }
        if DEBUG {
                // Panic during development to surface any imperfections in our hash.
        panic!("non-identical instances: (orig: {}, targs: {}) and {}", format!("{}", (*orig.lock().unwrap().as_ref().unwrap())), format_slice_wrapped_stringer(&targs), format!("{}", (*e.instance.lock().unwrap().as_ref().unwrap())));
    }
    } }
                // Panic during development to surface any imperfections in our hash.
        return Arc::new(Mutex::new(None));
    }

    /// update de-duplicates inst against previously seen types with the hash h.
    /// If an identical type is found with the type hash h, the previously seen
    /// type is returned. Otherwise, inst is returned, and recorded in the Context
    /// for the hash h.
    pub fn update(&mut self, h: Arc<Mutex<Option<String>>>, orig: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, inst: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        assert(Arc::new(Mutex::new(Some((*inst.lock().unwrap()).is_some()))));
        let __mutex_guard_source_3847 = self.mu.clone(); let __mutex_guard_3847 = __mutex_guard_source_3847.guard();
        // mu.Unlock() handled by RAII guard
        { let __range_holder = { let __map = { let __map_holder = self.type_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*h.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        if (*inst.lock().unwrap()).is_none() || identical(inst.clone(), e.instance.clone()) {
        return e.instance.clone();
    }
        if DEBUG {
                // Panic during development to surface any imperfections in our hash.
        panic!("{} and {} are not identical", format!("{}", (*inst.lock().unwrap().as_ref().unwrap())), format!("{}", (*e.instance.lock().unwrap().as_ref().unwrap())));
    }
    } }
                // Panic during development to surface any imperfections in our hash.
        { let __map_key = (*h.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = { let __slice = { let __map_holder = self.type_map.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&(*h.lock().unwrap().as_ref().unwrap()).clone()).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push(ctxtEntry { orig: orig.clone(), targs: targs.clone(), instance: inst.clone(), ..Default::default() }); __slice.clone() }; (*self.type_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return inst.clone();
    }

    /// getID returns a unique ID for the type t.
    pub fn get_i_d(&mut self, t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i32 {
        let __mutex_guard_source_4381 = self.mu.clone(); let __mutex_guard_4381 = __mutex_guard_source_4381.guard();
        // mu.Unlock() handled by RAII guard
        let (mut id, mut ok) = { let __map = { let __map_holder = self.origin_i_ds.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(t.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
        if !ok {
        { let new_val = { let __selector_holder = self.next_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *id.lock().unwrap() = Some(new_val); };
        { let __map_key = GoLocalPtrKey::new(t.clone()); let __map_value = Arc::new(Mutex::new(Some((*id.lock().unwrap().as_ref().unwrap()).clone()))); (*self.origin_i_ds.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let __target = self.next_i_d.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

/// NewContext creates a new Context.
pub fn new_context() -> Arc<Mutex<Option<Context>>> {
    Arc::new(Mutex::new(Some(Context { type_map: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<ctxtEntry>>>>>::new()))), origin_i_ds: Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Box<dyn Type + Send + Sync>>, Arc<Mutex<Option<i32>>>>::new()))), ..Default::default() })))
}

impl GoValueClone for Context {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ctxtEntry {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
