use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_any_slice, format_any_variadic, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values};

use crate::interface::*;
use crate::resolver::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_NEST_LEV: i32 = 100000;


pub(crate) const BASIC: i32 = 0;
pub(crate) const LABEL_OK: i32 = 1;
pub(crate) const RANGE_OK: i32 = 2;


/// The parser structure holds the parser's internal state.
#[derive(Clone)]
pub struct parser {
    pub file: Arc<Mutex<Option<go_token::position::File>>>,
    pub errors: Arc<Mutex<Option<go_scanner::errors::ErrorList>>>,
    pub scanner: Arc<Mutex<Option<go_scanner::r#mod::Scanner>>>,
    pub mode: Arc<Mutex<Option<Mode>>>,
    pub trace: Arc<Mutex<Option<bool>>>,
    pub indent: Arc<Mutex<Option<i32>>>,
    pub comments: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>>>>>,
    pub lead_comment: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>,
    pub line_comment: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>,
    pub top: Arc<Mutex<Option<bool>>>,
    pub go_version: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub tok: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub lit: Arc<Mutex<Option<String>>>,
    pub sync_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub sync_cnt: Arc<Mutex<Option<i32>>>,
    pub expr_lev: Arc<Mutex<Option<i32>>>,
    pub in_rhs: Arc<Mutex<Option<bool>>>,
    pub imports: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::ImportSpec>>>>>>>,
    pub nest_lev: Arc<Mutex<Option<i32>>>,
}

impl parser {
    pub fn __go_value_clone(&self) -> Self {
        Self { file: self.file.clone(), errors: self.errors.clone(), scanner: { let __guard = self.scanner.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, trace: { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, indent: { let __guard = self.indent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, comments: self.comments.clone(), lead_comment: self.lead_comment.clone(), line_comment: self.line_comment.clone(), top: { let __guard = self.top.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go_version: { let __guard = self.go_version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tok: { let __guard = self.tok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lit: { let __guard = self.lit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sync_pos: { let __guard = self.sync_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sync_cnt: { let __guard = self.sync_cnt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, expr_lev: { let __guard = self.expr_lev.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_rhs: { let __guard = self.in_rhs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, imports: self.imports.clone(), nest_lev: { let __guard = self.nest_lev.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for parser {
    fn default() -> Self {
        Self { file: Arc::new(Mutex::new(None)), errors: Arc::new(Mutex::new(Some(Default::default()))), scanner: Arc::new(Mutex::new(Some(Default::default()))), mode: Arc::new(Mutex::new(Some(crate::interface::Mode(Arc::new(Mutex::new(Some(0))))))), trace: Arc::new(Mutex::new(Some(false))), indent: Arc::new(Mutex::new(Some(0))), comments: Arc::new(Mutex::new(None)), lead_comment: Arc::new(Mutex::new(None)), line_comment: Arc::new(Mutex::new(None)), top: Arc::new(Mutex::new(Some(false))), go_version: Arc::new(Mutex::new(Some(String::new()))), pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), lit: Arc::new(Mutex::new(Some(String::new()))), sync_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), sync_cnt: Arc::new(Mutex::new(Some(0))), expr_lev: Arc::new(Mutex::new(Some(0))), in_rhs: Arc::new(Mutex::new(Some(false))), imports: Arc::new(Mutex::new(None)), nest_lev: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.errors.lock().unwrap().as_ref().unwrap()), (*self.scanner.lock().unwrap().as_ref().unwrap()), (*self.mode.lock().unwrap().as_ref().unwrap()), (*self.trace.lock().unwrap().as_ref().unwrap()), (*self.indent.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.comments), { let __guard = self.lead_comment.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.line_comment.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.top.lock().unwrap().as_ref().unwrap()), (*self.go_version.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.tok.lock().unwrap().as_ref().unwrap()), (*self.lit.lock().unwrap().as_ref().unwrap()), (*self.sync_pos.lock().unwrap().as_ref().unwrap()), (*self.sync_cnt.lock().unwrap().as_ref().unwrap()), (*self.expr_lev.lock().unwrap().as_ref().unwrap()), (*self.in_rhs.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.imports), (*self.nest_lev.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for parser {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A bailout panic is raised to indicate early termination. pos and msg are
/// only populated when bailing out of object resolution.
#[derive(Clone)]
pub struct bailout {
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub msg: Arc<Mutex<Option<String>>>,
}

impl bailout {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, msg: { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for bailout {
    fn default() -> Self {
        Self { pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), msg: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for bailout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.msg.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for bailout {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct field {
    pub name: Arc<Mutex<Option<go_ast::r#mod::Ident>>>,
    pub typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
}

impl field {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: self.name.clone(), typ: self.typ.clone() }
    }
}

impl std::fmt::Display for field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.typ.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for field {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type parseSpecFunction = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync>>>>;


pub(crate) static stmtStart: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static declStart: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static exprEnd: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *stmtStart.lock().unwrap() = Some(BTreeMap::new());
    *declStart.lock().unwrap() = Some(BTreeMap::new());
    *exprEnd.lock().unwrap() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>::new();
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_T_I_N_U_E as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_E_R as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_O_R as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O_T_O as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_F as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_T_U_R_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_L_E_C_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_W_I_T_C_H as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))), Arc::new(Mutex::new(Some(true))));
        *stmtStart.lock().unwrap() = Some(__go_map);
    }
    {
        let mut __go_map = BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>::new();
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))), Arc::new(Mutex::new(Some(true))));
        *declStart.lock().unwrap() = Some(__go_map);
    }
    {
        let mut __go_map = BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>::new();
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))), Arc::new(Mutex::new(Some(true))));
        *exprEnd.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_zero_globals() {
    *stmtStart.lock().unwrap() = Some(BTreeMap::new());
    *declStart.lock().unwrap() = Some(BTreeMap::new());
    *exprEnd.lock().unwrap() = Some(BTreeMap::new());
}


pub(crate) fn __go_init_order_0() {
    {
        let mut __go_map = BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>::new();
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_T_I_N_U_E as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_E_R as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_O_R as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O_T_O as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_F as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_T_U_R_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_L_E_C_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_W_I_T_C_H as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))), Arc::new(Mutex::new(Some(true))));
        *stmtStart.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_init_order_1() {
    {
        let mut __go_map = BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>::new();
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))), Arc::new(Mutex::new(Some(true))));
        *declStart.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_init_order_2() {
    {
        let mut __go_map = BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>::new();
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))), Arc::new(Mutex::new(Some(true))));
        __go_map.insert(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))), Arc::new(Mutex::new(Some(true))));
        *exprEnd.lock().unwrap() = Some(__go_map);
    }
}


impl parser {
    pub fn init(&mut self, file: Arc<Mutex<Option<go_token::position::File>>>, src: Arc<Mutex<Option<Vec<u8>>>>, mode: Arc<Mutex<Option<Mode>>>) {
        { let new_val = file.clone(); self.file = new_val; };
        let mut p_closure_clone = (*self).clone(); let mut eh = Arc::new(Mutex::new(Some(Box::new(move |pos: Arc<Mutex<Option<go_token::position::Position>>>, msg: Arc<Mutex<Option<String>>>| {
        (*p_closure_clone.errors.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Position>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>)));
        (*self.scanner.lock().unwrap().as_mut().unwrap()).init({ let __field = self.file.clone(); __field }, src.clone(), eh.clone(), Arc::new(Mutex::new(Some(go_scanner::r#mod::Mode(Arc::new(Mutex::new(Some(go_scanner::SCAN_COMMENTS as u64))))))));
        { let new_val = true; *self.top.lock().unwrap() = Some(new_val); };
        { let new_val = mode.lock().unwrap().as_ref().unwrap().clone(); *self.mode.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & TRACE as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y }; *self.trace.lock().unwrap() = Some(new_val); };
        self.next();
    }

    pub fn print_trace(&self, a: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        const dots: &'static str = ". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ";

        const n: i32 = dots.len() as i32;

        let mut pos = (*self.file.lock().unwrap().as_ref().unwrap()).position({ let __field = self.pos.clone(); __field });
        print!("{:5}:{:3}: ", (*{ let __field = (*pos.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()), (*{ let __field = (*pos.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = (*self.indent.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x > __tmp_y } {
        print!("{}", format!("{}", dots));
        { let __rhs = 64; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // i <= n
        print!("{}", format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &(dots); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap())));
        println!("{}", format_any_variadic(&a));
    }

    /// Advance to the next token.
    pub fn next0(&mut self) {
                // Because of one-token look-ahead, print the previous token
                // when tracing as it provides a more readable output. The
                // very first token (!p.pos.IsValid()) is not initialized
                // (it is token.ILLEGAL), so don't print it.
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) && go_token::position::Pos::is_valid(&(*self.pos.lock().unwrap().as_ref().unwrap())) {
        let mut s = go_token::r#mod::Token::string(&(*self.tok.lock().unwrap().as_ref().unwrap()));
        if go_token::r#mod::Token::is_literal(&(*self.tok.lock().unwrap().as_ref().unwrap())) {
            { self.print_trace(Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) };
        } else if go_token::r#mod::Token::is_operator(&(*self.tok.lock().unwrap().as_ref().unwrap())) || go_token::r#mod::Token::is_keyword(&(*self.tok.lock().unwrap().as_ref().unwrap())) {
            self.print_trace(Arc::new(Mutex::new(Some(vec![Box::new({ let mut __s = String::new(); __s.push_str(&format!("{}", "\"".to_string())); __s.push_str(&format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "\"".to_string())); __s }) as Box<dyn Any + Send + Sync>]))));
        } else {
            self.print_trace(Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        }
    }
        loop {
        { let (__tmp_0, __tmp_1, __tmp_2) = (*self.scanner.lock().unwrap().as_mut().unwrap()).scan(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.pos.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *self.tok.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *self.lit.lock().unwrap() = __moved_tmp_2; };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_E_N_T as i32)))); __tmp_x == __tmp_y } {
        if (*self.top.clone().lock().unwrap().as_ref().unwrap()) && strings::has_prefix({ let __field = self.lit.clone(); __field }, Arc::new(Mutex::new(Some("//go:build".to_string())))) {
        {
        let (mut x, mut err) = constraint::parse({ let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });;
        if (*err.lock().unwrap()).is_none() {
            { let new_val = constraint::go_version(x.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.go_version.lock().unwrap() = __moved_val; };;
        }
    }
    }
        if { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & PARSE_COMMENTS as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        continue
    }
    } else {
                // Found a non-comment; top of file is over.
        { let new_val = false; *self.top.lock().unwrap() = Some(new_val); };
    }
                // Found a non-comment; top of file is over.
        break
    }
    }

    /// Consume a comment and return it and the line on which it ends.
    pub fn consume_comment(&mut self) -> (Arc<Mutex<Option<go_ast::r#mod::Comment>>>, i32) {
    let mut comment: Arc<Mutex<Option<go_ast::r#mod::Comment>>> = Arc::new(Mutex::new(None));
    let mut endline: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

                // /*-style comments may end on a different line than where they start.
                // Scan the comment for '\n' chars and adjust endline accordingly.
        { let new_val = (*self.file.lock().unwrap().as_ref().unwrap()).line({ let __field = self.pos.clone(); __field }); *endline.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __s = &((*self.lit.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y } {
                // don't use range here - no need to decode Unicode code points
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*self.lit.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*self.lit.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } {
        { let mut guard = endline.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // don't use range here - no need to decode Unicode code points
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::Comment { slash: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), text: Arc::new(Mutex::new(Some({ let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone(); comment = new_val; };
        self.next0();
        return (comment, (*endline.lock().unwrap().as_ref().unwrap()));
    }

    /// Consume a group of adjacent comments, add it to the parser's
    /// comments list, and return it together with the line at which
    /// the last comment in the group ends. A non-comment token or n
    /// empty lines terminate a comment group.
    pub fn consume_comment_group(&mut self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, i32) {
    let mut comments: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>> = Arc::new(Mutex::new(None));
    let mut endline: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Comment>>>>>>> = Arc::new(Mutex::new(None));
        { let new_val = (*self.file.lock().unwrap().as_ref().unwrap()).line({ let __field = self.pos.clone(); __field }); *endline.lock().unwrap() = Some(new_val); };
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_E_N_T as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = (*self.file.lock().unwrap().as_ref().unwrap()).line({ let __field = self.pos.clone(); __field }); let __tmp_y = { let __tmp_x = { let __v = (*endline.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut comment: Arc<Mutex<Option<go_ast::r#mod::Comment>>> = Arc::new(Mutex::new(None));
        { let (__tmp_0, __tmp_1) = self.consume_comment(); comment = __tmp_0.clone(); *endline.lock().unwrap() = Some(__tmp_1); };
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(comment.clone()); __append_target.clone() }; list = new_val; };
    }
                // add comment group to the comments list
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::CommentGroup { list: list.clone(), ..Default::default() }))).clone(); comments = new_val; };
        { let new_val = { let __append_target = self.comments.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(comments.clone()); __append_target.clone() }; self.comments = new_val; };
        return (comments, (*endline.lock().unwrap().as_ref().unwrap()));
    }

    /// Advance to the next non-comment token. In the process, collect
    /// any comment groups encountered, and remember the last lead and
    /// line comments.
    ///
    /// A lead comment is a comment group that starts and ends in a
    /// line without any other tokens and that is followed by a non-comment
    /// token on the line immediately after the comment group.
    ///
    /// A line comment is a comment group that follows a non-comment
    /// token on the same line, and that has no tokens after it on the line
    /// where it ends.
    ///
    /// Lead and line comments may be considered documentation that is
    /// stored in the AST.
    pub fn next(&mut self) {
        *self.lead_comment.lock().unwrap() = None;
        *self.line_comment.lock().unwrap() = None;
        let mut prev = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next0();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_E_N_T as i32)))); __tmp_x == __tmp_y } {
        let mut comment: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>> = Arc::new(Mutex::new(None));
        let mut endline: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = (*self.file.lock().unwrap().as_ref().unwrap()).line({ let __field = self.pos.clone(); __field }); let __tmp_y = (*self.file.lock().unwrap().as_ref().unwrap()).line(Arc::new(Mutex::new(Some({ let __arg_holder = prev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x == __tmp_y } {
                // The comment is on same line as the previous token; it
                // cannot be a lead comment but may be a line comment.
        { let (__tmp_0, __tmp_1) = self.consume_comment_group(Arc::new(Mutex::new(Some(0)))); comment = __tmp_0.clone(); *endline.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = (*self.file.lock().unwrap().as_ref().unwrap()).line({ let __field = self.pos.clone(); __field }); let __tmp_y = { let __v = (*endline.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x == __tmp_y } {
                // The next token is on a different line, thus
                // the last comment group is a line comment.
        { let new_val = comment.clone(); self.line_comment = new_val; };
    }
    }
                // The comment is on same line as the previous token; it
                // cannot be a lead comment but may be a line comment.
                // The next token is on a different line, thus
                // the last comment group is a line comment.
                // consume successor comments, if any
        { let new_val = -1; *endline.lock().unwrap() = Some(new_val); };
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_E_N_T as i32)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.consume_comment_group(Arc::new(Mutex::new(Some(1)))); comment = __tmp_0.clone(); *endline.lock().unwrap() = Some(__tmp_1); };
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*endline.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = (*self.file.lock().unwrap().as_ref().unwrap()).line({ let __field = self.pos.clone(); __field }); __tmp_x == __tmp_y } {
                // The next token is following on the line immediately after the
                // comment group, thus the last comment group is a lead comment.
        { let new_val = comment.clone(); self.lead_comment = new_val; };
    }
    }
    }

    pub fn error(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, msg: Arc<Mutex<Option<String>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let msg_defer_captured = msg.clone(); let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some(format!("{}{}", "error: ".to_string(), { let __v = (*msg_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }));
    }
        let mut epos = (*self.file.lock().unwrap().as_ref().unwrap()).position(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // If AllErrors is not set, discard errors reported on the same line
                // as the last recorded error and stop parsing if there are more than
                // 10 errors.
        if { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & ALL_ERRORS as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.errors.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*(*(*{ let __seq_holder = { let __named_slice = (*self.errors.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap()).line.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*epos.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                // discard - likely a spurious error
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x > __tmp_y } {
        panic!("{}", bailout { pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), msg: Arc::new(Mutex::new(Some(String::new()))) });
    }
    }
                // discard - likely a spurious error
        (*self.errors.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = epos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }

    pub fn error_expected(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, mut msg: Arc<Mutex<Option<String>>>) {
        { let new_val = format!("{}{}", "expected ".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }); *msg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.pos.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // the error happened at the current position;
                // make the error message more specific
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = (*self.lit.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "\n".to_string(); __tmp_x == __tmp_y } {
            { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&", found newline".to_string()); };
        } else if go_token::r#mod::Token::is_literal(&(*self.tok.lock().unwrap().as_ref().unwrap())) {
                        // print 123 rather than 'INT', etc.
            { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", ", found ".to_string(), (*self.lit.clone().lock().unwrap().as_ref().unwrap()))); };
        } else {
            { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&{ let mut __s = String::new(); __s.push_str(&format!("{}", ", found '".to_string())); __s.push_str(&format!("{}", (*go_token::r#mod::Token::string(&(*self.tok.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "'".to_string())); __s }); };
        }
    }
                // the error happened at the current position;
                // make the error message more specific
                // print 123 rather than 'INT', etc.
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    pub fn expect(&mut self, tok: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*tok.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "'".to_string())); __s.push_str(&format!("{}", (*go_token::r#mod::Token::string(&(*tok.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "'".to_string())); __s }))));
    }
        self.next();
        return { let __owned = pos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// expect2 is like expect, but it returns an invalid position
    /// if the expected token is not found.
    pub fn expect2(&mut self, tok: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
    let mut pos: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(Default::default())));

        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*tok.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *pos.lock().unwrap() = Some(new_val); };
    } else {
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "'".to_string())); __s.push_str(&format!("{}", (*go_token::r#mod::Token::string(&(*tok.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "'".to_string())); __s }))); self.error_expected(__method_arg0, __method_arg1) };
    }
        self.next();
        pos
    }

    /// expectClosing is like expect but provides a better error message
    /// for the common case of a missing comma before a newline.
    pub fn expect_closing(&mut self, tok: Arc<Mutex<Option<go_token::r#mod::Token>>>, context: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*tok.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = (*self.lit.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "\n".to_string(); __tmp_x == __tmp_y } {
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some(format!("{}{}", "missing ',' before newline in ".to_string(), { let __v = (*context.lock().unwrap().as_ref().unwrap()).clone(); __v })))); self.error(__method_arg0, __method_arg1) };
        self.next();
    }
        self.expect(Arc::new(Mutex::new(Some({ let __arg_holder = tok.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// expectSemi consumes a semicolon and returns the applicable line comment.
    pub fn expect_semi(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>> {
    let mut comment: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>> = Arc::new(Mutex::new(None));

                // semicolon is optional before a closing ')' or '}'
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        {
        let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // permit a ',' instead of a ';' but complain
            { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("';'".to_string()))); self.error_expected(__method_arg0, __method_arg1) };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = (*self.lit.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ";".to_string(); __tmp_x == __tmp_y } {
                // explicit semicolon
        self.next();
        { let new_val = self.line_comment.clone(); comment = new_val; };
    } else {
                // artificial semicolon
        { let new_val = self.line_comment.clone(); comment = new_val; };
        self.next();
    }
                        // explicit semicolon
                        // use following comments
                        // artificial semicolon
                        // use preceding comments
            return comment.clone();
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("';'".to_string()))); self.error_expected(__method_arg0, __method_arg1) };
            self.advance(stmtStart.clone());
        }
    }
    }
                // permit a ',' instead of a ';' but complain
                // explicit semicolon
                // use following comments
                // artificial semicolon
                // use preceding comments
        return Arc::new(Mutex::new(None));
    }

    pub fn at_comma(&self, context: Arc<Mutex<Option<String>>>, follow: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> bool {
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*follow.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        let mut msg = Arc::new(Mutex::new(Some("missing ','".to_string())));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = (*self.lit.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "\n".to_string(); __tmp_x == __tmp_y } {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" before newline".to_string()); };
    }
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", " in ".to_string())); __s.push_str(&format!("{}", { let __v = (*context.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s }))); self.error(__method_arg0, __method_arg1) };
        return true;
    }
                // "insert" comma and continue
        false
    }

    /// advance consumes tokens until the current token p.tok
    /// is in the 'to' set, or token.EOF. For error recovery.
    pub fn advance(&mut self, to: Arc<Mutex<Option<BTreeMap<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>>>>) {
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        if { let __map = { let __map_holder = to.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
                // Return only if parser made some progress since last
                // sync or if it has not reached 10 advance calls without
                // progress. Otherwise consume at least one token to
                // avoid an endless parser loop (it is possible that
                // both parseOperand and parseStmt call advance and
                // correctly do not advance, thus the need for the
                // invocation limit p.syncCnt).
        if { let __tmp_x = (*self.pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.sync_pos.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __tmp_x = (*self.sync_cnt.lock().unwrap().as_ref().unwrap()); let __tmp_y = 10; __tmp_x < __tmp_y } {
        { let __target = self.sync_cnt.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    }
        if { let __tmp_x = (*self.pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.sync_pos.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.sync_pos.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *self.sync_cnt.lock().unwrap() = Some(new_val); };
        return;
    }
    }
        self.next();
    }
    }

    /// safePos returns a valid file position for a given position: If pos
    /// is valid to begin with, safePos returns pos. If pos is out-of-range,
    /// safePos returns the EOF position.
    ///
    /// This is hack to work around "artificial" end positions in the AST which
    /// are computed by adding 1 to (presumably valid) token positions. If the
    /// token positions are invalid due to parse errors, the resulting end position
    /// may be past the file's EOF position, which would lead to panics if used
    /// later on.
    pub fn safe_pos(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut p_defer_captured = self.clone(); let mut res_defer_captured = res.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        if (*Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>)).lock().unwrap()).is_some() {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = (*p_defer_captured.file.lock().unwrap().as_ref().unwrap()).base(); let __tmp_y = (*p_defer_captured.file.lock().unwrap().as_ref().unwrap()).size(); __tmp_x + __tmp_y } as i32)))); *res_defer_captured.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
                // EOF position
        let _ = (*self.file.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        { let new_val = pos.lock().unwrap().as_ref().unwrap().clone(); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
    }

    pub fn parse_ident(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::Ident>>> {
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut name = Arc::new(Mutex::new(Some("_".to_string())));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };
        self.next();
    } else {
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))))));
    }
                // use expect() error handling
        return Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { name_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }

