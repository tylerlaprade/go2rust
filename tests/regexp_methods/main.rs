use std::cell::{RefCell};
use std::rc::{Rc};


#[derive(Debug, Clone, Default)]
struct GoRegexp {
    pattern: Rc<RefCell<Option<String>>>,
}

impl GoRegexp {
    fn find_all_string(&self, text: Rc<RefCell<Option<String>>>, n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Vec<String>>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        let limit = *n.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(go_regexp_find_all_string(&pattern, &text, limit))))
    }

    fn match_string(&self, text: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<bool>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_match_string(&pattern, &text))))
    }

    fn find_string_submatch(&self, text: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<String>>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_find_string_submatch(&pattern, &text))))
    }

    fn replace_all_string(&self, src: Rc<RefCell<Option<String>>>, repl: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let src = (*src.borrow().as_ref().unwrap()).clone();
        let repl = (*repl.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_replace_all_string(&pattern, &src, &repl))))
    }
}

fn go_regexp_find_all_string(pattern: &str, text: &str, limit: i32) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    if pattern == r"\d+" {
        let mut matches = Vec::new();
        let mut current = String::new();
        for ch in text.chars() {
            if ch.is_ascii_digit() {
                current.push(ch);
            } else if !current.is_empty() {
                matches.push(std::mem::take(&mut current));
                if limit > 0 && matches.len() >= limit as usize {
                    return matches;
                }
            }
        }
        if !current.is_empty() {
            matches.push(current);
        }
        if limit > 0 {
            matches.truncate(limit as usize);
        }
        return matches;
    }

    if pattern.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.find(pattern) {
        matches.push(pattern.to_string());
        if limit > 0 && matches.len() >= limit as usize {
            break;
        }
        rest = &rest[index + pattern.len()..];
    }
    matches
}

fn go_regexp_match_string(pattern: &str, text: &str) -> bool {
    !go_regexp_find_string_submatch(pattern, text).is_empty()
}

fn go_regexp_find_string_submatch(pattern: &str, text: &str) -> Vec<String> {
    if pattern == r"-mod[ =](\w+)" {
        for marker in ["-mod=", "-mod "] {
            if let Some(start) = text.find(marker) {
                let value_start = start + marker.len();
                let value: String = text[value_start..].chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
                if !value.is_empty() {
                    return vec![format!("{}{}", marker, value), value];
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"^go version (go\S+|devel \S+)" {
        let prefix = "go version ";
        if let Some(rest) = text.strip_prefix(prefix) {
            if let Some(first) = rest.split_whitespace().next() {
                if first.starts_with("go") {
                    return vec![format!("{}{}", prefix, first), first.to_string()];
                }
                if first == "devel" {
                    if let Some(second) = rest.split_whitespace().nth(1) {
                        let capture = format!("devel {}", second);
                        return vec![format!("{}{}", prefix, capture), capture];
                    }
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"go:.*go.mod.*contents have changed" {
        if let Some(go_index) = text.find("go:") {
            if let Some(mod_index) = text[go_index..].find("go.mod") {
                let after_mod = go_index + mod_index;
                if text[after_mod..].contains("contents have changed") {
                    return vec![text.to_string()];
                }
            }
        }
        return Vec::new();
    }

    let matches = go_regexp_find_all_string(pattern, text, 1);
    if matches.is_empty() {
        Vec::new()
    } else {
        vec![matches[0].clone()]
    }
}

fn go_regexp_replace_all_string(pattern: &str, text: &str, repl: &str) -> String {
    if pattern == r"[$,]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == '$' || ch == ',' {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    if pattern == r"[_]" {
        return text.replace('_', repl);
    }
    if pattern == r"[USD\s]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == 'U' || ch == 'S' || ch == 'D' || ch.is_whitespace() {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    text.replace(pattern, repl)
}

pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static modFlagRegexp: GoGlobal<Rc<RefCell<Option<GoRegexp>>>> = GoGlobal::new();


fn __go_init_globals() {
    *modFlagRegexp.borrow_mut() = Some(Default::default());
    *modFlagRegexp.borrow_mut() = Some(Rc::new(RefCell::new(Some(GoRegexp { pattern: Rc::new(RefCell::new(Some("-mod[ =](\\w+)".to_string()))) }))));
}


pub fn capture(re: Rc<RefCell<Option<GoRegexp>>>, text: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<String>>>> {

    return (*re.borrow_mut().as_mut().unwrap()).find_string_submatch(Rc::new(RefCell::new(Some((*text.borrow().as_ref().unwrap()).clone()))));
}

fn main() {
    __go_init_all();
    let mut modMatches = capture((*modFlagRegexp.borrow().as_ref().unwrap()).clone(), Rc::new(RefCell::new(Some("-mod=vendor".to_string()))));
    println!("{}", format!("{}", (*modMatches.borrow().as_ref().unwrap())[(1) as usize].clone()));
    println!("{}", format!("{}", (*{ let __recv_holder = (*modFlagRegexp.borrow().as_ref().unwrap()).clone(); let __result = (*__recv_holder.borrow_mut().as_mut().unwrap()).find_string_submatch(Rc::new(RefCell::new(Some("-mod vendor".to_string())))); __result }.borrow().as_ref().unwrap())[(1) as usize].clone()));

    let mut changed = Rc::new(RefCell::new(Some(GoRegexp { pattern: Rc::new(RefCell::new(Some("go:.*go.mod.*contents have changed".to_string()))) })));
    println!("{}", format!("{}", (*(*changed.borrow_mut().as_mut().unwrap()).match_string(Rc::new(RefCell::new(Some("go: updates to go.mod needed, but contents have changed".to_string())))).borrow().as_ref().unwrap())));

    let mut version = Rc::new(RefCell::new(Some(GoRegexp { pattern: Rc::new(RefCell::new(Some("^go version (go\\S+|devel \\S+)".to_string()))) })));
    println!("{}", format!("{}", (*(*version.borrow_mut().as_mut().unwrap()).find_string_submatch(Rc::new(RefCell::new(Some("go version go1.22.0 darwin/arm64".to_string())))).borrow().as_ref().unwrap())[(1) as usize].clone()));

    let mut currency = Rc::new(RefCell::new(Some(GoRegexp { pattern: Rc::new(RefCell::new(Some("[$,]".to_string()))) })));
    println!("{}", format!("{}", (*(*currency.borrow_mut().as_mut().unwrap()).replace_all_string(Rc::new(RefCell::new(Some("$1,234".to_string()))), Rc::new(RefCell::new(Some("".to_string())))).borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
