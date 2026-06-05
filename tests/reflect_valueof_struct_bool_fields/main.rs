use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Rc<RefCell<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let raw = (*self.raw.borrow().as_ref().unwrap()).clone();
        let key = (*key.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Rc<RefCell<Option<String>>>,
    tag: Rc<RefCell<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Rc<RefCell<Option<String>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn num_field(&self) -> i32 {
        self.fields.borrow().as_ref().unwrap().len() as i32
    }

    fn field(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<GoReflectField>>> {
        let index = *index.borrow().as_ref().unwrap() as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool>;
type GoReflectBoolSetter = Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Rc<RefCell<Option<GoReflectType>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectValue>>>>,
    bool_getter: Rc<RefCell<Option<GoReflectBoolGetter>>>,
    bool_setter: Rc<RefCell<Option<GoReflectBoolSetter>>>,
}

impl GoReflectValue {
    fn elem(&self) -> Rc<RefCell<Option<GoReflectValue>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn r#type(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        self.typ.clone()
    }

    fn field(&self, index: i32) -> Rc<RefCell<Option<GoReflectValue>>> {
        let index = index as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }

    fn set_bool(&mut self, value: Rc<RefCell<Option<bool>>>) {
        let mut setter_guard = self.bool_setter.borrow_mut();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        let getter_guard = self.bool_getter.borrow();
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

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct reflect_Type {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl reflect_Type {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for reflect_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for reflect_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Type>")
    }
}

impl std::fmt::Display for reflect_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Type>")
    }
}

impl PartialEq for reflect_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for reflect_Type {}

impl PartialOrd for reflect_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for reflect_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Value;

impl std::fmt::Display for reflect_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Value>")
    }
}


impl reflect_Value {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn elem(&self) -> Rc<RefCell<Option<reflect_Value>>> {
        panic!("reflect_Value.elem bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn field<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<reflect_Value>>> {
        panic!("reflect_Value.field bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
    pub fn r#type(&self) -> Rc<RefCell<Option<reflect_Type>>> {
        panic!("reflect_Value.r#type bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct Flags {
    pub alpha: Rc<RefCell<Option<bool>>>,
    pub beta: Rc<RefCell<Option<bool>>>,
}

impl Flags {
    pub fn __go_value_clone(&self) -> Self {
        Self { alpha: { let __guard = self.alpha.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, beta: { let __guard = self.beta.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Flags {
    fn default() -> Self {
        Self { alpha: Rc::new(RefCell::new(Some(false))), beta: Rc::new(RefCell::new(Some(false))) }
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.alpha.borrow().as_ref().unwrap()), (*self.beta.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut flags = Rc::new(RefCell::new(Some(Flags { alpha: Rc::new(RefCell::new(Some(false))), beta: Rc::new(RefCell::new(Some(false))) })));
    let mut names = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()>>>>>::from([]))));

    let mut rv = { let __recv = { let __reflect_target = flags.clone(); Rc::new(RefCell::new(Some(GoReflectValue { typ: Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("main.Flags".to_string()))), fields: Rc::new(RefCell::new(Some(vec![GoReflectField { name: Rc::new(RefCell::new(Some("Alpha".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("Beta".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("".to_string()))) }))) }]))) }))), fields: Rc::new(RefCell::new(Some(vec![GoReflectValue { typ: Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("bool".to_string()))), fields: Rc::new(RefCell::new(Some(vec![]))) }))), fields: Rc::new(RefCell::new(Some(vec![]))), bool_getter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.borrow(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.alpha.borrow(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Rc<RefCell<Option<bool>>>| { let __new_value = (*__value.borrow().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.borrow_mut(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.alpha.borrow_mut() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("bool".to_string()))), fields: Rc::new(RefCell::new(Some(vec![]))) }))), fields: Rc::new(RefCell::new(Some(vec![]))), bool_getter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.borrow(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.beta.borrow(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Rc::new(RefCell::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Rc<RefCell<Option<bool>>>| { let __new_value = (*__value.borrow().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.borrow_mut(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.beta.borrow_mut() = Some(__new_value); }) as GoReflectBoolSetter }))) }]))), bool_getter: Rc::new(RefCell::new(None)), bool_setter: Rc::new(RefCell::new(None)) }))) }; let __result = (*__recv.borrow().as_ref().unwrap()).elem(); __result };
    let mut rt = (*rv.borrow().as_ref().unwrap()).r#type();
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < (*rt.borrow().as_ref().unwrap()).num_field() {
        let mut field = (*rv.borrow().as_ref().unwrap()).field({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() });
        { let __map_key = { let __map_key_holder = Rc::new(RefCell::new(Some({ let __s = (*(*(*rt.borrow().as_ref().unwrap()).field(Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(); __s.to_lowercase() }))).clone(); let __map_key_guard = __map_key_holder.borrow(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = Rc::new(RefCell::new(Some({ let mut __recv = (*field.borrow().as_ref().unwrap()).clone(); Box::new(move |__arg0: Rc<RefCell<Option<bool>>>| { __recv.set_bool(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()> }))); (*names.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    { let __f_holder = { let __map_holder = names.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"alpha".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(true)))) };
    println!("{} {}", format!("{}", (*(*flags.borrow().as_ref().unwrap()).alpha.borrow().as_ref().unwrap())), format!("{}", (*(*flags.borrow().as_ref().unwrap()).beta.borrow().as_ref().unwrap())));
}