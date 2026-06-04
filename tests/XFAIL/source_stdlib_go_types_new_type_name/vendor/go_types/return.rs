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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<ast_Ident>>>,
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
    pub fn is_terminating(&self, mut s: Arc<Mutex<Option<ast_Stmt>>>, label: Arc<Mutex<Option<String>>>) -> bool {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if _ts_val.and_then(|__v| __v.downcast_ref::<ast_BadStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_DeclStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_EmptyStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_SendStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_IncDecStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_AssignStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_GoStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_DeferStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_RangeStmt>()).is_some() {
        let s = s.clone();
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_LabeledStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_LabeledStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating({ let __field = (*s.lock().unwrap().as_ref().unwrap()).stmt.clone(); __field }, { let __field = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_ExprStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_ExprStmt>()).unwrap().clone())));
        drop(_ts_guard);
        {
        let (mut call, mut ok) = ({
        let val = ast::unparen({ let __go_arg = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<ast_CallExpr>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<ast_CallExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<ast_CallExpr>)), false)
        }
    });;
        if ok && { let __map = { let __map_holder = (*self.environment.lock().unwrap().as_ref().unwrap()).is_panic.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoPtrKey::new(call.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
            return true;;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_ReturnStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_ReturnStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_BranchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_BranchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = token::G_O_T_O; __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = token::F_A_L_L_T_H_R_O_U_G_H; __tmp_x == __tmp_y } {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_BlockStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_BlockStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).list.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_IfStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_IfStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && self.is_terminating({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some("".to_string())))) && self.is_terminating({ let __field = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone(); __field }, Arc::new(Mutex::new(Some("".to_string())))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_SwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_SwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating_switch({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_TypeSwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_TypeSwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return self.is_terminating_switch({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_SelectStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_SelectStmt>()).unwrap().clone())));
        drop(_ts_guard);
        { let __range_holder = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        let mut cc = ({
        let val = s;
        Arc::new(Mutex::new(Some(val.downcast_ref::<ast_CommClause>().expect("type assertion failed").clone())))
    }).clone();
        if !self.is_terminating_list({ let __field = (*cc.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some("".to_string())))) || has_break_list({ let __field = (*cc.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))) {
        return false;
    }
    } };
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_ForStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_ForStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).cond.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))) {
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

    pub fn is_terminating_list(&self, list: Arc<Mutex<Option<Vec<ast_Stmt>>>>, label: Arc<Mutex<Option<String>>>) -> bool {
                // trailing empty statements are permitted - skip them
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
        let (_, mut ok) = ({
        let val = { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        if let Some(typed_val) = val.downcast_ref::<ast_EmptyStmt>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
        } else {
            (Arc::new(Mutex::new(None::<ast_EmptyStmt>)), false)
        }
    });;
        if !ok {
            return self.is_terminating(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        false
    }

    pub fn is_terminating_switch(&self, body: Arc<Mutex<Option<ast_BlockStmt>>>, label: Arc<Mutex<Option<String>>>) -> bool {
        let mut hasDefault = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = (*body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        let mut cc = ({
        let val = s;
        Arc::new(Mutex::new(Some(val.downcast_ref::<ast_CaseClause>().expect("type assertion failed").clone())))
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
pub fn has_break(mut s: Arc<Mutex<Option<ast_Stmt>>>, label: Arc<Mutex<Option<String>>>, implicit: Arc<Mutex<Option<bool>>>) -> bool {
    {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if _ts_val.and_then(|__v| __v.downcast_ref::<ast_BadStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_DeclStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_EmptyStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_ExprStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_SendStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_IncDecStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_AssignStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_GoStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_DeferStmt>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ast_ReturnStmt>()).is_some() {
        let s = s.clone();
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_LabeledStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_LabeledStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break({ let __field = (*s.lock().unwrap().as_ref().unwrap()).stmt.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_BranchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_BranchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = token::B_R_E_A_K; __tmp_x == __tmp_y } {
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return { let __v = (*implicit.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        if { let __tmp_x = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*label.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_BlockStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_BlockStmt>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).list.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_IfStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_IfStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && has_break({ let __field = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_CaseClause>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_CaseClause>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_SwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_SwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_TypeSwitchStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_TypeSwitchStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_CommClause>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_CommClause>()).unwrap().clone())));
        drop(_ts_guard);
        return has_break_list({ let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_SelectStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_SelectStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_ForStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_ForStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ast_RangeStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_RangeStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = (*label.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && has_break({ let __arg = { let __field = (*s.lock().unwrap().as_ref().unwrap()).body.clone(); __field }; let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Stmt> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))) {
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

pub fn has_break_list(list: Arc<Mutex<Option<Vec<ast_Stmt>>>>, label: Arc<Mutex<Option<String>>>, implicit: Arc<Mutex<Option<bool>>>) -> bool {
    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        if has_break(Arc::new(Mutex::new(Some((*s).clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = label.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = implicit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
    } }
    false
}