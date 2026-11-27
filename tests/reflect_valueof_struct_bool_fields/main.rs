#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    go_panic_value_text(__payload)
}

fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    // Go prints a float in a panic as +1.500000e+000; a float32 is widened
    // to float64 first, as Go's print does.
    let __float = __payload
        .downcast_ref::<f64>()
        .copied()
        .or_else(|| __payload.downcast_ref::<f32>().map(|__value| *__value as f64));
    if let Some(__value) = __float {
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if __value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}

#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Arc<Mutex<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let raw = (*self.raw.lock().unwrap().as_ref().unwrap()).clone();
        let key = (*key.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Arc<Mutex<Option<String>>>,
    tag: Arc<Mutex<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Arc<Mutex<Option<String>>>,
    kind: Arc<Mutex<Option<reflect_Kind>>>,
    elem: Arc<Mutex<Option<Box<GoReflectType>>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.lock().unwrap() == *other.name.lock().unwrap() &&
            *self.kind.lock().unwrap() == *other.kind.lock().unwrap()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.name.lock().unwrap().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Arc<Mutex<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        let elem_guard = self.elem.lock().unwrap();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Arc::new(Mutex::new(Some(elem)))
    }

    fn num_field(&self) -> i64 {
        self.fields.lock().unwrap().as_ref().unwrap().len() as i64
    }

    fn field(&self, index: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<GoReflectField>>> {
        let index = *index.lock().unwrap().as_ref().unwrap() as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool + Send + Sync>;
type GoReflectBoolSetter = Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Arc<Mutex<Option<GoReflectType>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectValue>>>>,
    bool_getter: Arc<Mutex<Option<GoReflectBoolGetter>>>,
    bool_setter: Arc<Mutex<Option<GoReflectBoolSetter>>>,
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Arc<Mutex<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn r#type(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Arc<Mutex<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.lock().unwrap().as_ref().unwrap().kind()
    }

    fn field(&self, index: i64) -> Arc<Mutex<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Arc<Mutex<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.lock().unwrap();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
        let getter_guard = self.bool_getter.lock().unwrap();
        let getter = getter_guard.as_ref().expect("reflect.Value.Bool requires a bool field");
        getter()
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub alpha: Arc<Mutex<Option<bool>>>,
    pub beta: Arc<Mutex<Option<bool>>>,
}

impl Flags {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.alpha.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.beta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            alpha: __go_clone_0_0,
            beta: __go_clone_1_0,
        }
    }
}


impl Default for Flags {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            alpha: __go_default_0_0,
            beta: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.alpha.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.beta.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut flags = Arc::new(Mutex::new(Some(Flags { alpha: Arc::new(Mutex::new(Some(false))), beta: Arc::new(Mutex::new(Some(false))) })));
    let mut names = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>>>>>::from([]))));
    let mut rv = {
        let __recv = { let __reflect_target = flags.clone(); Arc::new(Mutex::new(Some(GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("main.Flags".to_string()))), kind: Arc::new(Mutex::new(Some(reflect_Kind(25)))), elem: Arc::new(Mutex::new(None::<Box<GoReflectType>>)), fields: Arc::new(Mutex::new(Some(vec![GoReflectField { name: Arc::new(Mutex::new(Some("Alpha".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("Beta".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }]))) }))), fields: Arc::new(Mutex::new(Some(vec![GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), kind: Arc::new(Mutex::new(Some(reflect_Kind(1)))), elem: Arc::new(Mutex::new(None::<Box<GoReflectType>>)), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.alpha.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.alpha.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))), unsupported: None }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), kind: Arc::new(Mutex::new(Some(reflect_Kind(1)))), elem: Arc::new(Mutex::new(None::<Box<GoReflectType>>)), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.beta.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.beta.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))), unsupported: None }]))), bool_getter: Arc::new(Mutex::new(None)), bool_setter: Arc::new(Mutex::new(None)), unsupported: None }))) };
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).elem();
        __result
    };
    let mut rt = (*rv.lock().unwrap().as_ref().unwrap()).r#type();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*rt.lock().unwrap().as_ref().unwrap()).num_field(); __tmp_x < __tmp_y } {
        let mut field = (*rv.lock().unwrap().as_ref().unwrap()).field({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        { let __map_key = { let __map_key_holder = strings::to_lower(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*rt.lock().unwrap().as_ref().unwrap()).field(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); std::mem::drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some({ let mut __recv = ({ let __v = (*field.lock().unwrap().as_ref().unwrap()).clone(); __v }); Arc::new(move |__arg0: Arc<Mutex<Option<bool>>>| { let mut __recv = __recv.clone(); __recv.set_bool(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }))); (*names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __f_holder = { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"alpha".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f: Arc<dyn Fn(Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(true)))) };
    println!("{} {}", format!("{}", (*{ let __field = (*flags.lock().unwrap().as_ref().unwrap()).alpha.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*flags.lock().unwrap().as_ref().unwrap()).beta.clone(); __field }.lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for Flags {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
