use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

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
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>>;
    }

    impl GoParserSourceArg for () {
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            std::fs::read_to_string(filename).map_err(|err| Box::new(err) as Box<dyn StdError + Send + Sync>)
        }
    }

    impl GoParserSourceArg for String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self)
        }
    }

    impl<'a> GoParserSourceArg for &'a str {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self.to_string())
        }
    }

    impl<'a> GoParserSourceArg for &'a String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self.clone())
        }
    }

    impl GoParserSourceArg for Vec<u8> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            String::from_utf8(self).map_err(|err| Box::new(err) as Box<dyn StdError + Send + Sync>)
        }
    }

    impl GoParserSourceArg for Arc<Mutex<Option<String>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self.lock().unwrap().as_ref().cloned().unwrap_or_default())
        }
    }

    impl GoParserSourceArg for Arc<Mutex<Option<Vec<u8>>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            let bytes = self.lock().unwrap().as_ref().cloned().unwrap_or_default();
            String::from_utf8(bytes).map_err(|err| Box::new(err) as Box<dyn StdError + Send + Sync>)
        }
    }

    pub const IMPORTS_ONLY: parser_Mode = parser_Mode(0);

    fn go_parser_error(message: String) -> Box<dyn StdError + Send + Sync> {
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

    fn go_parser_parse_file(source: &str) -> Result<ast_File, Box<dyn StdError + Send + Sync>> {
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
            Err(err) => return (Arc::new(Mutex::new(None::<ast_File>)), Arc::new(Mutex::new(Some::<Box<dyn StdError + Send + Sync>>(err)))),
        };
        match go_parser_parse_file(&source) {
            Ok(file) => (Arc::new(Mutex::new(Some::<ast_File>(file))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))),
            Err(err) => (Arc::new(Mutex::new(None::<ast_File>)), Arc::new(Mutex::new(Some::<Box<dyn StdError + Send + Sync>>(err)))),
        }
    }
}


pub mod token {
    use super::*;
    pub fn new_file_set() -> Arc<Mutex<Option<token_FileSet>>> {
        Arc::new(Mutex::new(Some::<token_FileSet>(Default::default())))
    }
}


fn main() {
    let mut fset = token::new_file_set();
    let (mut file, mut err) = parser::parse_file(fset.clone(), "input.go".to_string(), "package main\n\nimport (\n\t\"fmt\"\n\talias \"strings\"\n\t_ \"os\"\n)\n".to_string(), parser::IMPORTS_ONLY);
    println!("{} {} {}", format!("{}", (*err.lock().unwrap()).is_none()), format!("{}", (*(*(*file.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*file.lock().unwrap().as_ref().unwrap()).imports.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    println!("{}", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
}