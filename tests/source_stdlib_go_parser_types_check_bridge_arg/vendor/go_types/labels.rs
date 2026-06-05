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
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A block tracks label declarations in a block and its enclosing blocks.
#[derive(Clone, Default)]
pub struct block {
    pub parent: Arc<Mutex<Option<block>>>,
    pub lstmt: Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>,
    pub labels: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>>>>>,
}

impl block {
    pub fn __go_value_clone(&self) -> Self {
        Self { parent: self.parent.clone(), lstmt: self.lstmt.clone(), labels: self.labels.clone() }
    }
}

impl std::fmt::Display for block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.parent.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.lstmt.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_map(&self.labels))
    }
}

impl GoJsonDecode for block {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::check::Checker {
    /// labels checks correct label use in body.
    pub fn labels(&self, body: Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>>) {
                // set of all labels in this body
        let mut all = new_scope(Arc::new(Mutex::new(None)), { let __recv = body.clone(); let __recv_ptr: *const go_ast::r#mod::BlockStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BlockStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __recv = body.clone(); let __recv_ptr: *const go_ast::r#mod::BlockStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BlockStmt }; let __result = unsafe { &*__recv_ptr }.end(); __result }, Arc::new(Mutex::new(Some("label".to_string()))));
        let mut fwdJumps = self.block_branches(all.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), { let __field = (*body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });
                // If there are any forward jumps left, no label was found for
                // the corresponding goto statements. Either those labels were
                // never defined, or they are inside blocks and not reachable
                // for the respective gotos.
        { let __range_holder = fwdJumps.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for jmp in __range_values.iter() {
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut code: Arc<Mutex<Option<Code>>> = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0)))))));
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*jmp.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        {
        let mut alt = { let __recv = all.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*alt.lock().unwrap()).is_some() {
            { let new_val = "goto %s jumps into block".to_string(); *msg.lock().unwrap() = Some(new_val); };;
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(JUMP_INTO_BLOCK as i32)))); *code.lock().unwrap() = Some(new_val); };;
            { let new_val = true; *(*({
        let val = alt.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::LabelPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).used.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = "label %s not declared".to_string(); *msg.lock().unwrap() = Some(new_val); };;
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDECLARED_LABEL as i32)))); *code.lock().unwrap() = Some(new_val); };;
        }
    }
                // avoid another error
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*jmp.lock().unwrap().as_ref().unwrap()).label.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } }
                // avoid another error
                // spec: "It is illegal to define a label that is never used."
        for (name, mut obj) in { let __range_holder = (*all.lock().unwrap().as_ref().unwrap()).elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __iface_handle = resolve(Arc::new(Mutex::new(Some(name.clone()))), obj.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *obj.lock().unwrap() = (*__iface_guard).clone(); };
        {
        let mut lbl = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::LabelPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();;
        if !(*{ let __field = (*lbl.lock().unwrap().as_ref().unwrap()).used.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::LabelPtr(lbl.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_LABEL as i32))))))), Arc::new(Mutex::new(Some("label %s declared and not used".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*lbl.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));;
        }
    }
    }
    }

    /// blockBranches processes a block's statement list and returns the set of outgoing forward jumps.
    /// all is the scope of all declared labels, parent the set of labels declared in the immediately
    /// enclosing block, and lstmt is the labeled statement this block is associated with (or nil).
    pub fn block_branches(&self, all: Arc<Mutex<Option<Scope>>>, parent: Arc<Mutex<Option<block>>>, lstmt: Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>>>>> {
        let mut b = Arc::new(Mutex::new(Some(block { parent: parent.clone(), lstmt: lstmt.clone(), ..Default::default() })));
        let mut varDeclPos: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));let mut fwdJumps: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>>>>> = Arc::new(Mutex::new(None));let mut badJumps: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>>>>> = Arc::new(Mutex::new(None));
                // All forward jumps jumping over a variable declaration are possibly
                // invalid (they may still jump out of the block and be ok).
                // recordVarDecl records them for the given position.
        let mut badJumps_closure_clone = badJumps.clone(); let fwdJumps_closure_clone = fwdJumps.clone(); let mut varDeclPos_closure_clone = varDeclPos.clone(); let mut recordVarDecl = Arc::new(Mutex::new(Some(Box::new(move |pos: Arc<Mutex<Option<go_token::position::Pos>>>| {
        { let new_val = pos.lock().unwrap().as_ref().unwrap().clone(); *varDeclPos_closure_clone.lock().unwrap() = Some(new_val); };
        { let __append_target = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = badJumps_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = fwdJumps_closure_clone.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>) -> () + Send + Sync>)));
                // copy fwdJumps to badJumps
        let badJumps_closure_clone = badJumps.clone(); let varDeclPos_closure_clone = varDeclPos.clone(); let mut jumpsOverVarDecl = Arc::new(Mutex::new(Some(Box::new(move |jmp: Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>| -> bool {
        return go_token::position::Pos::is_valid(&(*varDeclPos_closure_clone.lock().unwrap().as_ref().unwrap())) && slices::contains::<Vec<Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>>, go_ast::r#mod::BranchStmt>(badJumps_closure_clone.clone(), jmp.clone());
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>) -> bool + Send + Sync>)));
        let all_closure_clone = all.clone(); let b_closure_clone = b.clone(); let mut check_closure_clone = (*self).clone(); let mut fwdJumps_closure_clone = fwdJumps.clone(); let mut blockBranches = Arc::new(Mutex::new(Some(Box::new(move |lstmt: Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>| {
        { let __append_target = fwdJumps_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = check_closure_clone.block_branches(all_closure_clone.clone(), b_closure_clone.clone(), lstmt.clone(), list.clone()).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync>)));
                // Unresolved forward jumps inside the nested block
                // become forward jumps in the current block.
        let mut stmtBranches: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let all_closure_clone = all.clone(); let b_closure_clone = b.clone(); let blockBranches_closure_clone = blockBranches.clone(); let mut check_closure_clone = (*self).clone(); let mut fwdJumps_closure_clone = fwdJumps.clone(); let jumpsOverVarDecl_closure_clone = jumpsOverVarDecl.clone(); let mut lstmt_closure_clone = lstmt.clone(); let recordVarDecl_closure_clone = recordVarDecl.clone(); let stmtBranches_closure_clone = stmtBranches.clone(); let varDeclPos_closure_clone = varDeclPos.clone(); { let __func_lit_target = stmtBranches_closure_clone.clone(); let new_val = Box::new(move |mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>| {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeclStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeclStmtPtr>()).unwrap().0.clone();
        {
        let (mut d, _) = ({
        let val = (*s.lock().unwrap().as_ref().unwrap()).decl.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Decl + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::GenDeclPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::GenDecl>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::GenDecl>)), false)
        }
    });;
        if (*d.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))); __tmp_x == __tmp_y } {
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>) -> () + Send + Sync> = { let mut __f_guard = recordVarDecl_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = d.clone(); let __recv_ptr: *const go_ast::r#mod::GenDecl = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::GenDecl }; let __result = unsafe { &*__recv_ptr }.pos(); __result }) };;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>()).unwrap().0.clone();
        {
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
            let mut lbl = new_label((*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).pos(), { let __field = check_closure_clone.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            {
        let mut alt = { let __recv = all_closure_clone.clone(); let __recv_ptr: *mut crate::scope::Scope = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::scope::Scope }; let __result = unsafe { &mut *__recv_ptr }.insert(Arc::new(Mutex::new(Some(Box::new(crate::object::LabelPtr(lbl.clone())) as Box<dyn Object + Send + Sync>)))); __result };;
        if (*alt.lock().unwrap()).is_some() {
            let mut err = check_closure_clone.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_LABEL as i32))))))));;
            { let new_val = true; *(*err.lock().unwrap().as_ref().unwrap()).soft.lock().unwrap() = Some(new_val); };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::object::LabelPtr(lbl.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("label %s already declared".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(alt.clone()); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
        } else {
            { let __recv = b_closure_clone.clone(); let __recv_ptr: *mut block = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut block }; let __result = unsafe { &mut *__recv_ptr }.insert(s.clone()); __result };;
            check_closure_clone.record_def({ let __field = (*s.lock().unwrap().as_ref().unwrap()).label.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::object::LabelPtr(lbl.clone())) as Box<dyn Object + Send + Sync>))));;
        }
    };
            let mut i = Arc::new(Mutex::new(Some(0)));;
            { let __range_holder = fwdJumps_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for jmp in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*(*jmp.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        { let new_val = true; *(*lbl.lock().unwrap().as_ref().unwrap()).used.lock().unwrap() = Some(new_val); };
        check_closure_clone.record_use({ let __field = (*jmp.lock().unwrap().as_ref().unwrap()).label.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::object::LabelPtr(lbl.clone())) as Box<dyn Object + Send + Sync>))));
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>) -> bool + Send + Sync> = { let mut __f_guard = jumpsOverVarDecl_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*jmp).clone()) } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*jmp.lock().unwrap().as_ref().unwrap()).label.clone())) as Box<dyn positioner + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(JUMP_OVER_DECL as i32))))))); let __method_arg2 = Arc::new(Mutex::new(Some("goto %s jumps over variable declaration at line %d".to_string()))); check_closure_clone.soft_errorf(__method_arg0, __method_arg1, __method_arg2, Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*(*check_closure_clone.fset.lock().unwrap().as_ref().unwrap()).position(Arc::new(Mutex::new(Some({ let __arg_holder = varDeclPos_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) };
    }
    } else {
        (*fwdJumps_closure_clone.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = jmp.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } };
            { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = fwdJumps_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fwdJumps_closure_clone.lock().unwrap() = __moved_val; };;
            { let new_val = s.clone(); lstmt_closure_clone = new_val; };;
        }
    };
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*s.lock().unwrap().as_ref().unwrap()).stmt.clone(); __field }) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmtPtr>()).unwrap().0.clone();
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return;
    };
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        { let _switch_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32))))) {
            let mut valid = Arc::new(Mutex::new(Some(false)));
            {
        let mut t = { let __recv = b_closure_clone.clone(); let __recv_ptr: *const block = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const block }; let __result = unsafe { &*__recv_ptr }.enclosing_target(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*t.lock().unwrap()).is_some() {
            {
    let _ts_subject = (*t.lock().unwrap().as_ref().unwrap()).stmt.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).is_some() {
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
    }
    };
        }
    }
            if !{ let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*s.lock().unwrap().as_ref().unwrap()).label.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_LABEL as i32))))))), Arc::new(Mutex::new(Some("invalid break label %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        return;
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_T_I_N_U_E as i32))))) {
            let mut valid = Arc::new(Mutex::new(Some(false)));
            {
        let mut t = { let __recv = b_closure_clone.clone(); let __recv_ptr: *const block = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const block }; let __result = unsafe { &*__recv_ptr }.enclosing_target(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*t.lock().unwrap()).is_some() {
            {
    let _ts_subject = (*t.lock().unwrap().as_ref().unwrap()).stmt.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).is_some() {
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
    }
    };
        }
    }
            if !{ let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*s.lock().unwrap().as_ref().unwrap()).label.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_LABEL as i32))))))), Arc::new(Mutex::new(Some("invalid continue label %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        return;
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O_T_O as i32))))) {
            if (*{ let __recv = b_closure_clone.clone(); let __recv_ptr: *const block = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const block }; let __result = unsafe { &*__recv_ptr }.goto_target(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }.lock().unwrap()).is_none() {
        { let __append_target = fwdJumps_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(s.clone()); __append_target.clone() };
        return;
    }
        } else {
            check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BranchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("branch statement: %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
            return;
        }
    };
        let mut obj = { let __recv = all_closure_clone.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        { let new_val = true; *(*({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::LabelPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).used.lock().unwrap() = Some(new_val); };;
        check_closure_clone.record_use({ let __field = (*s.lock().unwrap().as_ref().unwrap()).label.clone(); __field }, obj.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>) -> () + Send + Sync> = { let mut __f_guard = recordVarDecl_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __recv = s.clone(); let __recv_ptr: *const go_ast::r#mod::AssignStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::AssignStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result }) };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync> = { let mut __f_guard = blockBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(lstmt_closure_clone.clone(), { let __field = (*s.lock().unwrap().as_ref().unwrap()).list.clone(); __field }) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))) };;
        if { let __iface_handle = { let __field = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone(); __field }) };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClausePtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClausePtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync> = { let mut __f_guard = blockBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None)), { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClausePtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClausePtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync> = { let mut __f_guard = blockBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None)), { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))) };;
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync>; *__func_lit_target.lock().unwrap() = Some(new_val); };
                // declare non-blank label
                // ok to continue
                // resolve matching forward jumps and remove them from fwdJumps
                // match
                // ok to continue
                // no match - record new forward jump
                // checked in 1st pass (check.stmt)
                // determine and validate target
                // spec: "If there is a label, it must be that of an enclosing
                // "for", "switch", or "select" statement, and that is the one
                // whose execution terminates."
                // spec: "If there is a label, it must be that of an enclosing
                // "for" statement, and that is the one whose execution advances."
                // label may be declared later - add branch to forward jumps
                // record label use
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = stmtBranches.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(s.clone()) };
    } }
        return fwdJumps.clone();
    }
}

