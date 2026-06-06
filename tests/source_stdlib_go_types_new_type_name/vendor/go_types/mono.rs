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

use internal_types_errors::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct monoGraph {
    pub vertices: Arc<Mutex<Option<Vec<monoVertex>>>>,
    pub edges: Arc<Mutex<Option<Vec<monoEdge>>>>,
    pub canon: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<TypeParam>>>>>>>,
    pub name_idx: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<i32>>>>>>>,
}

impl monoGraph {
    pub fn __go_value_clone(&self) -> Self {
        Self { vertices: self.vertices.clone(), edges: self.edges.clone(), canon: self.canon.clone(), name_idx: self.name_idx.clone() }
    }
}

impl std::fmt::Display for monoGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", format_slice(&self.vertices), format_slice(&self.edges), format_map(&self.canon), format_map(&self.name_idx))
    }
}

impl GoJsonDecode for monoGraph {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct monoVertex {
    pub weight: Arc<Mutex<Option<i32>>>,
    pub pre: Arc<Mutex<Option<i32>>>,
    pub len: Arc<Mutex<Option<i32>>>,
    pub obj: Arc<Mutex<Option<TypeName>>>,
}

impl monoVertex {
    pub fn __go_value_clone(&self) -> Self {
        Self { weight: { let __guard = self.weight.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pre: { let __guard = self.pre.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, obj: self.obj.clone() }
    }
}


impl Default for monoVertex {
    fn default() -> Self {
        Self { weight: Arc::new(Mutex::new(Some(0))), pre: Arc::new(Mutex::new(Some(0))), len: Arc::new(Mutex::new(Some(0))), obj: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for monoVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.weight.lock().unwrap().as_ref().unwrap()), (*self.pre.lock().unwrap().as_ref().unwrap()), (*self.len.lock().unwrap().as_ref().unwrap()), { let __guard = self.obj.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for monoVertex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct monoEdge {
    pub dst: Arc<Mutex<Option<i32>>>,
    pub src: Arc<Mutex<Option<i32>>>,
    pub weight: Arc<Mutex<Option<i32>>>,
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl monoEdge {
    pub fn __go_value_clone(&self) -> Self {
        Self { dst: { let __guard = self.dst.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, src: { let __guard = self.src.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, weight: { let __guard = self.weight.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone() }
    }
}


impl Default for monoEdge {
    fn default() -> Self {
        Self { dst: Arc::new(Mutex::new(Some(0))), src: Arc::new(Mutex::new(Some(0))), weight: Arc::new(Mutex::new(Some(0))), pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), typ: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for monoEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.dst.lock().unwrap().as_ref().unwrap()), (*self.src.lock().unwrap().as_ref().unwrap()), (*self.weight.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.typ.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for monoEdge {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::check::Checker {
    pub fn monomorph(&self) {
                // We detect unbounded instantiation cycles using a variant of
                // Bellman-Ford's algorithm. Namely, instead of always running |V|
                // iterations, we run until we either reach a fixed point or we've
                // found a path of length |V|. This allows us to terminate earlier
                // when there are no cycles, which should be the common case.
        let mut again = Arc::new(Mutex::new(Some(true)));
        while { let __v = (*again.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = false; *again.lock().unwrap() = Some(new_val); };

        { let __range_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).edges.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, edge) in __range_values.iter().enumerate() {
        let mut src: Option<GoSliceElemPtr<monoVertex>> = Some(GoSliceElemPtr::new((*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(), ((*edge.src.lock().unwrap().as_ref().unwrap())) as usize));
        let mut dst: Option<GoSliceElemPtr<monoVertex>> = Some(GoSliceElemPtr::new((*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(), ((*edge.dst.lock().unwrap().as_ref().unwrap())) as usize));
                // N.B., we're looking for the greatest weight paths, unlike
                // typical Bellman-Ford.
        let mut w = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*src.as_ref().unwrap().borrow().as_ref().unwrap()).weight.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*edge.weight.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*dst.as_ref().unwrap().borrow().as_ref().unwrap()).weight.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        continue
    }
        { let new_val = i as i32; *(*dst.as_ref().unwrap().borrow().as_ref().unwrap()).pre.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = (*{ let __field = (*src.as_ref().unwrap().borrow().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y }; *(*dst.as_ref().unwrap().borrow().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ((*{ let __field = (*dst.as_ref().unwrap().borrow().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = (*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x == __tmp_y } {
        self.report_instance_loop(Arc::new(Mutex::new(Some({ let __selector_holder = edge.dst.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        return;
    }
        { let new_val = w.lock().unwrap().as_ref().unwrap().clone(); *(*dst.as_ref().unwrap().borrow().as_ref().unwrap()).weight.lock().unwrap() = Some(new_val); };
        { let new_val = true; *again.lock().unwrap() = Some(new_val); };
    } }
    }
    }

    pub fn report_instance_loop(&self, mut v: Arc<Mutex<Option<i32>>>) {
        let mut stack: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
        let mut seen = Arc::new(Mutex::new(Some(vec![false; (({ let __len_target = { let __field = (*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
                // We have a path that contains a cycle and ends at v, but v may
                // only be reachable from the cycle, not on the cycle itself. We
                // start by walking backwards along the path until we find a vertex
                // that appears twice.
        while !{ let __seq = { let __seq_holder = seen.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } {
        { let new_val = { let __append_target = stack.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*v.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; stack = new_val; };
        (*seen.lock().unwrap().as_mut().unwrap())[({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = true;
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).edges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*{ let __seq = { let __seq_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.pre.lock().unwrap().as_ref().unwrap())) as usize].clone() }.src.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *v.lock().unwrap() = Some(new_val); };
    }
                // Trim any vertices we visited before visiting v the first
                // time. Since v is the first vertex we found within the cycle, any
                // vertices we visited earlier cannot be part of the cycle.
        while { let __tmp_x = { let __seq = { let __seq_holder = stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() }))); stack = new_val; };
    }
                // TODO(mdempsky): Pivot stack so we report the cycle from the top?
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INSTANCE_CYCLE as i32))))))));
        let mut obj0 = { let __seq = { let __seq_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.obj.clone();
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj0.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("instantiation cycle:".to_string()))), Arc::new(Mutex::new(Some(vec![])))); __result };
        let mut qf = relative_to({ let __field = self.pkg.clone(); __field });
        { let __range_holder = stack.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter().copied() {
        let mut edge = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).edges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*{ let __seq = { let __seq_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(v) as usize].clone() }.pre.lock().unwrap().as_ref().unwrap())) as usize].clone() })));
        let mut obj = { let __seq = { let __seq_holder = (*self.mono.lock().unwrap().as_ref().unwrap()).vertices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*{ let __field = (*edge.lock().unwrap().as_ref().unwrap()).dst.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize].clone() }.obj.clone();
        {
    let _ts_subject = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.r#type(); __result }.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*edge.lock().unwrap().as_ref().unwrap()).pos.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s implicitly parameterized by %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = type_string({ let __field = (*edge.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, qf.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*edge.lock().unwrap().as_ref().unwrap()).pos.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s instantiated as %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = type_string({ let __field = (*edge.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, qf.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };;
    } else {
        std::panic::panic_any(Box::new("unexpected type".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    } }
                // secondary error, \t indented
                // secondary error, \t indented
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }
}

impl monoGraph {
    /// recordCanon records that tpar is the canonical type parameter
    /// corresponding to method type parameter mpar.
    pub fn record_canon(&mut self, mpar: Arc<Mutex<Option<TypeParam>>>, tpar: Arc<Mutex<Option<TypeParam>>>) {
        if { let __nil_target = self.canon.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<crate::typeparam::TypeParam>>>>::new()))); self.canon = new_val; };
    }
        { let __map_key = GoLocalPtrKey::new(mpar.clone()); let __map_value = tpar.clone(); (*self.canon.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    /// recordInstance records that the given type parameters were
    /// instantiated with the corresponding type arguments.
    pub fn record_instance(&mut self, pkg: Arc<Mutex<Option<Package>>>, mut pos: Arc<Mutex<Option<go_token::position::Pos>>>, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, xlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
        { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tpar) in __range_values.iter().enumerate() {
        let mut pos = { let __owned = pos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = (i as i32); let __tmp_y = ((*xlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = start_pos(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = xlist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos.lock().unwrap() = __moved_val; };
    }
        self.assign(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*tpar).clone(), { let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone());
    } }
    }

    /// assign records that tpar was instantiated as targ at pos.
    pub fn assign(&mut self, pkg: Arc<Mutex<Option<Package>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, tpar: Arc<Mutex<Option<TypeParam>>>, targ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
                // Go generics do not have an analog to C++`s template-templates,
                // where a template parameter can itself be an instantiable
                // template. So any instantiation cycles must occur within a single
                // package. Accordingly, we can ignore instantiations of imported
                // type parameters.
                //
                // TODO(mdempsky): Push this check up into recordInstance? All type
                // parameters in a list will appear in the same package.
        if { let __left = { let __recv = { let __recv = tpar.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.obj(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkg(); __result }; let __right = pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        return;
    }
                // flow adds an edge from vertex src representing that typ flows to tpar.
        let pos_closure_clone = pos.clone(); let targ_closure_clone = targ.clone(); let tpar_closure_clone = tpar.clone(); let mut w_closure_clone = (*self).clone(); let mut flow = Arc::new(Mutex::new(Some(Box::new(move |src: Arc<Mutex<Option<i32>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| {
        let mut weight = Arc::new(Mutex::new(Some(1)));
        if { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = targ_closure_clone.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        { let new_val = 0; *weight.lock().unwrap() = Some(new_val); };
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some(w_closure_clone.type_param_vertex(tpar_closure_clone.clone())))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg2 = Arc::new(Mutex::new(Some({ let __arg_holder = weight.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = pos_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg4 = targ_closure_clone.clone(); w_closure_clone.add_edge(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync>)));
                // Recursively walk the type argument to find any defined types or
                // type parameters.
        let mut r#do: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let do_closure_clone = r#do.clone(); let flow_closure_clone = flow.clone(); let pkg_closure_clone = pkg.clone(); let mut w_closure_clone = (*self).clone(); { let __func_lit_target = do_closure_clone.clone(); let new_val = Box::new(move |mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| {
        {
    let _ts_subject = unalias(typ.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some({ let __left = { let __recv = { let __recv = typ.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.obj(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkg(); __result }; let __right = pkg_closure_clone.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));;
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = flow_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(w_closure_clone.type_param_vertex(typ.clone())))), Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        {
        let mut src = w_closure_clone.local_named_vertex(pkg_closure_clone.clone(), { let __recv = typ.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result });;
        if { let __tmp_x = src; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = flow_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(src))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) };;
        }
    };
        let mut targs = { let __recv = typ.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result };;
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = targs.clone(); let __recv_ptr: *const crate::typelists::TypeList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeList }; let __result = unsafe { &*__recv_ptr }.len(); __result }; __tmp_x < __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = targs.clone(); let __recv_ptr: *const crate::typelists::TypeList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeList }; let __result = unsafe { &*__recv_ptr }.at(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }.clone()) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::array::Array = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::array::Array }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::chan::Chan = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::chan::Chan }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::map::Map = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::map::Map }; let __result = unsafe { &*__recv_ptr }.key(); __result }.clone()) };;
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::map::Map = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::map::Map }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::pointer::Pointer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::pointer::Pointer }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::slice::Slice = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::slice::Slice }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = typ.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.num_methods(); __result }; __tmp_x < __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = { let __recv = typ.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone()) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        let do_closure_clone_closure_clone = do_closure_clone.clone(); let mut tuple = Arc::new(Mutex::new(Some(Box::new(move |tup: Arc<Mutex<Option<Tuple>>>| {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = tup.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; __tmp_x < __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = { let __recv = tup.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.at(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone()) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Tuple>>>) -> () + Send + Sync>)));;
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Tuple>>>) -> () + Send + Sync> = { let mut __f_guard = tuple.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Tuple>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }) };;
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Tuple>>>) -> () + Send + Sync> = { let mut __f_guard = tuple.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Tuple>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = typ.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.results(); __result }) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = typ.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; __tmp_x < __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = do_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = { let __recv = typ.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.field(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone()) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
    } else {
        let typ = _ts_subject.clone();
        std::panic::panic_any(Box::new("unexpected type".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync>; *__func_lit_target.lock().unwrap() = Some(new_val); };
                // ok
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = r#do.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(targ.clone()) };
    }

    /// localNamedVertex returns the index of the vertex representing
    /// named, or -1 if named doesn't need representation.
    pub fn local_named_vertex(&mut self, pkg: Arc<Mutex<Option<Package>>>, named: Arc<Mutex<Option<Named>>>) -> i32 {
        let mut obj = { let __recv = named.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.obj(); __result };
        if { let __left = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pkg(); __result }; let __right = pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        return -(1);
    }
                // imported type
        let mut root = { let __recv = pkg.clone(); let __recv_ptr: *const crate::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::package::Package }; let __result = unsafe { &*__recv_ptr }.scope(); __result };
        if { let __left = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.parent(); __result }; let __right = root.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        return -(1);
    }
                // package scope, no ambient type parameters
        {
        let (mut idx, mut ok) = { let __map = { let __map_holder = self.name_idx.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(obj.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            return { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        }
    }
        let mut idx = Arc::new(Mutex::new(Some(-(1))));
                // Walk the type definition's scope to find any ambient type
                // parameters that it's implicitly parameterized by.
        let mut scope = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.parent(); __result };
    while { let __left = scope.clone(); let __right = root.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        for (_, mut elem) in { let __range_holder = (*scope.lock().unwrap().as_ref().unwrap()).elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        {
        let (mut elem, mut ok) = ({
        let val = elem.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
        }
    });;
        if ok && !{ let __recv = elem.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.is_alias(); __result } && { let __tmp_x = cmp_pos({ let __recv = elem.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pos(); __result }); let __tmp_y = 0; __tmp_x < __tmp_y } {
            {
        let (mut tpar, mut ok) = ({
        let val = { let __recv = elem.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.r#type(); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
        }
    });;
        if ok {
            if { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = ({ let __len_target = { let __field = self.vertices.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *idx.lock().unwrap() = Some(new_val); };
        { let new_val = { let __append_target = self.vertices.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(monoVertex { obj: obj.clone(), ..Default::default() }); __append_target.clone() }; self.vertices = new_val; };
    };
            { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = Arc::new(Mutex::new(Some(self.type_param_vertex(tpar.clone())))); let __method_arg2 = Arc::new(Mutex::new(Some(1))); let __method_arg3 = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __method_arg4 = Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>))); self.add_edge(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };;
        }
    };
        }
    }
    }
        { let new_val = { let __recv = scope.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.parent(); __result }.clone(); scope = new_val; };
    }
        if { let __nil_target = self.name_idx.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<i32>>>>::new()))); self.name_idx = new_val; };
    }
        { let __map_key = GoLocalPtrKey::new(obj.clone()); let __map_value = Arc::new(Mutex::new(Some((*idx.lock().unwrap().as_ref().unwrap()).clone()))); (*self.name_idx.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// typeParamVertex returns the index of the vertex representing tpar.
    pub fn type_param_vertex(&mut self, mut tpar: Arc<Mutex<Option<TypeParam>>>) -> i32 {
        {
        let (mut x, mut ok) = { let __map = { let __map_holder = self.canon.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(tpar.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if ok {
            { let new_val = x.clone(); tpar = new_val; };;
        }
    }
        let mut obj = { let __recv = tpar.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.obj(); __result };
        {
        let (mut idx, mut ok) = { let __map = { let __map_holder = self.name_idx.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(obj.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            return { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        }
    }
        if { let __nil_target = self.name_idx.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<i32>>>>::new()))); self.name_idx = new_val; };
    }
        let mut idx = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.vertices.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        { let new_val = { let __append_target = self.vertices.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(monoVertex { obj: obj.clone(), ..Default::default() }); __append_target.clone() }; self.vertices = new_val; };
        { let __map_key = GoLocalPtrKey::new(obj.clone()); let __map_value = Arc::new(Mutex::new(Some((*idx.lock().unwrap().as_ref().unwrap()).clone()))); (*self.name_idx.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    pub fn add_edge(&mut self, dst: Arc<Mutex<Option<i32>>>, src: Arc<Mutex<Option<i32>>>, weight: Arc<Mutex<Option<i32>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
                // TODO(mdempsky): Deduplicate redundant edges?
        { let new_val = { let __append_target = self.edges.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(monoEdge { dst: Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), src: Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), weight: Arc::new(Mutex::new(Some({ let __arg_holder = weight.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), ..Default::default() }); __append_target.clone() }; self.edges = new_val; };
    }
}

impl GoValueClone for monoGraph {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for monoVertex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for monoEdge {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