    pub fn parse_ident_list(&mut self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> = Arc::new(Mutex::new(None));

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("IdentList".to_string())))));
    }));
    }
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_ident()); __append_target.clone() }; list = new_val; };
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_ident()); __append_target.clone() }; list = new_val; };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return list;
    }
    }

    /// If lhs is set, result list elements which are identifiers are not resolved.
    pub fn parse_expr_list(&mut self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ExpressionList".to_string())))));
    }));
    }
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_expr().clone()); __append_target.clone() }; list = new_val; };
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_expr().clone()); __append_target.clone() }; list = new_val; };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return list;
    }
    }

    pub fn parse_list(&mut self, inRhs: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
        let mut old = Arc::new(Mutex::new(Some({ let __selector_holder = self.in_rhs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = inRhs.lock().unwrap().as_ref().unwrap().clone(); *self.in_rhs.lock().unwrap() = Some(new_val); };
        let mut list = self.parse_expr_list();
        { let new_val = old.lock().unwrap().as_ref().unwrap().clone(); *self.in_rhs.lock().unwrap() = Some(new_val); };
        return list.clone();
    }

    pub fn parse_type(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Type".to_string())))));
    }));
    }
        let mut typ = self.try_ident_or_type();
        if (*typ.lock().unwrap()).is_none() {
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type".to_string()))));
        self.advance(exprEnd.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return typ.clone();
    }
    }

    pub fn parse_qualified_ident(&mut self, ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("QualifiedIdent".to_string())))));
    }));
    }
        let mut typ = self.parse_type_name(ident.clone());
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = self.parse_type_instance(typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return typ.clone();
    }
    }

    /// If the result is an identifier, it is not resolved.
    pub fn parse_type_name(&mut self, mut ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("TypeName".to_string())))));
    }));
    }
        if (*ident.lock().unwrap()).is_none() {
        { let new_val = self.parse_ident().clone(); ident = new_val; };
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32)))); __tmp_x == __tmp_y } {
                // ident is a package name
        self.next();
        let mut sel = self.parse_ident();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { x: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(ident.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), sel: sel.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
                // ident is a package name
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(ident.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }

    /// "[" has already been consumed, and lbrack is its position.
    /// If len != nil it is the already consumed array length.
    pub fn parse_array_type(&mut self, lbrack: Arc<Mutex<Option<go_token::position::Pos>>>, mut len: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<go_ast::r#mod::ArrayType>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut len: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(len.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ArrayType".to_string())))));
    }));
    }
        if (*len.lock().unwrap()).is_none() {
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // always permit ellipsis for more fault-tolerant parsing
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_L_I_P_S_I_S as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EllipsisPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::Ellipsis { ellipsis: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *len.lock().unwrap() = (*__iface_guard).clone(); };
        self.next();
    } else if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } {
        { let __iface_handle = self.parse_rhs().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *len.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // always permit ellipsis for more fault-tolerant parsing
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
                // Trailing commas are accepted in type parameter
                // lists but not in array type declarations.
                // Accept for better error handling but complain.
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("unexpected comma; expecting ]".to_string()))); self.error(__method_arg0, __method_arg1) };
        self.next();
    }
                // Trailing commas are accepted in type parameter
                // lists but not in array type declarations.
                // Accept for better error handling but complain.
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        let mut elt = self.parse_type();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::ArrayType { lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), len: len.clone(), elt: elt.clone(), ..Default::default() })));
    }
    }

    pub fn parse_array_field_or_type_instance(&mut self, x: Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> (Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ArrayFieldOrTypeInstance".to_string())))));
    }));
    }
        let mut lbrack = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))))));
        let mut trailingComma = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))));
        let mut args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } {
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = { let __append_target = args.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_rhs().clone()); __append_target.clone() }; args = new_val; };
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
        let mut comma = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
        { let new_val = comma.lock().unwrap().as_ref().unwrap().clone(); *trailingComma.lock().unwrap() = Some(new_val); };
        break
    }
        { let new_val = { let __append_target = args.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_rhs().clone()); __append_target.clone() }; args = new_val; };
    }
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let mut rbrack = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        if { let __tmp_x = ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // x []E
        let mut elt = self.parse_type();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (x.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ArrayTypePtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ArrayType { lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), elt: elt.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))));
    }
    }
                // x []E
                // x [P]E or x[P]
        if { let __tmp_x = ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        let mut elt = self.try_ident_or_type();
        if (*elt.lock().unwrap()).is_some() {
                // x [P]E
        if go_token::position::Pos::is_valid(&(*trailingComma.lock().unwrap().as_ref().unwrap())) {
                // Trailing commas are invalid in array type fields.
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = trailingComma.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("unexpected comma; expecting ]".to_string()))));
    }
                // Trailing commas are invalid in array type fields.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (x.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ArrayTypePtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ArrayType { lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), len: { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), elt: elt.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))));
    }
    }
    }
                // x [P]E
                // Trailing commas are invalid in array type fields.
                // x[P], x[P1, P2], ...
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), pack_index_expr(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(x.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone());
    }
    }

    pub fn parse_field_decl(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::Field>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("FieldDecl".to_string())))));
    }));
    }
        let mut doc = self.lead_comment.clone();
        let mut names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> = Arc::new(Mutex::new(None));
        let mut typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) {
            let mut name = self.parse_ident();
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x == __tmp_y } {
                // embedded type
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(name.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = self.parse_qualified_ident(name.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } else {
                // name1, name2, ... T
        { let new_val = Arc::new(Mutex::new(Some(vec![name.clone()]))); names = new_val; };
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = { let __append_target = names.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_ident()); __append_target.clone() }; names = new_val; };
    }
                // Careful dance: We don't know if we have an embedded instantiated
                // type T[P1, P2, ...] or a field T of array type []E or [P]E.
        if { let __tmp_x = ((*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.parse_array_field_or_type_instance(name.clone()); name = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *typ.lock().unwrap() = __moved_tmp_1; };
        if (*name.lock().unwrap()).is_none() {
        *names.lock().unwrap() = None;
    }
    } else {
                // T P
        { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            let mut star = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
                // *(T)
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("cannot parenthesize embedded type".to_string()))); self.error(__method_arg0, __method_arg1) };
        self.next();
        { let __iface_handle = self.parse_qualified_ident(Arc::new(Mutex::new(None))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
                // expect closing ')' but no need to complain if missing
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
    }
    } else {
                // *T
        { let __iface_handle = self.parse_qualified_ident(Arc::new(Mutex::new(None))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                        // *(T)
                        // expect closing ')' but no need to complain if missing
                        // *T
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::StarExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::StarExpr { star: Arc::new(Mutex::new(Some({ let __arg_holder = star.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: typ.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
            { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("cannot parenthesize embedded type".to_string()))); self.error(__method_arg0, __method_arg1) };
            self.next();
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32)))); __tmp_x == __tmp_y } {
                // (*T)
        let mut star = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::StarExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::StarExpr { star: Arc::new(Mutex::new(Some({ let __arg_holder = star.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: self.parse_qualified_ident(Arc::new(Mutex::new(None))).clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
                // (T)
        { let __iface_handle = self.parse_qualified_ident(Arc::new(Mutex::new(None))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                        // (*T)
                        // (T)
                        // expect closing ')' but no need to complain if missing
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
    }
        } else {
            let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("field name or embedded type".to_string()))));
            self.advance(exprEnd.clone());
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        }
    }
                // embedded type
                // name1, name2, ... T
                // Careful dance: We don't know if we have an embedded instantiated
                // type T[P1, P2, ...] or a field T of array type []E or [P]E.
                // T P
                // *(T)
                // expect closing ')' but no need to complain if missing
                // *T
                // (*T)
                // (T)
                // expect closing ')' but no need to complain if missing
        let mut tag: Arc<Mutex<Option<go_ast::r#mod::BasicLit>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32)))); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::BasicLit { value_pos: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), kind: Arc::new(Mutex::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), value: Arc::new(Mutex::new(Some({ let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone(); tag = new_val; };
        self.next();
    }
        let mut comment = self.expect_semi();
        let mut field = Arc::new(Mutex::new(Some(go_ast::r#mod::Field { doc: doc.clone(), names: names.clone(), r#type: typ.clone(), tag: tag.clone(), comment: comment.clone(), ..Default::default() })));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return field.clone();
    }
    }

    pub fn parse_struct_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::StructType>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("StructType".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_U_C_T as i32))))))));
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Field>>>>>>> = Arc::new(Mutex::new(None));
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
                // a field declaration cannot start with a '(' but we accept
                // it here for more robust parsing and better error messages
                // (parseFieldDecl will check and complain if necessary)
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_field_decl()); __append_target.clone() }; list = new_val; };
    }
                // a field declaration cannot start with a '(' but we accept
                // it here for more robust parsing and better error messages
                // (parseFieldDecl will check and complain if necessary)
        let mut rbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::StructType { r#struct: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), fields: Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { opening: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), closing: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(), ..Default::default() })));
    }
    }

    pub fn parse_pointer_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::StarExpr>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("PointerType".to_string())))));
    }));
    }
        let mut star = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))))));
        let mut base = self.parse_type();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::StarExpr { star: Arc::new(Mutex::new(Some({ let __arg_holder = star.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: base.clone(), ..Default::default() })));
    }
    }

    pub fn parse_dots_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::Ellipsis>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("DotsType".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_L_I_P_S_I_S as i32))))))));
        let mut elt = self.parse_type();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::Ellipsis { ellipsis: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), elt: elt.clone(), ..Default::default() })));
    }
    }

    pub fn parse_param_decl(&mut self, name: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, typeSetsOK: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<field>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut f: Arc<Mutex<Option<field>>> = Arc::new(Mutex::new(Some(Default::default())));

                // TODO(rFindley) refactor to be more similar to paramDeclOrNil in the syntax
                // package
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ParamDecl".to_string())))));
    }));
    }
        let mut ptok = Arc::new(Mutex::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*name.lock().unwrap()).is_some() {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); *self.tok.lock().unwrap() = Some(new_val); };
    } else if { let __v = (*typeSetsOK.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); __tmp_x == __tmp_y } {
        {
        { let new_val = field { name: Default::default(), typ: self.embedded_elem(Arc::new(Mutex::new(None))).clone(), ..Default::default() }; *f.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f;
    }
    }
                // force token.IDENT case in switch below
                // "~" ...
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) {
                        // name
            if (*name.lock().unwrap()).is_some() {
        { let new_val = name.clone(); (*f.lock().unwrap().as_mut().unwrap()).name = new_val; };
        { let new_val = ptok.lock().unwrap().as_ref().unwrap().clone(); *self.tok.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = self.parse_ident().clone(); (*f.lock().unwrap().as_mut().unwrap()).name = new_val; };
    }
            { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_A_P as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_U_C_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T_E_R_F_A_C_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
                        // name type
            { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))) {
                        // name "[" type1, ..., typeN "]" or name "[" n "]" type
            { let (__tmp_0, __tmp_1) = self.parse_array_field_or_type_instance({ let __field = (*f.lock().unwrap().as_ref().unwrap()).name.clone(); __field }); (*f.lock().unwrap().as_mut().unwrap()).name = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*f.lock().unwrap().as_ref().unwrap()).typ.lock().unwrap() = __moved_tmp_1; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_L_I_P_S_I_S as i32))))) {
                        // name "..." type
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EllipsisPtr(self.parse_dots_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f;
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32))))) {
                        // name "." ...
            { let __iface_handle = self.parse_qualified_ident({ let __field = (*f.lock().unwrap().as_ref().unwrap()).name.clone(); __field }).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            *(*f.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = None;
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32))))) {
            if { let __v = (*typeSetsOK.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __iface_handle = self.embedded_elem(Arc::new(Mutex::new(None))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f;
    }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32))))) {
            if { let __v = (*typeSetsOK.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // name "|" typeset
        { let __iface_handle = self.embedded_elem(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*f.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        *(*f.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = None;
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f;
    }
    }
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_A_P as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_U_C_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T_E_R_F_A_C_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
                        // type
            { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_L_I_P_S_I_S as i32))))) {
                        // "..." type
                        // (always accepted)
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EllipsisPtr(self.parse_dots_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f;
    }
        } else {
                        // TODO(rfindley): this is incorrect in the case of type parameter lists
                        //                 (should be "']'" in that case)
            { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("')'".to_string()))); self.error_expected(__method_arg0, __method_arg1) };
            self.advance(exprEnd.clone());
        }
    }
                // name
                // name type
                // name "[" type1, ..., typeN "]" or name "[" n "]" type
                // name "..." type
                // don't allow ...type "|" ...
                // name "." ...
                // name "|" typeset
                // type
                // "..." type
                // (always accepted)
                // don't allow ...type "|" ...
                // TODO(rfindley): this is incorrect in the case of type parameter lists
                //                 (should be "']'" in that case)
                // [name] type "|"
        if { let __v = (*typeSetsOK.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32)))); __tmp_x == __tmp_y } && { let __iface_handle = { let __field = (*f.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let __iface_handle = self.embedded_elem((*f.lock().unwrap().as_ref().unwrap()).typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f;
    }
    }

    pub fn parse_parameter_list(&mut self, mut name0: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, mut typ0: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, closing: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Field>>>>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Field>>>>>>> = Arc::new(Mutex::new(None));

        let mut typ0: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(typ0.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ParameterList".to_string())))));
    }));
    }
                // Type parameters are the only parameter list closed by ']'.
        let mut tparams = Arc::new(Mutex::new(Some({ let __tmp_x = (*closing.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x == __tmp_y })));
        let mut pos0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*name0.lock().unwrap()).is_some() {
        { let new_val = { let __recv = name0.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos0.lock().unwrap() = __moved_val; };
    } else if (*typ0.lock().unwrap()).is_some() {
        { let new_val = (*typ0.lock().unwrap().as_ref().unwrap()).pos(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos0.lock().unwrap() = __moved_val; };
    }
                // Note: The code below matches the corresponding code in the syntax
                //       parser closely. Changes must be reflected in either parser.
                //       For the code to match, we use the local []field list that
                //       corresponds to []syntax.Field. At the end, the list must be
                //       converted into an []*ast.Field.
        let mut list: Arc<Mutex<Option<Vec<field>>>> = Arc::new(Mutex::new(None));
        let mut named: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let mut typed: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        while (*name0.lock().unwrap()).is_some() || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*closing.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        let mut par: Arc<Mutex<Option<field>>> = Arc::new(Mutex::new(Some(Default::default())));
        if (*typ0.lock().unwrap()).is_some() {
        if { let __v = (*tparams.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __iface_handle = self.embedded_elem(typ0.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ0.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let new_val = field { name: name0.clone(), typ: typ0.clone(), ..Default::default() }; *par.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = self.parse_param_decl(name0.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = tparams.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *par.lock().unwrap() = __moved_val; };
    }
        *name0.lock().unwrap() = None;
        *typ0.lock().unwrap() = None;
        if { let __nil_target = (*par.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __iface_handle = { let __field = (*par.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*par.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; list = new_val; };
        if { let __nil_target = (*par.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __iface_handle = { let __field = (*par.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let mut guard = named.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __iface_handle = { let __field = (*par.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let mut guard = typed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        if !self.at_comma(Arc::new(Mutex::new(Some("parameter list".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = closing.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        break
    }
        self.next();
    }
                // 1st name was consumed if present
                // 1st typ was consumed if present
        if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return params;
    }
    }
                // not uncommon
                // distribute parameter types (len(list) > 0)
        if { let __tmp_x = { let __v = (*named.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // all unnamed => found names are type names
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        let mut par: Option<GoSliceElemPtr<field>> = Some(GoSliceElemPtr::new(list.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        {
        let mut typ = (*par.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone();;
        if (*typ.lock().unwrap()).is_some() {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(typ.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*par.as_ref().unwrap().borrow_mut().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
            *(*par.as_ref().unwrap().borrow().as_ref().unwrap()).name.lock().unwrap() = None;;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __v = (*tparams.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // This is the same error handling as below, adjusted for type parameters only.
                // See comment below for details. (go.dev/issue/64534)
        let mut errPos: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __tmp_x = { let __v = (*named.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*typed.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *errPos.lock().unwrap() = Some(new_val); };
        { let new_val = "missing type constraint".to_string(); *msg.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = pos0.lock().unwrap().as_ref().unwrap().clone(); *errPos.lock().unwrap() = Some(new_val); };
        { let new_val = "missing type parameter name".to_string(); *msg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" or invalid array length".to_string()); };
    }
    }
                // position error at closing ]
                // position at opening [ or first name
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = errPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    } else if { let __tmp_x = ({ let __v = (*named.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        let mut errPos: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));
        let mut typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
        let mut par: Option<GoSliceElemPtr<field>> = Some(GoSliceElemPtr::new(list.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));;
        if { let __iface_handle = { let __field = (*par.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
            { let __iface_handle = (*par.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
            if { let __nil_target = (*par.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = (*typ.lock().unwrap().as_ref().unwrap()).pos(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *errPos.lock().unwrap() = __moved_val; };
        let mut n = go_ast::new_ident(Arc::new(Mutex::new(Some("_".to_string()))));
        { let new_val = errPos.lock().unwrap().as_ref().unwrap().clone(); *(*n.lock().unwrap().as_ref().unwrap()).name_pos.lock().unwrap() = Some(new_val); };
        { let new_val = n.clone(); (*par.as_ref().unwrap().borrow_mut().as_mut().unwrap()).name = new_val; };
    };
        } else if (*typ.lock().unwrap()).is_some() {
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*par.as_ref().unwrap().borrow_mut().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let new_val = (*(*par.as_ref().unwrap().borrow().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *errPos.lock().unwrap() = __moved_val; };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __arg_holder = errPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*par.as_ref().unwrap().borrow_mut().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if go_token::position::Pos::is_valid(&(*errPos.lock().unwrap().as_ref().unwrap())) {
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __tmp_x = { let __v = (*named.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*typed.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *errPos.lock().unwrap() = Some(new_val); };
        if { let __v = (*tparams.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = "missing type constraint".to_string(); *msg.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "missing parameter type".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
    } else {
        if { let __v = (*tparams.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = "missing type parameter name".to_string(); *msg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" or invalid array length".to_string()); };
    }
    } else {
        { let new_val = "missing parameter name".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
    }
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = errPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
                // all unnamed => found names are type names
                // This is the same error handling as below, adjusted for type parameters only.
                // See comment below for details. (go.dev/issue/64534)
                /* same as typed == 0 */
                // position error at closing ]
                // position at opening [ or first name
                // some named or we're in a type parameter list => all must be named
                // left-most error position (or invalid)
                // current type (from right to left)
                // correct position
                // par.typ == nil && typ == nil => we only have a par.name
                // Not all parameters are named because named != len(list).
                // If named == typed, there must be parameters that have no types.
                // They must be at the end of the parameter list, otherwise types
                // would have been filled in by the right-to-left sweep above and
                // there would be no error.
                // If tparams is set, the parameter list is a type parameter list.
                // position error at closing token ) or ]
                // go.dev/issue/60812
                // Convert list to []*ast.Field.
                // If list contains types only, each type gets its own ast.Field.
        if { let __tmp_x = { let __v = (*named.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // parameter list consists of types only
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for par in __range_values.iter() {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = par.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))), Arc::new(Mutex::new(Some("nil type in unnamed parameter list".to_string()))));
        { let new_val = { let __append_target = params.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(go_ast::r#mod::Field { r#type: { let __field = par.typ.clone(); __field }, ..Default::default() })))); __append_target.clone() }; params = new_val; };
    } }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return params;
    }
    }
                // parameter list consists of types only
                // If the parameter list consists of named parameters with types,
                // collect all names with the same types into a single ast.Field.
        let mut names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> = Arc::new(Mutex::new(None));
        let mut typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut names_closure_clone = names.clone(); let mut params_closure_clone = params.clone(); let typ_closure_clone = typ.clone(); let mut addParams = Arc::new(Mutex::new(Some(Box::new(move || {
        assert(Arc::new(Mutex::new(Some((*typ_closure_clone.lock().unwrap()).is_some()))), Arc::new(Mutex::new(Some("nil type in named parameter list".to_string()))));
        let mut field = Arc::new(Mutex::new(Some(go_ast::r#mod::Field { names: names_closure_clone.clone(), r#type: typ_closure_clone.clone(), ..Default::default() })));
        { let __append_target = params_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(field.clone()); __append_target.clone() };
        *names_closure_clone.lock().unwrap() = None;
    }) as Box<dyn FnMut() -> () + Send + Sync>)));
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for par in __range_values.iter() {
        if { let __left_holder = par.typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn go_ast::r#mod::Expr + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn go_ast::r#mod::Expr + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_expr(__right), _ => false }; !__eq } {
        if { let __tmp_x = ((*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = addParams.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
        { let __iface_handle = par.typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let new_val = { let __append_target = names.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __field = par.name.clone(); __field }); __append_target.clone() }; names = new_val; };
    } }
        if { let __tmp_x = ((*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = addParams.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return params;
    }
    }

    pub fn parse_parameters(&mut self, acceptTParams: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<go_ast::r#mod::FieldList>>>, Arc<Mutex<Option<go_ast::r#mod::FieldList>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut tparams: Arc<Mutex<Option<go_ast::r#mod::FieldList>>> = Arc::new(Mutex::new(None));
    let mut params: Arc<Mutex<Option<go_ast::r#mod::FieldList>>> = Arc::new(Mutex::new(None));

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Parameters".to_string())))));
    }));
    }
        if { let __v = (*acceptTParams.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
        let mut opening = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
                // [T any](params) syntax
        let mut list = self.parse_parameter_list(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        let mut rbrack = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { opening: Arc::new(Mutex::new(Some({ let __arg_holder = opening.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), closing: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); tparams = new_val; };
                // Type parameter lists must not be empty.
        if { let __tmp_x = { let __recv = tparams.clone(); let __recv_ptr: *const go_ast::r#mod::FieldList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FieldList }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.error({ let __field = (*tparams.lock().unwrap().as_ref().unwrap()).closing.clone(); __field }, Arc::new(Mutex::new(Some("empty type parameter list".to_string()))));
        *tparams.lock().unwrap() = None;
    }
    }
                // [T any](params) syntax
                // Type parameter lists must not be empty.
                // avoid follow-on errors
        let mut opening = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))))));
        let mut fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Field>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x != __tmp_y } {
        { let new_val = self.parse_parameter_list(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))))))); fields = new_val; };
    }
        let mut rparen = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32))))))));
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { opening: Arc::new(Mutex::new(Some({ let __arg_holder = opening.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: fields.clone(), closing: Arc::new(Mutex::new(Some({ let __arg_holder = rparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); params = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (tparams, params);
    }
    }

    pub fn parse_result(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::FieldList>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Result".to_string())))));
    }));
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
        let (_, mut results) = self.parse_parameters(Arc::new(Mutex::new(Some(false))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return results.clone();
    }
    }
        let mut typ = self.try_ident_or_type();
        if (*typ.lock().unwrap()).is_some() {
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Field>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (1) as usize])));
        (*list.lock().unwrap().as_mut().unwrap())[(0) as usize] = Arc::new(Mutex::new(Some(go_ast::r#mod::Field { r#type: typ.clone(), ..Default::default() })));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { list: list.clone(), ..Default::default() })));
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }

    pub fn parse_func_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::FuncType>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("FuncType".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))))));
        let (mut tparams, mut params) = self.parse_parameters(Arc::new(Mutex::new(Some(true))));
        if (*tparams.lock().unwrap()).is_some() {
        self.error({ let __recv = tparams.clone(); let __recv_ptr: *const go_ast::r#mod::FieldList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FieldList }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some("function type must have no type parameters".to_string()))));
    }
        let mut results = self.parse_result();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::FuncType { func: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), params: params.clone(), results: results.clone(), ..Default::default() })));
    }
    }

    pub fn parse_method_spec(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::Field>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("MethodSpec".to_string())))));
    }));
    }
        let mut doc = self.lead_comment.clone();
        let mut idents: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> = Arc::new(Mutex::new(None));
        let mut typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut x = self.parse_type_name(Arc::new(Mutex::new(None)));
        {
        let (mut ident, _) = ({
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
        if (*ident.lock().unwrap()).is_some() {
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
            let mut lbrack = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            let mut x = self.parse_expr();
            { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
            {
        let (mut name0, _) = ({
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
        if (*name0.lock().unwrap()).is_some() && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } {
            let _ = self.parse_parameter_list(name0.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));;
            let _ = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));;
            self.error(Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("interface method must have no type parameters".to_string()))));;
            let (_, mut params) = self.parse_parameters(Arc::new(Mutex::new(Some(false))));;
            let mut results = self.parse_result();;
            { let new_val = Arc::new(Mutex::new(Some(vec![ident.clone()]))); idents = new_val; };;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr(Arc::new(Mutex::new(Some(go_ast::r#mod::FuncType { func: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), params: params.clone(), results: results.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        } else {
            let mut list = Arc::new(Mutex::new(Some(vec![x.clone()])));;
            if self.at_comma(Arc::new(Mutex::new(Some("type argument list".to_string()))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))))))) {
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        self.next();
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_type().clone()); __append_target.clone() }; list = new_val; };
        if !self.at_comma(Arc::new(Mutex::new(Some("type argument list".to_string()))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))))))) {
        break
    }
        self.next();
    }
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    };
            let mut rbrack = self.expect_closing(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))), Arc::new(Mutex::new(Some("type argument list".to_string()))));;
            { let __iface_handle = pack_index_expr(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(ident.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        } else if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
            let (_, mut params) = self.parse_parameters(Arc::new(Mutex::new(Some(false))));
            let mut results = self.parse_result();
            { let new_val = Arc::new(Mutex::new(Some(vec![ident.clone()]))); idents = new_val; };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr(Arc::new(Mutex::new(Some(go_ast::r#mod::FuncType { func: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), params: params.clone(), results: results.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
            { let __iface_handle = x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        };
        } else {
            { let __iface_handle = x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = self.parse_type_instance(typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    };
        }
    }
                // generic method or embedded instantiated type
                // generic method m[T any]
                //
                // Interface methods do not have type parameters. We parse them for a
                // better error message and improved error recovery.
                // TODO(rfindley) refactor to share code with parseFuncType.
                // embedded instantiated type
                // TODO(rfindley) should resolve all identifiers in x.
                // ordinary method
                // TODO(rfindley) refactor to share code with parseFuncType.
                // embedded type
                // embedded, possibly instantiated type
                // embedded instantiated interface
                // Comment is added at the callsite: the field below may joined with
                // additional type specs using '|'.
                // TODO(rfindley) this should be refactored.
                // TODO(rfindley) add more tests for comment handling.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::Field { doc: doc.clone(), names: idents.clone(), r#type: typ.clone(), ..Default::default() })));
    }
    }

    pub fn embedded_elem(&mut self, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("EmbeddedElem".to_string())))));
    }));
    }
        if (*x.lock().unwrap()).is_none() {
        { let __iface_handle = self.embedded_term().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
    }
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32)))); __tmp_x == __tmp_y } {
        let mut t = Arc::new(Mutex::new(Some(go_ast::r#mod::BinaryExpr::default())));
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*t.lock().unwrap().as_ref().unwrap()).op_pos.lock().unwrap() = Some(new_val); };
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32)))); *(*t.lock().unwrap().as_ref().unwrap()).op.lock().unwrap() = Some(new_val); };
        self.next();
        { let __iface_handle = x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*t.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = self.embedded_term().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*t.lock().unwrap().as_mut().unwrap()).y.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BinaryExprPtr(t.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
    }

    pub fn embedded_term(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("EmbeddedTerm".to_string())))));
    }));
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); __tmp_x == __tmp_y } {
        let mut t = Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr::default())));
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*t.lock().unwrap().as_ref().unwrap()).op_pos.lock().unwrap() = Some(new_val); };
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); *(*t.lock().unwrap().as_ref().unwrap()).op.lock().unwrap() = Some(new_val); };
        self.next();
        { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*t.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = (*__iface_guard).clone(); };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(t.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
        let mut t = self.try_ident_or_type();
        if (*t.lock().unwrap()).is_none() {
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("~ term or type".to_string()))));
        self.advance(exprEnd.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return t.clone();
    }
    }

    pub fn parse_interface_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::InterfaceType>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("InterfaceType".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T_E_R_F_A_C_E as i32))))))));
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Field>>>>>>> = Arc::new(Mutex::new(None));
        'parse_elements: loop {
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); __tmp_x == __tmp_y } {
            let mut f = self.parse_method_spec();
            if { let __nil_target = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let __iface_handle = self.embedded_elem((*f.lock().unwrap().as_ref().unwrap()).r#type.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*f.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() = (*__iface_guard).clone(); };
    }
            { let new_val = self.expect_semi().clone(); (*f.lock().unwrap().as_mut().unwrap()).comment = new_val; };
            { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(f.clone()); __append_target.clone() }; list = new_val; };
        } else if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); __tmp_x == __tmp_y } {
            let mut typ = self.embedded_elem(Arc::new(Mutex::new(None)));
            let mut comment = self.expect_semi();
            { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(go_ast::r#mod::Field { r#type: typ.clone(), comment: comment.clone(), ..Default::default() })))); __append_target.clone() }; list = new_val; };
        } else {
            {
        let mut t = self.try_ident_or_type();;
        if (*t.lock().unwrap()).is_some() {
            let mut typ = self.embedded_elem(t.clone());;
            let mut comment = self.expect_semi();;
            { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(go_ast::r#mod::Field { r#type: typ.clone(), comment: comment.clone(), ..Default::default() })))); __append_target.clone() }; list = new_val; };;
        } else {
            break 'parse_elements;
        }
    }
        }
    }
                // TODO(rfindley): the error produced here could be improved, since we could
                // accept an identifier, 'type', or a '}' at this point.
        let mut rbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::InterfaceType { interface: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), methods: Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { opening: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), closing: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(), ..Default::default() })));
    }
    }

    pub fn parse_map_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::MapType>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("MapType".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_A_P as i32))))))));
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))))));
        let mut key = self.parse_type();
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        let mut value = self.parse_type();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::MapType { map: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), key: key.clone(), value: value.clone(), ..Default::default() })));
    }
    }

    pub fn parse_chan_type(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::ChanType>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ChanType".to_string())))));
    }));
    }
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut dir = Arc::new(Mutex::new(Some(go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some((go_ast::S_E_N_D as i32 | go_ast::R_E_C_V as i32) as i32)))))));
        let mut arrow: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *arrow.lock().unwrap() = Some(new_val); };
        self.next();
        { let new_val = go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32)))); *dir.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *arrow.lock().unwrap() = __moved_val; };
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_N as i32))))))));
        { let new_val = go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::R_E_C_V as i32)))); *dir.lock().unwrap() = Some(new_val); };
    }
        let mut value = self.parse_type();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::ChanType { begin: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), arrow: Arc::new(Mutex::new(Some({ let __arg_holder = arrow.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), dir: Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: value.clone(), ..Default::default() })));
    }
    }

    pub fn parse_type_instance(&mut self, typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("TypeInstance".to_string())))));
    }));
    }
        let mut opening = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))))));
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_type().clone()); __append_target.clone() }; list = new_val; };
        if !self.at_comma(Arc::new(Mutex::new(Some("type argument list".to_string()))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))))))) {
        break
    }
        self.next();
    }
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        let mut closing = self.expect_closing(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))), Arc::new(Mutex::new(Some("type argument list".to_string()))));
        if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = closing.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type argument list".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::IndexExpr { x: typ.clone(), lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = opening.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __tmp_x = (*opening.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(1 as i32)))); __tmp_x + __tmp_y }))), to: Arc::new(Mutex::new(Some({ let __arg_holder = closing.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), rbrack: Arc::new(Mutex::new(Some({ let __arg_holder = closing.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return pack_index_expr(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = opening.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = closing.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone();
    }
    }

    pub fn try_ident_or_type(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        dec_nest_lev(inc_nest_lev(Arc::new(Mutex::new(Some(p_defer_captured.clone())))));
    }));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) {
            let mut typ = self.parse_type_name(Arc::new(Mutex::new(None)));
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = self.parse_type_instance(typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return typ.clone();
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))) {
            let mut lbrack = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ArrayTypePtr(self.parse_array_type(Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_U_C_T as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::StructTypePtr(self.parse_struct_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::StarExprPtr(self.parse_pointer_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr(self.parse_func_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T_E_R_F_A_C_E as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::InterfaceTypePtr(self.parse_interface_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_A_P as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::MapTypePtr(self.parse_map_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ChanTypePtr(self.parse_chan_type().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
            let mut lparen = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            let mut typ = self.parse_type();
            let mut rparen = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32))))))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ParenExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ParenExpr { lparen: Arc::new(Mutex::new(Some({ let __arg_holder = lparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: typ.clone(), rparen: Arc::new(Mutex::new(Some({ let __arg_holder = rparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        }
    }
                // no type found
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }

    pub fn parse_stmt_list(&mut self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("StatementList".to_string())))));
    }));
    }
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_A_S_E as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_A_U_L_T as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_stmt().clone()); __append_target.clone() }; list = new_val; };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return list;
    }
    }

    pub fn parse_body(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Body".to_string())))));
    }));
    }
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut list = self.parse_stmt_list();
        let mut rbrace = self.expect2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::BlockStmt { lbrace: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), rbrace: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    }

    pub fn parse_block_stmt(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("BlockStmt".to_string())))));
    }));
    }
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut list = self.parse_stmt_list();
        let mut rbrace = self.expect2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::BlockStmt { lbrace: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), rbrace: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    }

    pub fn parse_func_type_or_lit(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("FuncTypeOrLit".to_string())))));
    }));
    }
        let mut typ = self.parse_func_type();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
                // function type only
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr(typ.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
                // function type only
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut body = self.parse_body();
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncLitPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::FuncLit { r#type: typ.clone(), body: body.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }

    /// parseOperand may return an expression or a raw type (incl. array
    /// types of the form [...]T). Callers must verify the result.
    pub fn parse_operand(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Operand".to_string())))));
    }));
    }
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) {
            let mut x = self.parse_ident();
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(x.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_A_G as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_R as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32))))) {
            let mut x = Arc::new(Mutex::new(Some(go_ast::r#mod::BasicLit { value_pos: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), kind: Arc::new(Mutex::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), value: Arc::new(Mutex::new(Some({ let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
            self.next();
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(x.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
            let mut lparen = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            let mut x = self.parse_rhs();
            { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
            let mut rparen = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32))))))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ParenExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ParenExpr { lparen: Arc::new(Mutex::new(Some({ let __arg_holder = lparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: x.clone(), rparen: Arc::new(Mutex::new(Some({ let __arg_holder = rparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return self.parse_func_type_or_lit().clone();
    }
        }
    }
                // types may be parenthesized: (some type)
        {
        let mut typ = self.try_ident_or_type();;
        if (*typ.lock().unwrap()).is_some() {
            let (_, mut isIdent) = ({
        let val = typ.clone();
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
            assert(Arc::new(Mutex::new(Some(!isIdent))), Arc::new(Mutex::new(Some("type cannot be identifier".to_string()))));;
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return typ.clone();
    };
        }
    }
                // could be type for composite literal or conversion
                // we have an error
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("operand".to_string()))));
        self.advance(stmtStart.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }

    pub fn parse_selector(&mut self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Selector".to_string())))));
    }));
    }
        let mut sel = self.parse_ident();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { x: x.clone(), sel: sel.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }

    pub fn parse_type_assertion(&mut self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("TypeAssertion".to_string())))));
    }));
    }
        let mut lparen = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))))));
        let mut typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32)))); __tmp_x == __tmp_y } {
                // type switch: typ == nil
        self.next();
    } else {
        { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // type switch: typ == nil
        let mut rparen = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32))))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeAssertExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::TypeAssertExpr { x: x.clone(), r#type: typ.clone(), lparen: Arc::new(Mutex::new(Some({ let __arg_holder = lparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rparen: Arc::new(Mutex::new(Some({ let __arg_holder = rparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }

    pub fn parse_index_or_slice_or_instance(&mut self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("parseIndexOrSliceOrInstance".to_string())))));
    }));
    }
        let mut lbrack = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))))));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
                // empty index, slice or index expressions are not permitted;
                // accept them for parsing tolerance, but complain
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("operand".to_string()))); self.error_expected(__method_arg0, __method_arg1) };
        let mut rbrack = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::IndexExpr { x: x.clone(), lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), rbrack: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
                // empty index, slice or index expressions are not permitted;
                // accept them for parsing tolerance, but complain
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        const N: i32 = 3;

        let mut args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut index: Arc<Mutex<Option<[Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>; 3]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let mut colons: Arc<Mutex<Option<[go_token::position::Pos; 2]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32)))); __tmp_x != __tmp_y } {
                // We can't know if we have an index expression or a type instantiation;
                // so even if we see a (named) type we are not going to be in type context.
        (*index.lock().unwrap().as_mut().unwrap())[(0) as usize] = self.parse_rhs().clone();
    }
                // We can't know if we have an index expression or a type instantiation;
                // so even if we see a (named) type we are not going to be in type context.
        let mut ncolons = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32))))) {
                        // slice expression
            while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = ({ let __v = (*ncolons.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        (*colons.lock().unwrap().as_mut().unwrap())[({ let __v = (*ncolons.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = self.pos.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
        { let mut guard = ncolons.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        self.next();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        (*index.lock().unwrap().as_mut().unwrap())[({ let __v = (*ncolons.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = self.parse_rhs().clone();
    }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32))))) {
                        // instance expression
            { let new_val = { let __append_target = args.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone()); __append_target.clone() }; args = new_val; };
            while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y } {
        self.next();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = args.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_type().clone()); __append_target.clone() }; args = new_val; };
    }
    }
        }
    }
                // slice expression
                // instance expression
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        let mut rbrack = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        if { let __tmp_x = { let __v = (*ncolons.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
                // slice expression
        let mut slice3 = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*ncolons.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } {
        { let new_val = true; *slice3.lock().unwrap() = Some(new_val); };
                // Check presence of middle and final index here rather than during type-checking
                // to prevent erroneous programs from passing through gofmt (was go.dev/issue/7305).
        if (*{ let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = colons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))), Arc::new(Mutex::new(Some("middle index required in 3-index slice".to_string()))));
        (*index.lock().unwrap().as_mut().unwrap())[(1) as usize] = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = colons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(1 as i32)))); __tmp_x + __tmp_y }))), to: Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = colons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        if (*{ let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = colons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }))), Arc::new(Mutex::new(Some("final index required in 3-index slice".to_string()))));
        (*index.lock().unwrap().as_mut().unwrap())[(2) as usize] = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = colons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(1 as i32)))); __tmp_x + __tmp_y }))), to: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
                // Check presence of middle and final index here rather than during type-checking
                // to prevent erroneous programs from passing through gofmt (was go.dev/issue/7305).
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SliceExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SliceExpr { x: x.clone(), lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), low: { let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), high: { let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone(), max: { let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.clone(), slice3: Arc::new(Mutex::new(Some({ let __arg_holder = slice3.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rbrack: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
                // slice expression
                // Check presence of middle and final index here rather than during type-checking
                // to prevent erroneous programs from passing through gofmt (was go.dev/issue/7305).
        if { let __tmp_x = ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // index expression
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::IndexExpr { x: x.clone(), lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: { let __seq = { let __seq_holder = index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), rbrack: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }
                // index expression
                // instance expression
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return pack_index_expr(x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone();
    }
    }

    pub fn parse_call_or_conversion(&mut self, fun: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<go_ast::r#mod::CallExpr>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("CallOrConversion".to_string())))));
    }));
    }
        let mut lparen = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))))));
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut ellipsis: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } && !go_token::position::Pos::is_valid(&(*ellipsis.lock().unwrap().as_ref().unwrap())) {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_rhs().clone()); __append_target.clone() }; list = new_val; };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_L_I_P_S_I_S as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *ellipsis.lock().unwrap() = Some(new_val); };
        self.next();
    }
        if !self.at_comma(Arc::new(Mutex::new(Some("argument list".to_string()))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))))))) {
        break
    }
        self.next();
    }
                // builtins may expect a type: make(some type, ...)
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        let mut rparen = self.expect_closing(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32))))))), Arc::new(Mutex::new(Some("argument list".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::CallExpr { fun: fun.clone(), lparen: Arc::new(Mutex::new(Some({ let __arg_holder = lparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args: list.clone(), ellipsis: Arc::new(Mutex::new(Some({ let __arg_holder = ellipsis.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rparen: Arc::new(Mutex::new(Some({ let __arg_holder = rparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    }

    pub fn parse_value(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Element".to_string())))));
    }));
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return self.parse_literal_value(Arc::new(Mutex::new(None))).clone();
    }
    }
        let mut x = self.parse_expr();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
    }

    pub fn parse_element(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Element".to_string())))));
    }));
    }
        let mut x = self.parse_value();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32)))); __tmp_x == __tmp_y } {
        let mut colon = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::KeyValueExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::KeyValueExpr { key: x.clone(), colon: Arc::new(Mutex::new(Some({ let __arg_holder = colon.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: self.parse_value().clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
    }

    pub fn parse_element_list(&mut self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ElementList".to_string())))));
    }));
    }
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_element().clone()); __append_target.clone() }; list = new_val; };
        if !self.at_comma(Arc::new(Mutex::new(Some("composite literal".to_string()))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))))))) {
        break
    }
        self.next();
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return list;
    }
    }

    pub fn parse_literal_value(&mut self, typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        dec_nest_lev(inc_nest_lev(Arc::new(Mutex::new(Some(p_defer_captured.clone())))));
    }));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("LiteralValue".to_string())))));
    }));
    }
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut elts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        { let new_val = self.parse_element_list(); elts = new_val; };
    }
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        let mut rbrace = self.expect_closing(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))), Arc::new(Mutex::new(Some("composite literal".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::CompositeLit { r#type: typ.clone(), lbrace: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), elts: elts.clone(), rbrace: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
    }

    pub fn parse_primary_expr(&mut self, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("PrimaryExpr".to_string())))));
    }));
    }
        if (*x.lock().unwrap()).is_none() {
        { let __iface_handle = self.parse_operand().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // We track the nesting here rather than at the entry for the function,
                // since it can iteratively produce a nested output, and we want to
                // limit how deep a structure we generate.
        let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let n_defer_captured = n.clone(); let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = p_defer_captured.nest_lev.clone(); let __rhs = (*n_defer_captured.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        { let new_val = 1; *n.lock().unwrap() = Some(new_val); };
    loop {
        inc_nest_lev(Arc::new(Mutex::new(Some(self.clone()))));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32))))) {
            self.next();
            { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) {
            { let __iface_handle = self.parse_selector(x.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
            { let __iface_handle = self.parse_type_assertion(x.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
            let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("selector or type assertion".to_string()))));
                        // TODO(rFindley) The check for token.RBRACE below is a targeted fix
                        //                to error recovery sufficient to make the x/tools tests to
                        //                pass with the new parsing logic introduced for type
                        //                parameters. Remove this once error recovery has been
                        //                more generally reconsidered.
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        self.next();
    }
                        // make progress
            let mut sel = Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { name_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), name: Arc::new(Mutex::new(Some("_".to_string()))), ..Default::default() })));
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { x: x.clone(), sel: sel.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))) {
            { let __iface_handle = self.parse_index_or_slice_or_instance(x.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(self.parse_call_or_conversion(x.clone()).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))) {
                        // operand may have returned a parenthesized complit
                        // type; accept it but complain if we have a complit
            let mut t = go_ast::unparen(x.clone());
                        // determine if '{' belongs to a composite literal or a block statement
            {
    let _ts_subject = t.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExprPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).is_some() {
        if { let __tmp_x = (*self.expr_lev.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        if { let __tmp_x = (*self.expr_lev.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ArrayTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::MapTypePtr>()).is_some() {
    } else {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    };
    }
    }
                        // x is possibly a composite literal type
                        // x is possibly a composite literal type
                        // x is a composite literal type
            if { let __left_holder = t.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn go_ast::r#mod::Expr + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = x.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn go_ast::r#mod::Expr + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_expr(__right), _ => false }; !__eq } {
        self.error((*t.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("cannot parenthesize type in composite literal".to_string()))));
    }
                        // already progressed, no need to advance
            { let __iface_handle = self.parse_literal_value(x.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
        }
    }
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    pub fn parse_unary_expr(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        dec_nest_lev(inc_nest_lev(Arc::new(Mutex::new(Some(p_defer_captured.clone())))));
    }));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("UnaryExpr".to_string())))));
    }));
    }
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_O_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32))))) {
            let (mut pos, mut op) = (Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some((*(*self.tok.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))));
            self.next();
            let mut x = self.parse_unary_expr();
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr { op_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), op: Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: x.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) {
                        // channel type or receive expression
            let mut arrow = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
                        // If the next token is token.CHAN we still don't know if it
                        // is a channel type or a receive operation - we only know
                        // once we have found the end of the unary expression. There
                        // are two cases:
                        //
                        //   <- type  => (<-type) must be channel type
                        //   <- expr  => <-(expr) is a receive from an expression
                        //
                        // In the first case, the arrow must be re-associated with
                        // the channel type parsed already:
                        //
                        //   <- (chan type)    =>  (<-chan type)
                        //   <- (chan<- type)  =>  (<-chan (<-type))
            let mut x = self.parse_unary_expr();
                        // determine which case we have
            {
        let (mut typ, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ChanTypePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::ChanType>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::ChanType>)), false)
        }
    });;
        if ok {
            let mut dir = Arc::new(Mutex::new(Some(go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32)))))));;
            while ok && { let __tmp_x = (*dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::R_E_C_V as i32)))); __tmp_x == __tmp_y } {
        self.error_expected({ let __field = (*typ.lock().unwrap().as_ref().unwrap()).arrow.clone(); __field }, Arc::new(Mutex::new(Some("'chan'".to_string()))));
    }
        { let __tmp_0 = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).arrow.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = (*arrow.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_2 = (*arrow.lock().unwrap().as_ref().unwrap()).clone(); *arrow.lock().unwrap() = Some(__tmp_0); *(*typ.lock().unwrap().as_ref().unwrap()).begin.lock().unwrap() = Some(__tmp_1); *(*typ.lock().unwrap().as_ref().unwrap()).arrow.lock().unwrap() = Some(__tmp_2); };
        { let __tmp_0 = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = go_ast::R_E_C_V; *dir.lock().unwrap() = Some(__tmp_0); *(*typ.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(__tmp_1 as i32))))); };
        { let (__tmp_0, __tmp_1) = ({
        let val = (*typ.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ChanTypePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::ChanType>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::ChanType>)), false)
        }
    }); typ = __tmp_0.clone(); ok = __tmp_1; };
    };
            if { let __tmp_x = (*dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32)))); __tmp_x == __tmp_y } {
        self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = arrow.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("channel type".to_string()))));
    };
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    };
        }
    }
                        // (<-type)
                        // re-associate position info and <-
                        // error: (<-type) is (<-(<-chan T))
                        // <-(expr)
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr { op_pos: Arc::new(Mutex::new(Some({ let __arg_holder = arrow.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), op: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))))), x: x.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
                        // pointer type or unary "*" expression
            let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            let mut x = self.parse_unary_expr();
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::StarExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::StarExpr { star: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x: x.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    }
        }
    }
                // channel type or receive expression
                // If the next token is token.CHAN we still don't know if it
                // is a channel type or a receive operation - we only know
                // once we have found the end of the unary expression. There
                // are two cases:
                //
                //   <- type  => (<-type) must be channel type
                //   <- expr  => <-(expr) is a receive from an expression
                //
                // In the first case, the arrow must be re-associated with
                // the channel type parsed already:
                //
                //   <- (chan type)    =>  (<-chan type)
                //   <- (chan<- type)  =>  (<-chan (<-type))
                // determine which case we have
                // (<-type)
                // re-associate position info and <-
                // error: (<-type) is (<-(<-chan T))
                // <-(expr)
                // pointer type or unary "*" expression
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return self.parse_primary_expr(Arc::new(Mutex::new(None))).clone();
    }
    }

    pub fn tok_prec(&self) -> (Arc<Mutex<Option<go_token::r#mod::Token>>>, i32) {
        let mut tok = Arc::new(Mutex::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*self.in_rhs.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32)))); *tok.lock().unwrap() = Some(new_val); };
    }
        return ({ let __owned = tok.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, go_token::r#mod::Token::precedence(&(*tok.lock().unwrap().as_ref().unwrap())));
    }

    /// parseBinaryExpr parses a (possibly) binary expression.
    /// If x is non-nil, it is used as the left operand.
    ///
    /// TODO(rfindley): parseBinaryExpr has become overloaded. Consider refactoring.
    pub fn parse_binary_expr(&mut self, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, prec1: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("BinaryExpr".to_string())))));
    }));
    }
        if (*x.lock().unwrap()).is_none() {
        { let __iface_handle = self.parse_unary_expr().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // We track the nesting here rather than at the entry for the function,
                // since it can iteratively produce a nested output, and we want to
                // limit how deep a structure we generate.
        let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let n_defer_captured = n.clone(); let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = p_defer_captured.nest_lev.clone(); let __rhs = (*n_defer_captured.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        { let new_val = 1; *n.lock().unwrap() = Some(new_val); };
    loop {
        inc_nest_lev(Arc::new(Mutex::new(Some(self.clone()))));
        let (mut op, mut oprec) = self.tok_prec();
        if { let __tmp_x = oprec; let __tmp_y = { let __v = (*prec1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return x.clone();
    }
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut y = self.parse_binary_expr(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __tmp_x = oprec; let __tmp_y = 1; __tmp_x + __tmp_y }))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BinaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BinaryExpr { x: x.clone(), op_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), op: Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), y: y.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    /// The result may be a type or even a raw type ([...]int).
    pub fn parse_expr(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Expression".to_string())))));
    }));
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return self.parse_binary_expr(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(1)))).clone();
    }
    }

    pub fn parse_rhs(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        let mut old = Arc::new(Mutex::new(Some({ let __selector_holder = self.in_rhs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = true; *self.in_rhs.lock().unwrap() = Some(new_val); };
        let mut x = self.parse_expr();
        { let new_val = old.lock().unwrap().as_ref().unwrap().clone(); *self.in_rhs.lock().unwrap() = Some(new_val); };
        return x.clone();
    }

    /// parseSimpleStmt returns true as 2nd result if it parsed the assignment
    /// of a range clause (with mode == rangeOk). The returned statement is an
    /// assignment with a right-hand side that is a single unary expression of
    /// the form "range x". No guarantees are given for the left-hand side.
    pub fn parse_simple_stmt(&mut self, mode: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>, bool) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("SimpleStmt".to_string())))));
    }));
    }
        let mut x = self.parse_list(Arc::new(Mutex::new(Some(false))));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_L__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_R__A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T__A_S_S_I_G_N as i32))))) {
                        // assignment statement, possibly part of a range clause
            let (mut pos, mut tok) = (Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some((*(*self.tok.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))));
            self.next();
            let mut y: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
            let mut isRange = Arc::new(Mutex::new(Some(false)));
            if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_A_N_G_E as i32)))); __tmp_x == __tmp_y } && ({ let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y }) {
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr { op_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), op: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_A_N_G_E as i32))))))), x: self.parse_rhs().clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))]))); y = new_val; };
        { let new_val = true; *isRange.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = self.parse_list(Arc::new(Mutex::new(Some(true)))); y = new_val; };
    }
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::AssignStmt { lhs: x.clone(), tok_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), tok: Arc::new(Mutex::new(Some({ let __arg_holder = tok.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rhs: y.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), { let __v = (*isRange.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        }
    }
                // assignment statement, possibly part of a range clause
        if { let __tmp_x = ((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        self.error_expected({ let __recv = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("1 expression".to_string()))));
    }
                // continue with first expression
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32))))) {
                        // labeled statement
            let mut colon = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            {
        let (mut label, mut isIdent) = ({
        let val = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
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
        if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } && isIdent {
            let mut stmt = Arc::new(Mutex::new(Some(go_ast::r#mod::LabeledStmt { label: label.clone(), colon: Arc::new(Mutex::new(Some({ let __arg_holder = colon.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), stmt: self.parse_stmt().clone(), ..Default::default() })));;
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::LabeledStmtPtr(stmt.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), false);
    };
        }
    }
                        // Go spec: The scope of a label is the body of the function
                        // in which it is declared and excludes the body of any nested
                        // function.
                        // The label declaration typically starts at x[0].Pos(), but the label
                        // declaration may be erroneous due to a token after that position (and
                        // before the ':'). If SpuriousErrors is not set, the (only) error
                        // reported for the line is the illegal label error instead of the token
                        // before the ':' that caused the problem. Thus, use the (latest) colon
                        // position for error reporting.
            self.error(Arc::new(Mutex::new(Some({ let __arg_holder = colon.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("illegal label declaration".to_string()))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadStmt { from: { let __recv = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, to: Arc::new(Mutex::new(Some({ let __tmp_x = (*colon.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(1 as i32)))); __tmp_x + __tmp_y }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), false);
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) {
                        // send statement
            let mut arrow = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.next();
            let mut y = self.parse_rhs();
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SendStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SendStmt { chan: { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), arrow: Arc::new(Mutex::new(Some({ let __arg_holder = arrow.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: y.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), false);
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_C as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_C as i32))))) {
                        // increment or decrement
            let mut s = Arc::new(Mutex::new(Some(go_ast::r#mod::IncDecStmt { x: { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), tok_pos: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), tok: Arc::new(Mutex::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
            self.next();
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IncDecStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), false);
    }
        }
    }
                // labeled statement
                // Go spec: The scope of a label is the body of the function
                // in which it is declared and excludes the body of any nested
                // function.
                // The label declaration typically starts at x[0].Pos(), but the label
                // declaration may be erroneous due to a token after that position (and
                // before the ':'). If SpuriousErrors is not set, the (only) error
                // reported for the line is the illegal label error instead of the token
                // before the ':' that caused the problem. Thus, use the (latest) colon
                // position for error reporting.
                // send statement
                // increment or decrement
                // expression
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ExprStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ExprStmt { x: { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), false);
    }
    }

    pub fn parse_call_expr(&mut self, callType: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<go_ast::r#mod::CallExpr>>> {
        let mut x = self.parse_rhs();
        {
        let mut t = go_ast::unparen(x.clone());;
        if { let __left_holder = t.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn go_ast::r#mod::Expr + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = x.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn go_ast::r#mod::Expr + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_expr(__right), _ => false }; !__eq } {
            self.error((*x.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some(format!("expression in {} must not be parenthesized", { let __v = (*callType.lock().unwrap().as_ref().unwrap()).clone(); __v })))));;
            { let __iface_handle = t.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        {
        let (mut call, mut isCall) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CallExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
        }
    });;
        if isCall {
            return call.clone();;
        }
    }
        {
        let (_, mut isBad) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::BadExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::BadExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::BadExpr>)), false)
        }
    });;
        if !isBad {
            { let __method_arg0 = self.safe_pos((*x.lock().unwrap().as_ref().unwrap()).end()); let __method_arg1 = Arc::new(Mutex::new(Some(format!("expression in {} must be function call", { let __v = (*callType.lock().unwrap().as_ref().unwrap()).clone(); __v })))); self.error(__method_arg0, __method_arg1) };;
        }
    }
                // only report error if it's a new one
        return Arc::new(Mutex::new(None));
    }

    pub fn parse_go_stmt(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("GoStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O as i32))))))));
        let mut call = self.parse_call_expr(Arc::new(Mutex::new(Some("go".to_string()))));
        self.expect_semi();
        if (*call.lock().unwrap()).is_none() {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadStmt { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __tmp_x = (*pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(2 as i32)))); __tmp_x + __tmp_y }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }
                // len("go")
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GoStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::GoStmt { go: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), call: call.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }

    pub fn parse_defer_stmt(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("DeferStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_E_R as i32))))))));
        let mut call = self.parse_call_expr(Arc::new(Mutex::new(Some("defer".to_string()))));
        self.expect_semi();
        if (*call.lock().unwrap()).is_none() {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadStmt { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __tmp_x = (*pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(5 as i32)))); __tmp_x + __tmp_y }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }
                // len("defer")
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::DeferStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::DeferStmt { defer: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), call: call.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }

    pub fn parse_return_stmt(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::ReturnStmt>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ReturnStmt".to_string())))));
    }));
    }
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_T_U_R_N as i32))))))));
        let mut x: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        { let new_val = self.parse_list(Arc::new(Mutex::new(Some(true)))); x = new_val; };
    }
        self.expect_semi();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::ReturnStmt { r#return: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), results: x.clone(), ..Default::default() })));
    }
    }

    pub fn parse_branch_stmt(&mut self, tok: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_ast::r#mod::BranchStmt>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("BranchStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some({ let __arg_holder = tok.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut label: Arc<Mutex<Option<go_ast::r#mod::Ident>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); __tmp_x == __tmp_y } {
        { let new_val = self.parse_ident().clone(); label = new_val; };
    }
        self.expect_semi();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::BranchStmt { tok_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), tok: Arc::new(Mutex::new(Some({ let __arg_holder = tok.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), label: label.clone(), ..Default::default() })));
    }
    }

    pub fn make_expr(&self, s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>, want: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        if (*s.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(None));
    }
        {
        let (mut es, mut isExpr) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ExprStmtPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::ExprStmt>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::ExprStmt>)), false)
        }
    });;
        if isExpr {
            return (*es.lock().unwrap().as_ref().unwrap()).x.clone();;
        }
    }
        let mut found = Arc::new(Mutex::new(Some("simple statement".to_string())));
        {
        let (_, mut isAss) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::AssignStmtPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::AssignStmt>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::AssignStmt>)), false)
        }
    });;
        if isAss {
            { let new_val = "assignment".to_string(); *found.lock().unwrap() = Some(new_val); };;
        }
    }
        self.error((*s.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some(format!("expected {}, found {} (missing parentheses around composite literal?)", { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: (*s.lock().unwrap().as_ref().unwrap()).pos(), to: self.safe_pos((*s.lock().unwrap().as_ref().unwrap()).end()), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))
    }

    /// parseIfHeader is an adjusted version of parser.header
    /// in cmd/compile/internal/syntax/parser.go, which has
    /// been tuned for better error handling.
    pub fn parse_if_header(&mut self) -> (Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
    let mut init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut cond: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x == __tmp_y } {
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("missing condition in if statement".to_string()))); self.error(__method_arg0, __method_arg1) };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *cond.lock().unwrap() = (*__iface_guard).clone(); };
        return (init, cond);
    }
                // p.tok != token.LBRACE
        let mut prevLev = Arc::new(Mutex::new(Some({ let __selector_holder = self.expr_lev.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = -1; *self.expr_lev.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x != __tmp_y } {
                // accept potential variable declaration but complain
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("var declaration not allowed in if initializer".to_string()))); self.error(__method_arg0, __method_arg1) };
    }
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *init.lock().unwrap() = __moved_tmp_0; };
    }
                // accept potential variable declaration but complain
        let mut condStmt: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut semi: Arc<Mutex<Option<AnonymousStruct1>>> = Arc::new(Mutex::new(Some(Default::default())));
                // ";" or "\n"; valid if pos.IsValid()
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*semi.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*semi.lock().unwrap().as_ref().unwrap()).lit.lock().unwrap() = Some(new_val); };
        self.next();
    } else {
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32))))))));
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *condStmt.lock().unwrap() = __moved_tmp_0; };
    }
    } else {
        { let __iface_handle = init.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *condStmt.lock().unwrap() = (*__iface_guard).clone(); };
        *init.lock().unwrap() = None;
    }
        if (*condStmt.lock().unwrap()).is_some() {
        { let __iface_handle = self.make_expr(condStmt.clone(), Arc::new(Mutex::new(Some("boolean expression".to_string())))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *cond.lock().unwrap() = (*__iface_guard).clone(); };
    } else if go_token::position::Pos::is_valid(&(*(*semi.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap())) {
        if { let __tmp_x = { let __selector_holder = (*semi.lock().unwrap().as_ref().unwrap()).lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "\n".to_string(); __tmp_x == __tmp_y } {
        self.error({ let __field = (*semi.lock().unwrap().as_ref().unwrap()).pos.clone(); __field }, Arc::new(Mutex::new(Some("unexpected newline, expecting { after if clause".to_string()))));
    } else {
        self.error({ let __field = (*semi.lock().unwrap().as_ref().unwrap()).pos.clone(); __field }, Arc::new(Mutex::new(Some("missing condition in if statement".to_string()))));
    }
    }
                // make sure we have a valid AST
        if (*cond.lock().unwrap()).is_none() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadExpr { from: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *cond.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let new_val = prevLev.lock().unwrap().as_ref().unwrap().clone(); *self.expr_lev.lock().unwrap() = Some(new_val); };
        (init, cond)
    }

    pub fn parse_if_stmt(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::IfStmt>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        dec_nest_lev(inc_nest_lev(Arc::new(Mutex::new(Some(p_defer_captured.clone())))));
    }));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("IfStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_F as i32))))))));
        let (mut init, mut cond) = self.parse_if_header();
        let mut body = self.parse_block_stmt();
        let mut else_: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_S_E as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_F as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IfStmtPtr(self.parse_if_stmt().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *else_.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr(self.parse_block_stmt().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *else_.lock().unwrap() = (*__iface_guard).clone(); };
            self.expect_semi();
        } else {
            { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("if statement or block".to_string()))); self.error_expected(__method_arg0, __method_arg1) };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadStmt { from: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *else_.lock().unwrap() = (*__iface_guard).clone(); };
        }
    }
    } else {
        self.expect_semi();
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::IfStmt { r#if: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), init: init.clone(), cond: cond.clone(), body: body.clone(), r#else: else_.clone(), ..Default::default() })));
    }
    }

    pub fn parse_case_clause(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::CaseClause>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("CaseClause".to_string())))));
    }));
    }
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_A_S_E as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = self.parse_list(Arc::new(Mutex::new(Some(true)))); list = new_val; };
    } else {
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_A_U_L_T as i32))))))));
    }
        let mut colon = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32))))))));
        let mut body = self.parse_stmt_list();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::CaseClause { case: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), colon: Arc::new(Mutex::new(Some({ let __arg_holder = colon.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), body: body.clone(), ..Default::default() })));
    }
    }

    pub fn is_type_switch_guard(&self, s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> bool {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).unwrap().0.clone();
        return is_type_switch_assert((*t.lock().unwrap().as_ref().unwrap()).x.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        if { let __tmp_x = (({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = (({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && is_type_switch_assert({ let __seq = { let __seq_holder = (*t.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone()) {
        {
        let _switch_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            self.error({ let __field = (*t.lock().unwrap().as_ref().unwrap()).tok_pos.clone(); __field }, Arc::new(Mutex::new(Some("expected ':=', found '='".to_string()))));
            _fallthrough = true;
        }
        if !_matched && (_switch_val == go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return true;
        }
    }
    };
    }
    }
                // x.(type)
                // v := x.(type)
                // permit v = x.(type) but complain
        false
    }

    pub fn parse_switch_stmt(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("SwitchStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_W_I_T_C_H as i32))))))));
        let mut s1: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut s2: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        let mut prevLev = Arc::new(Mutex::new(Some({ let __selector_holder = self.expr_lev.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = -1; *self.expr_lev.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *s2.lock().unwrap() = __moved_tmp_0; };
    }
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let __iface_handle = s2.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *s1.lock().unwrap() = (*__iface_guard).clone(); };
        *s2.lock().unwrap() = None;
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
                // A TypeSwitchGuard may declare a variable in addition
                // to the variable declared in the initial SimpleStmt.
                // Introduce extra scope to avoid redeclaration errors:
                //
                //	switch t := 0; t := x.(T) { ... }
                //
                // (this code is not valid Go because the first t
                // cannot be accessed and thus is never used, the extra
                // scope is needed for the correct error message).
                //
                // If we don't have a type switch, s2 must be an expression.
                // Having the extra nested but empty scope won't affect it.
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *s2.lock().unwrap() = __moved_tmp_0; };
    }
    }
                // A TypeSwitchGuard may declare a variable in addition
                // to the variable declared in the initial SimpleStmt.
                // Introduce extra scope to avoid redeclaration errors:
                //
                //	switch t := 0; t := x.(T) { ... }
                //
                // (this code is not valid Go because the first t
                // cannot be accessed and thus is never used, the extra
                // scope is needed for the correct error message).
                //
                // If we don't have a type switch, s2 must be an expression.
                // Having the extra nested but empty scope won't affect it.
        { let new_val = prevLev.lock().unwrap().as_ref().unwrap().clone(); *self.expr_lev.lock().unwrap() = Some(new_val); };
    }
                // A TypeSwitchGuard may declare a variable in addition
                // to the variable declared in the initial SimpleStmt.
                // Introduce extra scope to avoid redeclaration errors:
                //
                //	switch t := 0; t := x.(T) { ... }
                //
                // (this code is not valid Go because the first t
                // cannot be accessed and thus is never used, the extra
                // scope is needed for the correct error message).
                //
                // If we don't have a type switch, s2 must be an expression.
                // Having the extra nested but empty scope won't affect it.
        let mut typeSwitch = self.is_type_switch_guard(s2.clone());
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_A_S_E as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_A_U_L_T as i32)))); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CaseClausePtr(self.parse_case_clause().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))); __append_target.clone() }; list = new_val; };
    }
        let mut rbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))));
        self.expect_semi();
        let mut body = Arc::new(Mutex::new(Some(go_ast::r#mod::BlockStmt { lbrace: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), rbrace: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        if typeSwitch {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSwitchStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::TypeSwitchStmt { switch: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), init: s1.clone(), assign: s2.clone(), body: body.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SwitchStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SwitchStmt { switch: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), init: s1.clone(), tag: self.make_expr(s2.clone(), Arc::new(Mutex::new(Some("switch expression".to_string())))).clone(), body: body.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }

    pub fn parse_comm_clause(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::CommClause>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("CommClause".to_string())))));
    }));
    }
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut comm: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_A_S_E as i32)))); __tmp_x == __tmp_y } {
        self.next();
        let mut lhs = self.parse_list(Arc::new(Mutex::new(Some(false))));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32)))); __tmp_x == __tmp_y } {
                // SendStmt
        if { let __tmp_x = ((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        self.error_expected({ let __recv = { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("1 expression".to_string()))));
    }
                // continue with first expression
        let mut arrow = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        let mut rhs = self.parse_rhs();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SendStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SendStmt { chan: { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), arrow: Arc::new(Mutex::new(Some({ let __arg_holder = arrow.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: rhs.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *comm.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
                // RecvStmt
        {
        let mut tok = Arc::new(Mutex::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } {
            if { let __tmp_x = ((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x > __tmp_y } {
        self.error_expected({ let __recv = { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("1 or 2 expressions".to_string()))));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize..(2) as usize].to_vec() }))); lhs = new_val; };
    };
            let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
            self.next();;
            let mut rhs = self.parse_rhs();;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::AssignStmt { lhs: lhs.clone(), tok_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), tok: Arc::new(Mutex::new(Some({ let __arg_holder = tok.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rhs: Arc::new(Mutex::new(Some(vec![rhs.clone()]))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *comm.lock().unwrap() = (*__iface_guard).clone(); };;
        } else {
            if { let __tmp_x = ((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        self.error_expected({ let __recv = { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("1 expression".to_string()))));
    };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ExprStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ExprStmt { x: { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *comm.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    }
    } else {
        self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_A_U_L_T as i32))))))));
    }
                // SendStmt
                // continue with first expression
                // RecvStmt
                // RecvStmt with assignment
                // continue with first two expressions
                // lhs must be single receive operation
                // continue with first expression
        let mut colon = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32))))))));
        let mut body = self.parse_stmt_list();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::CommClause { case: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), comm: comm.clone(), colon: Arc::new(Mutex::new(Some({ let __arg_holder = colon.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), body: body.clone(), ..Default::default() })));
    }
    }

    pub fn parse_select_stmt(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::SelectStmt>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("SelectStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_L_E_C_T as i32))))))));
        let mut lbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))))));
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_A_S_E as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_A_U_L_T as i32)))); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CommClausePtr(self.parse_comm_clause().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))); __append_target.clone() }; list = new_val; };
    }
        let mut rbrace = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))))));
        self.expect_semi();
        let mut body = Arc::new(Mutex::new(Some(go_ast::r#mod::BlockStmt { lbrace: Arc::new(Mutex::new(Some({ let __arg_holder = lbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), rbrace: Arc::new(Mutex::new(Some({ let __arg_holder = rbrace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::SelectStmt { select: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), body: body.clone(), ..Default::default() })));
    }
    }

    pub fn parse_for_stmt(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ForStmt".to_string())))));
    }));
    }
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_O_R as i32))))))));
        let mut s1: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut s2: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut s3: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut isRange: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        let mut prevLev = Arc::new(Mutex::new(Some({ let __selector_holder = self.expr_lev.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = -1; *self.expr_lev.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x != __tmp_y } {
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_A_N_G_E as i32)))); __tmp_x == __tmp_y } {
                // "for range x" (nil lhs in assignment)
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        let mut y = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr { op_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), op: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_A_N_G_E as i32))))))), x: self.parse_rhs().clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))])));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::AssignStmt { rhs: y.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s2.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = true; *isRange.lock().unwrap() = Some(new_val); };
    } else {
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(2)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *s2.lock().unwrap() = __moved_tmp_0; *isRange.lock().unwrap() = Some(__tmp_1); };
    }
    }
                // "for range x" (nil lhs in assignment)
        if !{ let __v = (*isRange.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let __iface_handle = s2.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *s1.lock().unwrap() = (*__iface_guard).clone(); };
        *s2.lock().unwrap() = None;
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *s2.lock().unwrap() = __moved_tmp_0; };
    }
        self.expect_semi();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *s3.lock().unwrap() = __moved_tmp_0; };
    }
    }
        { let new_val = prevLev.lock().unwrap().as_ref().unwrap().clone(); *self.expr_lev.lock().unwrap() = Some(new_val); };
    }
                // "for range x" (nil lhs in assignment)
        let mut body = self.parse_block_stmt();
        self.expect_semi();
        if { let __v = (*isRange.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut r#as = ({
        let val = s2.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::AssignStmtPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
                // check lhs
        let mut key: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut value: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let _switch_val = ({ let __len_target = { let __field = (*r#as.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) });
    if _switch_val == (0) {
        } else if _switch_val == (1) {
            { let __iface_handle = { let __seq = { let __seq_holder = (*r#as.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *key.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (2) {
            { let __tmp_0 = { let __seq = { let __seq_holder = (*r#as.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __tmp_1 = { let __seq = { let __seq_holder = (*r#as.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone(); { let __iface_handle = __tmp_0; let __iface_guard = __iface_handle.lock().unwrap(); *key.lock().unwrap() = (*__iface_guard).clone(); } { let __iface_handle = __tmp_1; let __iface_guard = __iface_handle.lock().unwrap(); *value.lock().unwrap() = (*__iface_guard).clone(); } };
        } else {
            self.error_expected({ let __recv = { let __seq = { let __seq_holder = (*r#as.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = (*r#as.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("at most 2 expressions".to_string()))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadStmt { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: self.safe_pos({ let __recv = body.clone(); let __recv_ptr: *const go_ast::r#mod::BlockStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BlockStmt }; let __result = unsafe { &*__recv_ptr }.end(); __result }), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
        }
    }
                // nothing to do
                // parseSimpleStmt returned a right-hand side that
                // is a single unary expression of the form "range x"
        let mut x = (*({
        let val = { let __seq = { let __seq_holder = (*r#as.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::UnaryExprPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).x.clone();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::RangeStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::RangeStmt { r#for: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), key: key.clone(), value: value.clone(), tok_pos: Arc::new(Mutex::new(Some({ let __selector_holder = (*r#as.lock().unwrap().as_ref().unwrap()).tok_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), tok: Arc::new(Mutex::new(Some({ let __selector_holder = (*r#as.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), range: { let __recv = { let __seq = { let __seq_holder = (*r#as.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, x: x.clone(), body: body.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }
                // check lhs
                // nothing to do
                // parseSimpleStmt returned a right-hand side that
                // is a single unary expression of the form "range x"
                // regular for statement
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ForStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ForStmt { r#for: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), init: s1.clone(), cond: self.make_expr(s2.clone(), Arc::new(Mutex::new(Some("boolean or range expression".to_string())))).clone(), post: s3.clone(), body: body.clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)));
    }
    }

    pub fn parse_stmt(&mut self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        dec_nest_lev(inc_nest_lev(Arc::new(Mutex::new(Some(p_defer_captured.clone())))));
    }));
        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Statement".to_string())))));
    }));
    }
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::DeclStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::DeclStmt { decl: self.parse_decl(stmtStart.clone()).clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_A_G as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_R as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_U_C_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_A_P as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T_E_R_F_A_C_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_O_T as i32))))) {
            { let (__tmp_0, __tmp_1) = self.parse_simple_stmt(Arc::new(Mutex::new(Some(1)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_0; };
                        // because of the required look-ahead, labeled statements are
                        // parsed by parseSimpleStmt - don't expect a semicolon after
                        // them
            {
        let (_, mut isLabeledStmt) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::LabeledStmtPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::LabeledStmt>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::LabeledStmt>)), false)
        }
    });;
        if !isLabeledStmt {
            self.expect_semi();;
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O as i32))))) {
            { let __iface_handle = self.parse_go_stmt().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_E_R as i32))))) {
            { let __iface_handle = self.parse_defer_stmt().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_T_U_R_N as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ReturnStmtPtr(self.parse_return_stmt().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_T_I_N_U_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_O_T_O as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BranchStmtPtr({ let __method_arg0 = { let __field = self.tok.clone(); __field }; self.parse_branch_stmt(__method_arg0) }.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr(self.parse_block_stmt().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
            self.expect_semi();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_F as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IfStmtPtr(self.parse_if_stmt().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_W_I_T_C_H as i32))))) {
            { let __iface_handle = self.parse_switch_stmt().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_L_E_C_T as i32))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectStmtPtr(self.parse_select_stmt().clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_O_R as i32))))) {
            { let __iface_handle = self.parse_for_stmt().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32))))) {
                        // Is it ever possible to have an implicit semicolon
                        // producing an empty statement in a valid program?
                        // (handle correctly anyway)
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EmptyStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::EmptyStmt { semicolon: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), implicit: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.lit.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "\n".to_string(); __tmp_x == __tmp_y }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
            self.next();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32))))) {
                        // a semicolon may be omitted before a closing "}"
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EmptyStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::EmptyStmt { semicolon: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), implicit: Arc::new(Mutex::new(Some(true))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
                        // no statement found
            let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("statement".to_string()))));
            self.advance(stmtStart.clone());
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadStmt { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *s.lock().unwrap() = (*__iface_guard).clone(); };
        }
    }
                // tokens that may start an expression
                // operands
                // composite types
                // unary operators
                // because of the required look-ahead, labeled statements are
                // parsed by parseSimpleStmt - don't expect a semicolon after
                // them
                // Is it ever possible to have an implicit semicolon
                // producing an empty statement in a valid program?
                // (handle correctly anyway)
                // a semicolon may be omitted before a closing "}"
                // no statement found
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return s;
    }
    }

    pub fn parse_import_spec(&mut self, doc: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, _: Arc<Mutex<Option<go_token::r#mod::Token>>>, _: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("ImportSpec".to_string())))));
    }));
    }
        let mut ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>> = Arc::new(Mutex::new(None));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) {
            { let new_val = self.parse_ident().clone(); ident = new_val; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32))))) {
            { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { name_pos: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), name: Arc::new(Mutex::new(Some(".".to_string()))), ..Default::default() }))).clone(); ident = new_val; };
            self.next();
        }
    }
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut path: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32)))); __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = self.lit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *path.lock().unwrap() = Some(new_val); };
        self.next();
    } else if go_token::r#mod::Token::is_literal(&(*self.tok.lock().unwrap().as_ref().unwrap())) {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("import path must be a string".to_string()))));
        self.next();
    } else {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("missing import path".to_string()))));
        self.advance(exprEnd.clone());
    }
        let mut comment = self.expect_semi();
                // collect imports
        let mut spec = Arc::new(Mutex::new(Some(go_ast::r#mod::ImportSpec { doc: doc.clone(), name: ident.clone(), path: Arc::new(Mutex::new(Some(go_ast::r#mod::BasicLit { value_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), kind: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32))))))), value: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(), comment: comment.clone(), ..Default::default() })));
        { let new_val = { let __append_target = self.imports.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(spec.clone()); __append_target.clone() }; self.imports = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ImportSpecPtr(spec.clone())) as Box<dyn go_ast::r#mod::Spec + Send + Sync>)));
    }
    }

    pub fn parse_value_spec(&mut self, doc: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, keyword: Arc<Mutex<Option<go_token::r#mod::Token>>>, iota: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let keyword_defer_captured = keyword.clone(); let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some(format!("{}{}", (*go_token::r#mod::Token::string(&(*keyword_defer_captured.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()), "Spec".to_string()))))));
    }));
    }
        let mut idents = self.parse_ident_list();
        let mut typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut values: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        { let _switch_val = (*keyword.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32))))) {
                        // always permit optional type and initialization for more tolerant parsing
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x != __tmp_y } {
        { let __iface_handle = self.try_ident_or_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = self.parse_list(Arc::new(Mutex::new(Some(true)))); values = new_val; };
    }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))) {
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x != __tmp_y } {
        { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = self.parse_list(Arc::new(Mutex::new(Some(true)))); values = new_val; };
    }
        } else {
            panic!("unreachable");
        }
    }
                // always permit optional type and initialization for more tolerant parsing
        let mut comment = self.expect_semi();
        let mut spec = Arc::new(Mutex::new(Some(go_ast::r#mod::ValueSpec { doc: doc.clone(), names: idents.clone(), r#type: typ.clone(), values: values.clone(), comment: comment.clone(), ..Default::default() })));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(spec.clone())) as Box<dyn go_ast::r#mod::Spec + Send + Sync>)));
    }
    }

    pub fn parse_generic_type(&mut self, spec: Arc<Mutex<Option<go_ast::r#mod::TypeSpec>>>, openPos: Arc<Mutex<Option<go_token::position::Pos>>>, name0: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, typ0: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("parseGenericType".to_string())))));
    }));
    }
        let mut list = self.parse_parameter_list(name0.clone(), typ0.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        let mut closePos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32))))))));
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { opening: Arc::new(Mutex::new(Some({ let __arg_holder = openPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), list: list.clone(), closing: Arc::new(Mutex::new(Some({ let __arg_holder = closePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); (*spec.lock().unwrap().as_mut().unwrap()).type_params = new_val; };
                // Let the type checker decide whether to accept type parameters on aliases:
                // see go.dev/issue/46477.
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y } {
                // type alias
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*spec.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap() = Some(new_val); };
        self.next();
    }
                // type alias
        { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*spec.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() = (*__iface_guard).clone(); };

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }

    pub fn parse_type_spec(&mut self, doc: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, _: Arc<Mutex<Option<go_token::r#mod::Token>>>, _: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("TypeSpec".to_string())))));
    }));
    }
        let mut name = self.parse_ident();
        let mut spec = Arc::new(Mutex::new(Some(go_ast::r#mod::TypeSpec { doc: doc.clone(), name: name.clone(), ..Default::default() })));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x == __tmp_y } {
                // spec.Name "[" ...
                // array/slice type or type parameter list
        let mut lbrack = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.next();
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); __tmp_x == __tmp_y } {
                // We may have an array type or a type parameter list.
                // In either case we expect an expression x (which may
                // just be a name, or a more complex expression) which
                // we can analyze further.
                //
                // A type parameter list may have a type bound starting
                // with a "[" as in: P []E. In that case, simply parsing
                // an expression would lead to an error: P[] is invalid.
                // But since index or slice expressions are never constant
                // and thus invalid array length expressions, if the name
                // is followed by "[" it must be the start of an array or
                // slice constraint. Only if we don't see a "[" do we
                // need to parse a full expression. Notably, name <- x
                // is not a concern because name <- x is a statement and
                // not an expression.
        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(self.parse_ident().clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); __tmp_x != __tmp_y } {
                // To parse the expression starting with name, expand
                // the call sequence we would get by passing in name
                // to parser.expr, and pass in name to parsePrimaryExpr.
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut lhs = self.parse_primary_expr(x.clone());
        { let __iface_handle = self.parse_binary_expr(lhs.clone(), Arc::new(Mutex::new(Some(1)))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        { let __target = self.expr_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // To parse the expression starting with name, expand
                // the call sequence we would get by passing in name
                // to parser.expr, and pass in name to parsePrimaryExpr.
                // Analyze expression x. If we can split x into a type parameter
                // name, possibly followed by a type parameter type, we consider
                // this the start of a type parameter list, with some caveats:
                // a single name followed by "]" tilts the decision towards an
                // array declaration; a type parameter type that could also be
                // an ordinary expression but which is followed by a comma tilts
                // the decision towards a type parameter list.
        {
        let (mut pname, mut ptype) = extract_name(x.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); __tmp_x == __tmp_y }))));;
        if (*pname.lock().unwrap()).is_some() && ((*ptype.lock().unwrap()).is_some() || { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); __tmp_x != __tmp_y }) {
            self.parse_generic_type(spec.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pname.clone(), ptype.clone());;
        } else {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ArrayTypePtr(self.parse_array_type(Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x.clone()).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*spec.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    } else {
                // array type
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ArrayTypePtr(self.parse_array_type(Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*spec.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } else {
                // no type parameters
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32)))); __tmp_x == __tmp_y } {
                // type alias
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*spec.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap() = Some(new_val); };
        self.next();
    }
                // type alias
        { let __iface_handle = self.parse_type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*spec.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // spec.Name "[" ...
                // array/slice type or type parameter list
                // We may have an array type or a type parameter list.
                // In either case we expect an expression x (which may
                // just be a name, or a more complex expression) which
                // we can analyze further.
                //
                // A type parameter list may have a type bound starting
                // with a "[" as in: P []E. In that case, simply parsing
                // an expression would lead to an error: P[] is invalid.
                // But since index or slice expressions are never constant
                // and thus invalid array length expressions, if the name
                // is followed by "[" it must be the start of an array or
                // slice constraint. Only if we don't see a "[" do we
                // need to parse a full expression. Notably, name <- x
                // is not a concern because name <- x is a statement and
                // not an expression.
                // To parse the expression starting with name, expand
                // the call sequence we would get by passing in name
                // to parser.expr, and pass in name to parsePrimaryExpr.
                // Analyze expression x. If we can split x into a type parameter
                // name, possibly followed by a type parameter type, we consider
                // this the start of a type parameter list, with some caveats:
                // a single name followed by "]" tilts the decision towards an
                // array declaration; a type parameter type that could also be
                // an ordinary expression but which is followed by a comma tilts
                // the decision towards a type parameter list.
                // spec.Name "[" pname ...
                // spec.Name "[" pname ptype ...
                // spec.Name "[" pname ptype "," ...
                // ptype may be nil
                // spec.Name "[" pname "]" ...
                // spec.Name "[" x ...
                // array type
                // no type parameters
                // type alias
        { let new_val = self.expect_semi().clone(); (*spec.lock().unwrap().as_mut().unwrap()).comment = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSpecPtr(spec.clone())) as Box<dyn go_ast::r#mod::Spec + Send + Sync>)));
    }
    }

    pub fn parse_gen_decl(&mut self, keyword: Arc<Mutex<Option<go_token::r#mod::Token>>>, f: parseSpecFunction) -> Arc<Mutex<Option<go_ast::r#mod::GenDecl>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let keyword_defer_captured = keyword.clone(); let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "GenDecl(".to_string())); __s.push_str(&format!("{}", (*go_token::r#mod::Token::string(&(*keyword_defer_captured.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })))));
    }));
    }
        let mut doc = self.lead_comment.clone();
        let mut pos = self.expect(Arc::new(Mutex::new(Some({ let __arg_holder = keyword.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut lparen: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));let mut rparen: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *lparen.lock().unwrap() = Some(new_val); };
        self.next();
        let mut iota = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = self.lead_comment.clone(); __field }, keyword.clone(), iota.clone()) }.clone()); __append_target.clone() }; list = new_val; };
        { let mut guard = iota.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rparen.lock().unwrap() = __moved_val; };
        self.expect_semi();
    } else {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None)), keyword.clone(), Arc::new(Mutex::new(Some(0)))) }.clone()); __append_target.clone() }; list = new_val; };
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(go_ast::r#mod::GenDecl { doc: doc.clone(), tok_pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), tok: Arc::new(Mutex::new(Some({ let __arg_holder = keyword.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), lparen: Arc::new(Mutex::new(Some({ let __arg_holder = lparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), specs: list.clone(), rparen: Arc::new(Mutex::new(Some({ let __arg_holder = rparen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    }
    }

    pub fn parse_func_decl(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::FuncDecl>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("FunctionDecl".to_string())))));
    }));
    }
        let mut doc = self.lead_comment.clone();
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))))));
        let mut recv: Arc<Mutex<Option<go_ast::r#mod::FieldList>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = self.parse_parameters(Arc::new(Mutex::new(Some(false)))); recv = __tmp_1.clone(); };
    }
        let mut ident = self.parse_ident();
        let (mut tparams, mut params) = self.parse_parameters(Arc::new(Mutex::new(Some(true))));
        if (*recv.lock().unwrap()).is_some() && (*tparams.lock().unwrap()).is_some() {
                // Method declarations do not have type parameters. We parse them for a
                // better error message and improved error recovery.
        self.error({ let __field = (*tparams.lock().unwrap().as_ref().unwrap()).opening.clone(); __field }, Arc::new(Mutex::new(Some("method must have no type parameters".to_string()))));
        *tparams.lock().unwrap() = None;
    }
                // Method declarations do not have type parameters. We parse them for a
                // better error message and improved error recovery.
        let mut results = self.parse_result();
        let mut body: Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>> = Arc::new(Mutex::new(None));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32))))) {
            { let new_val = self.parse_body().clone(); body = new_val; };
            self.expect_semi();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32))))) {
            self.next();
            if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); __tmp_x == __tmp_y } {
                // opening { of function declaration on next line
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("unexpected semicolon or newline before {".to_string()))); self.error(__method_arg0, __method_arg1) };
        { let new_val = self.parse_body().clone(); body = new_val; };
        self.expect_semi();
    }
        } else {
            self.expect_semi();
        }
    }
                // opening { of function declaration on next line
        let mut decl = Arc::new(Mutex::new(Some(go_ast::r#mod::FuncDecl { doc: doc.clone(), recv: recv.clone(), name: ident.clone(), r#type: Arc::new(Mutex::new(Some(go_ast::r#mod::FuncType { func: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), type_params: tparams.clone(), params: params.clone(), results: results.clone(), ..Default::default() }))).clone(), body: body.clone(), ..Default::default() })));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return decl.clone();
    }
    }

    pub fn parse_decl(&mut self, sync: Arc<Mutex<Option<BTreeMap<go_token::r#mod::Token, Arc<Mutex<Option<bool>>>>>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("Declaration".to_string())))));
    }));
    }
        let mut f: parseSpecFunction = Arc::new(Mutex::new(None));
        { let _switch_val = { let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32))))) {
            { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, __arg1: Arc<Mutex<Option<go_token::r#mod::Token>>>, __arg2: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> { __recv.parse_import_spec(__arg0, __arg1, __arg2) }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> }))); f = new_val; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))) {
            { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, __arg1: Arc<Mutex<Option<go_token::r#mod::Token>>>, __arg2: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> { __recv.parse_value_spec(__arg0, __arg1, __arg2) }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> }))); f = new_val; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_Y_P_E as i32))))) {
            { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, __arg1: Arc<Mutex<Option<go_token::r#mod::Token>>>, __arg2: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> { __recv.parse_type_spec(__arg0, __arg1, __arg2) }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> }))); f = new_val; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_U_N_C as i32))))) {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncDeclPtr(self.parse_func_decl().clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)));
    }
        } else {
            let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            self.error_expected(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("declaration".to_string()))));
            self.advance(sync.clone());
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BadDeclPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::BadDecl { from: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), to: Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)));
    }
        }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr({ let __method_arg0 = { let __field = self.tok.clone(); __field }; let __method_arg1 = f.clone(); self.parse_gen_decl(__method_arg0, __method_arg1) }.clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)));
    }
    }

    pub fn parse_file(&mut self) -> Arc<Mutex<Option<go_ast::r#mod::File>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*self.trace.clone().lock().unwrap().as_ref().unwrap()) {
        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        un(trace(Arc::new(Mutex::new(Some(p_defer_captured.clone()))), Arc::new(Mutex::new(Some("File".to_string())))));
    }));
    }
                // Don't bother parsing the rest if we had errors scanning the first token.
                // Likely not a Go source file at all.
        if { let __tmp_x = (*self.errors.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }
                // package clause
        let mut doc = self.lead_comment.clone();
        let mut pos = self.expect(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_A_C_K_A_G_E as i32))))))));
                // Go spec: The package clause is not a declaration;
                // the package name does not appear in any scope.
        let mut ident = self.parse_ident();
        if { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & DECLARATION_ERRORS as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("invalid package name _".to_string()))); self.error(__method_arg0, __method_arg1) };
    }
        self.expect_semi();
                // Don't bother parsing the rest if we had errors parsing the package clause.
                // Likely not a Go source file at all.
        if { let __tmp_x = (*self.errors.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }
        let mut decls: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & PACKAGE_CLAUSE_ONLY as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // import decls
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32)))); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = decls.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr({ let __method_arg0 = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32))))))); let __method_arg1 = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, __arg1: Arc<Mutex<Option<go_token::r#mod::Token>>>, __arg2: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> { __recv.parse_import_spec(__arg0, __arg1, __arg2) }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::CommentGroup>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Spec + Send + Sync>>>> + Send + Sync> }))); self.parse_gen_decl(__method_arg0, __method_arg1) }.clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)))); __append_target.clone() }; decls = new_val; };
    }
        if { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & IMPORTS_ONLY as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // rest of package body
        let mut prev = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32)))))));
        while { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); __tmp_x != __tmp_y } {
                // Continue to accept import declarations for error tolerance, but complain.
        if { let __tmp_x = (*self.tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = (*prev.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_P_O_R_T as i32)))); __tmp_x != __tmp_y } {
        { let __method_arg0 = { let __field = self.pos.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some("imports must appear before other declarations".to_string()))); self.error(__method_arg0, __method_arg1) };
    }
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some((*(*self.tok.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *prev.lock().unwrap() = Some(new_val); };

        { let new_val = { let __append_target = decls.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.parse_decl(declStart.clone()).clone()); __append_target.clone() }; decls = new_val; };
    }
    }
    }
                // import decls
                // rest of package body
                // Continue to accept import declarations for error tolerance, but complain.
        let mut f = Arc::new(Mutex::new(Some(go_ast::r#mod::File { doc: doc.clone(), package: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), name: ident.clone(), decls: decls.clone(), imports: { let __field = self.imports.clone(); __field }, comments: { let __field = self.comments.clone(); __field }, go_version: Arc::new(Mutex::new(Some({ let __selector_holder = self.go_version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
                // File{Start,End} are set by the defer in the caller.
        let mut declErr: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & DECLARATION_ERRORS as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<go_token::position::Pos>>>, __arg1: Arc<Mutex<Option<String>>>| { __recv.error(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> }))); declErr = new_val; };
    }
        if { let __tmp_x = crate::interface::Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & SKIP_OBJECT_RESOLUTION as u64))))); let __tmp_y = crate::interface::Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        resolve_file(f.clone(), { let __field = self.file.clone(); __field }, declErr.clone());
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f.clone();
    }
    }
}

