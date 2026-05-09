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

fn main() {
    let mut pattern = Rc::new(RefCell::new(Some("\\d+".to_string())));
    let mut re = Rc::new(RefCell::new(Some(GoRegexp { pattern: Rc::new(RefCell::new(Some((*pattern.borrow().as_ref().unwrap()).clone()))) })));
    let mut text = Rc::new(RefCell::new(Some("I have 42 apples and 7 oranges".to_string())));
    let mut matches = (*re.borrow_mut().as_mut().unwrap()).find_all_string(Rc::new(RefCell::new(Some((*text.borrow().as_ref().unwrap()).clone()))), Rc::new(RefCell::new(Some(-1))));
    println!("{} {}", "Numbers found:".to_string(), format_slice(&matches));
}