impl block {
    /// insert records a new label declaration for the current block.
    /// The label must not have been declared before in any block.
    pub fn insert(&mut self, s: Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>) {
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if DEBUG {
        assert(Arc::new(Mutex::new(Some((*self.goto_target(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap()).is_none()))));
    }
        let mut labels = self.labels.clone();
        if (*labels.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>>>::new()))); labels = new_val; };
        { let new_val = labels.clone(); self.labels = new_val; };
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = s.clone(); (*labels.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    /// gotoTarget returns the labeled statement in the current
    /// or an enclosing block with the given label name, or nil.
    pub fn goto_target(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>> {
        let mut s = Arc::new(Mutex::new(Some(self.clone())));
    while (*s.lock().unwrap()).is_some() {
        {
        let mut t = { let __map = { let __map_holder = (*s.lock().unwrap().as_ref().unwrap()).labels.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*t.lock().unwrap()).is_some() {
            return t.clone();;
        }
    }
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); s = new_val; };
    }
        return Arc::new(Mutex::new(None));
    }

    /// enclosingTarget returns the innermost enclosing labeled
    /// statement with the given label name, or nil.
    pub fn enclosing_target(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<go_ast::r#mod::LabeledStmt>>> {
        let mut s = Arc::new(Mutex::new(Some(self.clone())));
    while (*s.lock().unwrap()).is_some() {
        {
        let mut t = (*s.lock().unwrap().as_ref().unwrap()).lstmt.clone();;
        if (*t.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*(*t.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
            return t.clone();;
        }
    }
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); s = new_val; };
    }
        return Arc::new(Mutex::new(None));
    }
}

impl GoValueClone for block {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