pub fn trace(p: Arc<Mutex<Option<parser>>>, msg: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<parser>>> {
    { let __recv = p.clone(); let __recv_ptr: *const parser = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const parser }; let __result = unsafe { &*__recv_ptr }.print_trace(Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new("(".to_string()) as Box<dyn Any + Send + Sync>])))); __result };
    { let __target = (*p.lock().unwrap().as_ref().unwrap()).indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    p.clone()
}

/// Usage pattern: defer un(trace(p, "..."))
pub fn un(p: Arc<Mutex<Option<parser>>>) {
    { let __target = (*p.lock().unwrap().as_ref().unwrap()).indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    { let __recv = p.clone(); let __recv_ptr: *const parser = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const parser }; let __result = unsafe { &*__recv_ptr }.print_trace(Arc::new(Mutex::new(Some(vec![Box::new(")".to_string()) as Box<dyn Any + Send + Sync>])))); __result };
}

pub fn inc_nest_lev(p: Arc<Mutex<Option<parser>>>) -> Arc<Mutex<Option<parser>>> {
    { let __target = (*p.lock().unwrap().as_ref().unwrap()).nest_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if { let __tmp_x = (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).nest_lev.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 100000; __tmp_x > __tmp_y } {
        { let __recv = p.clone(); let __recv_ptr: *const parser = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const parser }; let __result = unsafe { &*__recv_ptr }.error({ let __field = (*p.lock().unwrap().as_ref().unwrap()).pos.clone(); __field }, Arc::new(Mutex::new(Some("exceeded max nesting depth".to_string())))); __result };
        panic!("{}", bailout { pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), msg: Arc::new(Mutex::new(Some(String::new()))) });
    }
    p.clone()
}

