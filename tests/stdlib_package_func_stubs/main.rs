use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Default)]
pub struct ast_BasicLit {
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
pub struct ast_File {
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
pub struct ast_Ident {
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct parser_Mode(pub u32);

impl PartialEq<u32> for parser_Mode {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<parser_Mode> for u32 {
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
pub struct types_Alias;

impl std::fmt::Display for types_Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Alias>")
    }
}


impl types_Alias {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic;

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Basic>")
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Checker;

impl std::fmt::Display for types_Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Checker>")
    }
}


impl types_Checker {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Config;

impl std::fmt::Display for types_Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Config>")
    }
}


impl types_Config {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn check<T0, T1, T2, T3>(&self, _arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> (Arc<Mutex<Option<types_Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<types_Package>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Info;

impl std::fmt::Display for types_Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Info>")
    }
}


impl types_Info {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl types_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
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

    pub const R_E_C_V: ast_ChanDir = ast_ChanDir(0);
    pub const S_E_N_D: ast_ChanDir = ast_ChanDir(0);
}


pub mod binary {
    use super::*;
    pub const MAX_VARINT_LEN64: i32 = 0;
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

    pub const SKIP_OBJECT_RESOLUTION: parser_Mode = parser_Mode(0);

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

    fn go_parser_import_spec(name: Option<String>, path: String) -> Arc<Mutex<Option<ast_ImportSpec>>> {
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
            return Some((go_parser_import_spec(None, tokens[start].clone()), start + 1));
        }
        if start + 1 < tokens.len() && go_parser_is_string_lit(&tokens[start + 1]) {
            return Some((go_parser_import_spec(Some(tokens[start].clone()), tokens[start + 1].clone()), start + 2));
        }
        None
    }

    fn go_parser_parse_file(source: &str) -> Result<ast_File, Box<dyn std::error::Error + Send + Sync>> {
        let tokens = go_parser_tokens(source);
        let package_name = tokens
            .windows(2)
            .find_map(|pair| if pair[0] == "package" { Some(pair[1].clone()) } else { None })
            .ok_or_else(|| go_parser_error("missing package clause".to_string()))?;
        let mut imports = Vec::new();
        let mut i = 0usize;
        while i < tokens.len() {
            if tokens[i] != "import" {
                i += 1;
                continue;
            }
            i += 1;
            if i < tokens.len() && tokens[i] == "(" {
                i += 1;
                while i < tokens.len() && tokens[i] != ")" {
                    if let Some((spec, next)) = go_parser_import_from_tokens(&tokens, i) {
                        imports.push(spec);
                        i = next;
                    } else {
                        i += 1;
                    }
                }
                if i < tokens.len() && tokens[i] == ")" {
                    i += 1;
                }
                continue;
            }
            if let Some((spec, next)) = go_parser_import_from_tokens(&tokens, i) {
                imports.push(spec);
                i = next;
            }
        }
        Ok(ast_File {
            imports: Arc::new(Mutex::new(Some::<Vec<Arc<Mutex<Option<ast_ImportSpec>>>>>(imports))),
            name: go_parser_ident(package_name),
            ..Default::default()
        })
    }

    pub fn parse_file<T0, T1: GoParserFilenameArg, T2: GoParserSourceArg, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> (Arc<Mutex<Option<ast_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let filename = _arg1.into_go_parser_filename();
        let source = match _arg2.into_go_parser_source(&filename) {
            Ok(source) => source,
            Err(err) => return (Arc::new(Mutex::new(None::<ast_File>)), Arc::new(Mutex::new(Some::<Box<dyn std::error::Error + Send + Sync>>(err)))),
        };
        match go_parser_parse_file(&source) {
            Ok(file) => (Arc::new(Mutex::new(Some::<ast_File>(file))), Arc::new(Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>))),
            Err(err) => (Arc::new(Mutex::new(None::<ast_File>)), Arc::new(Mutex::new(Some::<Box<dyn std::error::Error + Send + Sync>>(err)))),
        }
    }
}


pub mod token {
    use super::*;
    pub fn new_file_set() -> Arc<Mutex<Option<token_FileSet>>> {
        Arc::new(Mutex::new(Some::<token_FileSet>(Default::default())))
    }
}


pub mod types {
    use super::*;
    pub fn Typ() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<types_Basic>>>>>>> {
        Arc::new(Mutex::new(Some::<Vec<Arc<Mutex<Option<types_Basic>>>>>(Default::default())))
    }

    pub fn new_checker<T0, T1, T2, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> Arc<Mutex<Option<types_Checker>>> {
        Arc::new(Mutex::new(Some::<types_Checker>(Default::default())))
    }

    pub fn new_package<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<types_Package>>> {
        Arc::new(Mutex::new(Some::<types_Package>(Default::default())))
    }

    pub fn unalias<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Type>>> {
        Arc::new(Mutex::new(Some::<types_Type>(Default::default())))
    }
}


fn main() {
    if false {
        let mut fset = token::new_file_set();
        let (mut f, _) = parser::parse_file(fset.clone(), "a.go".to_string(), "package p; type A = int".to_string(), parser::SKIP_OBJECT_RESOLUTION);
        { let (__tmp_0, __tmp_1) = { let __recv = Arc::new(Mutex::new(Some(types_Config::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).check("p".to_string(), fset.clone(), Arc::new(Mutex::new(Some(vec![f.clone()]))), Arc::new(Mutex::new(Some(types_Info::default())))); __result }; };
        let _ = types::new_checker(Arc::new(Mutex::new(Some(types_Config::default()))), fset.clone(), types::new_package("p".to_string(), "p".to_string()), Arc::new(Mutex::new(Some(types_Info::default()))));
        let mut alias: Arc<Mutex<Option<types_Alias>>> = Arc::new(Mutex::new(None));
        let _ = types::unalias(alias.clone());
        let _ = binary::MAX_VARINT_LEN64;
        let _ = types::Typ();
        let mut dir = Arc::new(Mutex::new(Some(ast::S_E_N_D)));
        { let new_val = { let __tmp_x = ast_ChanDir(ast::S_E_N_D.0 as i32); let __tmp_y = ast_ChanDir(ast::R_E_C_V.0 as i32); __tmp_x | __tmp_y }; *dir.lock().unwrap() = Some(new_val); };
        let _ = { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    println!("{}", format!("{}", "ok".to_string()));
}