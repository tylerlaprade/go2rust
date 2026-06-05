use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

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

use internal_types_errors::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// An Interface represents an interface type.
#[derive(Clone)]
pub struct Interface {
    pub check: Arc<Mutex<Option<Checker>>>,
    pub methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>,
    pub embeddeds: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>,
    pub embed_pos: Arc<Mutex<Option<Vec<go_token::position::Pos>>>>,
    pub implicit: Arc<Mutex<Option<bool>>>,
    pub complete: Arc<Mutex<Option<bool>>>,
    pub tset: Arc<Mutex<Option<_TypeSet>>>,
}

impl Interface {
    pub fn __go_value_clone(&self) -> Self {
        Self { check: self.check.clone(), methods: self.methods.clone(), embeddeds: self.embeddeds.clone(), embed_pos: self.embed_pos.clone(), implicit: { let __guard = self.implicit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, complete: { let __guard = self.complete.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tset: self.tset.clone() }
    }
}


impl Default for Interface {
    fn default() -> Self {
        Self { check: Arc::new(Mutex::new(None)), methods: Arc::new(Mutex::new(None)), embeddeds: Arc::new(Mutex::new(None)), embed_pos: Arc::new(Mutex::new(None)), implicit: Arc::new(Mutex::new(Some(false))), complete: Arc::new(Mutex::new(Some(false))), tset: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Interface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static emptyInterface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Interface>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *emptyInterface.lock().unwrap() = Some(Default::default());
    *emptyInterface.lock().unwrap() = Some(Interface { complete: Arc::new(Mutex::new(Some(true))), tset: topTypeSet.clone().clone(), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *emptyInterface.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_12() {
    *emptyInterface.lock().unwrap() = Some(Interface { complete: Arc::new(Mutex::new(Some(true))), tset: topTypeSet.clone().clone(), ..Default::default() });
}


impl Interface {
    /// typeSet returns the type set for interface t.
    pub fn type_set(&self) -> Arc<Mutex<Option<crate::typeset::_TypeSet>>> {
        compute_interface_type_set({ let __field = self.check.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(self.clone()))))
    }

    /// MarkImplicit marks the interface t as implicit, meaning this interface
    /// corresponds to a constraint literal such as ~T or A|B without explicit
    /// interface embedding. MarkImplicit should be called before any concurrent use
    /// of implicit interfaces.
    pub fn mark_implicit(&mut self) {
        { let new_val = true; *self.implicit.lock().unwrap() = Some(new_val); };
    }

    /// NumExplicitMethods returns the number of explicitly declared methods of interface t.
    pub fn num_explicit_methods(&self) -> i32 {
        ({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// ExplicitMethod returns the i'th explicitly declared method of interface t for 0 <= i < t.NumExplicitMethods().
    /// The methods are ordered by their unique [Id].
    pub fn explicit_method(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
        { let __seq = { let __seq_holder = self.methods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// NumEmbeddeds returns the number of embedded types in interface t.
    pub fn num_embeddeds(&self) -> i32 {
        ({ let __len_target = { let __field = self.embeddeds.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Embedded returns the i'th embedded defined (*[Named]) type of interface t for 0 <= i < t.NumEmbeddeds().
    /// The result is nil if the i'th embedded type is not a defined type.
    ///
    /// Deprecated: Use [Interface.EmbeddedType] which is not restricted to defined (*[Named]) types.
    pub fn embedded(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::named::Named>>> {
        as_named({ let __seq = { let __seq_holder = self.embeddeds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone())
    }

    /// EmbeddedType returns the i'th embedded type of interface t for 0 <= i < t.NumEmbeddeds().
    pub fn embedded_type(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        { let __seq = { let __seq_holder = self.embeddeds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// NumMethods returns the total number of methods of interface t.
    pub fn num_methods(&self) -> i32 {
        { let __recv = self.type_set(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).num_methods(); __result }
    }

    /// Method returns the i'th method of interface t for 0 <= i < t.NumMethods().
    /// The methods are ordered by their unique Id.
    pub fn method(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
        { let __recv = self.type_set(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }
    }

    /// Empty reports whether t is the empty interface.
    pub fn empty(&self) -> bool {
        { let __recv = self.type_set(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_all(); __result }
    }

    /// IsComparable reports whether each type in interface t's type set is comparable.
    pub fn is_comparable(&self) -> bool {
        { let __recv = self.type_set(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_comparable(Arc::new(Mutex::new(None))); __result }
    }

    /// IsMethodSet reports whether the interface t is fully described by its method
    /// set.
    pub fn is_method_set(&self) -> bool {
        { let __recv = self.type_set(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_method_set(); __result }
    }

    /// IsImplicit reports whether the interface t is a wrapper for a type set literal.
    pub fn is_implicit(&self) -> bool {
        return (*self.implicit.lock().unwrap().as_ref().unwrap());
    }

    /// Complete computes the interface's type set. It must be called by users of
    /// [NewInterfaceType] and [NewInterface] after the interface's embedded types are
    /// fully defined and before using the interface type in any way other than to
    /// form other types. The interface must not contain duplicate methods or a
    /// panic occurs. Complete returns the receiver.
    ///
    /// Interface types that have been completed are safe for concurrent use.
    pub fn complete(&mut self) -> Arc<Mutex<Option<Interface>>> {
        if !(*self.complete.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *self.complete.lock().unwrap() = Some(new_val); };
    }
        self.type_set();
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(InterfacePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(InterfacePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn cleanup(&mut self) {
        self.type_set();
        *self.check.lock().unwrap() = None;
        *self.embed_pos.lock().unwrap() = None;
    }
}

impl Type for Interface {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Interface::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Interface::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Interface>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct InterfacePtr(pub Arc<Mutex<Option<Interface>>>);

impl std::fmt::Display for InterfacePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for InterfacePtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Interface::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Interface::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfacePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cleaner for Interface {
    fn cleanup(&mut self) {
        Interface::cleanup(self)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Interface>() {
            false
        } else {
            false
        }
    }
}

impl cleaner for InterfacePtr {
    fn cleanup(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Interface::cleanup(__recv)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfacePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl crate::check::Checker {
    /// check may be nil
    pub fn new_interface(&mut self) -> Arc<Mutex<Option<Interface>>> {
        let mut typ = Arc::new(Mutex::new(Some(Interface { check: Arc::new(Mutex::new(Some(self.clone()))), ..Default::default() })));
        if true {
        self.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(InterfacePtr(typ.clone())) as Box<dyn cleaner + Send + Sync>))));
    }
        return typ.clone();
    }

    pub fn interface_type(&mut self, ityp: Arc<Mutex<Option<Interface>>>, iface: Arc<Mutex<Option<go_ast::r#mod::InterfaceType>>>, def: Arc<Mutex<Option<TypeName>>>) {
        let ityp_closure_clone = ityp.clone(); let mut addEmbedded = Arc::new(Mutex::new(Some(Box::new(move |pos: Arc<Mutex<Option<go_token::position::Pos>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| {
        { let new_val = { let __append_target = (*ityp_closure_clone.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(typ.clone()); __append_target.clone() }; (*ityp_closure_clone.lock().unwrap().as_mut().unwrap()).embeddeds = new_val; };
        if { let __nil_target = (*ityp_closure_clone.lock().unwrap().as_ref().unwrap()).embed_pos.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(Vec::<go_token::position::Pos>::default()))).clone(); (*ityp_closure_clone.lock().unwrap().as_mut().unwrap()).embed_pos = new_val; };
    }
        { let new_val = { let __append_target = (*ityp_closure_clone.lock().unwrap().as_ref().unwrap()).embed_pos.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*pos.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *(*ityp_closure_clone.lock().unwrap().as_ref().unwrap()).embed_pos.lock().unwrap() = __cloned_val; };
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync>)));
        { let __range_holder = (*(*iface.lock().unwrap().as_ref().unwrap()).methods.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        if { let __tmp_x = (({ let __len_target = { let __field = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = addEmbedded.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).pos(), parse_union(Arc::new(Mutex::new(Some(self.clone()))), (*f.lock().unwrap().as_ref().unwrap()).r#type.clone()).clone()) };
        continue
    }
                // f.Name != nil
                // We have a method with name f.Names[0].
        let mut name = { let __seq = { let __seq_holder = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        if { let __tmp_x = { let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BLANK_IFACE_METHOD as i32))))))), Arc::new(Mutex::new(Some("methods must have a unique non-blank name".to_string()))));
        continue
    }
                // ignore
        let mut typ = self.typ((*f.lock().unwrap().as_ref().unwrap()).r#type.clone());
        let (mut sig, _) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::signature::Signature>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::signature::Signature>)), false)
        }
    });
        if (*sig.lock().unwrap()).is_none() {
        if is_valid(typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("%s is not a method signature".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        continue
    }
                // ignore
                // The go/parser doesn't accept method type parameters but an ast.FuncType may have them.
        if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).tparams.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>)));
        {
        let (mut ftyp, _) = ({
        let val = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::FuncTypePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::FuncType>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::FuncType>)), false)
        }
    });;
        if (*ftyp.lock().unwrap()).is_some() && { let __nil_target = (*ftyp.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldListPtr((*ftyp.lock().unwrap().as_ref().unwrap()).type_params.clone())) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *at.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        self.error(at.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("methods cannot have type parameters".to_string()))));
    }
                // use named receiver type if available (for better error messages)
        let mut recvTyp: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(InterfacePtr(ityp.clone())) as Box<dyn Type + Send + Sync>)));
        if (*def.lock().unwrap()).is_some() {
        {
        let mut named = as_named((*(*def.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());;
        if (*named.lock().unwrap()).is_some() {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(named.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *recvTyp.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    }
        { let new_val = new_var({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), recvTyp.clone()).clone(); (*sig.lock().unwrap().as_mut().unwrap()).recv = new_val; };
        let mut m = new_func({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), sig.clone());
        self.record_def(name.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))));
        { let new_val = { let __append_target = (*ityp.lock().unwrap().as_ref().unwrap()).methods.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(m.clone()); __append_target.clone() }; (*ityp.lock().unwrap().as_mut().unwrap()).methods = new_val; };
    } }
                // f.Name != nil
                // We have a method with name f.Names[0].
                // ignore
                // ignore
                // The go/parser doesn't accept method type parameters but an ast.FuncType may have them.
                // use named receiver type if available (for better error messages)
                // All methods and embedded elements for this interface are collected;
                // i.e., this interface may be used in a type set computation.
        { let new_val = true; *(*ityp.lock().unwrap().as_ref().unwrap()).complete.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (({ let __len_target = { let __field = (*ityp.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (({ let __len_target = { let __field = (*ityp.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // empty interface
        { let new_val = topTypeSet.clone().clone(); (*ityp.lock().unwrap().as_mut().unwrap()).tset = new_val; };
        return;
    }
                // empty interface
                // sort for API stability
        sort_methods({ let __field = (*ityp.lock().unwrap().as_ref().unwrap()).methods.clone(); __field });
                // (don't sort embeddeds: they must correspond to *embedPos entries)
                // Compute type set as soon as possible to report any errors.
                // Subsequent uses of type sets will use this computed type
                // set and won't need to pass in a *Checker.
        let mut check_closure_clone = (*self).clone(); let iface_closure_clone = iface.clone(); let ityp_closure_clone = ityp.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let iface_closure_clone_closure_clone = iface_closure_clone.clone(); let ityp_closure_clone_closure_clone = ityp_closure_clone.clone(); Box::new(move || {
        compute_interface_type_set(Arc::new(Mutex::new(Some(check_closure_clone_closure_clone.clone()))), { let __recv = iface_closure_clone_closure_clone.clone(); let __recv_ptr: *const go_ast::r#mod::InterfaceType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::InterfaceType }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, ityp_closure_clone_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::InterfaceTypePtr(iface_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("compute type set for %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(ityp_closure_clone.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
}

/// NewInterfaceType returns a new interface for the given methods and embedded
/// types. NewInterfaceType takes ownership of the provided methods and may
/// modify their types by setting missing receivers.
///
/// To avoid race conditions, the interface's type set should be computed before
/// concurrent use of the interface, by explicitly calling Complete.
pub fn new_interface_type(methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>, embeddeds: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Interface>>> {
    if { let __tmp_x = ((*methods.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = ((*embeddeds.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return emptyInterface.clone();
    }

        // set method receivers if necessary
    let mut typ = __go_nil_recv_crate__check___checker_new_interface(Arc::new(Mutex::new(None::<Checker>)));
    { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        {
        let mut sig = ({
        let val = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();;
        if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
            { let new_val = new_var(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::new(InterfacePtr(typ.clone())) as Box<dyn Type + Send + Sync>)))).clone(); (*sig.lock().unwrap().as_mut().unwrap()).recv = new_val; };;
        }
    }
    } }

        // sort for API stability
    sort_methods(methods.clone());

    { let new_val = methods.clone(); (*typ.lock().unwrap().as_mut().unwrap()).methods = new_val; };
    { let new_val = embeddeds.clone(); (*typ.lock().unwrap().as_mut().unwrap()).embeddeds = new_val; };
    { let new_val = true; *(*typ.lock().unwrap().as_ref().unwrap()).complete.lock().unwrap() = Some(new_val); };

    return typ.clone();
}

pub fn __go_nil_recv_crate__check___checker_new_interface(check: Arc<Mutex<Option<Checker>>>) -> Arc<Mutex<Option<Interface>>> {
    let mut typ = Arc::new(Mutex::new(Some(Interface { check: check.clone(), ..Default::default() })));
    if (*check.lock().unwrap()).is_some() {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(InterfacePtr(typ.clone())) as Box<dyn cleaner + Send + Sync>)))); __result };
    }
    return typ.clone();
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Interface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
