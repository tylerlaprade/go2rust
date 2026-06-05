use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_any_slice, format_any_variadic, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values};

use crate::interface::*;
use crate::r#mod::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG_RESOLVE: bool = false;


pub(crate) const MAX_SCOPE_DEPTH: i32 = 1000;


#[derive(Clone)]
pub struct resolver {
    pub handle: Arc<Mutex<Option<go_token::position::File>>>,
    pub decl_err: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>>,
    pub pkg_scope: Arc<Mutex<Option<go_ast::scope::Scope>>>,
    pub top_scope: Arc<Mutex<Option<go_ast::scope::Scope>>>,
    pub unresolved: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>>,
    pub depth: Arc<Mutex<Option<i32>>>,
    pub label_scope: Arc<Mutex<Option<go_ast::scope::Scope>>>,
    pub target_stack: Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>>>,
}

impl resolver {
    pub fn __go_value_clone(&self) -> Self {
        Self { handle: self.handle.clone(), decl_err: self.decl_err.clone(), pkg_scope: self.pkg_scope.clone(), top_scope: self.top_scope.clone(), unresolved: self.unresolved.clone(), depth: { let __guard = self.depth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, label_scope: self.label_scope.clone(), target_stack: self.target_stack.clone() }
    }
}


impl Default for resolver {
    fn default() -> Self {
        Self { handle: Arc::new(Mutex::new(None)), decl_err: Arc::new(Mutex::new(None)), pkg_scope: Arc::new(Mutex::new(None)), top_scope: Arc::new(Mutex::new(None)), unresolved: Arc::new(Mutex::new(None)), depth: Arc::new(Mutex::new(Some(0))), label_scope: Arc::new(Mutex::new(None)), target_stack: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", { let __guard = self.handle.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, "<func>", { let __guard = self.pkg_scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.top_scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice_wrapped(&self.unresolved), (*self.depth.lock().unwrap().as_ref().unwrap()), { let __guard = self.label_scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_nested_slice_wrapped(&self.target_stack))
    }
}

impl GoJsonDecode for resolver {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub lit: Arc<Mutex<Option<String>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lit: { let __guard = self.lit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), lit: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.lit.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static unresolved: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<go_ast::scope::Object>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *unresolved.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *unresolved.lock().unwrap() = Some(Arc::new(Mutex::new(Some(go_ast::scope::Object::default()))));
}


pub(crate) fn __go_zero_globals() {
    *unresolved.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_3() {
    *unresolved.lock().unwrap() = Some(Arc::new(Mutex::new(Some(go_ast::scope::Object::default()))));
}


impl resolver {
    pub fn trace(&self, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        println!("{}", format!("{}", format!("{}{}", (*strings::repeat(Arc::new(Mutex::new(Some(". ".to_string()))), { let __field = self.depth.clone(); __field }).lock().unwrap().as_ref().unwrap()), (*self.sprintf(Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()).lock().unwrap().as_ref().unwrap()))));
    }

    pub fn sprintf(&self, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<String>>> {
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for (i, mut arg) in __range_values.into_iter().enumerate() {
        {
    let _ts_ref = &arg;
    let _ts_is_nil = false;
    let _ts_val: Option<&(dyn Any + Send + Sync)> = Some(_ts_ref.as_ref() as &(dyn Any + Send + Sync));
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_token::position::Pos>()).is_some() {
        let arg = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_token::position::Pos>()).unwrap().clone())));
        (*args.lock().unwrap().as_mut().unwrap())[(i) as usize] = Box::new((*(*self.handle.lock().unwrap().as_ref().unwrap()).position(Arc::new(Mutex::new(Some({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>;;
    }
    }
    } }
        Arc::new(Mutex::new(Some(format!("{}", (*format.lock().unwrap().as_ref().unwrap()).clone()))))
    }

    pub fn open_scope(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        { let __target = self.depth.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*self.depth.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1000; __tmp_x > __tmp_y } {
        panic!("{}", bailout { pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), msg: Arc::new(Mutex::new(Some("exceeded max scope depth during object resolution".to_string()))), ..Default::default() });
    }
        if DEBUG_RESOLVE {
        self.trace(Arc::new(Mutex::new(Some("opening scope @%v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let new_val = go_ast::new_scope({ let __field = self.top_scope.clone(); __field }).clone(); self.top_scope = new_val; };
    }

    pub fn close_scope(&mut self) {
        { let __target = self.depth.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if DEBUG_RESOLVE {
        self.trace(Arc::new(Mutex::new(Some("closing scope".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    }
        { let new_val = (*self.top_scope.lock().unwrap().as_ref().unwrap()).outer.clone(); self.top_scope = new_val; };
    }

    pub fn open_label_scope(&mut self) {
        { let new_val = go_ast::new_scope({ let __field = self.label_scope.clone(); __field }).clone(); self.label_scope = new_val; };
        { let new_val = { let __append_target = self.target_stack.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Default::default()); __append_target.clone() }; self.target_stack = new_val; };
    }

    pub fn close_label_scope(&mut self) {
                // resolve labels
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.target_stack.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut scope = self.label_scope.clone();
        for ident in &{ let __seq = { let __seq_holder = self.target_stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } {
        { let new_val = { let __recv = scope.clone(); let __recv_ptr: *const go_ast::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup({ let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field }); __result }.clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };
        if { let __nil_target = (*ident.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __nil_target = self.decl_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __f_holder = self.decl_err.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some(format!("label {} undefined", (*{ let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()))))) };
    }
    }
                // pop label scope
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.target_stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); self.target_stack = new_val; };
        { let new_val = (*self.label_scope.lock().unwrap().as_ref().unwrap()).outer.clone(); self.label_scope = new_val; };
    }

    pub fn declare(&self, decl: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, data: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, scope: Arc<Mutex<Option<go_ast::scope::Scope>>>, kind: Arc<Mutex<Option<go_ast::scope::ObjKind>>>, idents: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>>) {
        { let __range_holder = idents.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ident in __range_values.iter() {
        if { let __nil_target = (*ident.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        panic!("{}: identifier {} already declared or resolved", (*{ let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }.lock().unwrap().as_ref().unwrap()), (*{ let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    }
        let mut obj = go_ast::new_obj(Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field });
                // remember the corresponding declaration for redeclaration
                // errors and global variable resolution/typechecking phase
        { let new_val = decl.clone(); (*obj.lock().unwrap().as_mut().unwrap()).decl = new_val; };
        { let new_val = data.clone(); (*obj.lock().unwrap().as_mut().unwrap()).data = new_val; };
                // Identifiers (for receiver type parameters) are written to the scope, but
                // never set as the resolved object. See go.dev/issue/50956.
        {
        let (_, mut ok) = ({
        let val = decl.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::Ident>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if !ok {
            { let new_val = obj.clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };;
        }
    }
        if { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        if DEBUG_RESOLVE {
        self.trace(Arc::new(Mutex::new(Some("declaring %s@%v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        {
        let mut alt = { let __recv = scope.clone(); let __recv_ptr: *mut go_ast::scope::Scope = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut go_ast::scope::Scope }; let __result = unsafe { &mut *__recv_ptr }.insert(obj.clone()); __result };;
        if (*alt.lock().unwrap()).is_some() && { let __nil_target = self.decl_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            let mut prevDecl = Arc::new(Mutex::new(Some("".to_string())));;
            {
        let mut pos = { let __recv = alt.clone(); let __recv_ptr: *const go_ast::scope::Object = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::scope::Object }; let __result = unsafe { &*__recv_ptr }.pos(); __result };;
        if go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("\n\tprevious declaration at %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *prevDecl.lock().unwrap() = __moved_val; };;
        }
    };
            { let __f_holder = self.decl_err.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some(format!("{} redeclared in this block{}", (*{ let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), { let __v = (*prevDecl.lock().unwrap().as_ref().unwrap()).clone(); __v }))))) };;
        }
    }
    }
    } }
    }

    pub fn short_var_decl(&self, decl: Arc<Mutex<Option<go_ast::r#mod::AssignStmt>>>) {
                // Go spec: A short variable declaration may redeclare variables
                // provided they were originally declared in the same block with
                // the same type, and at least one of the non-blank variables is new.
        let mut n = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = (*decl.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        {
        let (mut ident, mut isIdent) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if isIdent {
            assert(Arc::new(Mutex::new(Some({ let __nil_target = (*ident.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))), Arc::new(Mutex::new(Some("identifier already declared or resolved".to_string()))));;
            let mut obj = go_ast::new_obj(Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))), { let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field });;
            { let new_val = Box::new(decl.clone()) as Box<dyn Any + Send + Sync>; *(*obj.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap() = Some(new_val); };;
            { let new_val = obj.clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };;
            if { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        if DEBUG_RESOLVE {
        self.trace(Arc::new(Mutex::new(Some("declaring %s@%v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        {
        let mut alt = (*self.top_scope.lock().unwrap().as_mut().unwrap()).insert(obj.clone());;
        if (*alt.lock().unwrap()).is_some() {
            { let new_val = alt.clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };;
        } else {
            { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        }
    }
    };
        }
    }
    } }
                // remember corresponding assignment for other tools
                // redeclaration
                // new declaration
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __nil_target = self.decl_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __f_holder = self.decl_err.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = { let __seq = { let __seq_holder = (*decl.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("no new variables on left side of :=".to_string())))) };
    }
    }

    /// If x is an identifier, resolve attempts to resolve x by looking up
    /// the object it denotes. If no object is found and collectUnresolved is
    /// set, x is marked as unresolved and collected in the list of unresolved
    /// identifiers.
    pub fn resolve(&mut self, ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, collectUnresolved: Arc<Mutex<Option<bool>>>) {
        if { let __nil_target = (*ident.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        panic!("{}", (*self.sprintf(Arc::new(Mutex::new(Some("%v: identifier %s already declared or resolved".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()));
    }
                // '_' should never refer to existing declarations, because it has special
                // handling in the spec.
        if { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        return;
    }
        let mut s = self.top_scope.clone();
    while (*s.lock().unwrap()).is_some() {
        {
        let mut obj = { let __recv = s.clone(); let __recv_ptr: *const go_ast::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup({ let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field }); __result };;
        if (*obj.lock().unwrap()).is_some() {
            if DEBUG_RESOLVE {
        self.trace(Arc::new(Mutex::new(Some("resolved %v:%s to %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(obj.clone()) as Box<dyn Any + Send + Sync>]))));
    };
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*obj.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y }))), Arc::new(Mutex::new(Some("obj with no name".to_string()))));;
            {
        let (_, mut ok) = ({
        let val = (*obj.lock().unwrap().as_ref().unwrap()).decl.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::Ident>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if !ok {
            { let new_val = obj.clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };;
        }
    };
            return;;
        }
    }
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).outer.clone(); s = new_val; };
    }
                // Identifiers (for receiver type parameters) are written to the scope,
                // but never set as the resolved object. See go.dev/issue/50956.
                // all local scopes are known, so any unresolved identifier
                // must be found either in the file scope, package scope
                // (perhaps in another file), or universe scope --- collect
                // them so that they can be resolved later
        if { let __v = (*collectUnresolved.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = (*unresolved.lock().unwrap().as_ref().unwrap()).clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };
        { let new_val = { let __append_target = self.unresolved.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(ident.clone()); __append_target.clone() }; self.unresolved = new_val; };
    }
    }

    pub fn walk_exprs(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for node in __range_values.iter() {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*node.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    } }
    }

    pub fn walk_l_h_s(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut expr in __range_values.iter().cloned() {
        let mut expr = go_ast::unparen(expr.clone());
        {
        let (_, mut ok) = ({
        let val = expr.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if !ok && (*expr.lock().unwrap()).is_some() {
            go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*expr.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        }
    }
    } }
    }

    pub fn walk_stmts(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) {
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*stmt.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    } }
    }

    pub fn visit(&mut self, node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::walk::Visitor + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if DEBUG_RESOLVE && (*node.lock().unwrap()).is_some() {
        self.trace(Arc::new(Mutex::new(Some("node %T@%v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = node.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*node.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        {
    let _ts_subject = node.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Node + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).unwrap().0.clone();
        self.resolve(n.clone(), Arc::new(Mutex::new(Some(true))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncLitPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncLitPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::FuncLit = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncLit }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_func_type({ let __field = (*n.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field });;
        self.walk_body({ let __field = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).unwrap().0.clone();
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructTypePtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::StructType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::StructType }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_field_list({ let __field = (*n.lock().unwrap().as_ref().unwrap()).fields.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncTypePtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::FuncType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncType }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_func_type(n.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CompositeLitPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CompositeLitPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        { let __range_holder = (*n.lock().unwrap().as_ref().unwrap()).elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        {
        let (mut kv, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });;
        if (*kv.lock().unwrap()).is_some() {
            {
        let (mut ident, _) = ({
        let val = (*kv.lock().unwrap().as_ref().unwrap()).key.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if (*ident.lock().unwrap()).is_some() {
            self.resolve(ident.clone(), Arc::new(Mutex::new(Some(false))));;
        } else {
            go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*kv.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        }
    };
            go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*kv.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        } else {
            go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceTypePtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::InterfaceType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::InterfaceType }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_field_list({ let __field = (*n.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::FUN as i32))))))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>()).unwrap().0.clone();
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(n.clone()) as Box<dyn Any + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = { let __field = self.label_scope.clone(); __field }; let __method_arg3 = Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::LBL as i32))))))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3, Arc::new(Mutex::new(Some(vec![{ let __field = (*n.lock().unwrap().as_ref().unwrap()).label.clone(); __field }])))) };;
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).stmt.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        self.walk_exprs({ let __field = (*n.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field });;
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } {
        self.short_var_decl(n.clone());
    } else {
        self.walk_exprs({ let __field = (*n.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmtPtr>()).unwrap().0.clone();
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32)))); __tmp_x != __tmp_y } && { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut depth = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.target_stack.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        { (*self.target_stack.lock().unwrap().as_mut().unwrap())[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].push({ let __field = (*n.lock().unwrap().as_ref().unwrap()).label.clone(); __field }); (*self.target_stack.lock().unwrap().as_ref().unwrap())[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::BlockStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BlockStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_stmts({ let __field = (*n.lock().unwrap().as_ref().unwrap()).list.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::IfStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::IfStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#else.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).r#else.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClausePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClausePtr>()).unwrap().0.clone();
        self.walk_exprs({ let __field = (*n.lock().unwrap().as_ref().unwrap()).list.clone(); __field });;
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::CaseClause = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CaseClause }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_stmts({ let __field = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::SwitchStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::SwitchStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).tag.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        self.open_scope((*(*n.lock().unwrap().as_ref().unwrap()).tag.lock().unwrap().as_ref().unwrap()).pos());
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));
    }
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).tag.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.walk_stmts({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::TypeSwitchStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::TypeSwitchStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result });
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        self.open_scope((*(*n.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap()).pos());;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.walk_stmts({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClausePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClausePtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::CommClause = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CommClause }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).comm.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).comm.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        self.walk_stmts({ let __field = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.walk_stmts({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::ForStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::ForStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).cond.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).post.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).post.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    };
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::RangeStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::RangeStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        let mut lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).key.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = { let __append_target = lhs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*n.lock().unwrap().as_ref().unwrap()).key.clone()); __append_target.clone() }; lhs = new_val; };
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).value.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = { let __append_target = lhs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*n.lock().unwrap().as_ref().unwrap()).value.clone()); __append_target.clone() }; lhs = new_val; };
    };
        if { let __tmp_x = ((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } {
        let mut r#as = Arc::new(Mutex::new(Some(go_ast::r#mod::AssignStmt { lhs: lhs.clone(), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32))))))), tok_pos: Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).tok_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), rhs: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr { op: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_A_N_G_E as i32))))))), x: { let __field = (*n.lock().unwrap().as_ref().unwrap()).x.clone(); __field }, ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))]))), ..Default::default() })));
        self.walk_l_h_s(lhs.clone());
        self.short_var_decl(r#as.clone());
    } else {
        self.walk_exprs(lhs.clone());
    }
    };
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).unwrap().0.clone();
        { let _switch_val = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))) {
            { let __range_holder = (*n.lock().unwrap().as_ref().unwrap()).specs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, mut spec) in __range_values.iter().cloned().enumerate() {
        let mut spec = ({
        let val = spec.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Spec + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ValueSpecPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        let mut kind = Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::CON as i32)))))));
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32)))); *kind.lock().unwrap() = Some(new_val); };
    }
        self.walk_exprs({ let __field = (*spec.lock().unwrap().as_ref().unwrap()).values.clone(); __field });
        if { let __iface_handle = { let __field = (*spec.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*spec.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(spec.clone()) as Box<dyn Any + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(i) as Box<dyn Any + Send + Sync>))); let __method_arg2 = { let __field = self.top_scope.clone(); __field }; let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg4 = { let __field = (*spec.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };
    } }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32))))) {
            { let __range_holder = (*n.lock().unwrap().as_ref().unwrap()).specs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut spec in __range_values.iter().cloned() {
        let mut spec = ({
        let val = spec.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Spec + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::TypeSpecPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(spec.clone()) as Box<dyn Any + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = { let __field = self.top_scope.clone(); __field }; let __method_arg3 = Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::TYP as i32))))))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3, Arc::new(Mutex::new(Some(vec![{ let __field = (*spec.lock().unwrap().as_ref().unwrap()).name.clone(); __field }])))) };
        if { let __nil_target = (*spec.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.open_scope({ let __recv = spec.clone(); let __recv_ptr: *const go_ast::r#mod::TypeSpec = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::TypeSpec }; let __result = unsafe { &*__recv_ptr }.pos(); __result });
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));
        self.walk_t_params({ let __field = (*spec.lock().unwrap().as_ref().unwrap()).type_params.clone(); __field });
    }
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*spec.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    } }
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).unwrap().0.clone();
        self.open_scope({ let __recv = n.clone(); let __recv_ptr: *const go_ast::r#mod::FuncDecl = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncDecl }; let __result = unsafe { &*__recv_ptr }.pos(); __result });;
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_scope();
    }));;
        self.walk_recv({ let __field = (*n.lock().unwrap().as_ref().unwrap()).recv.clone(); __field });;
        if { let __nil_target = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.walk_t_params({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.clone(); __field });
    };
        self.resolve_list({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).params.clone(); __field });;
        self.resolve_list({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).results.clone(); __field });;
        self.declare_list({ let __field = (*n.lock().unwrap().as_ref().unwrap()).recv.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))));;
        self.declare_list({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).params.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))));;
        self.declare_list({ let __field = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))));;
        self.walk_body({ let __field = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); __field });;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __selector_holder = (*(*n.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "init".to_string(); __tmp_x != __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(n.clone()) as Box<dyn Any + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = { let __field = self.pkg_scope.clone(); __field }; let __method_arg3 = Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::FUN as i32))))))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3, Arc::new(Mutex::new(Some(vec![{ let __field = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); __field }])))) };
    };
    } else {
        let n = node.clone();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>)));
    };
    }
    }
                // Expressions.
                // Note: don't try to resolve n.Sel, as we don't support qualified
                // resolution.
                // See go.dev/issue/45160: try to resolve composite lit keys, but don't
                // collect them as unresolved if resolution failed. This replicates
                // existing behavior when resolving during parsing.
                // Statements
                // add to list of unresolved targets
                // The scope below reproduces some unnecessary behavior of the parser,
                // opening an extra scope in case this is a type switch. It's not needed
                // for expression switches.
                // TODO: remove this once we've matched the parser resolution exactly.
                // s.Body consists only of case clauses, so does not get its own
                // scope.
                // as for switch statements, select statement bodies don't get their own
                // scope.
                // Note: we can't exactly match the behavior of object resolution
                // during the parsing pass here, as it uses the position of the RANGE
                // token for the RHS OpPos. That information is not contained within
                // the AST.
                // TODO(rFindley): this walkLHS reproduced the parser resolution, but
                // is it necessary? By comparison, for a normal AssignStmt we don't
                // walk the LHS in case there is an invalid identifier list.
                // Declarations
                // Go spec: The scope of a type identifier declared inside a function begins
                // at the identifier in the TypeSpec and ends at the end of the innermost
                // containing block.
                // Open the function scope.
                // Type parameters are walked normally: they can reference each other, and
                // can be referenced by normal parameters.
                // TODO(rFindley): need to address receiver type parameters.
                // Resolve and declare parameters in a specific order to get duplicate
                // declaration errors in the correct location.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }

    pub fn walk_func_type(&self, typ: Arc<Mutex<Option<go_ast::r#mod::FuncType>>>) {
                // typ.TypeParams must be walked separately for FuncDecls.
        self.resolve_list({ let __field = (*typ.lock().unwrap().as_ref().unwrap()).params.clone(); __field });
        self.resolve_list({ let __field = (*typ.lock().unwrap().as_ref().unwrap()).results.clone(); __field });
        self.declare_list({ let __field = (*typ.lock().unwrap().as_ref().unwrap()).params.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))));
        self.declare_list({ let __field = (*typ.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::VAR as i32))))))));
    }

    pub fn resolve_list(&self, list: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>) {
        if (*list.lock().unwrap()).is_none() {
        return;
    }
        { let __range_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        if { let __iface_handle = { let __field = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    }
    } }
    }

    pub fn declare_list(&self, list: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>, kind: Arc<Mutex<Option<go_ast::scope::ObjKind>>>) {
        if (*list.lock().unwrap()).is_none() {
        return;
    }
        { let __range_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(f.clone()) as Box<dyn Any + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = { let __field = self.top_scope.clone(); __field }; let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg4 = { let __field = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };
    } }
    }

    pub fn walk_recv(&self, recv: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>) {
                // If our receiver has receiver type parameters, we must declare them before
                // trying to resolve the rest of the receiver, and avoid re-resolving the
                // type parameter identifiers.
        if (*recv.lock().unwrap()).is_none() || { let __tmp_x = (({ let __len_target = { let __field = (*recv.lock().unwrap().as_ref().unwrap()).list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return;
    }
                // nothing to do
        let mut typ = (*{ let __seq = { let __seq_holder = (*recv.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).r#type.clone();
        {
        let (mut ptr, mut ok) = ({
        let val = typ.clone();
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
        if ok {
            { let __iface_handle = (*ptr.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        let mut declareExprs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut resolveExprs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = typ.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).unwrap().0.clone();
        { let new_val = Arc::new(Mutex::new(Some(vec![(*typ.lock().unwrap().as_ref().unwrap()).index.clone()]))); declareExprs = new_val; };;
        { let new_val = { let __append_target = resolveExprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*typ.lock().unwrap().as_ref().unwrap()).x.clone()); __append_target.clone() }; resolveExprs = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).unwrap().0.clone();
        { let new_val = (*typ.lock().unwrap().as_ref().unwrap()).indices.clone(); declareExprs = new_val; };;
        { let new_val = { let __append_target = resolveExprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*typ.lock().unwrap().as_ref().unwrap()).x.clone()); __append_target.clone() }; resolveExprs = new_val; };;
    } else {
        let typ = typ.clone();
        { let new_val = { let __append_target = resolveExprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(typ.clone()); __append_target.clone() }; resolveExprs = new_val; };;
    }
    }
        { let __range_holder = declareExprs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for expr in __range_values.iter() {
        {
        let (mut id, _) = ({
        let val = expr.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if (*id.lock().unwrap()).is_some() {
            { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(expr.clone()) as Box<dyn Any + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = { let __field = self.top_scope.clone(); __field }; let __method_arg3 = Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::TYP as i32))))))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3, Arc::new(Mutex::new(Some(vec![id.clone()])))) };;
        } else {
            { let new_val = { let __append_target = resolveExprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*expr).clone()); __append_target.clone() }; resolveExprs = new_val; };;
        }
    }
    } }
                // The receiver type parameter expression is invalid, but try to resolve
                // it anyway for consistency.
        { let __range_holder = resolveExprs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for expr in __range_values.iter() {
        if (*expr.lock().unwrap()).is_some() {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*expr.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    }
    } }
                // The receiver is invalid, but try to resolve it anyway for consistency.
        for f in &{ let __seq = { let __seq_holder = (*recv.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() } {
        if { let __iface_handle = { let __field = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    }
    }
    }

    pub fn walk_field_list(&self, list: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>, kind: Arc<Mutex<Option<go_ast::scope::ObjKind>>>) {
        if (*list.lock().unwrap()).is_none() {
        return;
    }
        self.resolve_list(list.clone());
        self.declare_list(list.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// walkTParams is like walkFieldList, but declares type parameters eagerly so
    /// that they may be resolved in the constraint expressions held in the field
    /// Type.
    pub fn walk_t_params(&self, list: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>) {
        self.declare_list(list.clone(), Arc::new(Mutex::new(Some(go_ast::scope::ObjKind(Arc::new(Mutex::new(Some(go_ast::TYP as i32))))))));
        self.resolve_list(list.clone());
    }

    pub fn walk_body(&mut self, body: Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*body.lock().unwrap()).is_none() {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        self.open_label_scope();
        let mut r_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.close_label_scope();
    }));
        self.walk_stmts({ let __field = (*body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }
}

#[derive(Clone)]
pub struct resolverPtr(pub Arc<Mutex<Option<resolver>>>);

impl std::fmt::Display for resolverPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl go_ast::walk::Visitor for resolverPtr {
    fn visit(&mut self, node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::walk::Visitor + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        resolver::visit(__recv, node)
    }
    fn __go_clone_box_visitor(&self) -> Box<dyn go_ast::walk::Visitor + Send + Sync> {
        Box::new(self.clone()) as Box<dyn go_ast::walk::Visitor + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_visitor(&self, other: &(dyn go_ast::walk::Visitor + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<resolverPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// resolveFile walks the given file to resolve identifiers within the file
/// scope, updating ast.Ident.Obj fields with declaration information.
///
/// If declErr is non-nil, it is used to report declaration errors during
/// resolution. tok is used to format position in error messages.
pub fn resolve_file(file: Arc<Mutex<Option<go_ast::r#mod::File>>>, handle: Arc<Mutex<Option<go_token::position::File>>>, declErr: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>>) {
    let mut pkgScope = go_ast::new_scope(Arc::new(Mutex::new(None)));
    let mut r = Arc::new(Mutex::new(Some(resolver { handle: handle.clone(), decl_err: declErr.clone(), top_scope: pkgScope.clone(), pkg_scope: pkgScope.clone(), depth: Arc::new(Mutex::new(Some(1))), ..Default::default() })));

    { let __range_holder = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for decl in __range_values.iter() {
        go_ast::walk(Arc::new(Mutex::new(Some(Box::new(resolverPtr(r.clone())) as Box<dyn go_ast::walk::Visitor + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new((*decl.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
    } }

    { let __recv = r.clone(); let __recv_ptr: *mut resolver = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut resolver }; let __result = unsafe { &mut *__recv_ptr }.close_scope(); __result };
    assert(Arc::new(Mutex::new(Some({ let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).top_scope.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))), Arc::new(Mutex::new(Some("unbalanced scopes".to_string()))));
    assert(Arc::new(Mutex::new(Some({ let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).label_scope.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))), Arc::new(Mutex::new(Some("unbalanced label scopes".to_string()))));

        // resolve global identifiers within the same file
    let mut i = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = (*r.lock().unwrap().as_ref().unwrap()).unresolved.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ident in __range_values.iter() {
                // i <= index for current ident
        assert(Arc::new(Mutex::new(Some({ let __left = (*ident.lock().unwrap().as_ref().unwrap()).obj.clone(); let __right = (*unresolved.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))), Arc::new(Mutex::new(Some("object already resolved".to_string()))));
        { let new_val = (*(*r.lock().unwrap().as_ref().unwrap()).pkg_scope.lock().unwrap().as_ref().unwrap()).lookup({ let __field = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); __field }).clone(); (*ident.lock().unwrap().as_mut().unwrap()).obj = new_val; };
        if { let __nil_target = (*ident.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        (*(*r.lock().unwrap().as_ref().unwrap()).unresolved.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ident.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else if DEBUG_RESOLVE {
        let mut pos = { let __recv = ({
        let val = (*(*ident.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).decl.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ArrayTypePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::AssignStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BadDeclPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BadExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BadStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BasicLitPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BinaryExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BlockStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::BranchStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::CallExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::CaseClausePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ChanTypePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::CommClausePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::CommentGroupPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::CommentPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::CompositeLitPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::DeclStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::DeferStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::EllipsisPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::EmptyStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ExprStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::FieldListPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::FieldPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::FilePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ForStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::FuncDeclPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::FuncLitPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::FuncTypePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::GenDeclPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::GoStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::IdentPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::IfStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ImportSpecPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::IncDecStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::IndexExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::IndexListExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::InterfaceTypePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::MapTypePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::PackagePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ParenExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::RangeStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ReturnStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::SelectStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::SelectorExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::SendStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::SliceExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::StarExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::StructTypePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::TypeAssertExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::TypeSpecPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::UnaryExprPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::r#mod::ValueSpecPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<go_ast::scope::Object>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result };
        { let __recv = r.clone(); let __recv_ptr: *const resolver = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const resolver }; let __result = unsafe { &*__recv_ptr }.trace(Arc::new(Mutex::new(Some("resolved %s@%v to package object %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    }
    } }
        // i <= index for current ident
        // also removes unresolved sentinel
    { let new_val = (*r.lock().unwrap().as_ref().unwrap()).pkg_scope.clone(); (*file.lock().unwrap().as_mut().unwrap()).scope = new_val; };
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*r.lock().unwrap().as_ref().unwrap()).unresolved.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); (*file.lock().unwrap().as_mut().unwrap()).unresolved = new_val; };
}

pub trait GoAnonymousInterface1: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool;
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
}

impl Clone for Box<dyn GoAnonymousInterface1 + Send + Sync> {
    fn clone(&self) -> Self {
        GoAnonymousInterface1::__go_clone_box_go_anonymous_interface1(self.as_ref())
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ArrayTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ArrayTypePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ArrayTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::AssignStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::AssignStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::AssignStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BadDeclPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BadDeclPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BadDeclPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BadExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BadExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BadExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BadStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BadStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BadStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BasicLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BasicLitPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BasicLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BinaryExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BinaryExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BinaryExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BlockStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BlockStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BlockStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::BranchStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::BranchStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BranchStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::CallExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::CallExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CallExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::CaseClausePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::CaseClausePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CaseClausePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ChanTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ChanTypePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ChanTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::CommClausePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::CommClausePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CommClausePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::CommentGroupPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::CommentGroupPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CommentGroupPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::CommentPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::CommentPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CommentPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::CompositeLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::CompositeLitPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CompositeLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::DeclStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::DeclStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::DeclStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::DeferStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::DeferStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::DeferStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::EllipsisPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::EllipsisPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::EllipsisPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::EmptyStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::EmptyStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::EmptyStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ExprStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ExprStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ExprStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::FieldListPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::FieldListPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FieldListPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::FieldPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::FieldPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FieldPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::FilePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::FilePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FilePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ForStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ForStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ForStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::FuncDeclPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::FuncDeclPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FuncDeclPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::FuncLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::FuncLitPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FuncLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::FuncTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::FuncTypePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FuncTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::GenDeclPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::GenDeclPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::GenDeclPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::GoStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::GoStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::GoStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::IdentPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::IdentPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IdentPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::IfStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::IfStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IfStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ImportSpecPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ImportSpecPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ImportSpecPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::IncDecStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::IncDecStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IncDecStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::IndexExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::IndexExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IndexExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::IndexListExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::IndexListExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IndexListExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::InterfaceTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::InterfaceTypePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::InterfaceTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::KeyValueExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::KeyValueExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::LabeledStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::LabeledStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::LabeledStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::MapTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::MapTypePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::MapTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::PackagePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::PackagePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::PackagePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ParenExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ParenExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ParenExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::RangeStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::RangeStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::RangeStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ReturnStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ReturnStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ReturnStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::SelectStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::SelectStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::SelectStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::SelectorExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::SelectorExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::SelectorExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::SendStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::SendStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::SendStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::SliceExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::SliceExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::SliceExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::StarExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::StarExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::StarExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::StructTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::StructTypePtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::StructTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::SwitchStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::SwitchStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::SwitchStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::TypeAssertExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::TypeAssertExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::TypeAssertExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::TypeSpecPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::TypeSpecPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::TypeSpecPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::TypeSwitchStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::TypeSwitchStmtPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::UnaryExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::UnaryExprPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::UnaryExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::r#mod::ValueSpecPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::r#mod::ValueSpecPtr::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ValueSpecPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for go_ast::scope::Object {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        go_ast::scope::Object::pos(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<go_ast::scope::Object>() {
            false
        } else {
            false
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for resolver {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
