use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Default)]
pub struct ast_ArrayType {
    pub elt: Arc<Mutex<Option<ast_Expr>>>,
    pub len: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ArrayType>")
    }
}


impl ast_ArrayType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_AssignStmt {
    pub lhs: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub rhs: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
}

impl std::fmt::Display for ast_AssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_AssignStmt>")
    }
}


impl ast_AssignStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BasicLit {
    pub kind: Arc<Mutex<Option<token_Token>>>,
    pub pos: Arc<Mutex<Option<token_Pos>>>,
    pub value: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for ast_BasicLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BasicLit>")
    }
}


impl ast_BasicLit {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BinaryExpr {
    pub op: Arc<Mutex<Option<token_Token>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
    pub y: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BinaryExpr>")
    }
}


impl ast_BinaryExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BlockStmt {
    pub lbrace: Arc<Mutex<Option<token_Pos>>>,
    pub list: Arc<Mutex<Option<Vec<ast_Stmt>>>>,
}

impl std::fmt::Display for ast_BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BlockStmt>")
    }
}


impl ast_BlockStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_BranchStmt {
    pub label: Arc<Mutex<Option<ast_Ident>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
}

impl std::fmt::Display for ast_BranchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_BranchStmt>")
    }
}


impl ast_BranchStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CallExpr {
    pub args: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub ellipsis: Arc<Mutex<Option<token_Pos>>>,
    pub fun: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CallExpr>")
    }
}


impl ast_CallExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CaseClause {
    pub body: Arc<Mutex<Option<Vec<ast_Stmt>>>>,
    pub colon: Arc<Mutex<Option<token_Pos>>>,
    pub list: Arc<Mutex<Option<Vec<ast_Expr>>>>,
}

impl std::fmt::Display for ast_CaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CaseClause>")
    }
}


impl ast_CaseClause {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_ChanDir(pub i32);

impl PartialEq<i32> for ast_ChanDir {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ast_ChanDir> for i32 {
    fn eq(&self, other: &ast_ChanDir) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for ast_ChanDir {
    type Output = ast_ChanDir;
    fn bitand(self, other: Self) -> ast_ChanDir {
        ast_ChanDir(self.0 & other.0)
    }
}

impl std::ops::BitOr for ast_ChanDir {
    type Output = ast_ChanDir;
    fn bitor(self, other: Self) -> ast_ChanDir {
        ast_ChanDir(self.0 | other.0)
    }
}

impl std::fmt::Display for ast_ChanDir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ChanDir>")
    }
}


impl ast_ChanDir {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ChanType {
    pub dir: Arc<Mutex<Option<ast_ChanDir>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ChanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ChanType>")
    }
}


impl ast_ChanType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CommClause {
    pub body: Arc<Mutex<Option<Vec<ast_Stmt>>>>,
    pub comm: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_CommClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CommClause>")
    }
}


impl ast_CommClause {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_CompositeLit {
    pub elts: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_CompositeLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_CompositeLit>")
    }
}


impl ast_CompositeLit {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Decl {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Decl {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Decl {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Decl>")
    }
}

impl std::fmt::Display for ast_Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Decl>")
    }
}

impl PartialEq for ast_Decl {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Decl {}

impl PartialOrd for ast_Decl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Decl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_DeclStmt {
    pub decl: Arc<Mutex<Option<ast_Decl>>>,
}

impl std::fmt::Display for ast_DeclStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_DeclStmt>")
    }
}


impl ast_DeclStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_DeferStmt {
    pub call: Arc<Mutex<Option<ast_CallExpr>>>,
}

impl std::fmt::Display for ast_DeferStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_DeferStmt>")
    }
}


impl ast_DeferStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_Ellipsis {
    pub elt: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_Ellipsis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ellipsis>")
    }
}


impl ast_Ellipsis {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_EmptyStmt;

impl std::fmt::Display for ast_EmptyStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_EmptyStmt>")
    }
}


impl ast_EmptyStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Expr {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Expr {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Expr {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}

impl std::fmt::Display for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}

impl PartialEq for ast_Expr {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Expr {}

impl PartialOrd for ast_Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ExprStmt {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ExprStmt>")
    }
}


impl ast_ExprStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_Field {
    pub names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Ident>>>>>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub tag: Arc<Mutex<Option<ast_BasicLit>>>,
}

impl std::fmt::Display for ast_Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Field>")
    }
}


impl ast_Field {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FieldList {
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Field>>>>>>>,
}

impl std::fmt::Display for ast_FieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FieldList>")
    }
}


impl ast_FieldList {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_File {
    pub __go_filename: Arc<Mutex<Option<String>>>,
    pub __go_source: Arc<Mutex<Option<String>>>,
    pub decls: Arc<Mutex<Option<Vec<ast_Decl>>>>,
    pub imports: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_ImportSpec>>>>>>>,
    pub name: Arc<Mutex<Option<ast_Ident>>>,
}

impl std::fmt::Display for ast_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_File>")
    }
}


impl ast_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ForStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub cond: Arc<Mutex<Option<ast_Expr>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
    pub post: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_ForStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ForStmt>")
    }
}


impl ast_ForStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FuncDecl {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub r#type: Arc<Mutex<Option<ast_FuncType>>>,
    pub recv: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_FuncDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FuncDecl>")
    }
}


impl ast_FuncDecl {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FuncLit {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub r#type: Arc<Mutex<Option<ast_FuncType>>>,
}

impl std::fmt::Display for ast_FuncLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FuncLit>")
    }
}


impl ast_FuncLit {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_FuncType {
    pub params: Arc<Mutex<Option<ast_FieldList>>>,
    pub results: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_FuncType>")
    }
}


impl ast_FuncType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_GenDecl {
    pub specs: Arc<Mutex<Option<Vec<ast_Spec>>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
}

impl std::fmt::Display for ast_GenDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_GenDecl>")
    }
}


impl ast_GenDecl {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_GoStmt {
    pub call: Arc<Mutex<Option<ast_CallExpr>>>,
}

impl std::fmt::Display for ast_GoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_GoStmt>")
    }
}


impl ast_GoStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_Ident {
    pub __go_pos: i32,
    pub name: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for ast_Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ident>")
    }
}


impl ast_Ident {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IfStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub cond: Arc<Mutex<Option<ast_Expr>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
    pub r#else: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IfStmt>")
    }
}


impl ast_IfStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ImportSpec {
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub path: Arc<Mutex<Option<ast_BasicLit>>>,
}

impl std::fmt::Display for ast_ImportSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ImportSpec>")
    }
}


impl ast_ImportSpec {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IncDecStmt {
    pub tok: Arc<Mutex<Option<token_Token>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_IncDecStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IncDecStmt>")
    }
}


