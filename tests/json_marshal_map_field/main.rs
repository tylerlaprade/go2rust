use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}

fn go_json_escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c < ' ' => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}

#[derive(Debug, Clone, Default)]
pub struct OverlayJSON {
    // tags: `json:"replace,omitempty"`
    pub replace: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>,
}

impl OverlayJSON {
    pub fn __go_value_clone(&self) -> Self {
        Self { replace: self.replace.clone() }
    }
}

impl std::fmt::Display for OverlayJSON {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_map(&self.replace))
    }
}


fn main() {
    let mut overlays = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("b.go".to_string(), Rc::new(RefCell::new(Some("tmp-b".to_string())))), ("a.go".to_string(), Rc::new(RefCell::new(Some("tmp-a".to_string()))))]))));

    let (mut data, _) = { let __json_value = OverlayJSON { replace: overlays.clone(), ..Default::default() }; let mut __json_fields: Vec<String> = Vec::new(); { let __map_guard = __json_value.replace.borrow(); if let Some(__map) = __map_guard.as_ref() { if !__map.is_empty() { let __map_entries = __map.iter().map(|(__k, __v)| { let __v_guard = __v.borrow(); format!("\"{}\":\"{}\"", go_json_escape(__k), go_json_escape(__v_guard.as_ref().unwrap())) }).collect::<Vec<_>>().join(","); __json_fields.push(format!("\"replace\":{{{}}}", __map_entries)); } } } let __json = format!("{{{}}}", __json_fields.join(",")); (Rc::new(RefCell::new(Some(__json.into_bytes()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))) };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*data.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));

    let (mut empty, _) = { let __json_value = OverlayJSON { replace: Rc::new(RefCell::new(Some(BTreeMap::new()))) }; let mut __json_fields: Vec<String> = Vec::new(); { let __map_guard = __json_value.replace.borrow(); if let Some(__map) = __map_guard.as_ref() { if !__map.is_empty() { let __map_entries = __map.iter().map(|(__k, __v)| { let __v_guard = __v.borrow(); format!("\"{}\":\"{}\"", go_json_escape(__k), go_json_escape(__v_guard.as_ref().unwrap())) }).collect::<Vec<_>>().join(","); __json_fields.push(format!("\"replace\":{{{}}}", __map_entries)); } } } let __json = format!("{{{}}}", __json_fields.join(",")); (Rc::new(RefCell::new(Some(__json.into_bytes()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))) };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*empty.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}