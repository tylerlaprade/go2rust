use go2rust_stdlib_stubs::*;

use crate::{format_any, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::commentmap::*;
use crate::filter::*;
use crate::import::*;
use crate::print::*;
use crate::resolve::*;
use crate::scope::*;
use crate::walk::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const S_E_N_D: i32 = 1 << 0;
pub const R_E_C_V: i32 = 1 << 1;


/// All node types implement the Node interface.
pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool;
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
}

impl Clone for Box<dyn Node + Send + Sync> {
    fn clone(&self) -> Self {
        Node::__go_clone_box_node(self.as_ref())
    }
}

impl GoValueClone for Box<dyn Node + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        Node::__go_clone_box_node(self.as_ref())
    }
}

/// All expression nodes implement the Expr interface.
pub trait Expr: Node + std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync>;
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool;
    fn expr_node(&self);
}

impl Clone for Box<dyn Expr + Send + Sync> {
    fn clone(&self) -> Self {
        Expr::__go_clone_box_expr(self.as_ref())
    }
}

impl GoValueClone for Box<dyn Expr + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        Expr::__go_clone_box_expr(self.as_ref())
    }
}

impl Node for Box<dyn Expr + Send + Sync> {
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        (**self).__go_eq_node(other)
    }
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).end()
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
}

/// All statement nodes implement the Stmt interface.
pub trait Stmt: Node + std::fmt::Display + Any {
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync>;
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool;
    fn stmt_node(&self);
}

impl Clone for Box<dyn Stmt + Send + Sync> {
    fn clone(&self) -> Self {
        Stmt::__go_clone_box_stmt(self.as_ref())
    }
}

impl GoValueClone for Box<dyn Stmt + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        Stmt::__go_clone_box_stmt(self.as_ref())
    }
}

impl Node for Box<dyn Stmt + Send + Sync> {
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        (**self).__go_eq_node(other)
    }
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).end()
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
}

/// All declaration nodes implement the Decl interface.
pub trait Decl: Node + std::fmt::Display + Any {
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync>;
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool;
    fn decl_node(&self);
}

impl Clone for Box<dyn Decl + Send + Sync> {
    fn clone(&self) -> Self {
        Decl::__go_clone_box_decl(self.as_ref())
    }
}

impl GoValueClone for Box<dyn Decl + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        Decl::__go_clone_box_decl(self.as_ref())
    }
}

impl Node for Box<dyn Decl + Send + Sync> {
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        (**self).__go_eq_node(other)
    }
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).end()
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
}

/// A Comment node represents a single //-style or /*-style comment.
///
/// The Text field contains the comment text without carriage returns (\r) that
/// may have been present in the source. Because a comment's end position is
/// computed using len(Text), the position reported by [Comment.End] does not match the
/// true source end position for comments containing carriage returns.
#[derive(Clone)]
pub struct Comment {
    pub slash: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub text: Arc<Mutex<Option<String>>>,
}