impl ast_IncDecStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IndexExpr {
    pub index: Arc<Mutex<Option<ast_Expr>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IndexExpr>")
    }
}


impl ast_IndexExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_IndexListExpr {
    pub indices: Arc<Mutex<Option<Vec<ast_Expr>>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_IndexListExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_IndexListExpr>")
    }
}


impl ast_IndexListExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_InterfaceType {
    pub methods: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_InterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_InterfaceType>")
    }
}


impl ast_InterfaceType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_KeyValueExpr {
    pub key: Arc<Mutex<Option<ast_Expr>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_KeyValueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_KeyValueExpr>")
    }
}


impl ast_KeyValueExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_LabeledStmt {
    pub label: Arc<Mutex<Option<ast_Ident>>>,
    pub stmt: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_LabeledStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_LabeledStmt>")
    }
}


impl ast_LabeledStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_MapType {
    pub key: Arc<Mutex<Option<ast_Expr>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_MapType>")
    }
}


impl ast_MapType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ParenExpr {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ParenExpr>")
    }
}


impl ast_ParenExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_RangeStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub key: Arc<Mutex<Option<ast_Expr>>>,
    pub tok: Arc<Mutex<Option<token_Token>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_RangeStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_RangeStmt>")
    }
}


impl ast_RangeStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ReturnStmt {
    pub results: Arc<Mutex<Option<Vec<ast_Expr>>>>,
}

impl std::fmt::Display for ast_ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ReturnStmt>")
    }
}


impl ast_ReturnStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SelectStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
}

impl std::fmt::Display for ast_SelectStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectStmt>")
    }
}


impl ast_SelectStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SelectorExpr {
    pub sel: Arc<Mutex<Option<ast_Ident>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SelectorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectorExpr>")
    }
}


impl ast_SelectorExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SendStmt {
    pub chan: Arc<Mutex<Option<ast_Expr>>>,
    pub value: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SendStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SendStmt>")
    }
}


impl ast_SendStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SliceExpr {
    pub high: Arc<Mutex<Option<ast_Expr>>>,
    pub low: Arc<Mutex<Option<ast_Expr>>>,
    pub max: Arc<Mutex<Option<ast_Expr>>>,
    pub slice3: Arc<Mutex<Option<bool>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SliceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SliceExpr>")
    }
}


impl ast_SliceExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Spec {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Spec {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Spec {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Spec>")
    }
}

impl std::fmt::Display for ast_Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Spec>")
    }
}

impl PartialEq for ast_Spec {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Spec {}

impl PartialOrd for ast_Spec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Spec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_StarExpr {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_StarExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_StarExpr>")
    }
}


impl ast_StarExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Stmt {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Stmt {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Stmt {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}

impl std::fmt::Display for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}

impl PartialEq for ast_Stmt {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Stmt {}

impl PartialOrd for ast_Stmt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Stmt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_StructType {
    pub fields: Arc<Mutex<Option<ast_FieldList>>>,
}

impl std::fmt::Display for ast_StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_StructType>")
    }
}


impl ast_StructType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SwitchStmt {
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
    pub tag: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SwitchStmt>")
    }
}


impl ast_SwitchStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_TypeAssertExpr {
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_TypeAssertExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_TypeAssertExpr>")
    }
}


impl ast_TypeAssertExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_TypeSpec {
    pub assign: Arc<Mutex<Option<token_Pos>>>,
    pub name: Arc<Mutex<Option<ast_Ident>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_TypeSpec>")
    }
}


impl ast_TypeSpec {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_TypeSwitchStmt {
    pub assign: Arc<Mutex<Option<ast_Stmt>>>,
    pub body: Arc<Mutex<Option<ast_BlockStmt>>>,
    pub init: Arc<Mutex<Option<ast_Stmt>>>,
}

impl std::fmt::Display for ast_TypeSwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_TypeSwitchStmt>")
    }
}


impl ast_TypeSwitchStmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_UnaryExpr {
    pub op: Arc<Mutex<Option<token_Token>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_UnaryExpr>")
    }
}


impl ast_UnaryExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_ValueSpec {
    pub names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Ident>>>>>>>,
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
    pub values: Arc<Mutex<Option<Vec<ast_Expr>>>>,
}

impl std::fmt::Display for ast_ValueSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ValueSpec>")
    }
}


impl ast_ValueSpec {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct parser_Mode(pub u64);

impl PartialEq<u64> for parser_Mode {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<parser_Mode> for u64 {
    fn eq(&self, other: &parser_Mode) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for parser_Mode {
    type Output = parser_Mode;
    fn bitand(self, other: Self) -> parser_Mode {
        parser_Mode(self.0 & other.0)
    }
}

impl std::ops::BitOr for parser_Mode {
    type Output = parser_Mode;
    fn bitor(self, other: Self) -> parser_Mode {
        parser_Mode(self.0 | other.0)
    }
}

impl std::fmt::Display for parser_Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<parser_Mode>")
    }
}


impl parser_Mode {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_FileSet;

impl std::fmt::Display for token_FileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_FileSet>")
    }
}


impl token_FileSet {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Pos(pub i32);

impl PartialEq<i32> for token_Pos {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Pos> for i32 {
    fn eq(&self, other: &token_Pos) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Pos {
    type Output = token_Pos;
    fn bitand(self, other: Self) -> token_Pos {
        token_Pos(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Pos {
    type Output = token_Pos;
    fn bitor(self, other: Self) -> token_Pos {
        token_Pos(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Pos>")
    }
}


impl token_Pos {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Token(pub i32);

impl PartialEq<i32> for token_Token {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Token> for i32 {
    fn eq(&self, other: &token_Token) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Token {
    type Output = token_Token;
    fn bitand(self, other: Self) -> token_Token {
        token_Token(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Token {
    type Output = token_Token;
    fn bitor(self, other: Self) -> token_Token {
        token_Token(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Token>")
    }
}


impl token_Token {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod ast {
    use super::*;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    pub const R_E_C_V: ast_ChanDir = ast_ChanDir(2);
    pub const S_E_N_D: ast_ChanDir = ast_ChanDir(1);
}


pub mod binary {
    use super::*;
    pub const MAX_VARINT_LEN64: i32 = 10;
}


pub mod parser {
    use super::*;

    pub trait GoParserFilenameArg {
        fn into_go_parser_filename(self) -> String;
    }

    impl GoParserFilenameArg for String {
        fn into_go_parser_filename(self) -> String {
            self
        }
    }

    impl<'a> GoParserFilenameArg for &'a str {
        fn into_go_parser_filename(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoParserFilenameArg for &'a String {
        fn into_go_parser_filename(self) -> String {
            self.clone()
        }
    }

    impl GoParserFilenameArg for Arc<Mutex<Option<String>>> {
        fn into_go_parser_filename(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    pub trait GoParserSourceArg {
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    }

    impl GoParserSourceArg for () {
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            std::fs::read_to_string(filename).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    impl GoParserSourceArg for String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self)
        }
    }

    impl<'a> GoParserSourceArg for &'a str {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.to_string())
        }
    }

    impl<'a> GoParserSourceArg for &'a String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.clone())
        }
    }

    impl GoParserSourceArg for Vec<u8> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            String::from_utf8(self).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    impl GoParserSourceArg for Arc<Mutex<Option<String>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.lock().unwrap().as_ref().cloned().unwrap_or_default())
        }
    }

    impl GoParserSourceArg for Arc<Mutex<Option<Vec<u8>>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let bytes = self.lock().unwrap().as_ref().cloned().unwrap_or_default();
            String::from_utf8(bytes).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    pub const SKIP_OBJECT_RESOLUTION: parser_Mode = parser_Mode(64);

    fn go_parser_error(message: String) -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message))
    }

    fn go_parser_string(value: String) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(value)))
    }

