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
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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


impl crate::check::Checker {
    /// isTerminating reports if s is a terminating statement.
    /// If s is labeled, label is the label name; otherwise s
    /// is "".
    pub fn is_terminating(&self, mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>, label: Arc<Mutex<Option<String>>>) -> bool {
        let mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = s.clone();
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeclStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::EmptyStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SendStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IncDecStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GoStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeferStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmt>()).is_some() {
        let s = s.clone();
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating((*s.lock().unwrap().as_ref().unwrap()).stmt.clone(), { let __field = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmt>()).unwrap().clone())));
        drop(_ts_guard);
        {
        let (mut call, mut ok) = ({
        let val = go_ast::unparen((*s.lock().unwrap().as_ref().unwrap()).x.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CallExpr>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
        }
    });;
        if ok && { let __map = { let __map_holder = (*self.environment.lock().unwrap().as_ref().unwrap()).is_panic.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(call.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
            return true;;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ReturnStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ReturnStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O_T_O as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32)))); __tmp_x == __tmp_y } {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).list.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if (*(*s.lock().unwrap().as_ref().unwrap()).r#else.lock().unwrap()).is_some() && self.is_terminating(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some("".to_string())))) && self.is_terminating((*s.lock().unwrap().as_ref().unwrap()).r#else.clone(), Arc::new(Mutex::new(Some("".to_string())))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating_switch({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating_switch({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmt>()).unwrap().clone())));
        drop(_ts_guard);
        { let __range_holder = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        let mut cc = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CommClause>() {
            Arc::new(Mutex::new(Some(typed_val.clone())))
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        if !self.is_terminating_list({ let __field = (*cc.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some("".to_string())))) || has_break_list({ let __field = (*cc.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))) {
        return false;
    }
    } };
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if (*(*s.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap()).is_none() && !has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))) {
        return true;
    };
    } else {
        let s = s.clone();
        drop(_ts_guard);
        panic!("unreachable");;
    }
    }
                // no chance
                // calling the predeclared (possibly parenthesized) panic() function is terminating
        false
    }

    pub fn is_terminating_list(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>, label: Arc<Mutex<Option<String>>>) -> bool {
                // trailing empty statements are permitted - skip them
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
        let (_, mut ok) = ({
        let val = { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::EmptyStmt>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::EmptyStmt>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::EmptyStmt>)), false)
        }
    });;
        if !ok {
            return self.is_terminating({ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        false
    }

    pub fn is_terminating_switch(&self, body: Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>>, label: Arc<Mutex<Option<String>>>) -> bool {
        let mut hasDefault = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = (*body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        let mut cc = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CaseClause>() {
            Arc::new(Mutex::new(Some(typed_val.clone())))
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        if { let __nil_target = (*cc.lock().unwrap().as_ref().unwrap()).list.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = true; *hasDefault.lock().unwrap() = Some(new_val); };
    }
        if !self.is_terminating_list({ let __field = (*cc.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some("".to_string())))) || has_break_list({ let __field = (*cc.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))) {
        return false;
    }
    } }
        return { let __v = (*hasDefault.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

/// hasBreak reports if s is or contains a break statement
/// referring to the label-ed statement or implicit-ly the
/// closest outer breakable statement.
pub fn has_break(mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>, label: Arc<Mutex<Option<String>>>, implicit: Arc<Mutex<Option<bool>>>) -> bool {
    let mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = s.clone();
    {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeclStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::EmptyStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SendStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IncDecStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GoStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeferStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ReturnStmt>()).is_some() {
        let s = s.clone();
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break((*s.lock().unwrap().as_ref().unwrap()).stmt.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32)))); __tmp_x == __tmp_y } {
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return { let __v = (*implicit.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        if { let __tmp_x = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*label.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).list.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || (*(*s.lock().unwrap().as_ref().unwrap()).r#else.lock().unwrap()).is_some() && has_break((*s.lock().unwrap().as_ref().unwrap()).r#else.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClause>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClause>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClause>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClause>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else {
        let s = s.clone();
        drop(_ts_guard);
        panic!("unreachable");;
    }
    }

        // no chance
    false
}

pub fn has_break_list(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>, label: Arc<Mutex<Option<String>>>, implicit: Arc<Mutex<Option<bool>>>) -> bool {
    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        if has_break(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
    } }
    false
}