impl Comment {
    pub fn __go_value_clone(&self) -> Self {
        Self { slash: { let __guard = self.slash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, text: { let __guard = self.text.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Comment {
    fn default() -> Self {
        Self { slash: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), text: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.slash.lock().unwrap().as_ref().unwrap()), (*self.text.lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Comment {
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

impl GoJsonDecode for Comment {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Text") {
            out.text = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A CommentGroup represents a sequence of comments
/// with no other tokens and no empty lines between.
#[derive(Clone, Default)]
pub struct CommentGroup {
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Comment>>>>>>>,
}

impl CommentGroup {
    pub fn __go_value_clone(&self) -> Self {
        Self { list: self.list.clone() }
    }
}

impl std::fmt::Display for CommentGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped(&self.list))
    }
}
impl GoComparable for CommentGroup {
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

impl GoJsonDecode for CommentGroup {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Field represents a Field declaration list in a struct type,
/// a method list in an interface type, or a parameter/result declaration
/// in a signature.
/// [Field.Names] is nil for unnamed parameters (parameter lists which only contain types)
/// and embedded struct fields. In the latter case, the field name is the type name.
#[derive(Clone, Default)]
pub struct Field {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Ident>>>>>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub tag: Arc<Mutex<Option<BasicLit>>>,
    pub comment: Arc<Mutex<Option<CommentGroup>>>,
}

impl Field {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), names: self.names.clone(), r#type: self.r#type.clone(), tag: self.tag.clone(), comment: self.comment.clone() }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice_wrapped(&self.names), (*self.r#type.lock().unwrap().as_ref().unwrap()), { let __guard = self.tag.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.comment.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}
impl GoComparable for Field {
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

impl GoJsonDecode for Field {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A FieldList represents a list of Fields, enclosed by parentheses,
/// curly braces, or square brackets.
#[derive(Clone)]
pub struct FieldList {
    pub opening: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Field>>>>>>>,
    pub closing: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl FieldList {
    pub fn __go_value_clone(&self) -> Self {
        Self { opening: { let __guard = self.opening.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: self.list.clone(), closing: { let __guard = self.closing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for FieldList {
    fn default() -> Self {
        Self { opening: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), list: Arc::new(Mutex::new(None)), closing: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for FieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.opening.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.list), (*self.closing.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for FieldList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct BadExpr {
    pub from: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub to: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl BadExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { from: { let __guard = self.from.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, to: { let __guard = self.to.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for BadExpr {
    fn default() -> Self {
        Self { from: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), to: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for BadExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.from.lock().unwrap().as_ref().unwrap()), (*self.to.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for BadExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct Ident {
    pub name_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub obj: Arc<Mutex<Option<Object>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { name_pos: { let __guard = self.name_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, obj: self.obj.clone() }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { name_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), name: Arc::new(Mutex::new(Some(String::new()))), obj: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Ident {
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

impl GoJsonDecode for Ident {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct Ellipsis {
    pub ellipsis: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub elt: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl Ellipsis {
    pub fn __go_value_clone(&self) -> Self {
        Self { ellipsis: { let __guard = self.ellipsis.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, elt: self.elt.clone() }
    }
}


impl Default for Ellipsis {
    fn default() -> Self {
        Self { ellipsis: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), elt: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Ellipsis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.ellipsis.lock().unwrap().as_ref().unwrap()), (*self.elt.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Ellipsis {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct BasicLit {
    pub value_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub kind: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub value: Arc<Mutex<Option<String>>>,
}

impl BasicLit {
    pub fn __go_value_clone(&self) -> Self {
        Self { value_pos: { let __guard = self.value_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for BasicLit {
    fn default() -> Self {
        Self { value_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), kind: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), value: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for BasicLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.value_pos.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for BasicLit {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Value") {
            out.value = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone, Default)]
pub struct FuncLit {
    pub r#type: Arc<Mutex<Option<FuncType>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl FuncLit {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone(), body: self.body.clone() }
    }
}

impl std::fmt::Display for FuncLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.r#type.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for FuncLit {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct CompositeLit {
    pub r#type: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub lbrace: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub elts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
    pub rbrace: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub incomplete: Arc<Mutex<Option<bool>>>,
}

impl CompositeLit {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone(), lbrace: { let __guard = self.lbrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, elts: self.elts.clone(), rbrace: { let __guard = self.rbrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, incomplete: { let __guard = self.incomplete.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for CompositeLit {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(None)), lbrace: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), elts: Arc::new(Mutex::new(None)), rbrace: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), incomplete: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for CompositeLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.lbrace.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.elts), (*self.rbrace.lock().unwrap().as_ref().unwrap()), (*self.incomplete.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for CompositeLit {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Incomplete") {
            out.incomplete = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct ParenExpr {
    pub lparen: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub rparen: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl ParenExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { lparen: { let __guard = self.lparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: self.x.clone(), rparen: { let __guard = self.rparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ParenExpr {
    fn default() -> Self {
        Self { lparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), x: Arc::new(Mutex::new(None)), rparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lparen.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()), (*self.rparen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ParenExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone, Default)]
pub struct SelectorExpr {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub sel: Arc<Mutex<Option<Ident>>>,
}

impl SelectorExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), sel: self.sel.clone() }
    }
}

impl std::fmt::Display for SelectorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), { let __guard = self.sel.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for SelectorExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct IndexExpr {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub lbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub index: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub rbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl IndexExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), lbrack: { let __guard = self.lbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: self.index.clone(), rbrack: { let __guard = self.rbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IndexExpr {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(None)), lbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), index: Arc::new(Mutex::new(None)), rbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.lbrack.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), (*self.rbrack.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IndexExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct IndexListExpr {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub lbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub indices: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
    pub rbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl IndexListExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), lbrack: { let __guard = self.lbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, indices: self.indices.clone(), rbrack: { let __guard = self.rbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IndexListExpr {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(None)), lbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), indices: Arc::new(Mutex::new(None)), rbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for IndexListExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.lbrack.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.indices), (*self.rbrack.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IndexListExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct SliceExpr {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub lbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub low: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub high: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub max: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub slice3: Arc<Mutex<Option<bool>>>,
    pub rbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl SliceExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), lbrack: { let __guard = self.lbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, low: self.low.clone(), high: self.high.clone(), max: self.max.clone(), slice3: { let __guard = self.slice3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rbrack: { let __guard = self.rbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for SliceExpr {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(None)), lbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), low: Arc::new(Mutex::new(None)), high: Arc::new(Mutex::new(None)), max: Arc::new(Mutex::new(None)), slice3: Arc::new(Mutex::new(Some(false))), rbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for SliceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.lbrack.lock().unwrap().as_ref().unwrap()), (*self.low.lock().unwrap().as_ref().unwrap()), (*self.high.lock().unwrap().as_ref().unwrap()), (*self.max.lock().unwrap().as_ref().unwrap()), (*self.slice3.lock().unwrap().as_ref().unwrap()), (*self.rbrack.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SliceExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Slice3") {
            out.slice3 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct TypeAssertExpr {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub lparen: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub rparen: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl TypeAssertExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), lparen: { let __guard = self.lparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: self.r#type.clone(), rparen: { let __guard = self.rparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for TypeAssertExpr {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(None)), lparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), r#type: Arc::new(Mutex::new(None)), rparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for TypeAssertExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.lparen.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.rparen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for TypeAssertExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct CallExpr {
    pub fun: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub lparen: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
    pub ellipsis: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub rparen: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl CallExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { fun: self.fun.clone(), lparen: { let __guard = self.lparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, args: self.args.clone(), ellipsis: { let __guard = self.ellipsis.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rparen: { let __guard = self.rparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for CallExpr {
    fn default() -> Self {
        Self { fun: Arc::new(Mutex::new(None)), lparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), args: Arc::new(Mutex::new(None)), ellipsis: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), rparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.fun.lock().unwrap().as_ref().unwrap()), (*self.lparen.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.args), (*self.ellipsis.lock().unwrap().as_ref().unwrap()), (*self.rparen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for CallExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct StarExpr {
    pub star: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl StarExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { star: { let __guard = self.star.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: self.x.clone() }
    }
}


impl Default for StarExpr {
    fn default() -> Self {
        Self { star: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), x: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for StarExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.star.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for StarExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct UnaryExpr {
    pub op_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub op: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl UnaryExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { op_pos: { let __guard = self.op_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, op: { let __guard = self.op.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: self.x.clone() }
    }
}


impl Default for UnaryExpr {
    fn default() -> Self {
        Self { op_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), op: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), x: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.op_pos.lock().unwrap().as_ref().unwrap()), (*self.op.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for UnaryExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct BinaryExpr {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub op_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub op: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub y: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl BinaryExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), op_pos: { let __guard = self.op_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, op: { let __guard = self.op.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, y: self.y.clone() }
    }
}


impl Default for BinaryExpr {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(None)), op_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), op: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), y: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.op_pos.lock().unwrap().as_ref().unwrap()), (*self.op.lock().unwrap().as_ref().unwrap()), (*self.y.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for BinaryExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An expression is represented by a tree consisting of one
/// or more of the following concrete expression nodes.
#[derive(Clone)]
pub struct KeyValueExpr {
    pub key: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub colon: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub value: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl KeyValueExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: self.key.clone(), colon: { let __guard = self.colon.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: self.value.clone() }
    }
}


impl Default for KeyValueExpr {
    fn default() -> Self {
        Self { key: Arc::new(Mutex::new(None)), colon: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), value: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for KeyValueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.key.lock().unwrap().as_ref().unwrap()), (*self.colon.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for KeyValueExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The direction of a channel type is indicated by a bit
/// mask including one or both of the following constants.
#[derive(Debug, Clone, Default)]
pub struct ChanDir(pub Arc<Mutex<Option<i32>>>);

impl Display for ChanDir {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ChanDir {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for ChanDir {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ChanDir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for ChanDir {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ChanDir> for i32 {
    fn eq(&self, other: &ChanDir) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ChanDir> for i32 {
    fn partial_cmp(&self, other: &ChanDir) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ChanDir {
    type Output = ChanDir;
    fn add(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ChanDir {
    type Output = ChanDir;
    fn add(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ChanDir> for i32 {
    type Output = ChanDir;
    fn add(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ChanDir {
    type Output = ChanDir;
    fn sub(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ChanDir {
    type Output = ChanDir;
    fn sub(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ChanDir> for i32 {
    type Output = ChanDir;
    fn sub(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ChanDir {
    type Output = ChanDir;
    fn mul(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for ChanDir {
    type Output = ChanDir;
    fn mul(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ChanDir> for i32 {
    type Output = ChanDir;
    fn mul(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ChanDir {
    type Output = ChanDir;
    fn div(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for ChanDir {
    type Output = ChanDir;
    fn div(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ChanDir> for i32 {
    type Output = ChanDir;
    fn div(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ChanDir {
    type Output = ChanDir;
    fn neg(self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ChanDir {
    type Output = ChanDir;
    fn rem(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ChanDir {
    type Output = ChanDir;
    fn rem(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ChanDir> for i32 {
    type Output = ChanDir;
    fn rem(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ChanDir {
    type Output = ChanDir;
    fn bitand(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ChanDir {
    type Output = ChanDir;
    fn bitand(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ChanDir> for i32 {
    type Output = ChanDir;
    fn bitand(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ChanDir {
    type Output = ChanDir;
    fn bitor(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ChanDir {
    type Output = ChanDir;
    fn bitor(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ChanDir> for i32 {
    type Output = ChanDir;
    fn bitor(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ChanDir {
    type Output = ChanDir;
    fn bitxor(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ChanDir {
    type Output = ChanDir;
    fn bitxor(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ChanDir> for i32 {
    type Output = ChanDir;
    fn bitxor(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ChanDir {
    type Output = ChanDir;
    fn not(self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: usize) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: usize) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ChanDir {}

impl Ord for ChanDir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A type is represented by a tree consisting of one
/// or more of the following type-specific expression
/// nodes.
#[derive(Clone)]
pub struct ArrayType {
    pub lbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub len: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub elt: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl ArrayType {
    pub fn __go_value_clone(&self) -> Self {
        Self { lbrack: { let __guard = self.lbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, len: self.len.clone(), elt: self.elt.clone() }
    }
}


impl Default for ArrayType {
    fn default() -> Self {
        Self { lbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), len: Arc::new(Mutex::new(None)), elt: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lbrack.lock().unwrap().as_ref().unwrap()), (*self.len.lock().unwrap().as_ref().unwrap()), (*self.elt.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ArrayType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A type is represented by a tree consisting of one
/// or more of the following type-specific expression
/// nodes.
#[derive(Clone)]
pub struct StructType {
    pub r#struct: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub fields: Arc<Mutex<Option<FieldList>>>,
    pub incomplete: Arc<Mutex<Option<bool>>>,
}

impl StructType {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#struct: { let __guard = self.r#struct.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fields: self.fields.clone(), incomplete: { let __guard = self.incomplete.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for StructType {
    fn default() -> Self {
        Self { r#struct: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), fields: Arc::new(Mutex::new(None)), incomplete: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.r#struct.lock().unwrap().as_ref().unwrap()), { let __guard = self.fields.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.incomplete.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for StructType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Incomplete") {
            out.incomplete = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A type is represented by a tree consisting of one
/// or more of the following type-specific expression
/// nodes.
#[derive(Clone)]
pub struct FuncType {
    pub func: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub type_params: Arc<Mutex<Option<FieldList>>>,
    pub params: Arc<Mutex<Option<FieldList>>>,
    pub results: Arc<Mutex<Option<FieldList>>>,
}

impl FuncType {
    pub fn __go_value_clone(&self) -> Self {
        Self { func: { let __guard = self.func.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, type_params: self.type_params.clone(), params: self.params.clone(), results: self.results.clone() }
    }
}


impl Default for FuncType {
    fn default() -> Self {
        Self { func: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), type_params: Arc::new(Mutex::new(None)), params: Arc::new(Mutex::new(None)), results: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.func.lock().unwrap().as_ref().unwrap()), { let __guard = self.type_params.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.params.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.results.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for FuncType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A type is represented by a tree consisting of one
/// or more of the following type-specific expression
/// nodes.
#[derive(Clone)]
pub struct InterfaceType {
    pub interface: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub methods: Arc<Mutex<Option<FieldList>>>,
    pub incomplete: Arc<Mutex<Option<bool>>>,
}

impl InterfaceType {
    pub fn __go_value_clone(&self) -> Self {
        Self { interface: { let __guard = self.interface.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, methods: self.methods.clone(), incomplete: { let __guard = self.incomplete.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for InterfaceType {
    fn default() -> Self {
        Self { interface: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), methods: Arc::new(Mutex::new(None)), incomplete: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.interface.lock().unwrap().as_ref().unwrap()), { let __guard = self.methods.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.incomplete.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for InterfaceType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Incomplete") {
            out.incomplete = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A type is represented by a tree consisting of one
/// or more of the following type-specific expression
/// nodes.
#[derive(Clone)]
pub struct MapType {
    pub map: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub key: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub value: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl MapType {
    pub fn __go_value_clone(&self) -> Self {
        Self { map: { let __guard = self.map.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, key: self.key.clone(), value: self.value.clone() }
    }
}


impl Default for MapType {
    fn default() -> Self {
        Self { map: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), key: Arc::new(Mutex::new(None)), value: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.map.lock().unwrap().as_ref().unwrap()), (*self.key.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for MapType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A type is represented by a tree consisting of one
/// or more of the following type-specific expression
/// nodes.
#[derive(Clone)]
pub struct ChanType {
    pub begin: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub arrow: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub dir: Arc<Mutex<Option<ChanDir>>>,
    pub value: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl ChanType {
    pub fn __go_value_clone(&self) -> Self {
        Self { begin: { let __guard = self.begin.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arrow: { let __guard = self.arrow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: self.value.clone() }
    }
}


impl Default for ChanType {
    fn default() -> Self {
        Self { begin: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), arrow: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), dir: Arc::new(Mutex::new(Some(ChanDir(Arc::new(Mutex::new(Some(0))))))), value: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for ChanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.begin.lock().unwrap().as_ref().unwrap()), (*self.arrow.lock().unwrap().as_ref().unwrap()), (*self.dir.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ChanType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct BadStmt {
    pub from: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub to: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl BadStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { from: { let __guard = self.from.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, to: { let __guard = self.to.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for BadStmt {
    fn default() -> Self {
        Self { from: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), to: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for BadStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.from.lock().unwrap().as_ref().unwrap()), (*self.to.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for BadStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone, Default)]
pub struct DeclStmt {
    pub decl: Arc<Mutex<Option<Box<dyn Decl + Send + Sync>>>>,
}

impl DeclStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { decl: self.decl.clone() }
    }
}

impl std::fmt::Display for DeclStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.decl.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for DeclStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct EmptyStmt {
    pub semicolon: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub implicit: Arc<Mutex<Option<bool>>>,
}

impl EmptyStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { semicolon: { let __guard = self.semicolon.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, implicit: { let __guard = self.implicit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for EmptyStmt {
    fn default() -> Self {
        Self { semicolon: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), implicit: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for EmptyStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.semicolon.lock().unwrap().as_ref().unwrap()), (*self.implicit.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for EmptyStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Implicit") {
            out.implicit = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct LabeledStmt {
    pub label: Arc<Mutex<Option<Ident>>>,
    pub colon: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub stmt: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
}

impl LabeledStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { label: self.label.clone(), colon: { let __guard = self.colon.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stmt: self.stmt.clone() }
    }
}


impl Default for LabeledStmt {
    fn default() -> Self {
        Self { label: Arc::new(Mutex::new(None)), colon: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), stmt: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for LabeledStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.label.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.colon.lock().unwrap().as_ref().unwrap()), (*self.stmt.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for LabeledStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone, Default)]
pub struct ExprStmt {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl ExprStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone() }
    }
}

impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ExprStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct SendStmt {
    pub chan: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub arrow: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub value: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
}

impl SendStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { chan: self.chan.clone(), arrow: { let __guard = self.arrow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: self.value.clone() }
    }
}


impl Default for SendStmt {
    fn default() -> Self {
        Self { chan: Arc::new(Mutex::new(None)), arrow: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), value: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for SendStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.chan.lock().unwrap().as_ref().unwrap()), (*self.arrow.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SendStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct IncDecStmt {
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub tok_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub tok: Arc<Mutex<Option<go_token::r#mod::Token>>>,
}

impl IncDecStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), tok_pos: { let __guard = self.tok_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tok: { let __guard = self.tok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IncDecStmt {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(None)), tok_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for IncDecStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.tok_pos.lock().unwrap().as_ref().unwrap()), (*self.tok.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IncDecStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct AssignStmt {
    pub lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
    pub tok_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub tok: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub rhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
}

impl AssignStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { lhs: self.lhs.clone(), tok_pos: { let __guard = self.tok_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tok: { let __guard = self.tok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rhs: self.rhs.clone() }
    }
}


impl Default for AssignStmt {
    fn default() -> Self {
        Self { lhs: Arc::new(Mutex::new(None)), tok_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), rhs: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", format_slice_wrapped_stringer(&self.lhs), (*self.tok_pos.lock().unwrap().as_ref().unwrap()), (*self.tok.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.rhs))
    }
}

impl GoJsonDecode for AssignStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct GoStmt {
    pub go: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub call: Arc<Mutex<Option<CallExpr>>>,
}

impl GoStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { go: { let __guard = self.go.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, call: self.call.clone() }
    }
}


impl Default for GoStmt {
    fn default() -> Self {
        Self { go: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), call: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for GoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.go.lock().unwrap().as_ref().unwrap()), { let __guard = self.call.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for GoStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct DeferStmt {
    pub defer: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub call: Arc<Mutex<Option<CallExpr>>>,
}

impl DeferStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { defer: { let __guard = self.defer.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, call: self.call.clone() }
    }
}


impl Default for DeferStmt {
    fn default() -> Self {
        Self { defer: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), call: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for DeferStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.defer.lock().unwrap().as_ref().unwrap()), { let __guard = self.call.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for DeferStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct ReturnStmt {
    pub r#return: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub results: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
}

impl ReturnStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#return: { let __guard = self.r#return.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, results: self.results.clone() }
    }
}


impl Default for ReturnStmt {
    fn default() -> Self {
        Self { r#return: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), results: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.r#return.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.results))
    }
}

impl GoJsonDecode for ReturnStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct BranchStmt {
    pub tok_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub tok: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub label: Arc<Mutex<Option<Ident>>>,
}

impl BranchStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { tok_pos: { let __guard = self.tok_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tok: { let __guard = self.tok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, label: self.label.clone() }
    }
}


impl Default for BranchStmt {
    fn default() -> Self {
        Self { tok_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), label: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for BranchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.tok_pos.lock().unwrap().as_ref().unwrap()), (*self.tok.lock().unwrap().as_ref().unwrap()), { let __guard = self.label.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}
impl GoComparable for BranchStmt {
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

impl GoJsonDecode for BranchStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct BlockStmt {
    pub lbrace: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>>>>>,
    pub rbrace: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl BlockStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { lbrace: { let __guard = self.lbrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: self.list.clone(), rbrace: { let __guard = self.rbrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for BlockStmt {
    fn default() -> Self {
        Self { lbrace: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), list: Arc::new(Mutex::new(None)), rbrace: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lbrace.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.list), (*self.rbrace.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for BlockStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct IfStmt {
    pub r#if: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub init: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub cond: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
    pub r#else: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
}

impl IfStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#if: { let __guard = self.r#if.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: self.init.clone(), cond: self.cond.clone(), body: self.body.clone(), r#else: self.r#else.clone() }
    }
}


impl Default for IfStmt {
    fn default() -> Self {
        Self { r#if: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), init: Arc::new(Mutex::new(None)), cond: Arc::new(Mutex::new(None)), body: Arc::new(Mutex::new(None)), r#else: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.r#if.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()), (*self.cond.lock().unwrap().as_ref().unwrap()), { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.r#else.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IfStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct CaseClause {
    pub case: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
    pub colon: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub body: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>>>>>,
}

impl CaseClause {
    pub fn __go_value_clone(&self) -> Self {
        Self { case: { let __guard = self.case.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: self.list.clone(), colon: { let __guard = self.colon.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, body: self.body.clone() }
    }
}


impl Default for CaseClause {
    fn default() -> Self {
        Self { case: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), list: Arc::new(Mutex::new(None)), colon: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for CaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.case.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.list), (*self.colon.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.body))
    }
}

impl GoJsonDecode for CaseClause {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct SwitchStmt {
    pub switch: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub init: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub tag: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl SwitchStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { switch: { let __guard = self.switch.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: self.init.clone(), tag: self.tag.clone(), body: self.body.clone() }
    }
}


impl Default for SwitchStmt {
    fn default() -> Self {
        Self { switch: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), init: Arc::new(Mutex::new(None)), tag: Arc::new(Mutex::new(None)), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for SwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.switch.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()), (*self.tag.lock().unwrap().as_ref().unwrap()), { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for SwitchStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct TypeSwitchStmt {
    pub switch: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub init: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub assign: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl TypeSwitchStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { switch: { let __guard = self.switch.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: self.init.clone(), assign: self.assign.clone(), body: self.body.clone() }
    }
}


impl Default for TypeSwitchStmt {
    fn default() -> Self {
        Self { switch: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), init: Arc::new(Mutex::new(None)), assign: Arc::new(Mutex::new(None)), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for TypeSwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.switch.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()), (*self.assign.lock().unwrap().as_ref().unwrap()), { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for TypeSwitchStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct CommClause {
    pub case: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub comm: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub colon: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub body: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>>>>>,
}

impl CommClause {
    pub fn __go_value_clone(&self) -> Self {
        Self { case: { let __guard = self.case.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, comm: self.comm.clone(), colon: { let __guard = self.colon.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, body: self.body.clone() }
    }
}


impl Default for CommClause {
    fn default() -> Self {
        Self { case: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), comm: Arc::new(Mutex::new(None)), colon: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for CommClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.case.lock().unwrap().as_ref().unwrap()), (*self.comm.lock().unwrap().as_ref().unwrap()), (*self.colon.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.body))
    }
}

impl GoJsonDecode for CommClause {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct SelectStmt {
    pub select: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl SelectStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { select: { let __guard = self.select.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, body: self.body.clone() }
    }
}


impl Default for SelectStmt {
    fn default() -> Self {
        Self { select: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for SelectStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.select.lock().unwrap().as_ref().unwrap()), { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for SelectStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct ForStmt {
    pub r#for: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub init: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub cond: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub post: Arc<Mutex<Option<Box<dyn Stmt + Send + Sync>>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl ForStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#for: { let __guard = self.r#for.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: self.init.clone(), cond: self.cond.clone(), post: self.post.clone(), body: self.body.clone() }
    }
}


impl Default for ForStmt {
    fn default() -> Self {
        Self { r#for: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), init: Arc::new(Mutex::new(None)), cond: Arc::new(Mutex::new(None)), post: Arc::new(Mutex::new(None)), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for ForStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.r#for.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()), (*self.cond.lock().unwrap().as_ref().unwrap()), (*self.post.lock().unwrap().as_ref().unwrap()), { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for ForStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A statement is represented by a tree consisting of one
/// or more of the following concrete statement nodes.
#[derive(Clone)]
pub struct RangeStmt {
    pub r#for: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub key: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub value: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub tok_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub tok: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub range: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl RangeStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#for: { let __guard = self.r#for.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, key: self.key.clone(), value: self.value.clone(), tok_pos: { let __guard = self.tok_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tok: { let __guard = self.tok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, range: { let __guard = self.range.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: self.x.clone(), body: self.body.clone() }
    }
}


impl Default for RangeStmt {
    fn default() -> Self {
        Self { r#for: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), key: Arc::new(Mutex::new(None)), value: Arc::new(Mutex::new(None)), tok_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), range: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), x: Arc::new(Mutex::new(None)), body: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for RangeStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.r#for.lock().unwrap().as_ref().unwrap()), (*self.key.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()), (*self.tok_pos.lock().unwrap().as_ref().unwrap()), (*self.tok.lock().unwrap().as_ref().unwrap()), (*self.range.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()), { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for RangeStmt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Spec node represents a single (non-parenthesized) import,
/// constant, type, or variable declaration.
pub trait Spec: Node + std::fmt::Display + Any {
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync>;
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool;
    fn spec_node(&self);
}

impl Clone for Box<dyn Spec + Send + Sync> {
    fn clone(&self) -> Self {
        Spec::__go_clone_box_spec(self.as_ref())
    }
}

impl GoValueClone for Box<dyn Spec + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        Spec::__go_clone_box_spec(self.as_ref())
    }
}

impl Node for Box<dyn Spec + Send + Sync> {
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        (**self).__go_eq_node(other)
    }
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).end()
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
}

/// A Spec node represents a single (non-parenthesized) import,
/// constant, type, or variable declaration.
#[derive(Clone)]
pub struct ImportSpec {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub name: Arc<Mutex<Option<Ident>>>,
    pub path: Arc<Mutex<Option<BasicLit>>>,
    pub comment: Arc<Mutex<Option<CommentGroup>>>,
    pub end_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl ImportSpec {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), name: self.name.clone(), path: self.path.clone(), comment: self.comment.clone(), end_pos: { let __guard = self.end_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ImportSpec {
    fn default() -> Self {
        Self { doc: Arc::new(Mutex::new(None)), name: Arc::new(Mutex::new(None)), path: Arc::new(Mutex::new(None)), comment: Arc::new(Mutex::new(None)), end_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for ImportSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.path.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.comment.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.end_pos.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ImportSpec {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Spec node represents a single (non-parenthesized) import,
/// constant, type, or variable declaration.
#[derive(Clone, Default)]
pub struct ValueSpec {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Ident>>>>>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub values: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>>>>>,
    pub comment: Arc<Mutex<Option<CommentGroup>>>,
}

impl ValueSpec {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), names: self.names.clone(), r#type: self.r#type.clone(), values: self.values.clone(), comment: self.comment.clone() }
    }
}

impl std::fmt::Display for ValueSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice_wrapped(&self.names), (*self.r#type.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.values), { let __guard = self.comment.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for ValueSpec {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Spec node represents a single (non-parenthesized) import,
/// constant, type, or variable declaration.
#[derive(Clone)]
pub struct TypeSpec {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub name: Arc<Mutex<Option<Ident>>>,
    pub type_params: Arc<Mutex<Option<FieldList>>>,
    pub assign: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>,
    pub comment: Arc<Mutex<Option<CommentGroup>>>,
}

impl TypeSpec {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), name: self.name.clone(), type_params: self.type_params.clone(), assign: { let __guard = self.assign.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: self.r#type.clone(), comment: self.comment.clone() }
    }
}


impl Default for TypeSpec {
    fn default() -> Self {
        Self { doc: Arc::new(Mutex::new(None)), name: Arc::new(Mutex::new(None)), type_params: Arc::new(Mutex::new(None)), assign: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), r#type: Arc::new(Mutex::new(None)), comment: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.type_params.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.assign.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), { let __guard = self.comment.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for TypeSpec {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A declaration is represented by one of the following declaration nodes.
#[derive(Clone)]
pub struct BadDecl {
    pub from: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub to: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl BadDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { from: { let __guard = self.from.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, to: { let __guard = self.to.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for BadDecl {
    fn default() -> Self {
        Self { from: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), to: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for BadDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.from.lock().unwrap().as_ref().unwrap()), (*self.to.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for BadDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A declaration is represented by one of the following declaration nodes.
#[derive(Clone)]
pub struct GenDecl {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub tok_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub tok: Arc<Mutex<Option<go_token::r#mod::Token>>>,
    pub lparen: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub specs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Spec + Send + Sync>>>>>>>>,
    pub rparen: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl GenDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), tok_pos: { let __guard = self.tok_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tok: { let __guard = self.tok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lparen: { let __guard = self.lparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, specs: self.specs.clone(), rparen: { let __guard = self.rparen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for GenDecl {
    fn default() -> Self {
        Self { doc: Arc::new(Mutex::new(None)), tok_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0))))))), lparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), specs: Arc::new(Mutex::new(None)), rparen: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for GenDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.tok_pos.lock().unwrap().as_ref().unwrap()), (*self.tok.lock().unwrap().as_ref().unwrap()), (*self.lparen.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.specs), (*self.rparen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for GenDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A declaration is represented by one of the following declaration nodes.
#[derive(Clone, Default)]
pub struct FuncDecl {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub recv: Arc<Mutex<Option<FieldList>>>,
    pub name: Arc<Mutex<Option<Ident>>>,
    pub r#type: Arc<Mutex<Option<FuncType>>>,
    pub body: Arc<Mutex<Option<BlockStmt>>>,
}

impl FuncDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), recv: self.recv.clone(), name: self.name.clone(), r#type: self.r#type.clone(), body: self.body.clone() }
    }
}

impl std::fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.recv.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.r#type.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.body.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for FuncDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A File node represents a Go source file.
///
/// The Comments list contains all comments in the source file in order of
/// appearance, including the comments that are pointed to from other nodes
/// via Doc and Comment fields.
///
/// For correct printing of source code containing comments (using packages
/// go/format and go/printer), special care must be taken to update comments
/// when a File's syntax tree is modified: For printing, comments are interspersed
/// between tokens based on their position. If syntax tree nodes are
/// removed or moved, relevant comments in their vicinity must also be removed
/// (from the [File.Comments] list) or moved accordingly (by updating their
/// positions). A [CommentMap] may be used to facilitate some of these operations.
///
/// Whether and how a comment is associated with a node depends on the
/// interpretation of the syntax tree by the manipulating program: except for Doc
/// and [Comment] comments directly associated with nodes, the remaining comments
/// are "free-floating" (see also issues [#18593], [#20744]).
///
/// [#18593]: https://go.dev/issue/18593
/// [#20744]: https://go.dev/issue/20744
#[derive(Clone)]
pub struct File {
    pub doc: Arc<Mutex<Option<CommentGroup>>>,
    pub package: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub name: Arc<Mutex<Option<Ident>>>,
    pub decls: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Decl + Send + Sync>>>>>>>>,
    pub file_start: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub file_end: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub scope: Arc<Mutex<Option<Scope>>>,
    pub imports: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ImportSpec>>>>>>>,
    pub unresolved: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Ident>>>>>>>,
    pub comments: Arc<Mutex<Option<Vec<Arc<Mutex<Option<CommentGroup>>>>>>>,
    pub go_version: Arc<Mutex<Option<String>>>,
}

impl File {
    pub fn __go_value_clone(&self) -> Self {
        Self { doc: self.doc.clone(), package: { let __guard = self.package.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: self.name.clone(), decls: self.decls.clone(), file_start: { let __guard = self.file_start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, file_end: { let __guard = self.file_end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scope: self.scope.clone(), imports: self.imports.clone(), unresolved: self.unresolved.clone(), comments: self.comments.clone(), go_version: { let __guard = self.go_version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for File {
    fn default() -> Self {
        Self { doc: Arc::new(Mutex::new(None)), package: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), name: Arc::new(Mutex::new(None)), decls: Arc::new(Mutex::new(None)), file_start: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), file_end: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), scope: Arc::new(Mutex::new(None)), imports: Arc::new(Mutex::new(None)), unresolved: Arc::new(Mutex::new(None)), comments: Arc::new(Mutex::new(None)), go_version: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.package.lock().unwrap().as_ref().unwrap()), { let __guard = self.name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice_wrapped_stringer(&self.decls), (*self.file_start.lock().unwrap().as_ref().unwrap()), (*self.file_end.lock().unwrap().as_ref().unwrap()), { let __guard = self.scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice_wrapped(&self.imports), format_slice_wrapped(&self.unresolved), format_slice_wrapped(&self.comments), (*self.go_version.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for File {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("GoVersion") {
            out.go_version = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A Package node represents a set of source files
/// collectively building a Go package.
///
/// Deprecated: use the type checker [go/types] instead; see [Object].
#[derive(Clone)]
pub struct Package {
    pub name: Arc<Mutex<Option<String>>>,
    pub scope: Arc<Mutex<Option<Scope>>>,
    pub imports: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Object>>>>>>>,
    pub files: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<File>>>>>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scope: self.scope.clone(), imports: self.imports.clone(), files: self.files.clone() }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), scope: Arc::new(Mutex::new(None)), imports: Arc::new(Mutex::new(None)), files: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), { let __guard = self.scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_map(&self.imports), format_map(&self.files))
    }
}

impl GoJsonDecode for Package {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl Comment {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.slash.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = ((*Arc::new(Mutex::new(Some((*(*self.slash.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*self.text.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32)))))))
    }
}

impl Node for Comment {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Comment::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Comment::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Comment>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CommentPtr(pub Arc<Mutex<Option<Comment>>>);

impl std::fmt::Display for CommentPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for CommentPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Comment::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Comment::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommentPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl CommentGroup {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        { let __recv = { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        { let __recv = { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result }
    }

    /// Text returns the text of the comment.
    /// Comment markers (//, /*, and */), the first space of a line comment, and
    /// leading and trailing empty lines are removed.
    /// Comment directives like "//line" and "//go:noinline" are also removed.
    /// Multiple empty lines are reduced to one, and trailing space on lines is trimmed.
    /// Unless the result is empty, it is newline-terminated.
    pub fn text(&self) -> Arc<Mutex<Option<String>>> {
        if false {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut comments = Arc::new(Mutex::new(Some(vec!["".to_string(); (({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
        { let __range_holder = self.list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, c) in __range_values.iter().enumerate() {
        (*comments.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).text.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone();
    } }
        let mut lines = Arc::new(Mutex::new(Some(Vec::<String>::with_capacity((10) as usize))));
        { let __range_holder = comments.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut c in __range_values.iter().cloned() {
                // Remove comment markers.
                // The parser has given us exactly the comment text.
        '__go_switch_1: loop {
        { let _switch_val = { let __s = &(c); __s.as_bytes()[(1) as usize] };
    if _switch_val == (('/' as i32) as u8) {
                        //-style comment (no newline at the end)
            { let new_val = { let __s = &(c); let __low = (2) as usize; __s[__low..].to_string() }; c = new_val; };
            if { let __tmp_x = (c.len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // empty line
        break '__go_switch_1
    }
                        // empty line
            if { let __tmp_x = { let __s = &(c); __s.as_bytes()[(0) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } {
                // strip first space - required for Example tests
        { let new_val = { let __s = &(c); let __low = (1) as usize; __s[__low..].to_string() }; c = new_val; };
        break '__go_switch_1
    }
                        // strip first space - required for Example tests
            if is_directive(Arc::new(Mutex::new(Some(c.clone())))) {
                // Ignore //go:noinline, //line, and so on.
        continue
    }
        } else if _switch_val == (('*' as i32) as u8) {
                        /*-style comment */
            { let new_val = { let __s = &(c); let __low = (2) as usize; let __high = ({ let __tmp_x = (c.len() as i32); let __tmp_y = 2; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }; c = new_val; };
        }
    };
        break;
    }
                //-style comment (no newline at the end)
                // empty line
                // strip first space - required for Example tests
                // Ignore //go:noinline, //line, and so on.
                /*-style comment */
                // Split on newlines.
        let mut cl = strings::split(Arc::new(Mutex::new(Some(c.clone()))), Arc::new(Mutex::new(Some("\n".to_string()))));
                // Walk lines, stripping trailing white space and adding to list.
        { let __range_holder = cl.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for l in __range_values.iter() {
        { let new_val = { let __append_target = lines.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*strip_trailing_whitespace(Arc::new(Mutex::new(Some((*l).clone())))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; lines = new_val; };
    } }
    } }
                // Remove comment markers.
                // The parser has given us exactly the comment text.
                //-style comment (no newline at the end)
                // empty line
                // strip first space - required for Example tests
                // Ignore //go:noinline, //line, and so on.
                /*-style comment */
                // Split on newlines.
                // Walk lines, stripping trailing white space and adding to list.
                // Remove leading blank lines; convert runs of
                // interior blank lines to a single blank line.
        let mut n = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for line in __range_values.iter() {
        if { let __tmp_x = (*line).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        (*lines.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = line.clone();
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); lines = new_val; };
                // Add final "" entry to get trailing newline from Join.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = lines.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push("".to_string()); __append_target.clone() }; lines = new_val; };
    }
        return strings::join(lines.clone(), Arc::new(Mutex::new(Some("\n".to_string()))));
    }
}

impl Node for CommentGroup {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CommentGroup::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CommentGroup::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommentGroup>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CommentGroupPtr(pub Arc<Mutex<Option<CommentGroup>>>);

impl std::fmt::Display for CommentGroupPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for CommentGroupPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CommentGroup::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CommentGroup::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommentGroupPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Field {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __tmp_x = (({ let __len_target = { let __field = self.names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return { let __recv = { let __seq = { let __seq_holder = self.names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result };
    }
        if { let __iface_handle = { let __field = self.r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return (*self.r#type.lock().unwrap().as_ref().unwrap()).pos();
    }
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __nil_target = self.tag.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return (*self.tag.lock().unwrap().as_ref().unwrap()).end();
    }
        if { let __iface_handle = { let __field = self.r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return (*self.r#type.lock().unwrap().as_ref().unwrap()).end();
    }
        if { let __tmp_x = (({ let __len_target = { let __field = self.names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return { let __recv = { let __seq = { let __seq_holder = self.names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }
}

impl Node for Field {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Field::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Field::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Field>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FieldPtr(pub Arc<Mutex<Option<Field>>>);

impl std::fmt::Display for FieldPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for FieldPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Field::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Field::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FieldPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl FieldList {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if go_token::position::Pos::is_valid(&(*self.opening.lock().unwrap().as_ref().unwrap())) {
        return self.opening.clone();
    }
                // the list should not be empty in this case;
                // be conservative and guard against bad ASTs
        if { let __tmp_x = (({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return { let __recv = { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result };
    }
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if go_token::position::Pos::is_valid(&(*self.closing.lock().unwrap().as_ref().unwrap())) {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.closing.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }
                // the list should not be empty in this case;
                // be conservative and guard against bad ASTs
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }

    /// NumFields returns the number of parameters or struct fields represented by a [FieldList].
    pub fn num_fields(&self) -> i32 {
        let mut n = Arc::new(Mutex::new(Some(0)));
        if true {
        { let __range_holder = self.list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for g in __range_values.iter() {
        let mut m = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*g.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 1; *m.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = (*m.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    }
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

impl Node for FieldList {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FieldList::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FieldList::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FieldList>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FieldListPtr(pub Arc<Mutex<Option<FieldList>>>);

impl std::fmt::Display for FieldListPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for FieldListPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FieldList::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FieldList::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FieldListPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BadExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.from.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.to.clone();
    }

    /// exprNode() ensures that only expression/type nodes can be
    /// assigned to an Expr.
    pub fn expr_node(&self) {
    }
}

impl Expr for BadExpr {
    fn expr_node(&self) {
        BadExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BadExprPtr(pub Arc<Mutex<Option<BadExpr>>>);

impl std::fmt::Display for BadExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for BadExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for BadExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BadExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BadExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for BadExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Ident {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.name_pos.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = ((*Arc::new(Mutex::new(Some((*(*self.name_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*self.name.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32)))))))
    }

    pub fn expr_node(&self) {
    }

    /// IsExported reports whether id starts with an upper-case letter.
    pub fn is_exported(&self) -> bool {
        go_token::is_exported(Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if true {
        return self.name.clone();
    }
        Arc::new(Mutex::new(Some("<nil>".to_string())))
    }
}

impl Expr for Ident {
    fn expr_node(&self) {
        Ident::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ident>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IdentPtr(pub Arc<Mutex<Option<Ident>>>);

impl std::fmt::Display for IdentPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for IdentPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IdentPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for Ident {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Ident::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Ident::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ident>() {
            false
        } else {
            false
        }
    }
}

impl Node for IdentPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IdentPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Ellipsis {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.ellipsis.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __iface_handle = { let __field = self.elt.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return (*self.elt.lock().unwrap().as_ref().unwrap()).end();
    }
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.ellipsis.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 3))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for Ellipsis {
    fn expr_node(&self) {
        Ellipsis::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ellipsis>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct EllipsisPtr(pub Arc<Mutex<Option<Ellipsis>>>);

impl std::fmt::Display for EllipsisPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for EllipsisPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Ellipsis::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<EllipsisPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for Ellipsis {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Ellipsis::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Ellipsis::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ellipsis>() {
            false
        } else {
            false
        }
    }
}

impl Node for EllipsisPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Ellipsis::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Ellipsis::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<EllipsisPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BasicLit {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.value_pos.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = ((*Arc::new(Mutex::new(Some((*(*self.value_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*self.value.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32)))))))
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for BasicLit {
    fn expr_node(&self) {
        BasicLit::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicLit>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BasicLitPtr(pub Arc<Mutex<Option<BasicLit>>>);

impl std::fmt::Display for BasicLitPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for BasicLitPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BasicLit::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicLitPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for BasicLit {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BasicLit::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BasicLit::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicLit>() {
            false
        } else {
            false
        }
    }
}

impl Node for BasicLitPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BasicLit::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BasicLit::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicLitPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl FuncLit {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.r#type.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for FuncLit {
    fn expr_node(&self) {
        FuncLit::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncLit>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FuncLitPtr(pub Arc<Mutex<Option<FuncLit>>>);

impl std::fmt::Display for FuncLitPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for FuncLitPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncLit::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncLitPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for FuncLit {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FuncLit::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FuncLit::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncLit>() {
            false
        } else {
            false
        }
    }
}

impl Node for FuncLitPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncLit::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncLit::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncLitPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl CompositeLit {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __iface_handle = { let __field = self.r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return (*self.r#type.lock().unwrap().as_ref().unwrap()).pos();
    }
        return self.lbrace.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rbrace.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for CompositeLit {
    fn expr_node(&self) {
        CompositeLit::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CompositeLit>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CompositeLitPtr(pub Arc<Mutex<Option<CompositeLit>>>);

impl std::fmt::Display for CompositeLitPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for CompositeLitPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CompositeLit::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CompositeLitPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for CompositeLit {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CompositeLit::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CompositeLit::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CompositeLit>() {
            false
        } else {
            false
        }
    }
}

impl Node for CompositeLitPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CompositeLit::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CompositeLit::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CompositeLitPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ParenExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.lparen.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rparen.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for ParenExpr {
    fn expr_node(&self) {
        ParenExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ParenExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ParenExprPtr(pub Arc<Mutex<Option<ParenExpr>>>);

impl std::fmt::Display for ParenExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for ParenExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ParenExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ParenExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for ParenExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ParenExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ParenExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ParenExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for ParenExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ParenExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ParenExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ParenExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SelectorExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.sel.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for SelectorExpr {
    fn expr_node(&self) {
        SelectorExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectorExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SelectorExprPtr(pub Arc<Mutex<Option<SelectorExpr>>>);

impl std::fmt::Display for SelectorExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for SelectorExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SelectorExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectorExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for SelectorExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SelectorExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SelectorExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectorExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for SelectorExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SelectorExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SelectorExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectorExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl IndexExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rbrack.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for IndexExpr {
    fn expr_node(&self) {
        IndexExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IndexExprPtr(pub Arc<Mutex<Option<IndexExpr>>>);

impl std::fmt::Display for IndexExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for IndexExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IndexExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for IndexExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IndexExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IndexExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for IndexExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IndexExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IndexExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl IndexListExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rbrack.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for IndexListExpr {
    fn expr_node(&self) {
        IndexListExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexListExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IndexListExprPtr(pub Arc<Mutex<Option<IndexListExpr>>>);

impl std::fmt::Display for IndexListExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for IndexListExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IndexListExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexListExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for IndexListExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IndexListExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IndexListExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexListExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for IndexListExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IndexListExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IndexListExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IndexListExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SliceExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rbrack.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for SliceExpr {
    fn expr_node(&self) {
        SliceExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SliceExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SliceExprPtr(pub Arc<Mutex<Option<SliceExpr>>>);

impl std::fmt::Display for SliceExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for SliceExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SliceExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SliceExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for SliceExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SliceExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SliceExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SliceExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for SliceExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SliceExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SliceExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SliceExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl TypeAssertExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rparen.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for TypeAssertExpr {
    fn expr_node(&self) {
        TypeAssertExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeAssertExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TypeAssertExprPtr(pub Arc<Mutex<Option<TypeAssertExpr>>>);

impl std::fmt::Display for TypeAssertExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for TypeAssertExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeAssertExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeAssertExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for TypeAssertExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeAssertExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeAssertExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeAssertExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for TypeAssertExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeAssertExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeAssertExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeAssertExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl CallExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.fun.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rparen.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for CallExpr {
    fn expr_node(&self) {
        CallExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CallExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CallExprPtr(pub Arc<Mutex<Option<CallExpr>>>);

impl std::fmt::Display for CallExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for CallExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CallExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CallExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for CallExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CallExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CallExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CallExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for CallExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CallExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CallExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CallExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl StarExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.star.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for StarExpr {
    fn expr_node(&self) {
        StarExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StarExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct StarExprPtr(pub Arc<Mutex<Option<StarExpr>>>);

impl std::fmt::Display for StarExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for StarExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        StarExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StarExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for StarExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        StarExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        StarExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StarExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for StarExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        StarExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        StarExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StarExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl UnaryExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.op_pos.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for UnaryExpr {
    fn expr_node(&self) {
        UnaryExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<UnaryExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct UnaryExprPtr(pub Arc<Mutex<Option<UnaryExpr>>>);

impl std::fmt::Display for UnaryExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for UnaryExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        UnaryExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<UnaryExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for UnaryExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        UnaryExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        UnaryExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<UnaryExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for UnaryExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        UnaryExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        UnaryExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<UnaryExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BinaryExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.y.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for BinaryExpr {
    fn expr_node(&self) {
        BinaryExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BinaryExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BinaryExprPtr(pub Arc<Mutex<Option<BinaryExpr>>>);

impl std::fmt::Display for BinaryExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for BinaryExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BinaryExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BinaryExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for BinaryExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BinaryExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BinaryExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BinaryExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for BinaryExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BinaryExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BinaryExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BinaryExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl KeyValueExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.key.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.value.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for KeyValueExpr {
    fn expr_node(&self) {
        KeyValueExpr::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<KeyValueExpr>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct KeyValueExprPtr(pub Arc<Mutex<Option<KeyValueExpr>>>);

impl std::fmt::Display for KeyValueExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for KeyValueExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        KeyValueExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<KeyValueExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for KeyValueExpr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        KeyValueExpr::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        KeyValueExpr::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<KeyValueExpr>() {
            false
        } else {
            false
        }
    }
}

impl Node for KeyValueExprPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        KeyValueExpr::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        KeyValueExpr::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<KeyValueExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ArrayType {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.lbrack.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.elt.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for ArrayType {
    fn expr_node(&self) {
        ArrayType::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ArrayType>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ArrayTypePtr(pub Arc<Mutex<Option<ArrayType>>>);

impl std::fmt::Display for ArrayTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for ArrayTypePtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ArrayType::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ArrayTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for ArrayType {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ArrayType::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ArrayType::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ArrayType>() {
            false
        } else {
            false
        }
    }
}

impl Node for ArrayTypePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ArrayType::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ArrayType::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ArrayTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl StructType {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.r#struct.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.fields.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for StructType {
    fn expr_node(&self) {
        StructType::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StructType>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct StructTypePtr(pub Arc<Mutex<Option<StructType>>>);

impl std::fmt::Display for StructTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for StructTypePtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        StructType::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StructTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for StructType {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        StructType::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        StructType::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StructType>() {
            false
        } else {
            false
        }
    }
}

impl Node for StructTypePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        StructType::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        StructType::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StructTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl FuncType {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if go_token::position::Pos::is_valid(&(*self.func.lock().unwrap().as_ref().unwrap())) || { let __nil_target = self.params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return self.func.clone();
    }
        (*self.params.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __nil_target = self.results.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return (*self.results.lock().unwrap().as_ref().unwrap()).end();
    }
        (*self.params.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for FuncType {
    fn expr_node(&self) {
        FuncType::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncType>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FuncTypePtr(pub Arc<Mutex<Option<FuncType>>>);

impl std::fmt::Display for FuncTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for FuncTypePtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncType::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for FuncType {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FuncType::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FuncType::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncType>() {
            false
        } else {
            false
        }
    }
}

impl Node for FuncTypePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncType::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncType::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl InterfaceType {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.interface.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.methods.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for InterfaceType {
    fn expr_node(&self) {
        InterfaceType::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceType>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct InterfaceTypePtr(pub Arc<Mutex<Option<InterfaceType>>>);

impl std::fmt::Display for InterfaceTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for InterfaceTypePtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        InterfaceType::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for InterfaceType {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        InterfaceType::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        InterfaceType::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceType>() {
            false
        } else {
            false
        }
    }
}

impl Node for InterfaceTypePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        InterfaceType::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        InterfaceType::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl MapType {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.map.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.value.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for MapType {
    fn expr_node(&self) {
        MapType::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<MapType>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct MapTypePtr(pub Arc<Mutex<Option<MapType>>>);

impl std::fmt::Display for MapTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for MapTypePtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        MapType::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<MapTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for MapType {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        MapType::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        MapType::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<MapType>() {
            false
        } else {
            false
        }
    }
}

impl Node for MapTypePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        MapType::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        MapType::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<MapTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ChanType {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.begin.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.value.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for ChanType {
    fn expr_node(&self) {
        ChanType::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanType>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ChanTypePtr(pub Arc<Mutex<Option<ChanType>>>);

impl std::fmt::Display for ChanTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for ChanTypePtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ChanType::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Expr + Send + Sync>
    }
    fn __go_eq_expr(&self, other: &(dyn Expr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for ChanType {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ChanType::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ChanType::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanType>() {
            false
        } else {
            false
        }
    }
}

impl Node for ChanTypePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ChanType::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ChanType::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BadStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.from.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.to.clone();
    }

    /// stmtNode() ensures that only statement nodes can be
    /// assigned to a Stmt.
    pub fn stmt_node(&self) {
    }
}

impl Node for BadStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BadStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BadStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BadStmtPtr(pub Arc<Mutex<Option<BadStmt>>>);

impl std::fmt::Display for BadStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for BadStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for BadStmt {
    fn stmt_node(&self) {
        BadStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for BadStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl DeclStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.decl.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.decl.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for DeclStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        DeclStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        DeclStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeclStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct DeclStmtPtr(pub Arc<Mutex<Option<DeclStmt>>>);

impl std::fmt::Display for DeclStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for DeclStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        DeclStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        DeclStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeclStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for DeclStmt {
    fn stmt_node(&self) {
        DeclStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeclStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for DeclStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        DeclStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeclStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl EmptyStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.semicolon.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if (*self.implicit.clone().lock().unwrap().as_ref().unwrap()) {
        return self.semicolon.clone();
    }
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.semicolon.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for EmptyStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        EmptyStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        EmptyStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<EmptyStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct EmptyStmtPtr(pub Arc<Mutex<Option<EmptyStmt>>>);

impl std::fmt::Display for EmptyStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for EmptyStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        EmptyStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        EmptyStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<EmptyStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for EmptyStmt {
    fn stmt_node(&self) {
        EmptyStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<EmptyStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for EmptyStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        EmptyStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<EmptyStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl LabeledStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.label.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.stmt.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for LabeledStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        LabeledStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        LabeledStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LabeledStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct LabeledStmtPtr(pub Arc<Mutex<Option<LabeledStmt>>>);

impl std::fmt::Display for LabeledStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for LabeledStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        LabeledStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        LabeledStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LabeledStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for LabeledStmt {
    fn stmt_node(&self) {
        LabeledStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LabeledStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for LabeledStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        LabeledStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LabeledStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ExprStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for ExprStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ExprStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ExprStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ExprStmtPtr(pub Arc<Mutex<Option<ExprStmt>>>);

impl std::fmt::Display for ExprStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for ExprStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ExprStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ExprStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for ExprStmt {
    fn stmt_node(&self) {
        ExprStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for ExprStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ExprStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SendStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.chan.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.value.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for SendStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SendStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SendStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SendStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SendStmtPtr(pub Arc<Mutex<Option<SendStmt>>>);

impl std::fmt::Display for SendStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for SendStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SendStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SendStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SendStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for SendStmt {
    fn stmt_node(&self) {
        SendStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SendStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for SendStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SendStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SendStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl IncDecStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.x.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.tok_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 2))))))));
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for IncDecStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IncDecStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IncDecStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IncDecStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IncDecStmtPtr(pub Arc<Mutex<Option<IncDecStmt>>>);

impl std::fmt::Display for IncDecStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for IncDecStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IncDecStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IncDecStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IncDecStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for IncDecStmt {
    fn stmt_node(&self) {
        IncDecStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IncDecStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for IncDecStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IncDecStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IncDecStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl AssignStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        { let __recv = { let __seq = { let __seq_holder = self.lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        { let __recv = { let __seq = { let __seq_holder = self.rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.rhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result }
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for AssignStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        AssignStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        AssignStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AssignStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct AssignStmtPtr(pub Arc<Mutex<Option<AssignStmt>>>);

impl std::fmt::Display for AssignStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for AssignStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        AssignStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        AssignStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AssignStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for AssignStmt {
    fn stmt_node(&self) {
        AssignStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AssignStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for AssignStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        AssignStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AssignStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl GoStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.go.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.call.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for GoStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        GoStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        GoStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GoStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct GoStmtPtr(pub Arc<Mutex<Option<GoStmt>>>);

impl std::fmt::Display for GoStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for GoStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        GoStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        GoStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GoStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for GoStmt {
    fn stmt_node(&self) {
        GoStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GoStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for GoStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        GoStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GoStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl DeferStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.defer.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.call.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for DeferStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        DeferStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        DeferStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeferStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct DeferStmtPtr(pub Arc<Mutex<Option<DeferStmt>>>);

impl std::fmt::Display for DeferStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for DeferStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        DeferStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        DeferStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeferStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for DeferStmt {
    fn stmt_node(&self) {
        DeferStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeferStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for DeferStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        DeferStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeferStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ReturnStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.r#return.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.results.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.results.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.r#return.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 6))))))));
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for ReturnStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ReturnStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ReturnStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ReturnStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ReturnStmtPtr(pub Arc<Mutex<Option<ReturnStmt>>>);

impl std::fmt::Display for ReturnStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for ReturnStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ReturnStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ReturnStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ReturnStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for ReturnStmt {
    fn stmt_node(&self) {
        ReturnStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ReturnStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for ReturnStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ReturnStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ReturnStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BranchStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.tok_pos.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __nil_target = self.label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return (*self.label.lock().unwrap().as_ref().unwrap()).end();
    }
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = ((*Arc::new(Mutex::new(Some((*(*self.tok_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*go_token::r#mod::Token::string(&(*self.tok.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32)))))))
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for BranchStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BranchStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BranchStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BranchStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BranchStmtPtr(pub Arc<Mutex<Option<BranchStmt>>>);

impl std::fmt::Display for BranchStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for BranchStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BranchStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BranchStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BranchStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for BranchStmt {
    fn stmt_node(&self) {
        BranchStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BranchStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for BranchStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BranchStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BranchStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BlockStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.lbrace.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if go_token::position::Pos::is_valid(&(*self.rbrace.lock().unwrap().as_ref().unwrap())) {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rbrace.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.lbrace.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for BlockStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BlockStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BlockStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BlockStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BlockStmtPtr(pub Arc<Mutex<Option<BlockStmt>>>);

impl std::fmt::Display for BlockStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for BlockStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BlockStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BlockStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BlockStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for BlockStmt {
    fn stmt_node(&self) {
        BlockStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BlockStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for BlockStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BlockStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BlockStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl IfStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.r#if.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __iface_handle = { let __field = self.r#else.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return (*self.r#else.lock().unwrap().as_ref().unwrap()).end();
    }
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for IfStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IfStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        IfStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IfStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IfStmtPtr(pub Arc<Mutex<Option<IfStmt>>>);

impl std::fmt::Display for IfStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for IfStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IfStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IfStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IfStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for IfStmt {
    fn stmt_node(&self) {
        IfStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IfStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for IfStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        IfStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IfStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl CaseClause {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.case.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.body.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.body.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.colon.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for CaseClause {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CaseClause::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CaseClause::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CaseClause>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CaseClausePtr(pub Arc<Mutex<Option<CaseClause>>>);

impl std::fmt::Display for CaseClausePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for CaseClausePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CaseClause::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CaseClause::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CaseClausePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for CaseClause {
    fn stmt_node(&self) {
        CaseClause::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CaseClause>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for CaseClausePtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CaseClause::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CaseClausePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SwitchStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.switch.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for SwitchStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SwitchStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SwitchStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SwitchStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SwitchStmtPtr(pub Arc<Mutex<Option<SwitchStmt>>>);

impl std::fmt::Display for SwitchStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for SwitchStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SwitchStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SwitchStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SwitchStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for SwitchStmt {
    fn stmt_node(&self) {
        SwitchStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SwitchStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for SwitchStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SwitchStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SwitchStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl TypeSwitchStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.switch.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for TypeSwitchStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeSwitchStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeSwitchStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSwitchStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TypeSwitchStmtPtr(pub Arc<Mutex<Option<TypeSwitchStmt>>>);

impl std::fmt::Display for TypeSwitchStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for TypeSwitchStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeSwitchStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeSwitchStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSwitchStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for TypeSwitchStmt {
    fn stmt_node(&self) {
        TypeSwitchStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSwitchStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for TypeSwitchStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeSwitchStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSwitchStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl CommClause {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.case.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.body.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.body.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.colon.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for CommClause {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CommClause::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        CommClause::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommClause>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CommClausePtr(pub Arc<Mutex<Option<CommClause>>>);

impl std::fmt::Display for CommClausePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for CommClausePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CommClause::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CommClause::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommClausePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for CommClause {
    fn stmt_node(&self) {
        CommClause::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommClause>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for CommClausePtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        CommClause::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CommClausePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SelectStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.select.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for SelectStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SelectStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        SelectStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SelectStmtPtr(pub Arc<Mutex<Option<SelectStmt>>>);

impl std::fmt::Display for SelectStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for SelectStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SelectStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SelectStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for SelectStmt {
    fn stmt_node(&self) {
        SelectStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for SelectStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SelectStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ForStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.r#for.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for ForStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ForStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ForStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ForStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ForStmtPtr(pub Arc<Mutex<Option<ForStmt>>>);

impl std::fmt::Display for ForStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for ForStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ForStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ForStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ForStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for ForStmt {
    fn stmt_node(&self) {
        ForStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ForStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for ForStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ForStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ForStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl RangeStmt {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.r#for.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.body.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for RangeStmt {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        RangeStmt::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        RangeStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RangeStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RangeStmtPtr(pub Arc<Mutex<Option<RangeStmt>>>);

impl std::fmt::Display for RangeStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for RangeStmtPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        RangeStmt::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        RangeStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RangeStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for RangeStmt {
    fn stmt_node(&self) {
        RangeStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RangeStmt>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl Stmt for RangeStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        RangeStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stmt + Send + Sync>
    }
    fn __go_eq_stmt(&self, other: &(dyn Stmt + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RangeStmtPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ImportSpec {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __nil_target = self.name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return (*self.name.lock().unwrap().as_ref().unwrap()).pos();
    }
        (*self.path.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __tmp_x = (*self.end_pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::position::Pos(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        return self.end_pos.clone();
    }
        (*self.path.lock().unwrap().as_ref().unwrap()).end()
    }

    /// specNode() ensures that only spec nodes can be
    /// assigned to a Spec.
    pub fn spec_node(&self) {
    }
}

impl Node for ImportSpec {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ImportSpec::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ImportSpec::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImportSpec>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ImportSpecPtr(pub Arc<Mutex<Option<ImportSpec>>>);

impl std::fmt::Display for ImportSpecPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for ImportSpecPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ImportSpec::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ImportSpec::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImportSpecPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Spec for ImportSpec {
    fn spec_node(&self) {
        ImportSpec::spec_node(self)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Spec + Send + Sync>
    }
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImportSpec>() {
            false
        } else {
            false
        }
    }
}

impl Spec for ImportSpecPtr {
    fn spec_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ImportSpec::spec_node(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Spec + Send + Sync>
    }
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImportSpecPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ValueSpec {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        { let __recv = { let __seq = { let __seq_holder = self.names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        if { let __iface_handle = { let __field = self.r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return (*self.r#type.lock().unwrap().as_ref().unwrap()).end();
    }
        { let __recv = { let __seq = { let __seq_holder = self.names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result }
    }

    pub fn spec_node(&self) {
    }
}

impl Node for ValueSpec {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ValueSpec::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        ValueSpec::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ValueSpec>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ValueSpecPtr(pub Arc<Mutex<Option<ValueSpec>>>);

impl std::fmt::Display for ValueSpecPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for ValueSpecPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ValueSpec::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ValueSpec::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ValueSpecPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Spec for ValueSpec {
    fn spec_node(&self) {
        ValueSpec::spec_node(self)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Spec + Send + Sync>
    }
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ValueSpec>() {
            false
        } else {
            false
        }
    }
}

impl Spec for ValueSpecPtr {
    fn spec_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ValueSpec::spec_node(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Spec + Send + Sync>
    }
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ValueSpecPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl TypeSpec {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.name.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.r#type.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn spec_node(&self) {
    }
}

impl Node for TypeSpec {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeSpec::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeSpec::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSpec>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TypeSpecPtr(pub Arc<Mutex<Option<TypeSpec>>>);

impl std::fmt::Display for TypeSpecPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for TypeSpecPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeSpec::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeSpec::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSpecPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Spec for TypeSpec {
    fn spec_node(&self) {
        TypeSpec::spec_node(self)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Spec + Send + Sync>
    }
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSpec>() {
            false
        } else {
            false
        }
    }
}

impl Spec for TypeSpecPtr {
    fn spec_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeSpec::spec_node(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Spec + Send + Sync>
    }
    fn __go_eq_spec(&self, other: &(dyn Spec + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeSpecPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BadDecl {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.from.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.to.clone();
    }

    /// declNode() ensures that only declaration nodes can be
    /// assigned to a Decl.
    pub fn decl_node(&self) {
    }
}

impl Decl for BadDecl {
    fn decl_node(&self) {
        BadDecl::decl_node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Decl + Send + Sync>
    }
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BadDeclPtr(pub Arc<Mutex<Option<BadDecl>>>);

impl std::fmt::Display for BadDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Decl for BadDeclPtr {
    fn decl_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadDecl::decl_node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Decl + Send + Sync>
    }
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for BadDecl {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BadDecl::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        BadDecl::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadDecl>() {
            false
        } else {
            false
        }
    }
}

impl Node for BadDeclPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadDecl::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        BadDecl::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl GenDecl {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.tok_pos.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if go_token::position::Pos::is_valid(&(*self.rparen.lock().unwrap().as_ref().unwrap())) {
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(((*(*self.rparen.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    }
        { let __recv = { let __seq = { let __seq_holder = self.specs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result }
    }

    pub fn decl_node(&self) {
    }
}

impl Decl for GenDecl {
    fn decl_node(&self) {
        GenDecl::decl_node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Decl + Send + Sync>
    }
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GenDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct GenDeclPtr(pub Arc<Mutex<Option<GenDecl>>>);

impl std::fmt::Display for GenDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Decl for GenDeclPtr {
    fn decl_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        GenDecl::decl_node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Decl + Send + Sync>
    }
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GenDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for GenDecl {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        GenDecl::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        GenDecl::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GenDecl>() {
            false
        } else {
            false
        }
    }
}

impl Node for GenDeclPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        GenDecl::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        GenDecl::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<GenDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl FuncDecl {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.r#type.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        if { let __nil_target = self.body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return (*self.body.lock().unwrap().as_ref().unwrap()).end();
    }
        (*self.r#type.lock().unwrap().as_ref().unwrap()).end()
    }

    pub fn decl_node(&self) {
    }
}

impl Decl for FuncDecl {
    fn decl_node(&self) {
        FuncDecl::decl_node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Decl + Send + Sync>
    }
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FuncDeclPtr(pub Arc<Mutex<Option<FuncDecl>>>);

impl std::fmt::Display for FuncDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Decl for FuncDeclPtr {
    fn decl_node(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncDecl::decl_node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Decl + Send + Sync>
    }
    fn __go_eq_decl(&self, other: &(dyn Decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for FuncDecl {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FuncDecl::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        FuncDecl::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncDecl>() {
            false
        } else {
            false
        }
    }
}

impl Node for FuncDeclPtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncDecl::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncDecl::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl File {
    /// Pos returns the position of the package declaration.
    /// It may be invalid, for example in an empty file.
    ///
    /// (Use FileStart for the start of the entire file. It is always valid.)
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.package.clone();
    }

    /// End returns the end of the last declaration in the file.
    /// It may be invalid, for example in an empty file.
    ///
    /// (Use FileEnd for the end of the entire file. It is always valid.)
    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.decls.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return { let __recv = { let __seq = { let __seq_holder = self.decls.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        (*self.name.lock().unwrap().as_ref().unwrap()).end()
    }
}

impl Node for File {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        File::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        File::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<File>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FilePtr(pub Arc<Mutex<Option<File>>>);

impl std::fmt::Display for FilePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for FilePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        File::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        File::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Package {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }
}

impl Node for Package {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Package::end(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Package::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Package>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct PackagePtr(pub Arc<Mutex<Option<Package>>>);

impl std::fmt::Display for PackagePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for PackagePtr {
    fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Package::end(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Package::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PackagePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ChanDir {
}

impl cmp::r#mod::Ordered for ChanDir {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanDir>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn is_whitespace(ch: Arc<Mutex<Option<u8>>>) -> bool {
    return { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\t' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\r' as i32) as u8; __tmp_x == __tmp_y };
}

pub fn strip_trailing_whitespace(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && is_whitespace(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() })));
}

/// isDirective reports whether c is a comment directive.
/// This code is also in go/printer.
pub fn is_directive(c: Arc<Mutex<Option<String>>>) -> bool {
        // "//line " is a line directive.
        // "//extern " is for gccgo.
        // "//export " is for cgo.
        // (The // has been removed.)
    if strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("line ".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("extern ".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("export ".to_string())))) {
        return true;
    }

        // "//[a-z0-9]+:[a-z0-9]"
        // (The // has been removed.)
    let mut colon = strings::index(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(":".to_string()))));
    if { let __tmp_x = colon; let __tmp_y = 0; __tmp_x <= __tmp_y } || { let __tmp_x = ({ let __tmp_x = colon; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*c.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } {
        return false;
    }
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = colon; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = colon; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let mut b = Arc::new(Mutex::new(Some({ let __s = &((*c.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if !({ let __tmp_x = ('a' as i32) as u8; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('z' as i32) as u8; __tmp_x <= __tmp_y } || { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y }) {
        return false;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    true
}

/// NewIdent creates a new [Ident] without position.
/// Useful for ASTs generated by code other than the Go parser.
pub fn new_ident(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Ident>>> {
    Arc::new(Mutex::new(Some(Ident { name_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), obj: Default::default(), ..Default::default() })))
}

/// Unparen returns the expression with any enclosing parentheses removed.
pub fn unparen(mut e: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>> {
    let mut e: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>> = Arc::new(Mutex::new(e.lock().unwrap().as_ref().map(|__v| Expr::__go_clone_box_expr(__v.as_ref()))));
    loop {
        let (mut paren, mut ok) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<ParenExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<ParenExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<ParenExpr>)), false)
        }
    });
        if !ok {
        return e.clone();
    }
        { let __iface_handle = (*paren.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *e.lock().unwrap() = (*__iface_guard).clone(); };
    }
}

impl GoValueClone for Comment {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for CommentGroup {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Field {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FieldList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BadExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Ident {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Ellipsis {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BasicLit {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FuncLit {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for CompositeLit {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ParenExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SelectorExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IndexExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IndexListExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SliceExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeAssertExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for CallExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for StarExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for UnaryExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BinaryExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for KeyValueExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ArrayType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for StructType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FuncType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for InterfaceType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for MapType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ChanType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BadStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for DeclStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for EmptyStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for LabeledStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ExprStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SendStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IncDecStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for AssignStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for GoStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for DeferStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ReturnStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BranchStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BlockStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IfStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for CaseClause {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SwitchStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeSwitchStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for CommClause {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SelectStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ForStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RangeStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ImportSpec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ValueSpec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeSpec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BadDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for GenDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FuncDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for File {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Package {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
