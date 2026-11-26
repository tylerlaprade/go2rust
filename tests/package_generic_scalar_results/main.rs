#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>, nil_text: &str) -> String
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
                    format!("{}:{}", go_display_value(k), go_display_value(val))
                } else {
                    format!("{}:{}", go_display_value(k), nil_text)
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
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
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

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

pub fn double(n: Rc<RefCell<Option<i64>>>) -> i64 {
    (*n.borrow().as_ref().unwrap()) * 2
}

fn main() {
    ::example_com_packagegenericscalarresults_dep::__go_init_all();

    let mut x = example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(7))));
    let mut y = Rc::new(RefCell::new(Some((*example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(8)))).borrow().as_ref().unwrap()) + 1)));
    let mut z: Rc<RefCell<Option<i64>>> = example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(10))));
    println!("{} {} {} {} {} {}", format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*z.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*x.borrow().as_ref().unwrap()) + (*z.borrow().as_ref().unwrap())), format!("{}", (*example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(9)))).borrow().as_ref().unwrap())), format!("{}", (*example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(6)))).borrow().as_ref().unwrap())));
    let mut f = Rc::new(RefCell::new(Some((*example_com_packagegenericscalarresults_dep::pick::<f64>(Rc::new(RefCell::new(Some(1.5)))).borrow().as_ref().unwrap()) * 2.0)));
    let mut ok = Rc::new(RefCell::new(Some((*example_com_packagegenericscalarresults_dep::pick::<bool>(Rc::new(RefCell::new(Some(true)))).borrow().as_ref().unwrap()) && !((*example_com_packagegenericscalarresults_dep::pick::<bool>(Rc::new(RefCell::new(Some(false)))).borrow().as_ref().unwrap())))));
    println!("{} {} {} {}", { let __text = go_float_string(({ let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }) as f64, false); __text }, format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", double(example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(4)))))), format!("{}", example_com_packagegenericscalarresults_dep::size::<String>(Rc::new(RefCell::new(Some("s".to_string())))) + 1));
    let (__tmp_0, __tmp_1) = example_com_packagegenericscalarresults_dep::pair::<i64, f64>(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(2.5)))); let mut a = { let __tmp_holder = __tmp_0.clone(); let __tmp_guard = __tmp_holder.borrow(); (*__tmp_guard.as_ref().unwrap()).clone() }; let mut b = { let __tmp_holder = __tmp_1.clone(); let __tmp_guard = __tmp_holder.borrow(); (*__tmp_guard.as_ref().unwrap()).clone() };;
    println!("{} {}", format!("{}", a + 1), { let __text = go_float_string((b * 2.0) as f64, false); __text });
    { let (__tmp_0, __tmp_1) = example_com_packagegenericscalarresults_dep::pair::<i64, f64>(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(0.5)))); a = { let __tmp_holder = __tmp_0.clone(); let __tmp_guard = __tmp_holder.borrow(); (*__tmp_guard.as_ref().unwrap()).clone() }; b = { let __tmp_holder = __tmp_1.clone(); let __tmp_guard = __tmp_holder.borrow(); (*__tmp_guard.as_ref().unwrap()).clone() }; };
    println!("{} {}", format!("{}", a), { let __text = go_float_string((b) as f64, false); __text });
    { let new_val = example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(11)))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *z.borrow_mut() = __moved_val; };
    { let __rhs = (*example_com_packagegenericscalarresults_dep::max::<i64>(2, 12).borrow().as_ref().unwrap()); let mut guard = z.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {}", format!("{}", { let __v = (*z.borrow().as_ref().unwrap()).clone(); __v }), { let __text = go_float_string(((*example_com_packagegenericscalarresults_dep::max::<f64>(1.5, -(2.0)).borrow().as_ref().unwrap())) as f64, false); __text }, format!("{}", example_com_packagegenericscalarresults_dep::larger::<i64>(3, 2)));
    if example_com_packagegenericscalarresults_dep::larger::<f64>(2.5, 1.0) && (*example_com_packagegenericscalarresults_dep::max::<i64>(3, 4).borrow().as_ref().unwrap()) == 4 {
        println!("{}", format!("{}", "compared".to_string()));
    }
    let mut nums = Rc::new(RefCell::new(Some(vec![{ let __v = example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(1)))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }, { let __v = example_com_packagegenericscalarresults_dep::max::<i64>(2, 1); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }])));
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<f64>>>>::from([("k".to_string(), example_com_packagegenericscalarresults_dep::max::<f64>(0.5, 0.25))]))));
    { let __go_print_arg_1 = Rc::new(RefCell::new(Some((({ let __len = (*nums.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) + ((*example_com_packagegenericscalarresults_dep::pick::<i64>(Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()) as i64)))); let __go_print_value = println!("{} {} {}", format!("{}", format_slice(&nums)), format!("{}", format_map(&m)), format!("{}", { let __v = (*__go_print_arg_1.borrow().as_ref().unwrap()).clone(); __v })); __go_print_value };
}