    fn go_parser_ident(name: String) -> Arc<Mutex<Option<ast_Ident>>> {
        Arc::new(Mutex::new(Some::<ast_Ident>(ast_Ident { name: go_parser_string(name), ..Default::default() })))
    }

    fn go_parser_basic_lit(value: String) -> Arc<Mutex<Option<ast_BasicLit>>> {
        Arc::new(Mutex::new(Some::<ast_BasicLit>(ast_BasicLit { value: go_parser_string(value), ..Default::default() })))
    }

    fn go_parser_import_spec_from_parts(name: Option<String>, path: String) -> Arc<Mutex<Option<ast_ImportSpec>>> {
        Arc::new(Mutex::new(Some::<ast_ImportSpec>(ast_ImportSpec { name: name.map(go_parser_ident).unwrap_or_else(|| Arc::new(Mutex::new(None::<ast_Ident>))), path: go_parser_basic_lit(path), ..Default::default() })))
    }

    fn go_parser_is_ident_start(ch: char) -> bool {
        ch == '_' || ch.is_alphabetic()
    }

    fn go_parser_is_ident_continue(ch: char) -> bool {
        ch == '_' || ch.is_alphanumeric()
    }

    fn go_parser_tokens(source: &str) -> Vec<String> {
        let chars: Vec<char> = source.chars().collect();
        let mut tokens = Vec::new();
        let mut i = 0usize;
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_whitespace() {
                i += 1;
                continue;
            }
            if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                i += 2;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                i = (i + 2).min(chars.len());
                continue;
            }
            if ch == '"' {
                let start = i;
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\\' {
                        i = (i + 2).min(chars.len());
                        continue;
                    }
                    if chars[i] == '"' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                tokens.push(chars[start..i].iter().collect());
                continue;
            }
            if ch == char::from(96) {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != char::from(96) {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                tokens.push(chars[start..i].iter().collect());
                continue;
            }
            if go_parser_is_ident_start(ch) {
                let start = i;
                i += 1;
                while i < chars.len() && go_parser_is_ident_continue(chars[i]) {
                    i += 1;
                }
                tokens.push(chars[start..i].iter().collect());
                continue;
            }
            if ch == '(' || ch == ')' || ch == ';' || ch == '.' {
                tokens.push(ch.to_string());
            }
            i += 1;
        }
        tokens
    }

    fn go_parser_is_string_lit(token: &str) -> bool {
        token.starts_with('"') || token.starts_with(char::from(96))
    }

    fn go_parser_import_from_tokens(tokens: &[String], start: usize) -> Option<(Arc<Mutex<Option<ast_ImportSpec>>>, usize)> {
        if start >= tokens.len() {
            return None;
        }
        if go_parser_is_string_lit(&tokens[start]) {
            return Some((go_parser_import_spec_from_parts(None, tokens[start].clone()), start + 1));
        }
        if start + 1 < tokens.len() && go_parser_is_string_lit(&tokens[start + 1]) {
            return Some((go_parser_import_spec_from_parts(Some(tokens[start].clone()), tokens[start + 1].clone()), start + 2));
        }
        None
    }

    fn go_parser_some<T>(value: T) -> Arc<Mutex<Option<T>>> {
        Arc::new(Mutex::new(Some(value)))
    }

    fn go_parser_none<T>() -> Arc<Mutex<Option<T>>> {
        Arc::new(Mutex::new(None::<T>))
    }

    fn go_parser_pos(pos: usize) -> Arc<Mutex<Option<token_Pos>>> {
        go_parser_some(token_Pos(go_parser_pos_value(pos)))
    }

    fn go_parser_no_pos() -> Arc<Mutex<Option<token_Pos>>> {
        go_parser_some(token_Pos(0))
    }

    fn go_parser_pos_value(pos: usize) -> i32 {
        pos as i32 + 1
    }

    fn go_parser_token(tok: token_Token) -> Arc<Mutex<Option<token_Token>>> {
        go_parser_some(tok)
    }

    fn go_parser_lit_kind(kind: gosyn::token::LitKind) -> token_Token {
        match kind {
            gosyn::token::LitKind::Ident => token::I_D_E_N_T,
            gosyn::token::LitKind::String => token::S_T_R_I_N_G,
            gosyn::token::LitKind::Integer => token::I_N_T,
            gosyn::token::LitKind::Float => token::F_L_O_A_T,
            gosyn::token::LitKind::Imag => token::I_M_A_G,
            gosyn::token::LitKind::Char => token::C_H_A_R,
        }
    }

