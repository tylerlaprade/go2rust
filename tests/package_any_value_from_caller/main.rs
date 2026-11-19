use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display};
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

fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else {
        "<unknown>".to_string()
    }
}

/// A value put into an interface does not stay in the package that put it
/// there. main creates these values; another package prints and copies them.
fn main() {
    ::example_com_anyfromcaller_logger::__go_init_all();

    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({
        let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("bad".to_string()))))).clone();
        let __err_guard = __err_holder.borrow();
        match __err_guard.as_ref() {
            None => panic!("nil error-to-any lowering requires nil interface representation"),
            Some(__err) => {
                if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else {
                    Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any>
                }
            }
        }
    }))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![{
        let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("later".to_string()))))).clone();
        let __err_guard = __err_holder.borrow();
        match __err_guard.as_ref() {
            None => panic!("nil error-to-any lowering requires nil interface representation"),
            Some(__err) => {
                if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else {
                    Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any>
                }
            }
        }
    }]))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![{
        let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("wrapped {}", 7)))))).clone();
        let __err_guard = __err_holder.borrow();
        match __err_guard.as_ref() {
            None => panic!("nil error-to-any lowering requires nil interface representation"),
            Some(__err) => {
                if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else {
                    Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any>
                }
            }
        }
    }, Box::new("text".to_string()) as Box<dyn Any>, Box::new(3) as Box<dyn Any>]))));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some({
        let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("described".to_string()))))).clone();
        let __err_guard = __err_holder.borrow();
        match __err_guard.as_ref() {
            None => panic!("nil error-to-any lowering requires nil interface representation"),
            Some(__err) => {
                if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() {
                    go_box_any_with_metadata(typed_val.clone(), "pointer", true)
                } else {
                    Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any>
                }
            }
        }
    })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some(Box::new(3) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1, 2]))).clone()) as Box<dyn Any>))));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any>))));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(1))))])))).clone()) as Box<dyn Any>))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![3, 4]))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("m".to_string(), Rc::new(RefCell::new(Some(2))))])))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1.5]))).clone()) as Box<dyn Any>]))));
    println!("{} {}", format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["x".to_string()]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    let mut numbers: Rc<RefCell<Option<Box<dyn Any>>>> = example_com_anyfromcaller_logger::numbers().clone();
    println!("{}", format!("{}", format_any(numbers.borrow().as_ref().unwrap().as_ref())));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(7) as Box<dyn Any>))));
}