/// decNestLev is used to track nesting depth during parsing to prevent stack exhaustion.
/// It is used along with incNestLev in a similar fashion to how un and trace are used.
pub fn dec_nest_lev(p: Arc<Mutex<Option<parser>>>) {
    { let __target = (*p.lock().unwrap().as_ref().unwrap()).nest_lev.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
}

pub fn assert(cond: Arc<Mutex<Option<bool>>>, msg: Arc<Mutex<Option<String>>>) {
    if !{ let __v = (*cond.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        panic!("{}", format!("{}{}", "go/parser internal error: ".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    }
}

pub fn is_type_switch_assert(x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> bool {
    let (mut a, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::TypeAssertExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::TypeAssertExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::TypeAssertExpr>)), false)
        }
    });
    return ok && { let __iface_handle = { let __field = (*a.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() };
}

/// extractName splits the expression x into (name, expr) if syntactically
/// x can be written as name expr. The split only happens if expr is a type
/// element (per the isTypeElem predicate) or if force is set.
/// If x is just a name, the result is (name, nil). If the split succeeds,
/// the result is (name, expr). Otherwise the result is (nil, x).
/// Examples:
///
///	x           force    name    expr
///	------------------------------------
///	P*[]int     T/F      P       *[]int
///	P*E         T        P       *E
///	P*E         F        nil     P*E
///	P([]int)    T/F      P       ([]int)
///	P(E)        T        P       (E)
///	P(E)        F        nil     P(E)
///	P*E|F|~G    T/F      P       *E|F|~G
///	P*E|F|G     T        P       *E|F|G
///	P*E|F|G     F        nil     P*E|F|G
pub fn extract_name(mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, force: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<go_ast::r#mod::Ident>>>, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
    let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
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
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).unwrap().0.clone();
        return (x.clone(), Arc::new(Mutex::new(None)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExprPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExprPtr>()).unwrap().0.clone();
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            {
        let (mut name, _) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).x.clone();
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
        if (*name.lock().unwrap()).is_some() && ({ let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } || is_type_elem((*x.lock().unwrap().as_ref().unwrap()).y.clone())) {
            return (name.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::StarExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::StarExpr { star: Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).op_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), x: { let __field = (*x.lock().unwrap().as_ref().unwrap()).y.clone(); __field }, ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))));;
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32))))) {
            {
        let (mut name, mut lhs) = extract_name((*x.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(Some({ let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } || is_type_elem((*x.lock().unwrap().as_ref().unwrap()).y.clone())))));;
        if (*name.lock().unwrap()).is_some() && (*lhs.lock().unwrap()).is_some() {
            let mut op = Arc::new(Mutex::new(Some({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v })));;
            { let __iface_handle = lhs.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*op.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = (*__iface_guard).clone(); };;
            return (name.clone(), Arc::new(Mutex::new(Some(Box::new((*op.clone().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))));;
        }
    }
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CallExprPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CallExprPtr>()).unwrap().0.clone();
        {
        let (mut name, _) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).fun.clone();
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
        if (*name.lock().unwrap()).is_some() {
            if { let __tmp_x = (({ let __len_target = { let __field = (*x.lock().unwrap().as_ref().unwrap()).args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).ellipsis.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))); __tmp_x == __tmp_y } && ({ let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } || is_type_elem({ let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone())) {
        return (name.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ParenExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ParenExpr { lparen: Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).lparen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), x: { let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), rparen: Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).rparen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))));
    };
        }
    };
    }
    }
        // x = name *x.Y
        // x = name lhs|x.Y
        // x = name (x.Args[0])
        // (Note that the cmd/compile/internal/syntax parser does not care
        // about syntax tree fidelity and does not preserve parentheses here.)
    return (Arc::new(Mutex::new(None)), x.clone());
}

