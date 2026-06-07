use std::cell::{RefCell};
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
    kind: Rc<RefCell<Option<reflect_Kind>>>,
    elem: Rc<RefCell<Option<Box<GoReflectType>>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.borrow() == *other.name.borrow() &&
            *self.kind.borrow() == *other.kind.borrow()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        let elem_guard = self.elem.borrow();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Rc::new(RefCell::new(Some(elem)))
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
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn r#type(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.borrow().as_ref().unwrap().kind()
    }

    fn field(&self, index: i32) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Rc<RefCell<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.borrow_mut();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Kind(pub u64);

impl PartialEq<u64> for reflect_Kind {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<reflect_Kind> for u64 {
    fn eq(&self, other: &reflect_Kind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for reflect_Kind {
    type Output = reflect_Kind;
    fn bitand(self, other: Self) -> reflect_Kind {
        reflect_Kind(self.0 & other.0)
    }
}

impl std::ops::BitOr for reflect_Kind {
    type Output = reflect_Kind;
    fn bitor(self, other: Self) -> reflect_Kind {
        reflect_Kind(self.0 | other.0)
    }
}

impl std::fmt::Display for reflect_Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Kind>")
    }
}


impl reflect_Kind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct namedInt(pub Rc<RefCell<Option<i64>>>);

impl Display for namedInt {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for namedInt {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for namedInt {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for namedInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for namedInt {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<namedInt> for i64 {
    fn eq(&self, other: &namedInt) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<namedInt> for i64 {
    fn partial_cmp(&self, other: &namedInt) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for namedInt {
    type Output = namedInt;
    fn add(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for namedInt {
    type Output = namedInt;
    fn add(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<namedInt> for i64 {
    type Output = namedInt;
    fn add(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for namedInt {
    type Output = namedInt;
    fn sub(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for namedInt {
    type Output = namedInt;
    fn sub(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<namedInt> for i64 {
    type Output = namedInt;
    fn sub(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for namedInt {
    type Output = namedInt;
    fn mul(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for namedInt {
    type Output = namedInt;
    fn mul(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<namedInt> for i64 {
    type Output = namedInt;
    fn mul(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for namedInt {
    type Output = namedInt;
    fn div(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for namedInt {
    type Output = namedInt;
    fn div(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<namedInt> for i64 {
    type Output = namedInt;
    fn div(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for namedInt {
    type Output = namedInt;
    fn neg(self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for namedInt {
    type Output = namedInt;
    fn rem(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for namedInt {
    type Output = namedInt;
    fn rem(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<namedInt> for i64 {
    type Output = namedInt;
    fn rem(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for namedInt {
    type Output = namedInt;
    fn bitand(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for namedInt {
    type Output = namedInt;
    fn bitand(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<namedInt> for i64 {
    type Output = namedInt;
    fn bitand(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for namedInt {
    type Output = namedInt;
    fn bitor(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for namedInt {
    type Output = namedInt;
    fn bitor(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<namedInt> for i64 {
    type Output = namedInt;
    fn bitor(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for namedInt {
    type Output = namedInt;
    fn bitxor(self, other: Self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for namedInt {
    type Output = namedInt;
    fn bitxor(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<namedInt> for i64 {
    type Output = namedInt;
    fn bitxor(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for namedInt {
    type Output = namedInt;
    fn not(self) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for namedInt {
    type Output = namedInt;
    fn shl(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for namedInt {
    type Output = namedInt;
    fn shl(self, other: i32) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for namedInt {
    type Output = namedInt;
    fn shl(self, other: i8) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for namedInt {
    type Output = namedInt;
    fn shl(self, other: i16) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for namedInt {
    type Output = namedInt;
    fn shl(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for namedInt {
    type Output = namedInt;
    fn shl(self, other: u32) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for namedInt {
    type Output = namedInt;
    fn shl(self, other: u8) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for namedInt {
    type Output = namedInt;
    fn shl(self, other: u16) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for namedInt {
    type Output = namedInt;
    fn shl(self, other: u64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for namedInt {
    type Output = namedInt;
    fn shl(self, other: usize) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for namedInt {
    type Output = namedInt;
    fn shr(self, other: namedInt) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for namedInt {
    type Output = namedInt;
    fn shr(self, other: i32) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for namedInt {
    type Output = namedInt;
    fn shr(self, other: i8) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for namedInt {
    type Output = namedInt;
    fn shr(self, other: i16) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for namedInt {
    type Output = namedInt;
    fn shr(self, other: i64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for namedInt {
    type Output = namedInt;
    fn shr(self, other: u32) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for namedInt {
    type Output = namedInt;
    fn shr(self, other: u8) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for namedInt {
    type Output = namedInt;
    fn shr(self, other: u16) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for namedInt {
    type Output = namedInt;
    fn shr(self, other: u64) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for namedInt {
    type Output = namedInt;
    fn shr(self, other: usize) -> namedInt {
        namedInt(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for namedInt {}

impl Ord for namedInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    let mut n: Rc<RefCell<Option<namedInt>>> = Rc::new(RefCell::new(Some(namedInt(Rc::new(RefCell::new(Some(7 as i64)))))));
    let mut s: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some("value".to_string())));

    println!("{}", format!("{}", (*{ let __recv = Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("main.namedInt".to_string()))), kind: Rc::new(RefCell::new(Some(reflect_Kind(6)))), elem: Rc::new(RefCell::new(None::<Box<GoReflectType>>)), fields: Rc::new(RefCell::new(Some(vec![]))) }))); let __result = (*__recv.borrow().as_ref().unwrap()).string(); __result }.borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*{ let __recv = Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("string".to_string()))), kind: Rc::new(RefCell::new(Some(reflect_Kind(24)))), elem: Rc::new(RefCell::new(None::<Box<GoReflectType>>)), fields: Rc::new(RefCell::new(Some(vec![]))) }))); let __result = (*__recv.borrow().as_ref().unwrap()).string(); __result }.borrow().as_ref().unwrap())));
}