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
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
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

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    /// check may be nil.
    pub fn sprintf(&self, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<String>>> {
        let mut fset: Arc<Mutex<Option<go_token::position::FileSet>>> = Arc::new(Mutex::new(None));
        let mut qf: crate::typestring::Qualifier = Arc::new(Mutex::new(None));
        if true {
        { let new_val = self.fset.clone(); fset = new_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }))); qf = new_val; };
    }
        return sprintf(fset.clone(), qf.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone());
    }

    pub fn trace(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        print!("{}:\t{}{}\n", (*(*self.fset.lock().unwrap().as_ref().unwrap()).position(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some({ let __s = ".  ".to_string(); let __count = { let __selector_holder = self.indent.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __s.repeat(__count as usize) }))).lock().unwrap().as_ref().unwrap()), (*sprintf({ let __field = self.fset.clone(); __field }, Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()).lock().unwrap().as_ref().unwrap()));
    }

    /// dump is only needed for debugging
    pub fn dump(&self, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        println!("{}", format!("{}", (*sprintf({ let __field = self.fset.clone(); __field }, Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()).lock().unwrap().as_ref().unwrap())));
    }

    pub fn qualifier(&mut self, pkg: Arc<Mutex<Option<Package>>>) -> Arc<Mutex<Option<String>>> {
                // Qualify the package unless it's the package being type-checked.
        if { let __left = pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        if { let __nil_target = self.pkg_path_map.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>::new()))); self.pkg_path_map = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::package::Package>, Arc<Mutex<Option<bool>>>>::new()))); self.seen_pkg_map = new_val; };
        { let __method_arg0 = { let __field = self.pkg.clone(); __field }; self.mark_imports(__method_arg0) };
    }
                // If the same package name was used by multiple packages, display the full path.
        if { let __tmp_x = ((*{ let __map = { let __map_holder = self.pkg_path_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        return Arc::new(Mutex::new(Some(format!("{:?}", (*(*pkg.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).clone()))));
    }
        return Arc::new(Mutex::new(Some({ let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
                // If the same package name was used by multiple packages, display the full path.
        Arc::new(Mutex::new(Some("".to_string())))
    }

    /// markImports recursively walks pkg and its imports, to record unique import
    /// paths in pkgPathMap.
    pub fn mark_imports(&mut self, pkg: Arc<Mutex<Option<Package>>>) {
        if { let __map = { let __map_holder = self.seen_pkg_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(pkg.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        return;
    }
        { let __map_key = GoLocalPtrKey::new(pkg.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.seen_pkg_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        let (mut forName, mut ok) = { let __map = { let __map_holder = self.pkg_path_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::new()))); forName = new_val; };
        { let __map_key = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __map_value = forName.clone(); (*self.pkg_path_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        { let __map_key = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some(true))); (*forName.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let __range_holder = (*pkg.lock().unwrap().as_ref().unwrap()).imports.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for imp in __range_values.iter() {
        self.mark_imports((*imp).clone());
    } }
    }
}

pub fn sprintf(fset: Arc<Mutex<Option<go_token::position::FileSet>>>, qf: crate::typestring::Qualifier, tpSubscripts: Arc<Mutex<Option<bool>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<String>>> {
    { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = { let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); drop(__range_guard); let mut __range_values = Vec::with_capacity(__range_len); for __range_index in 0..__range_len { let mut __range_guard = __range_holder.lock().unwrap(); let __range_value = std::mem::replace(&mut __range_guard.as_mut().unwrap()[__range_index], Box::new(()) as Box<dyn Any + Send + Sync>); drop(__range_guard); __range_values.push(__range_value); } __range_values }; for (i, mut arg) in __range_values.into_iter().enumerate() {
        {
    let _ts_ref = &arg;
    let _ts_is_nil = false;
    let _ts_val: Option<&(dyn Any + Send + Sync)> = Some(_ts_ref.as_ref() as &(dyn Any + Send + Sync));
    if _ts_is_nil {
        let a = _ts_val.unwrap();
        { let new_val = Box::new("<nil>".to_string()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::operand::operand>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::operand::operand>()).unwrap().clone())));
        panic!("got operand instead of *operand");;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::operand::operand>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::operand::operand>()).unwrap().clone())));
        { let new_val = Box::new((*operand_string(a.clone(), qf.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_token::position::Pos>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_token::position::Pos>()).unwrap().clone())));
        if (*fset.lock().unwrap()).is_some() {
        { let new_val = Box::new((*{ let __recv = { let __recv = fset.clone(); let __recv_ptr: *const go_token::position::FileSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::FileSet }; let __result = unsafe { &*__recv_ptr }.position(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>()).unwrap().clone())));
        { let new_val = Box::new((*expr_string(a.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>()).unwrap().clone())));
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));;
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(('[' as i32) as u8);;
        write_expr_list(buf.clone(), a.clone());;
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte((']' as i32) as u8);;
        { let new_val = Box::new((*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::LabelPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::NilPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::PkgNamePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::objectPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::scope::lazyObjectPtr>()).is_some() {
        let a: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = unimplemented!("type info required: type switch on interface case with 10 concrete implementors needs a synthesized trait object");
        { let new_val = Box::new((*object_string(a.clone(), qf.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let a: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = unimplemented!("type info required: type switch on interface case with 14 concrete implementors needs a synthesized trait object");
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));;
        let mut w = new_type_writer(buf.clone(), qf.clone());;
        { let new_val = tpSubscripts.lock().unwrap().as_ref().unwrap().clone(); *(*w.lock().unwrap().as_ref().unwrap()).tp_subscripts.lock().unwrap() = Some(new_val); };;
        { let __recv = w.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.typ(a.clone()); __result };;
        { let new_val = Box::new((*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>()).unwrap().clone())));
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));;
        let mut w = new_type_writer(buf.clone(), qf.clone());;
        { let new_val = tpSubscripts.lock().unwrap().as_ref().unwrap().clone(); *(*w.lock().unwrap().as_ref().unwrap()).tp_subscripts.lock().unwrap() = Some(new_val); };;
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(('[' as i32) as u8);;
        { let __range_holder = a.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(", ".to_string());
    }
        { let __recv = w.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.typ(x.clone()); __result };
    } };
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte((']' as i32) as u8);;
        { let new_val = Box::new((*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>>()).is_some() {
        let a = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>>()).unwrap().clone())));
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));;
        let mut w = new_type_writer(buf.clone(), qf.clone());;
        { let new_val = tpSubscripts.lock().unwrap().as_ref().unwrap().clone(); *(*w.lock().unwrap().as_ref().unwrap()).tp_subscripts.lock().unwrap() = Some(new_val); };;
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(('[' as i32) as u8);;
        { let __range_holder = a.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(", ".to_string());
    }
        { let __recv = w.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.typ(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(x.clone())) as Box<dyn Type + Send + Sync>)))); __result };
    } };
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte((']' as i32) as u8);;
        { let new_val = Box::new((*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>; arg = new_val; };;
    }
    }
        (*args.lock().unwrap().as_mut().unwrap())[(i) as usize] = arg;
    } }
    Arc::new(Mutex::new(Some(format!("{}", (*format.lock().unwrap().as_ref().unwrap()).clone()))))
}

/// stripAnnotations removes internal (type) annotations from s.
pub fn strip_annotations(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut buf: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(Default::default())));
    for (_, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
                // strip #'s and subscript digits
        if { let __tmp_x = r; let __tmp_y = '\u{2080}'; __tmp_x < __tmp_y } || { let __tmp_x = { let __tmp_x = ('\u{2080}' as i32); let __tmp_y = 10; __tmp_x + __tmp_y } as i32; let __tmp_y = (r as i32); __tmp_x <= __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).push(r);
    }
    }
        // strip #'s and subscript digits
        // '₀' == U+2080
    if { let __tmp_x = ((*buf.lock().unwrap().as_ref().unwrap()).len() as i32 as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __builder = buf.clone(); let __guard = __builder.lock().unwrap(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    }
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}