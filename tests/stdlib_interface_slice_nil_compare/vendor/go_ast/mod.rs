use go2rust_stdlib_stubs::*;

use crate::{format_any, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::commentmap::*;
use crate::filter::*;
use crate::import::*;
use crate::print::*;
use crate::resolve::*;
use crate::scope::*;
use crate::walk::*;

use std::any::Any;
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

#[derive(Clone)]
pub struct GoNodeInterfaceKey(pub Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>);

impl GoNodeInterfaceKey {
    pub fn new(value: Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>) -> Self { GoNodeInterfaceKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
    fn identity(&self) -> (u64, String) {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() {
            None => (0, String::new()),
            Some(__v) => {
                let mut __hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&__v.as_ref().__go_as_any().type_id(), &mut __hasher);
                (std::hash::Hasher::finish(&__hasher), format!("{}", __v))
            }
        }
    }
}
impl PartialEq for GoNodeInterfaceKey {
    fn eq(&self, other: &Self) -> bool {
        let __left_guard = self.0.lock().unwrap();
        let __right_guard = other.0.lock().unwrap();
        match (__left_guard.as_ref(), __right_guard.as_ref()) {
            (None, None) => true,
            (Some(__left), Some(__right)) => __left.as_ref().__go_eq_node(__right.as_ref()),
            _ => false,
        }
    }
}
impl Eq for GoNodeInterfaceKey {}
impl PartialOrd for GoNodeInterfaceKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for GoNodeInterfaceKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other { return std::cmp::Ordering::Equal; }
        match self.identity().cmp(&other.identity()) {
            std::cmp::Ordering::Equal => self.addr().cmp(&other.addr()),
            ordering => ordering,
        }
    }
}
impl std::fmt::Debug for GoNodeInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}
impl std::fmt::Display for GoNodeInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
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
        let mut cl = Arc::new(Mutex::new(Some({ let __s = c.clone(); let __sep = "\n".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() })));
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
        return Arc::new(Mutex::new(Some({ let __parts = (*lines.lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = "\n".to_string(); __parts.join(&__sep) })));
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

#[derive(Clone)]
pub struct FuncDeclPtr(pub Arc<Mutex<Option<FuncDecl>>>);

impl std::fmt::Display for FuncDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
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
    if (*Arc::new(Mutex::new(Some({ let __s = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "line ".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) || (*Arc::new(Mutex::new(Some({ let __s = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "extern ".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) || (*Arc::new(Mutex::new(Some({ let __s = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "export ".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        return true;
    }

        // "//[a-z0-9]+:[a-z0-9]"
        // (The // has been removed.)
    let mut colon = Arc::new(Mutex::new(Some({ let __s = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __substr = ":".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) })));
    if { let __tmp_x = { let __v = (*colon.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } || { let __tmp_x = ({ let __tmp_x = { let __v = (*colon.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*c.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } {
        return false;
    }
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*colon.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*colon.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
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


impl GoValueClone for Ident {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BasicLit {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FuncType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for LabeledStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for AssignStmt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for BlockStmt {
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


impl GoValueClone for FuncDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