    fn go_parser_operator(op: gosyn::token::Operator) -> token_Token {
        match op {
            gosyn::token::Operator::Add => token::A_D_D,
            gosyn::token::Operator::Sub => token::S_U_B,
            gosyn::token::Operator::Star => token::M_U_L,
            gosyn::token::Operator::Quo => token::Q_U_O,
            gosyn::token::Operator::Rem => token::R_E_M,
            gosyn::token::Operator::And => token::A_N_D,
            gosyn::token::Operator::Or => token::O_R,
            gosyn::token::Operator::Xor => token::X_O_R,
            gosyn::token::Operator::Shl => token::S_H_L,
            gosyn::token::Operator::Shr => token::S_H_R,
            gosyn::token::Operator::AndNot => token::A_N_D__N_O_T,
            gosyn::token::Operator::AddAssign => token::A_D_D__A_S_S_I_G_N,
            gosyn::token::Operator::SubAssign => token::S_U_B__A_S_S_I_G_N,
            gosyn::token::Operator::MulAssign => token::M_U_L__A_S_S_I_G_N,
            gosyn::token::Operator::QuoAssign => token::Q_U_O__A_S_S_I_G_N,
            gosyn::token::Operator::RemAssign => token::R_E_M__A_S_S_I_G_N,
            gosyn::token::Operator::AndAssign => token::A_N_D__A_S_S_I_G_N,
            gosyn::token::Operator::OrAssign => token::O_R__A_S_S_I_G_N,
            gosyn::token::Operator::XorAssign => token::X_O_R__A_S_S_I_G_N,
            gosyn::token::Operator::ShlAssign => token::S_H_L__A_S_S_I_G_N,
            gosyn::token::Operator::ShrAssign => token::S_H_R__A_S_S_I_G_N,
            gosyn::token::Operator::AndAnd => token::L_A_N_D,
            gosyn::token::Operator::OrOr => token::L_O_R,
            gosyn::token::Operator::Arrow => token::A_R_R_O_W,
            gosyn::token::Operator::Inc => token::I_N_C,
            gosyn::token::Operator::Dec => token::D_E_C,
            gosyn::token::Operator::Equal => token::E_Q_L,
            gosyn::token::Operator::Less => token::L_S_S,
            gosyn::token::Operator::Greater => token::G_T_R,
            gosyn::token::Operator::Assign => token::A_S_S_I_G_N,
            gosyn::token::Operator::Not => token::N_O_T,
            gosyn::token::Operator::Tiled => token::T_I_L_D_E,
            gosyn::token::Operator::NotEqual => token::N_E_Q,
            gosyn::token::Operator::LessEqual => token::L_E_Q,
            gosyn::token::Operator::GreaterEqual => token::G_E_Q,
            gosyn::token::Operator::Define => token::D_E_F_I_N_E,
            gosyn::token::Operator::DotDotDot => token::E_L_L_I_P_S_I_S,
            _ => token_Token(0),
        }
    }

    fn go_parser_keyword(tok: gosyn::token::Keyword) -> token_Token {
        match tok {
            gosyn::token::Keyword::Break => token::B_R_E_A_K,
            gosyn::token::Keyword::Case => token::C_A_S_E,
            gosyn::token::Keyword::Chan => token::C_H_A_N,
            gosyn::token::Keyword::Const => token::C_O_N_S_T,
            gosyn::token::Keyword::Continue => token::C_O_N_T_I_N_U_E,
            gosyn::token::Keyword::Default => token::D_E_F_A_U_L_T,
            gosyn::token::Keyword::Defer => token::D_E_F_E_R,
            gosyn::token::Keyword::Else => token::E_L_S_E,
            gosyn::token::Keyword::FallThrough => token::F_A_L_L_T_H_R_O_U_G_H,
            gosyn::token::Keyword::For => token::F_O_R,
            gosyn::token::Keyword::Func => token::F_U_N_C,
            gosyn::token::Keyword::Go => token::G_O,
            gosyn::token::Keyword::Goto => token::G_O_T_O,
            gosyn::token::Keyword::If => token::I_F,
            gosyn::token::Keyword::Import => token::I_M_P_O_R_T,
            gosyn::token::Keyword::Interface => token::I_N_T_E_R_F_A_C_E,
            gosyn::token::Keyword::Map => token::M_A_P,
            gosyn::token::Keyword::Package => token::P_A_C_K_A_G_E,
            gosyn::token::Keyword::Range => token::R_A_N_G_E,
            gosyn::token::Keyword::Return => token::R_E_T_U_R_N,
            gosyn::token::Keyword::Select => token::S_E_L_E_C_T,
            gosyn::token::Keyword::Struct => token::S_T_R_U_C_T,
            gosyn::token::Keyword::Switch => token::S_W_I_T_C_H,
            gosyn::token::Keyword::Type => token::T_Y_P_E,
            gosyn::token::Keyword::Var => token::V_A_R,
        }
    }

    fn go_parser_ident_struct(id: gosyn::ast::Ident) -> ast_Ident {
        ast_Ident { __go_pos: go_parser_pos_value(id.pos), name: go_parser_some(id.name), ..Default::default() }
    }

    fn go_parser_ident_expr(id: gosyn::ast::Ident) -> ast_Expr {
        let pos = id.pos;
        ast_Expr::__go_from_with_pos(go_parser_ident_struct(id), go_parser_pos_value(pos))
    }

    fn go_parser_basic_lit_expr(lit: gosyn::ast::BasicLit) -> ast_Expr {
        ast_Expr::__go_from_with_pos(ast_BasicLit {
            kind: go_parser_token(go_parser_lit_kind(lit.kind)),
            pos: go_parser_pos(lit.pos),
            value: go_parser_some(lit.value),
            ..Default::default()
        }, go_parser_pos_value(lit.pos))
    }

    fn go_parser_field_list(list: gosyn::ast::FieldList) -> Arc<Mutex<Option<ast_FieldList>>> {
        let fields = list.list.into_iter().map(go_parser_field).map(|field| go_parser_some(field)).collect::<Vec<_>>();
        go_parser_some(ast_FieldList { list: go_parser_some(fields), ..Default::default() })
    }

    fn go_parser_field(field: gosyn::ast::Field) -> ast_Field {
        let names = field.name.into_iter().map(go_parser_ident_struct).map(go_parser_some).collect::<Vec<_>>();
        ast_Field {
            names: go_parser_some(names),
            r#type: go_parser_some(go_parser_expr(field.typ)),
            tag: field.tag.map(|tag| ast_BasicLit {
                kind: go_parser_token(token::S_T_R_I_N_G),
                value: go_parser_some(tag.value),
                ..Default::default()
            }).map(go_parser_some).unwrap_or_else(go_parser_none),
            ..Default::default()
        }
    }

    fn go_parser_func_type(typ: gosyn::ast::FuncType) -> ast_FuncType {
        ast_FuncType {
            params: go_parser_field_list(typ.params),
            results: go_parser_field_list(typ.result),
            ..Default::default()
        }
    }

    fn go_parser_call_expr(call: gosyn::ast::Call) -> ast_CallExpr {
        ast_CallExpr {
            fun: go_parser_some(go_parser_expr(*call.func)),
            args: go_parser_some(call.args.into_iter().map(go_parser_expr).collect()),
            ellipsis: call.dots.map(go_parser_pos).unwrap_or_else(go_parser_no_pos),
            ..Default::default()
        }
    }

    fn go_parser_lit_element(element: gosyn::ast::Element) -> ast_Expr {
        match element {
            gosyn::ast::Element::Expr(expr) => go_parser_expr(expr),
            gosyn::ast::Element::LitValue(value) => ast_Expr::__go_from_with_pos(ast_CompositeLit {
                elts: go_parser_some(go_parser_lit_values(value)),
                ..Default::default()
            }, 0),
        }
    }

