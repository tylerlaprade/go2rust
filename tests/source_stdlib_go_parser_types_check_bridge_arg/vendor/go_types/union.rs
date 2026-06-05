use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use internal_types_errors::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_TERM_COUNT: i32 = 100;


/// A Union represents a union of terms embedded in an interface.
#[derive(Clone, Default)]
pub struct Union {
    pub terms: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Term>>>>>>>,
}

impl Union {
    pub fn __go_value_clone(&self) -> Self {
        Self { terms: self.terms.clone() }
    }
}

impl std::fmt::Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Union {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Term represents a term in a [Union].
#[derive(Clone)]
pub struct Term {
    pub tilde: Arc<Mutex<Option<bool>>>,
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl Term {
    pub fn __go_value_clone(&self) -> Self {
        Self { tilde: { let __guard = self.tilde.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone() }
    }
}


impl Default for Term {
    fn default() -> Self {
        Self { tilde: Arc::new(Mutex::new(Some(false))), typ: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Term {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for Term {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
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


impl Union {
    pub fn len(&self) -> i32 {
        ({ let __len_target = { let __field = self.terms.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    pub fn term(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Term>>> {
        { let __seq = { let __seq_holder = self.terms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(UnionPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(UnionPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }
}

impl Type for Union {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Union::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Union::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Union>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct UnionPtr(pub Arc<Mutex<Option<Union>>>);

impl std::fmt::Display for UnionPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for UnionPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Union::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Union::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<UnionPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Term {
    pub fn tilde(&self) -> bool {
        return (*self.tilde.lock().unwrap().as_ref().unwrap());
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        return { let __field = self.typ.clone(); __field };
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        { let __recv = Arc::new(Mutex::new(Some(term::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }
    }
}

/// NewUnion returns a new [Union] type with the given terms.
/// It is an error to create an empty union; they are syntactically not possible.
pub fn new_union(terms: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Term>>>>>>>) -> Arc<Mutex<Option<Union>>> {
    if { let __tmp_x = ((*terms.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("empty union".to_string()) as Box<dyn Any + Send + Sync>);
    }
    Arc::new(Mutex::new(Some(Union { terms: terms.clone(), ..Default::default() })))
}

/// NewTerm returns a new union term.
pub fn new_term(tilde: Arc<Mutex<Option<bool>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Term>>> {
    Arc::new(Mutex::new(Some(Term { tilde: Arc::new(Mutex::new(Some({ let __arg_holder = tilde.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), ..Default::default() })))
}

/// parseUnion parses uexpr as a union of expressions.
/// The result is a Union type, or Typ[Invalid] for some errors.
pub fn parse_union(check: Arc<Mutex<Option<Checker>>>, uexpr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let (mut blist, mut tlist) = flatten_union(Arc::new(Mutex::new(None)), uexpr.clone());
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*blist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __tmp_x = ((*tlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y }))));

    let mut terms: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Term>>>>>>> = Arc::new(Mutex::new(None));

    let mut u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = tlist.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        let mut term = parse_tilde(check.clone(), x.clone());
        if { let __tmp_x = ((*tlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && !(*{ let __field = (*term.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Single type. Ok to return early because all relevant
                // checks have been performed in parseTilde (no need to
                // run through term validity check below).
        return { let __field = (*term.lock().unwrap().as_ref().unwrap()).typ.clone(); __field };
    }
                // Single type. Ok to return early because all relevant
                // checks have been performed in parseTilde (no need to
                // run through term validity check below).
                // typ already recorded through check.typ in parseTilde
        if { let __tmp_x = ((*terms.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 100; __tmp_x >= __tmp_y } {
        if is_valid(u.clone()) {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("cannot handle more than %d union terms (implementation limitation)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(MAX_TERM_COUNT) as Box<dyn Any + Send + Sync>])))); __result };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } else {
        { let new_val = { let __append_target = terms.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(term.clone()); __append_target.clone() }; terms = new_val; };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(UnionPtr(Arc::new(Mutex::new(Some(Union { terms: terms.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };
    }
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.record_type_and_value({ let __seq = { let __seq_holder = blist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone(), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), u.clone(), Arc::new(Mutex::new(None))); __result };
    }
    } }

        // Single type. Ok to return early because all relevant
        // checks have been performed in parseTilde (no need to
        // run through term validity check below).
        // typ already recorded through check.typ in parseTilde
    if !is_valid(u.clone()) {
        return u.clone();
    }

        // Check validity of terms.
        // Do this check later because it requires types to be set up.
        // Note: This is a quadratic algorithm, but unions tend to be short.
    let check_closure_clone = check.clone(); let terms_closure_clone = terms.clone(); let tlist_closure_clone = tlist.clone(); { let __recv = { let __recv = check_closure_clone.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.later(Arc::new(Mutex::new(Some({ let check_closure_clone_closure_clone = check_closure_clone.clone(); Box::new(move || {
        { let __range_holder = terms_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, t) in __range_values.iter().enumerate() {
        if !is_valid({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        continue
    }
        let mut u = under({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
        let (mut f, _) = ({
        let val = u.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });
        if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if (*f.lock().unwrap()).is_some() {
        { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = tlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("invalid use of ~ (%s is an interface)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        continue
    }
        if !identical(u.clone(), { let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = tlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("invalid use of ~ (underlying type of %s is %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
        continue
    }
    }
        if (*f.lock().unwrap()).is_some() {
        let mut tset = { let __recv = f.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };
        if { let __tmp_x = { let __recv = tset.clone(); let __recv_ptr: *const crate::typeset::_TypeSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeset::_TypeSet }; let __result = unsafe { &*__recv_ptr }.num_methods(); __result }; let __tmp_y = 0; __tmp_x != __tmp_y } {
            { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = tlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("cannot use %s in union (%s contains methods)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(t.clone()) as Box<dyn Any + Send + Sync>, Box::new(t.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        } else if { let __left_holder = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*universeComparable.lock().unwrap().as_ref().unwrap()).r#type().clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
            { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.error(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = tlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("cannot use comparable in union".to_string())))); __result };
        } else if (*{ let __field = (*tset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
            { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = tlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("cannot use %s in union (%s embeds comparable)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(t.clone()) as Box<dyn Any + Send + Sync>, Box::new(t.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        }
        continue
    }
        {
        let mut j = overlapping_term(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = terms_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(i) as usize].to_vec() }))), (*t).clone());;
        if { let __tmp_x = j; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.soft_errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = tlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("overlapping terms %s and %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(t.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __seq = { let __seq_holder = terms_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(j) as usize].clone() }.clone()) as Box<dyn Any + Send + Sync>])))); __result };;
        }
    }
    } }
    }) as Box<dyn FnMut() -> () + Send + Sync> })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = uexpr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("check term validity %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = uexpr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };

        // don't report another error for t
        // Stand-alone embedded interfaces are ok and are handled by the single-type case
        // in the beginning. Embedded interfaces with tilde are excluded above. If we reach
        // here, we must have at least two terms in the syntactic term list (but not necessarily
        // in the term list of the union's type set).
        // terms with interface types are not subject to the no-overlap rule
        // Report overlapping (non-disjoint) terms such as
        // a|a, a|~a, ~a|~a, and ~a|A (where under(A) == a).
    return u.clone();
}

pub fn parse_tilde(check: Arc<Mutex<Option<Checker>>>, tx: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Term>>> {
    let mut x = tx.clone();
    let mut tilde: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    {
        let (mut op, _) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::UnaryExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::UnaryExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::UnaryExpr>)), false)
        }
    });;
        if (*op.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*op.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); __tmp_x == __tmp_y } {
            { let __iface_handle = { let __field = (*op.lock().unwrap().as_ref().unwrap()).x.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };;
            { let new_val = true; *tilde.lock().unwrap() = Some(new_val); };;
        }
    }
    let mut typ = { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.typ(x.clone()); __result };

        // Embedding stand-alone type parameters is not permitted (go.dev/issue/47127).
        // We don't need this restriction anymore if we make the underlying type of a type
        // parameter its constraint interface: if we embed a lone type parameter, we will
        // simply use its underlying type (like we do for other named, embedded interfaces),
        // and since the underlying type is an interface the embedding is well defined.
    if is_type_param(typ.clone()) {
        if { let __v = (*tilde.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_TYPE_PARAM as i32))))))), Arc::new(Mutex::new(Some("type in term %s cannot be a type parameter".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = tx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    } else {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_TYPE_PARAM as i32))))))), Arc::new(Mutex::new(Some("term cannot be a type parameter".to_string())))); __result };
    }
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    let mut term = new_term(Arc::new(Mutex::new(Some({ let __arg_holder = tilde.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone());
    if { let __v = (*tilde.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.record_type_and_value(tx.clone(), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), Arc::new(Mutex::new(Some(Box::new(UnionPtr(Arc::new(Mutex::new(Some(Union { terms: Arc::new(Mutex::new(Some(vec![term.clone()]))), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None))); __result };
    }
    return term.clone();
}

/// overlappingTerm reports the index of the term x in terms which is
/// overlapping (not disjoint) from y. The result is < 0 if there is no
/// such term. The type of term y must not be an interface, and terms
/// with an interface type are ignored in the terms list.
pub fn overlapping_term(terms: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Term>>>>>>>, y: Arc<Mutex<Option<Term>>>) -> i32 {
    assert(Arc::new(Mutex::new(Some(!is_interface({ let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field })))));
    { let __range_holder = terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        if is_interface({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        continue
    }
                // disjoint requires non-nil, non-top arguments,
                // and non-interface types as term types.
        if DEBUG {
        if (*x.lock().unwrap()).is_none() || { let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } || (*y.lock().unwrap()).is_none() || { let __iface_handle = { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        std::panic::panic_any(Box::new("empty or top union term".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }
        if !{ let __recv = Arc::new(Mutex::new(Some(term::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).disjoint(Arc::new(Mutex::new(Some(term::default())))); __result } {
        return i as i32;
    }
    } }
        // disjoint requires non-nil, non-top arguments,
        // and non-interface types as term types.
    -(1)
}

/// flattenUnion walks a union type expression of the form A | B | C | ...,
/// extracting both the binary exprs (blist) and leaf types (tlist).
pub fn flatten_union(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
    let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
    let mut blist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    let mut tlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

    {
        let (mut o, _) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::BinaryExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::BinaryExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::BinaryExpr>)), false)
        }
    });;
        if (*o.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*o.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32)))); __tmp_x == __tmp_y } {
            { let (__tmp_0, __tmp_1) = flatten_union(list.clone(), { let __field = (*o.lock().unwrap().as_ref().unwrap()).x.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *blist.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *tlist.lock().unwrap() = __moved_tmp_1; };;
            { let new_val = { let __append_target = blist.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BinaryExprPtr(o.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))); __append_target.clone() }; blist = new_val; };;
            { let __iface_handle = { let __field = (*o.lock().unwrap().as_ref().unwrap()).y.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    return (blist.clone(), { let __append_target = tlist.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(x.clone()); __append_target.clone() });
}

impl GoValueClone for Union {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Term {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