/// isTypeElem reports whether x is a (possibly parenthesized) type element expression.
/// The result is false if x could be a type element OR an ordinary (value) expression.
pub fn is_type_elem(mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> bool {
    let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ArrayTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::MapTypePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ChanTypePtr>()).is_some() {
        let x = x.clone();
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExprPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExprPtr>()).unwrap().0.clone();
        return is_type_elem((*x.lock().unwrap().as_ref().unwrap()).x.clone()) || is_type_elem((*x.lock().unwrap().as_ref().unwrap()).y.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExprPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExprPtr>()).unwrap().0.clone();
        return { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); __tmp_x == __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExprPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExprPtr>()).unwrap().0.clone();
        return is_type_elem((*x.lock().unwrap().as_ref().unwrap()).x.clone());;
    }
    }
    false
}

/// packIndexExpr returns an IndexExpr x[expr0] or IndexListExpr x[expr0, ...].
pub fn pack_index_expr(x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, lbrack: Arc<Mutex<Option<go_token::position::Pos>>>, exprs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, rbrack: Arc<Mutex<Option<go_token::position::Pos>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    { let _switch_val = (*exprs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0);
    if _switch_val == (0) {
            panic!("internal error: packIndexExpr with empty expr slice");
        } else if _switch_val == (1) {
            return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::IndexExpr { x: x.clone(), lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: { let __seq = { let __seq_holder = exprs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), rbrack: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
        } else {
            return Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexListExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::IndexListExpr { x: x.clone(), lbrack: Arc::new(Mutex::new(Some({ let __arg_holder = lbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), indices: exprs.clone(), rbrack: Arc::new(Mutex::new(Some({ let __arg_holder = rbrack.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
        }
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


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for parser {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for bailout {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for field {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
