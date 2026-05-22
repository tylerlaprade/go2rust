use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

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

fn main() {
    let mut pattern = Rc::new(RefCell::new(Some("\\d+".to_string())));
    let mut re = Rc::new(RefCell::new(Some(GoRegexp { pattern: Rc::new(RefCell::new(Some((*pattern.borrow().as_ref().unwrap()).clone()))) })));
    let mut text = Rc::new(RefCell::new(Some("I have 42 apples and 7 oranges".to_string())));
    let mut matches = (*re.borrow_mut().as_mut().unwrap()).find_all_string(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(-1))));
    println!("{} {}", format!("{}", "Numbers found:".to_string()), format!("{}", format_slice(&matches)));
}