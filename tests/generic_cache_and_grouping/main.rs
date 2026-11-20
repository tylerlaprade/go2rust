use std::any::Any;
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

#[derive(Debug, Clone)]
pub struct Cache__string__User {
    pub data: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<User>>>>>>>,
    pub order: Rc<RefCell<Option<Vec<String>>>>,
    pub limit: Rc<RefCell<Option<i64>>>,
}

impl Cache__string__User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__string__User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            data: __go_default_0_0,
            order: __go_default_1_0,
            limit: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Cache__string__User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.data));
        let __go_fmt_1 = format!("{}", format_slice(&self.order));
        let __go_fmt_2 = format!("{}", (*self.limit.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Cache__int__slice_string {
    pub data: Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<Vec<String>>>>>>>>,
    pub order: Rc<RefCell<Option<Vec<i64>>>>,
    pub limit: Rc<RefCell<Option<i64>>>,
}

impl Cache__int__slice_string {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__int__slice_string {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            data: __go_default_0_0,
            order: __go_default_1_0,
            limit: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Cache__int__slice_string {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<map>");
        let __go_fmt_1 = format!("{}", format_slice(&self.order));
        let __go_fmt_2 = format!("{}", (*self.limit.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i64>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub trait Number: std::fmt::Display + Any {
    fn __go_clone_box_number(&self) -> Box<dyn Number>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_number(&self, other: &dyn Number) -> bool;
}

impl Clone for Box<dyn Number> {
    fn clone(&self) -> Self {
        Number::__go_clone_box_number(self.as_ref())
    }
}

impl Number for i8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for isize {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<isize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for usize {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<usize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for String {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct Celsius(pub Rc<RefCell<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Celsius(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}


impl Cache__int__slice_string {
    pub fn put(&mut self, k: Rc<RefCell<Option<i64>>>, v: Rc<RefCell<Option<Vec<String>>>>) {
        {
        let (_, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !ok {
            { let new_val = { let __append_target = self.order.clone(); let __append_value = (*k.borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.order = new_val; };;
        }
    }
        { let __map_key = (*k.borrow().as_ref().unwrap()); let __map_value = v.clone(); (*self.data.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64) > ((*self.limit.clone().borrow().as_ref().unwrap()) as i64) {
        let mut oldest = Rc::new(RefCell::new(Some((*self.order.borrow().as_ref().unwrap())[(0) as usize].clone())));
        { let new_val = Rc::new(RefCell::new(Some({ let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*oldest.borrow().as_ref().unwrap())); };
    }
    }

    pub fn get(&self, k: Rc<RefCell<Option<i64>>>) -> (Rc<RefCell<Option<Vec<String>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let (mut v, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
        let mut zero: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        return (zero.clone(), Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("miss".to_string()))))));
    }
        return (v.clone(), Rc::new(RefCell::new(None)));
    }
}

impl Cache__string__User {
    pub fn put(&mut self, k: Rc<RefCell<Option<String>>>, v: Rc<RefCell<Option<User>>>) {
        {
        let (_, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };;
        if !ok {
            { let new_val = { let __append_target = self.order.clone(); let __append_value = (*k.borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.order = new_val; };;
        }
    }
        { let __map_key = (*k.borrow().as_ref().unwrap()).clone(); let __map_value = { let __owned = v.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }; (*self.data.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64) > ((*self.limit.clone().borrow().as_ref().unwrap()) as i64) {
        let mut oldest = Rc::new(RefCell::new(Some((*self.order.borrow().as_ref().unwrap())[(0) as usize].clone())));
        { let new_val = Rc::new(RefCell::new(Some({ let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*oldest.borrow().as_ref().unwrap()).clone()); };
    }
    }

    pub fn get(&self, k: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<User>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        let (mut v, mut ok) = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*k.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
        if !ok {
        let mut zero: Rc<RefCell<Option<User>>> = Rc::new(RefCell::new(Some(Default::default())));
        return ({ let __owned = zero.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("miss".to_string()))))));
    }
        return ({ let __owned = v.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }, Rc::new(RefCell::new(None)));
    }
}

impl Celsius {
}

impl Number for Celsius {
    fn __go_clone_box_number(&self) -> Box<dyn Number> {
        Box::new(self.clone()) as Box<dyn Number>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &dyn Number) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Celsius>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn new_cache__string___user(limit: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Cache__string__User>>> {
    Rc::new(RefCell::new(Some(Cache__string__User { data: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<User>>>>::new()))), limit: Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn new_cache__int__slice_string(limit: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Cache__int__slice_string>>> {
    Rc::new(RefCell::new(Some(Cache__int__slice_string { data: Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<Vec<String>>>>>::new()))), limit: Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn sum_by___user__int(xs: Rc<RefCell<Option<Vec<User>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>>>>) -> i64 {
    let mut total: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x).__go_value_clone())))) }; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

pub fn sum_by___user__float64(xs: Rc<RefCell<Option<Vec<User>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64>>>>) -> f64 {
    let mut total: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x).__go_value_clone())))) }; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

pub fn sum_by___celsius___celsius(xs: Rc<RefCell<Option<Vec<Celsius>>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>>>>>) -> Rc<RefCell<Option<Celsius>>> {
    let mut total: Rc<RefCell<Option<Celsius>>> = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(0.0)))))));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().cloned() {
        { let __rhs = (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(x.clone())))) }.borrow().as_ref().unwrap()).clone(); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    } }
    return { let __owned = total.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn group_by___user__int(xs: Rc<RefCell<Option<Vec<User>>>>, key: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>>>>) -> Rc<RefCell<Option<BTreeMap<i64, Rc<RefCell<Option<Vec<User>>>>>>>> {
    let mut out = Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<Vec<User>>>>>::new())));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        let mut k = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> = { let mut __f_guard = key.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x).__go_value_clone())))) };
        { let __map_key = k; let __map_value = { let __slice = { let __map_holder = out.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&k).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = (*x).__go_value_clone(); (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*out.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return out.clone();
}

fn main() {
    let mut c = new_cache__string___user(Rc::new(RefCell::new(Some(2))));
    (*c.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Ann".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() }))));
    (*c.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() }))));
    (*c.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some("c".to_string()))), Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Cy".to_string()))), age: Rc::new(RefCell::new(Some(40))), ..Default::default() }))));
    let (_, mut err) = (*c.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("a".to_string()))));
    let (mut u, mut err2) = (*c.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("c".to_string()))));
    println!("{} {} {} {} {}", format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*u.borrow().as_ref().unwrap())), format!("{}", { let __text = match err2.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ({ let __len_target = { let __field = (*c.borrow().as_ref().unwrap()).data.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })), format!("{}", format_slice_values(&(*(*c.borrow().as_ref().unwrap()).order.borrow().as_ref().unwrap()).clone())));

    let mut users = Rc::new(RefCell::new(Some(vec![User { name: Rc::new(RefCell::new(Some("Ann".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() }, User { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() }, User { name: Rc::new(RefCell::new(Some("Cy".to_string()))), age: Rc::new(RefCell::new(Some(40))), ..Default::default() }, User { name: Rc::new(RefCell::new(Some("Di".to_string()))), age: Rc::new(RefCell::new(Some(25))), ..Default::default() }])));
    println!("{}", format!("{}", sum_by___user__int(users.clone(), Rc::new(RefCell::new(Some(Box::new(move |u: Rc<RefCell<Option<User>>>| -> i64 {
        return (*(*u.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>))))));
    println!("{}", format!("{}", sum_by___user__float64(users.clone(), Rc::new(RefCell::new(Some(Box::new(move |u: Rc<RefCell<Option<User>>>| -> f64 {
        (*Rc::new(RefCell::new(Some({ let __selector_holder = (*u.borrow().as_ref().unwrap()).age.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as f64))).borrow().as_ref().unwrap()) / 2.0
    }) as Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> f64>))))));
    let mut temps = Rc::new(RefCell::new(Some(vec![Celsius(Rc::new(RefCell::new(Some(20.5)))), Celsius(Rc::new(RefCell::new(Some(22.0)))), Celsius(Rc::new(RefCell::new(Some(19.5))))])));
    println!("{}", format!("{}", (*sum_by___celsius___celsius(temps.clone(), Rc::new(RefCell::new(Some(Box::new(move |c: Rc<RefCell<Option<Celsius>>>| -> Rc<RefCell<Option<Celsius>>> {
        { let __owned = c.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
    }) as Box<dyn FnMut(Rc<RefCell<Option<Celsius>>>) -> Rc<RefCell<Option<Celsius>>>>)))).borrow().as_ref().unwrap())));
    let mut groups = group_by___user__int(users.clone(), Rc::new(RefCell::new(Some(Box::new(move |u: Rc<RefCell<Option<User>>>| -> i64 {
        return (*(*u.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<User>>>) -> i64>))));
    let mut ages = Rc::new(RefCell::new(Some(Vec::<i64>::with_capacity(((*groups.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (age, _) in { let __range_holder = groups.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = ages.clone(); let __append_value = age; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; ages = new_val; };
    }
    { let __sort_target = ages.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = ages.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for age in __range_values.iter().copied() {
        let mut names: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        { let __range_holder = { let __map_holder = groups.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&age)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for u in __range_values.iter() {
        let u = &u.__go_value_clone();
        { let new_val = { let __append_target = names.clone(); let __append_value = { let __selector_holder = u.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; names = new_val; };
    } }
        println!("{} {}", format!("{}", age), format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*names.borrow()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
    } }
    let mut ic = new_cache__int__slice_string(Rc::new(RefCell::new(Some(3))));
    (*ic.borrow_mut().as_mut().unwrap()).put(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(vec!["x".to_string()]))));
    let (mut got, _) = (*ic.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some(1))));
    println!("{} {}", format!("{}", format_slice(&got)), format!("{}", (*got.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}