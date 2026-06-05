use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Struct represents a struct type.
#[derive(Clone, Default)]
pub struct Struct {
    pub fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>,
    pub tags: Arc<Mutex<Option<Vec<String>>>>,
}

impl Struct {
    pub fn __go_value_clone(&self) -> Self {
        Self { fields: self.fields.clone(), tags: self.tags.clone() }
    }
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Struct {
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


impl Struct {
    /// NumFields returns the number of fields in the struct (including blank and embedded fields).
    pub fn num_fields(&self) -> i32 {
        ({ let __len_target = { let __field = self.fields.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Field returns the i'th field for 0 <= i < NumFields().
    pub fn field(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::object::Var>>> {
        { let __seq = { let __seq_holder = self.fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// Tag returns the i'th field tag for 0 <= i < NumFields().
    pub fn tag(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.tags.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
    }
        Arc::new(Mutex::new(Some("".to_string())))
    }

    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(StructPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(StructPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn mark_complete(&mut self) {
        if { let __nil_target = self.fields.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(Vec::<Arc<Mutex<Option<crate::object::Var>>>>::with_capacity(0)))); self.fields = new_val; };
    }
    }
}

impl Type for Struct {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Struct::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Struct::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Struct>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct StructPtr(pub Arc<Mutex<Option<Struct>>>);

impl std::fmt::Display for StructPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for StructPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Struct::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Struct::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StructPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl crate::check::Checker {
    pub fn struct_type(&mut self, styp: Arc<Mutex<Option<Struct>>>, e: Arc<Mutex<Option<go_ast::r#mod::StructType>>>) {
        let mut list = (*e.lock().unwrap().as_ref().unwrap()).fields.clone();
        if (*list.lock().unwrap()).is_none() {
        { let __recv = styp.clone(); let __recv_ptr: *mut Struct = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Struct }; let __result = unsafe { &mut *__recv_ptr }.mark_complete(); __result };
        return;
    }
                // struct fields and tags
        let mut fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        let mut tags: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
                // for double-declaration checks
        let mut fset: Arc<Mutex<Option<objset>>> = Arc::new(Mutex::new(Some(crate::objset::objset(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>::new())))))));
                // current field typ and tag
        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut tag: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut check_closure_clone = (*self).clone(); let mut fields_closure_clone = fields.clone(); let fset_closure_clone = fset.clone(); let tag_closure_clone = tag.clone(); let mut tags_closure_clone = tags.clone(); let typ_closure_clone = typ.clone(); let mut add = Arc::new(Mutex::new(Some(Box::new(move |ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, embedded: Arc<Mutex<Option<bool>>>| {
        if { let __tmp_x = (*tag_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && (*tags_closure_clone.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(vec!["".to_string(); ((*fields_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tags_closure_clone.lock().unwrap() = __moved_val; };
    }
        if (*tags_closure_clone.lock().unwrap()).is_some() {
        { let __append_target = tags_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*tag_closure_clone.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() };
    }
        let mut pos = { let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut fld = new_field(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = check_closure_clone.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = embedded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } || check_closure_clone.declare_in_set(fset_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(fld.clone())) as Box<dyn Object + Send + Sync>)))) {
        { let __append_target = fields_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(fld.clone()); __append_target.clone() };
        check_closure_clone.record_def(ident.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(fld.clone())) as Box<dyn Object + Send + Sync>))));
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync>)));
                // spec: "Within a struct, non-blank field names must be unique."
                // addInvalid adds an embedded field of invalid type to the struct for
                // fields with errors; this keeps the number of struct fields in sync
                // with the source as long as the fields are _ or have different names
                // (go.dev/issue/25627).
        let add_closure_clone = add.clone(); let mut tag_closure_clone = tag.clone(); let mut typ_closure_clone = typ.clone(); let mut addInvalid = Arc::new(Mutex::new(Some(Box::new(move |ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>>| {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ_closure_clone.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = "".to_string(); *tag_closure_clone.lock().unwrap() = Some(new_val); };
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = { let mut __f_guard = add_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(ident.clone(), Arc::new(Mutex::new(Some(true)))) };
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> () + Send + Sync>)));
        { let __range_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        { let __iface_handle = self.var_type((*f.lock().unwrap().as_ref().unwrap()).r#type.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = self.tag({ let __field = (*f.lock().unwrap().as_ref().unwrap()).tag.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tag.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (({ let __len_target = { let __field = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // named fields
        { let __range_holder = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = { let mut __f_guard = add.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*name).clone(), Arc::new(Mutex::new(Some(false)))) };
    } }
    } else {
                // embedded field
                // spec: "An embedded type must be specified as a type name T or as a
                // pointer to a non-interface type name *T, and T itself may not be a
                // pointer type."
        let mut pos = (*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).pos();
        let mut name = embedded_field_ident((*f.lock().unwrap().as_ref().unwrap()).r#type.clone());
        if (*name.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("embedded field type %s has no name".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = go_ast::new_ident(Arc::new(Mutex::new(Some("_".to_string())))).clone(); name = new_val; };
        { let new_val = pos.lock().unwrap().as_ref().unwrap().clone(); *(*name.lock().unwrap().as_ref().unwrap()).name_pos.lock().unwrap() = Some(new_val); };
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> () + Send + Sync> = { let mut __f_guard = addInvalid.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(name.clone()) };
        continue
    }
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = { let mut __f_guard = add.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(name.clone(), Arc::new(Mutex::new(Some(true)))) };
                // Because we have a name, typ must be of the form T or *T, where T is the name
                // of a (named or alias) type, and t (= deref(typ)) must be the type of T.
                // We must delay this check to the end because we don't want to instantiate
                // (via under(t)) a possibly incomplete type.
                // for use in the closure below
        let mut embeddedTyp = typ.clone();
        let mut embeddedPos = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone();
        let mut check_closure_clone = (*self).clone(); let embeddedPos_closure_clone = embeddedPos.clone(); let embeddedTyp_closure_clone = embeddedTyp.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let embeddedPos_closure_clone_closure_clone = embeddedPos_closure_clone.clone(); let embeddedTyp_closure_clone_closure_clone = embeddedTyp_closure_clone.clone(); Box::new(move || {
        let (mut t, mut isPtr) = deref(embeddedTyp_closure_clone_closure_clone.clone());
        '__go_switch_1: loop {
    {
    let _ts_subject = under(t.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if !is_valid(t.clone()) {
        return;
    };
        if { let __tmp_x = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNSAFE_POINTER as i32)))); __tmp_x == __tmp_y } {
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = embeddedPos_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_PTR_EMBED as i32))))))), Arc::new(Mutex::new(Some("embedded field type cannot be unsafe.Pointer".to_string()))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = embeddedPos_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_PTR_EMBED as i32))))))), Arc::new(Mutex::new(Some("embedded field type cannot be a pointer".to_string()))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        if is_type_param(t.clone()) {
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = embeddedPos_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_TYPE_PARAM as i32))))))), Arc::new(Mutex::new(Some("embedded field type cannot be a (pointer to a) type parameter".to_string()))));
        break '__go_switch_1
    };
        if isPtr {
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = embeddedPos_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_PTR_EMBED as i32))))))), Arc::new(Mutex::new(Some("embedded field type cannot be a pointer to an interface".to_string()))));
    };
    }
    };
    break;
}
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = embeddedPos_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("check embedded type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = embeddedTyp_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    }
    } }
                // named fields
                // embedded field
                // spec: "An embedded type must be specified as a type name T or as a
                // pointer to a non-interface type name *T, and T itself may not be a
                // pointer type."
                // position of type, for errors
                // struct{p.T} field has position of T
                // Because we have a name, typ must be of the form T or *T, where T is the name
                // of a (named or alias) type, and t (= deref(typ)) must be the type of T.
                // We must delay this check to the end because we don't want to instantiate
                // (via under(t)) a possibly incomplete type.
                // for use in the closure below
                // error was reported before
                // unsafe.Pointer is treated like a regular pointer
                // The error code here is inconsistent with other error codes for
                // invalid embedding, because this restriction may be relaxed in the
                // future, and so it did not warrant a new error code.
        { let new_val = fields.clone(); (*styp.lock().unwrap().as_mut().unwrap()).fields = new_val; };
        { let new_val = tags.clone(); (*styp.lock().unwrap().as_mut().unwrap()).tags = new_val; };
        { let __recv = styp.clone(); let __recv_ptr: *mut Struct = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Struct }; let __result = unsafe { &mut *__recv_ptr }.mark_complete(); __result };
    }

    pub fn declare_in_set(&self, oset: Arc<Mutex<Option<objset>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> bool {
        {
        let mut alt = { let __recv = oset.clone(); let __recv_ptr: *mut crate::objset::objset = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::objset::objset }; let __result = unsafe { &mut *__recv_ptr }.insert(obj.clone()); __result };;
        if (*alt.lock().unwrap()).is_some() {
            let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DECL as i32))))))));;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s redeclared".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(alt.clone()); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
            return false;;
        }
    }
        true
    }

    pub fn tag(&self, t: Arc<Mutex<Option<go_ast::r#mod::BasicLit>>>) -> Arc<Mutex<Option<String>>> {
        if (*t.lock().unwrap()).is_some() {
        if { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32)))); __tmp_x == __tmp_y } {
        {
        let (mut val, mut err) = strconv::unquote({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });;
        if (*err.lock().unwrap()).is_none() {
            return { let __owned = val.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(t.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect tag syntax: %q".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
        Arc::new(Mutex::new(Some("".to_string())))
    }
}

pub fn embedded_field_ident(mut e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<go_ast::r#mod::Ident>>> {
    let mut e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(e.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
    {
    let _ts_subject = e.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).unwrap().0.clone();
        return e.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExprPtr>()).unwrap().0.clone();
        {
        let (_, mut ok) = ({
        let val = (*e.lock().unwrap().as_ref().unwrap()).x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::StarExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::StarExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::StarExpr>)), false)
        }
    });;
        if !ok {
            return embedded_field_ident((*e.lock().unwrap().as_ref().unwrap()).x.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).unwrap().0.clone();
        return (*e.lock().unwrap().as_ref().unwrap()).sel.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).unwrap().0.clone();
        return embedded_field_ident((*e.lock().unwrap().as_ref().unwrap()).x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).unwrap().0.clone();
        return embedded_field_ident((*e.lock().unwrap().as_ref().unwrap()).x.clone());;
    }
    }
        // *T is valid, but **T is not
    return Arc::new(Mutex::new(None));
}

impl GoValueClone for Struct {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
