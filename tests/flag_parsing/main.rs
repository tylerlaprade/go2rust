use std::cell::{RefCell};
use std::rc::{Rc};

pub mod flag {
    use super::*;

    type StringFlag = Rc<RefCell<Option<String>>>;
    type BoolFlag = Rc<RefCell<Option<bool>>>;

    thread_local! {
        static STRING_FLAGS: std::cell::RefCell<Vec<(String, StringFlag)>> = std::cell::RefCell::new(Vec::new());
        static BOOL_FLAGS: std::cell::RefCell<Vec<(String, BoolFlag)>> = std::cell::RefCell::new(Vec::new());
        static REMAINING_ARGS: std::cell::RefCell<Option<Vec<String>>> = std::cell::RefCell::new(None);
    }

    pub fn string<T0: Into<String>, T1: Into<String>, T2>(_name: T0, value: T1, _usage: T2) -> StringFlag {
        let name = _name.into();
        let handle = Rc::new(RefCell::new(Some(value.into())));
        STRING_FLAGS.with(|flags| flags.borrow_mut().push((name, handle.clone())));
        handle
    }

    pub fn bool<T0: Into<String>, T2>(_name: T0, value: bool, _usage: T2) -> BoolFlag {
        let name = _name.into();
        let handle = Rc::new(RefCell::new(Some(value)));
        BOOL_FLAGS.with(|flags| flags.borrow_mut().push((name, handle.clone())));
        handle
    }

    pub fn parse() {
        let argv: Vec<String> = std::env::args().skip(1).collect();
        let mut remaining = Vec::new();
        let mut index = 0usize;
        while index < argv.len() {
            let arg = argv[index].clone();
            if arg == "--" {
                remaining.extend(argv[index + 1..].iter().cloned());
                break;
            }
            if !arg.starts_with('-') || arg == "-" {
                remaining.push(arg);
                index += 1;
                continue;
            }

            let flag_text = arg.trim_start_matches('-');
            let (name, inline_value) = match flag_text.split_once('=') {
                Some((name, value)) => (name.to_string(), Some(value.to_string())),
                None => (flag_text.to_string(), None),
            };

            if set_bool_flag(&name, inline_value.as_deref().map(parse_bool_value).unwrap_or(true)) {
                index += 1;
                continue;
            }

            if has_string_flag(&name) {
                let value = if let Some(value) = inline_value {
                    value
                } else if index + 1 < argv.len() {
                    index += 1;
                    argv[index].clone()
                } else {
                    String::new()
                };
                set_string_flag(&name, value);
                index += 1;
                continue;
            }

            remaining.push(arg);
            index += 1;
        }

        REMAINING_ARGS.with(|args| *args.borrow_mut() = Some(remaining));
    }

    pub fn args() -> Rc<RefCell<Option<Vec<String>>>> {
        let needs_parse = REMAINING_ARGS.with(|args| args.borrow().is_none());
        if needs_parse {
            parse();
        }
        Rc::new(RefCell::new(Some(REMAINING_ARGS.with(|args| args.borrow().as_ref().cloned().unwrap_or_default()))))
    }

    fn has_string_flag(name: &str) -> bool {
        STRING_FLAGS.with(|flags| flags.borrow().iter().any(|(flag_name, _)| flag_name == name))
    }

    fn set_string_flag(name: &str, value: String) -> bool {
        let mut found = false;
        STRING_FLAGS.with(|flags| {
            for (flag_name, target) in flags.borrow().iter() {
                if flag_name == name {
                    *target.borrow_mut() = Some(value.clone());
                    found = true;
                }
            }
        });
        found
    }

    fn set_bool_flag(name: &str, value: bool) -> bool {
        let mut found = false;
        BOOL_FLAGS.with(|flags| {
            for (flag_name, target) in flags.borrow().iter() {
                if flag_name == name {
                    *target.borrow_mut() = Some(value);
                    found = true;
                }
            }
        });
        found
    }

    fn parse_bool_value(value: &str) -> bool {
        matches!(value, "1" | "t" | "T" | "true" | "TRUE" | "True" | "y" | "yes" | "on")
    }
}


fn main() {
    let mut name = flag::string("name".to_string(), "World".to_string(), "a name to say hello to".to_string());
    flag::parse();
    print!("Hello {}!\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v });
}