    fn go_parser_lit_values(value: gosyn::ast::LiteralValue) -> Vec<ast_Expr> {
        value.values.into_iter().map(|element| {
            let val = go_parser_lit_element(element.val);
            match element.key {
                Some(key) => ast_Expr::__go_from_with_pos(ast_KeyValueExpr {
                    key: go_parser_some(go_parser_lit_element(key)),
                    value: go_parser_some(val),
                    ..Default::default()
                }, 0),
                None => val,
            }
        }).collect()
    }

    fn go_parser_expr(expr: gosyn::ast::Expression) -> ast_Expr {
        match expr {
            gosyn::ast::Expression::Ident(id) => go_parser_ident_expr(id),
            gosyn::ast::Expression::BasicLit(lit) => go_parser_basic_lit_expr(lit),
            gosyn::ast::Expression::Call(call) => {
                let pos = call.pos.0;
                ast_Expr::__go_from_with_pos(go_parser_call_expr(call), go_parser_pos_value(pos))
            }
            gosyn::ast::Expression::Selector(sel) => ast_Expr::__go_from_with_pos(ast_SelectorExpr {
                x: go_parser_some(go_parser_expr(*sel.x)),
                sel: go_parser_some(go_parser_ident_struct(sel.sel)),
                ..Default::default()
            }, go_parser_pos_value(sel.pos)),
            gosyn::ast::Expression::Index(index) => ast_Expr::__go_from_with_pos(ast_IndexExpr {
                x: go_parser_some(go_parser_expr(*index.left)),
                index: go_parser_some(go_parser_expr(*index.index)),
                ..Default::default()
            }, go_parser_pos_value(index.pos.0)),
            gosyn::ast::Expression::IndexList(index) => ast_Expr::__go_from_with_pos(ast_IndexListExpr {
                x: go_parser_some(go_parser_expr(*index.left)),
                indices: go_parser_some(index.indices.into_iter().map(go_parser_expr).collect()),
                ..Default::default()
            }, go_parser_pos_value(index.pos.0)),
            gosyn::ast::Expression::Slice(slice) => ast_Expr::__go_from_with_pos(ast_SliceExpr {
                x: go_parser_some(go_parser_expr(*slice.left)),
                low: slice.index[0].as_ref().map(|expr| go_parser_expr((**expr).clone())).map(go_parser_some).unwrap_or_else(go_parser_none),
                high: slice.index[1].as_ref().map(|expr| go_parser_expr((**expr).clone())).map(go_parser_some).unwrap_or_else(go_parser_none),
                max: slice.index[2].as_ref().map(|expr| go_parser_expr((**expr).clone())).map(go_parser_some).unwrap_or_else(go_parser_none),
                slice3: go_parser_some(slice.index[2].is_some()),
                ..Default::default()
            }, go_parser_pos_value(slice.pos.0)),
            gosyn::ast::Expression::FuncLit(lit) => ast_Expr::__go_from_with_pos(ast_FuncLit {
                r#type: go_parser_some(go_parser_func_type(lit.typ)),
                body: go_parser_some(go_parser_block(lit.body)),
                ..Default::default()
            }, 0),
            gosyn::ast::Expression::Ellipsis(ellipsis) => ast_Expr::__go_from_with_pos(ast_Ellipsis {
                elt: ellipsis.elt.map(|expr| go_parser_expr(*expr)).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }, go_parser_pos_value(ellipsis.pos)),
            gosyn::ast::Expression::Star(star) => ast_Expr::__go_from_with_pos(ast_StarExpr {
                x: go_parser_some(go_parser_expr(*star.right)),
                ..Default::default()
            }, go_parser_pos_value(star.pos)),
            gosyn::ast::Expression::Paren(paren) => ast_Expr::__go_from_with_pos(ast_ParenExpr {
                x: go_parser_some(go_parser_expr(*paren.expr)),
                ..Default::default()
            }, go_parser_pos_value(paren.pos.0)),
            gosyn::ast::Expression::TypeAssert(assertion) => ast_Expr::__go_from_with_pos(ast_TypeAssertExpr {
                x: go_parser_some(go_parser_expr(*assertion.left)),
                r#type: assertion.right.map(|expr| go_parser_expr(*expr)).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }, go_parser_pos_value(assertion.pos.0)),
            gosyn::ast::Expression::CompositeLit(lit) => {
                let pos = lit.val.pos.0;
                ast_Expr::__go_from_with_pos(ast_CompositeLit {
                    r#type: go_parser_some(go_parser_expr(*lit.typ)),
                    elts: go_parser_some(go_parser_lit_values(lit.val)),
                    ..Default::default()
                }, go_parser_pos_value(pos))
            }
            gosyn::ast::Expression::Operation(op) => {
                let token = go_parser_operator(op.op);
                match op.y {
                    Some(y) => ast_Expr::__go_from_with_pos(ast_BinaryExpr {
                        x: go_parser_some(go_parser_expr(*op.x)),
                        y: go_parser_some(go_parser_expr(*y)),
                        op: go_parser_token(token),
                        ..Default::default()
                    }, go_parser_pos_value(op.pos)),
                    None if token == token::M_U_L => ast_Expr::__go_from_with_pos(ast_StarExpr {
                        x: go_parser_some(go_parser_expr(*op.x)),
                        ..Default::default()
                    }, go_parser_pos_value(op.pos)),
                    None => ast_Expr::__go_from_with_pos(ast_UnaryExpr {
                        x: go_parser_some(go_parser_expr(*op.x)),
                        op: go_parser_token(token),
                        ..Default::default()
                    }, go_parser_pos_value(op.pos)),
                }
            }
            gosyn::ast::Expression::TypeMap(map) => ast_Expr::__go_from_with_pos(ast_MapType {
                key: go_parser_some(go_parser_expr(*map.key)),
                value: go_parser_some(go_parser_expr(*map.val)),
                ..Default::default()
            }, go_parser_pos_value(map.pos.0)),
            gosyn::ast::Expression::TypeArray(array) => ast_Expr::__go_from_with_pos(ast_ArrayType {
                len: go_parser_some(go_parser_expr(*array.len)),
                elt: go_parser_some(go_parser_expr(*array.typ)),
                ..Default::default()
            }, go_parser_pos_value(array.pos.0)),
            gosyn::ast::Expression::TypeSlice(slice) => ast_Expr::__go_from_with_pos(ast_ArrayType {
                len: go_parser_none(),
                elt: go_parser_some(go_parser_expr(*slice.typ)),
                ..Default::default()
            }, go_parser_pos_value(slice.pos.0)),
            gosyn::ast::Expression::TypeFunction(typ) => {
                let pos = typ.pos;
                ast_Expr::__go_from_with_pos(go_parser_func_type(typ), go_parser_pos_value(pos))
            }
            gosyn::ast::Expression::TypeStruct(typ) => ast_Expr::__go_from_with_pos(ast_StructType {
                fields: go_parser_some(ast_FieldList {
                    list: go_parser_some(typ.fields.into_iter().map(go_parser_field).map(go_parser_some).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }, go_parser_pos_value(typ.pos.0)),
            gosyn::ast::Expression::TypeInterface(typ) => ast_Expr::__go_from_with_pos(ast_InterfaceType {
                methods: go_parser_field_list(typ.methods),
                ..Default::default()
            }, go_parser_pos_value(typ.pos)),
            gosyn::ast::Expression::TypePointer(ptr) => ast_Expr::__go_from_with_pos(ast_StarExpr {
                x: go_parser_some(go_parser_expr(*ptr.typ)),
                ..Default::default()
            }, go_parser_pos_value(ptr.pos)),
            gosyn::ast::Expression::TypeChannel(chan) => {
                let dir = match chan.dir {
                    Some(gosyn::ast::ChanMode::Send) => ast_ChanDir(1),
                    Some(gosyn::ast::ChanMode::Recv) => ast_ChanDir(2),
                    None => ast_ChanDir(3),
                };
                ast_Expr::__go_from_with_pos(ast_ChanType {
                    dir: go_parser_some(dir),
                    value: go_parser_some(go_parser_expr(*chan.typ)),
                    ..Default::default()
                }, go_parser_pos_value(chan.pos.0))
            }
            gosyn::ast::Expression::List(list) => list.into_iter().next().map(go_parser_expr).unwrap_or_default(),
            gosyn::ast::Expression::Range(range) => ast_Expr::__go_from_with_pos(ast_UnaryExpr {
                op: go_parser_token(token::R_A_N_G_E),
                x: go_parser_some(go_parser_expr(*range.right)),
                ..Default::default()
            }, go_parser_pos_value(range.pos)),
        }
    }

    fn go_parser_block(block: gosyn::ast::BlockStmt) -> ast_BlockStmt {
        ast_BlockStmt {
            lbrace: go_parser_pos(block.pos.0),
            list: go_parser_some(block.list.into_iter().map(go_parser_stmt).collect()),
            ..Default::default()
        }
    }

    fn go_parser_expr_from_stmt(stmt: gosyn::ast::Statement) -> Arc<Mutex<Option<ast_Expr>>> {
        match stmt {
            gosyn::ast::Statement::Expr(expr) => go_parser_some(go_parser_expr(expr.expr)),
            _ => go_parser_none(),
        }
    }

    fn go_parser_decl_stmt(decl: gosyn::ast::DeclStmt) -> ast_Decl {
        match decl {
            gosyn::ast::DeclStmt::Type(decl) => go_parser_gen_decl(token::T_Y_P_E, decl.specs.into_iter().map(go_parser_type_spec).collect()),
            gosyn::ast::DeclStmt::Const(decl) => go_parser_gen_decl(token::C_O_N_S_T, decl.specs.into_iter().map(go_parser_const_spec).collect()),
            gosyn::ast::DeclStmt::Variable(decl) => go_parser_gen_decl(token::V_A_R, decl.specs.into_iter().map(go_parser_var_spec).collect()),
        }
    }

    fn go_parser_stmt(stmt: gosyn::ast::Statement) -> ast_Stmt {
        match stmt {
            gosyn::ast::Statement::Expr(stmt) => ast_Stmt::__go_from(ast_ExprStmt {
                x: go_parser_some(go_parser_expr(stmt.expr)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Assign(stmt) => ast_Stmt::__go_from(ast_AssignStmt {
                lhs: go_parser_some(stmt.left.into_iter().map(go_parser_expr).collect()),
                rhs: go_parser_some(stmt.right.into_iter().map(go_parser_expr).collect()),
                tok: go_parser_token(go_parser_operator(stmt.op)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Return(stmt) => ast_Stmt::__go_from(ast_ReturnStmt {
                results: go_parser_some(stmt.ret.into_iter().map(go_parser_expr).collect()),
                ..Default::default()
            }),
            gosyn::ast::Statement::Block(block) => ast_Stmt::__go_from(go_parser_block(block)),
            gosyn::ast::Statement::If(stmt) => ast_Stmt::__go_from(ast_IfStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                cond: go_parser_some(go_parser_expr(stmt.cond)),
                body: go_parser_some(go_parser_block(stmt.body)),
                r#else: stmt.else_.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }),
            gosyn::ast::Statement::For(stmt) => ast_Stmt::__go_from(ast_ForStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                cond: stmt.cond.map(|stmt| go_parser_expr_from_stmt(*stmt)).unwrap_or_else(go_parser_none),
                post: stmt.post.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                body: go_parser_some(go_parser_block(stmt.body)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Range(stmt) => ast_Stmt::__go_from(ast_RangeStmt {
                key: stmt.key.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
                value: stmt.value.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
                tok: go_parser_token(stmt.op.map(|op| go_parser_operator(op.1)).unwrap_or(token::A_S_S_I_G_N)),
                x: go_parser_some(go_parser_expr(stmt.expr)),
                body: go_parser_some(go_parser_block(stmt.body)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Go(stmt) => ast_Stmt::__go_from(ast_GoStmt {
                call: go_parser_some(go_parser_call_expr(stmt.call)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Defer(stmt) => ast_Stmt::__go_from(ast_DeferStmt {
                call: go_parser_some(go_parser_call_expr(stmt.call)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Send(stmt) => ast_Stmt::__go_from(ast_SendStmt {
                chan: go_parser_some(go_parser_expr(stmt.chan)),
                value: go_parser_some(go_parser_expr(stmt.value)),
                ..Default::default()
            }),
            gosyn::ast::Statement::IncDec(stmt) => ast_Stmt::__go_from(ast_IncDecStmt {
                x: go_parser_some(go_parser_expr(stmt.expr)),
                tok: go_parser_token(go_parser_operator(stmt.op)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Branch(stmt) => ast_Stmt::__go_from(ast_BranchStmt {
                tok: go_parser_token(go_parser_keyword(stmt.key)),
                label: stmt.ident.map(go_parser_ident_struct).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }),
            gosyn::ast::Statement::Label(stmt) => ast_Stmt::__go_from(ast_LabeledStmt {
                label: go_parser_some(go_parser_ident_struct(stmt.name)),
                stmt: go_parser_some(go_parser_stmt(*stmt.stmt)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Declaration(decl) => ast_Stmt::__go_from(ast_DeclStmt {
                decl: go_parser_some(go_parser_decl_stmt(decl)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Switch(stmt) => ast_Stmt::__go_from(ast_SwitchStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                tag: stmt.tag.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
                body: go_parser_some(ast_BlockStmt {
                    lbrace: go_parser_pos(stmt.block.pos.0),
                    list: go_parser_some(stmt.block.body.into_iter().map(go_parser_case_clause).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            gosyn::ast::Statement::TypeSwitch(stmt) => ast_Stmt::__go_from(ast_TypeSwitchStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                assign: stmt.tag.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                body: go_parser_some(ast_BlockStmt {
                    lbrace: go_parser_pos(stmt.block.pos.0),
                    list: go_parser_some(stmt.block.body.into_iter().map(go_parser_case_clause).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            gosyn::ast::Statement::Select(stmt) => ast_Stmt::__go_from(ast_SelectStmt {
                body: go_parser_some(ast_BlockStmt {
                    lbrace: go_parser_pos(stmt.body.pos.0),
                    list: go_parser_some(stmt.body.body.into_iter().map(go_parser_comm_clause).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            gosyn::ast::Statement::Empty(_) => ast_Stmt::__go_from(ast_EmptyStmt),
        }
    }

    fn go_parser_case_clause(clause: gosyn::ast::CaseClause) -> ast_Stmt {
        ast_Stmt::__go_from(ast_CaseClause {
            list: go_parser_some(clause.list.into_iter().map(go_parser_expr).collect()),
            body: go_parser_some((*clause.body).into_iter().map(go_parser_stmt).collect()),
            colon: go_parser_pos(clause.pos.1),
            ..Default::default()
        })
    }

    fn go_parser_comm_clause(clause: gosyn::ast::CommClause) -> ast_Stmt {
        ast_Stmt::__go_from(ast_CommClause {
            comm: clause.comm.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
            body: go_parser_some((*clause.body).into_iter().map(go_parser_stmt).collect()),
            ..Default::default()
        })
    }

    fn go_parser_import_spec(import: gosyn::ast::Import) -> Arc<Mutex<Option<ast_ImportSpec>>> {
        go_parser_some(ast_ImportSpec {
            name: import.name.map(go_parser_ident_struct).map(go_parser_some).unwrap_or_else(go_parser_none),
            path: go_parser_some(ast_BasicLit {
                kind: go_parser_token(token::S_T_R_I_N_G),
                value: go_parser_some(import.path.value),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    fn go_parser_gen_decl(tok: token_Token, specs: Vec<ast_Spec>) -> ast_Decl {
        ast_Decl::__go_from(ast_GenDecl {
            tok: go_parser_token(tok),
            specs: go_parser_some(specs),
            ..Default::default()
        })
    }

    fn go_parser_var_spec(spec: gosyn::ast::VarSpec) -> ast_Spec {
        ast_Spec::__go_from(ast_ValueSpec {
            names: go_parser_some(spec.name.into_iter().map(go_parser_ident_struct).map(go_parser_some).collect()),
            r#type: spec.typ.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
            values: go_parser_some(spec.values.into_iter().map(go_parser_expr).collect()),
            ..Default::default()
        })
    }

    fn go_parser_const_spec(spec: gosyn::ast::ConstSpec) -> ast_Spec {
        ast_Spec::__go_from(ast_ValueSpec {
            names: go_parser_some(spec.name.into_iter().map(go_parser_ident_struct).map(go_parser_some).collect()),
            r#type: spec.typ.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
            values: go_parser_some(spec.values.into_iter().map(go_parser_expr).collect()),
            ..Default::default()
        })
    }

    fn go_parser_type_spec(spec: gosyn::ast::TypeSpec) -> ast_Spec {
        ast_Spec::__go_from(ast_TypeSpec {
            name: go_parser_some(go_parser_ident_struct(spec.name)),
            r#type: go_parser_some(go_parser_expr(spec.typ)),
            assign: if spec.alias { go_parser_pos(1) } else { go_parser_no_pos() },
            ..Default::default()
        })
    }

    fn go_parser_func_decl(decl: gosyn::ast::FuncDecl) -> ast_Decl {
        ast_Decl::__go_from(ast_FuncDecl {
            recv: decl.recv.map(go_parser_field_list).unwrap_or_else(go_parser_none),
            name: go_parser_some(go_parser_ident_struct(decl.name)),
            r#type: go_parser_some(go_parser_func_type(decl.typ)),
            body: decl.body.map(go_parser_block).map(go_parser_some).unwrap_or_else(go_parser_none),
            ..Default::default()
        })
    }

    fn go_parser_decl(decl: gosyn::ast::Declaration) -> ast_Decl {
        match decl {
            gosyn::ast::Declaration::Function(decl) => go_parser_func_decl(decl),
            gosyn::ast::Declaration::Type(decl) => go_parser_gen_decl(token::T_Y_P_E, decl.specs.into_iter().map(go_parser_type_spec).collect()),
            gosyn::ast::Declaration::Const(decl) => go_parser_gen_decl(token::C_O_N_S_T, decl.specs.into_iter().map(go_parser_const_spec).collect()),
            gosyn::ast::Declaration::Variable(decl) => go_parser_gen_decl(token::V_A_R, decl.specs.into_iter().map(go_parser_var_spec).collect()),
        }
    }

    fn go_parser_parse_file(filename: &str, source: &str) -> Result<ast_File, Box<dyn std::error::Error + Send + Sync>> {
        let parsed = gosyn::parse_source(source).map_err(|err| go_parser_error(err.to_string()))?;
        Ok(ast_File {
            __go_filename: go_parser_some(filename.to_string()),
            __go_source: go_parser_some(source.to_string()),
            imports: go_parser_some(parsed.imports.into_iter().map(go_parser_import_spec).collect()),
            decls: go_parser_some(parsed.decl.into_iter().map(go_parser_decl).collect()),
            name: go_parser_some(go_parser_ident_struct(parsed.pkg_name)),
            ..Default::default()
        })
    }

    pub fn parse_file<T0, T1: GoParserFilenameArg, T2: GoParserSourceArg, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> (Arc<Mutex<Option<ast_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let filename = _arg1.into_go_parser_filename();
        let source = match _arg2.into_go_parser_source(&filename) {
            Ok(source) => source,
            Err(err) => return (Arc::new(Mutex::new(None::<ast_File>)), Arc::new(Mutex::new(Some::<Box<dyn std::error::Error + Send + Sync>>(err)))),
        };
        match go_parser_parse_file(&filename, &source) {
            Ok(file) => (Arc::new(Mutex::new(Some::<ast_File>(file))), Arc::new(Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>))),
            Err(err) => (Arc::new(Mutex::new(None::<ast_File>)), Arc::new(Mutex::new(Some::<Box<dyn std::error::Error + Send + Sync>>(err)))),
        }
    }
}


pub mod token {
    use super::*;

    pub const A_D_D: token_Token = token_Token(12);
    pub const A_D_D__A_S_S_I_G_N: token_Token = token_Token(23);
    pub const A_N_D: token_Token = token_Token(17);
    pub const A_N_D__A_S_S_I_G_N: token_Token = token_Token(28);
    pub const A_N_D__N_O_T: token_Token = token_Token(22);
    pub const A_N_D__N_O_T__A_S_S_I_G_N: token_Token = token_Token(33);
    pub const A_R_R_O_W: token_Token = token_Token(36);
    pub const A_S_S_I_G_N: token_Token = token_Token(42);
    pub const B_R_E_A_K: token_Token = token_Token(61);
    pub const C_A_S_E: token_Token = token_Token(62);
    pub const C_H_A_N: token_Token = token_Token(63);
    pub const C_H_A_R: token_Token = token_Token(8);
    pub const C_O_L_O_N: token_Token = token_Token(58);
    pub const C_O_M_M_A: token_Token = token_Token(52);
    pub const C_O_M_M_E_N_T: token_Token = token_Token(2);
    pub const C_O_N_S_T: token_Token = token_Token(64);
    pub const C_O_N_T_I_N_U_E: token_Token = token_Token(65);
    pub const D_E_C: token_Token = token_Token(38);
    pub const D_E_F_A_U_L_T: token_Token = token_Token(66);
    pub const D_E_F_E_R: token_Token = token_Token(67);
    pub const D_E_F_I_N_E: token_Token = token_Token(47);
    pub const E_L_L_I_P_S_I_S: token_Token = token_Token(48);
    pub const E_L_S_E: token_Token = token_Token(68);
    pub const E_O_F: token_Token = token_Token(1);
    pub const E_Q_L: token_Token = token_Token(39);
    pub const F_A_L_L_T_H_R_O_U_G_H: token_Token = token_Token(69);
    pub const F_L_O_A_T: token_Token = token_Token(6);
    pub const F_O_R: token_Token = token_Token(70);
    pub const F_U_N_C: token_Token = token_Token(71);
    pub const G_E_Q: token_Token = token_Token(46);
    pub const G_O: token_Token = token_Token(72);
    pub const G_O_T_O: token_Token = token_Token(73);
    pub const G_T_R: token_Token = token_Token(41);
    pub const I_D_E_N_T: token_Token = token_Token(4);
    pub const I_F: token_Token = token_Token(74);
    pub const I_L_L_E_G_A_L: token_Token = token_Token(0);
    pub const I_M_A_G: token_Token = token_Token(7);
    pub const I_M_P_O_R_T: token_Token = token_Token(75);
    pub const I_N_C: token_Token = token_Token(37);
    pub const I_N_T: token_Token = token_Token(5);
    pub const I_N_T_E_R_F_A_C_E: token_Token = token_Token(76);
    pub const L_A_N_D: token_Token = token_Token(34);
    pub const L_B_R_A_C_E: token_Token = token_Token(51);
    pub const L_B_R_A_C_K: token_Token = token_Token(50);
    pub const L_E_Q: token_Token = token_Token(45);
    pub const L_O_R: token_Token = token_Token(35);
    pub const L_P_A_R_E_N: token_Token = token_Token(49);
    pub const L_S_S: token_Token = token_Token(40);
    pub const M_A_P: token_Token = token_Token(77);
    pub const M_U_L: token_Token = token_Token(14);
    pub const M_U_L__A_S_S_I_G_N: token_Token = token_Token(25);
    pub const N_E_Q: token_Token = token_Token(44);
    pub const N_O_T: token_Token = token_Token(43);
    pub const O_R: token_Token = token_Token(18);
    pub const O_R__A_S_S_I_G_N: token_Token = token_Token(29);
    pub const P_A_C_K_A_G_E: token_Token = token_Token(78);
    pub const P_E_R_I_O_D: token_Token = token_Token(53);
    pub const Q_U_O: token_Token = token_Token(15);
    pub const Q_U_O__A_S_S_I_G_N: token_Token = token_Token(26);
    pub const R_A_N_G_E: token_Token = token_Token(79);
    pub const R_B_R_A_C_E: token_Token = token_Token(56);
    pub const R_B_R_A_C_K: token_Token = token_Token(55);
    pub const R_E_M: token_Token = token_Token(16);
    pub const R_E_M__A_S_S_I_G_N: token_Token = token_Token(27);
    pub const R_E_T_U_R_N: token_Token = token_Token(80);
    pub const R_P_A_R_E_N: token_Token = token_Token(54);
    pub const S_E_L_E_C_T: token_Token = token_Token(81);
    pub const S_E_M_I_C_O_L_O_N: token_Token = token_Token(57);
    pub const S_H_L: token_Token = token_Token(20);
    pub const S_H_L__A_S_S_I_G_N: token_Token = token_Token(31);
    pub const S_H_R: token_Token = token_Token(21);
    pub const S_H_R__A_S_S_I_G_N: token_Token = token_Token(32);
    pub const S_T_R_I_N_G: token_Token = token_Token(9);
    pub const S_T_R_U_C_T: token_Token = token_Token(82);
    pub const S_U_B: token_Token = token_Token(13);
    pub const S_U_B__A_S_S_I_G_N: token_Token = token_Token(24);
    pub const S_W_I_T_C_H: token_Token = token_Token(83);
    pub const T_I_L_D_E: token_Token = token_Token(88);
    pub const T_Y_P_E: token_Token = token_Token(84);
    pub const V_A_R: token_Token = token_Token(85);
    pub const X_O_R: token_Token = token_Token(19);
    pub const X_O_R__A_S_S_I_G_N: token_Token = token_Token(30);

    pub fn new_file_set() -> Arc<Mutex<Option<token_FileSet>>> {
        panic!("new_file_set bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


fn main() {
    if false {
        let mut fset = token::new_file_set();
        { let (__tmp_0, __tmp_1) = parser::parse_file(fset.clone(), "a.go".to_string(), "package p; type A = int".to_string(), parser::SKIP_OBJECT_RESOLUTION); };
        let _ = binary::MAX_VARINT_LEN64;
        let mut dir = Arc::new(Mutex::new(Some(ast::S_E_N_D)));
        { let new_val = ast_ChanDir((((ast::S_E_N_D).0 as i32) | ((ast::R_E_C_V).0 as i32)) as i32); *dir.lock().unwrap() = Some(new_val); };
        let _ = { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    println!("{}", format!("{}", "ok".to_string()));
}