use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const NEED_NAME: i32 = 1 << 0;
pub const NEED_FILES: i32 = 1 << 1;
pub const NEED_IMPORTS: i32 = 1 << 2;


#[derive(Debug, Clone, Default)]
pub struct LoadMode(pub Rc<RefCell<Option<i32>>>);

impl Display for LoadMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for LoadMode {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for LoadMode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for LoadMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for LoadMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<LoadMode> for i32 {
    fn eq(&self, other: &LoadMode) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<LoadMode> for i32 {
    fn partial_cmp(&self, other: &LoadMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for LoadMode {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for LoadMode {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<LoadMode> for i32 {
    type Output = i32;
    fn add(self, other: LoadMode) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for LoadMode {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for LoadMode {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<LoadMode> for i32 {
    type Output = i32;
    fn sub(self, other: LoadMode) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for LoadMode {
    type Output = LoadMode;
    fn bitand(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for LoadMode {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<LoadMode> for i32 {
    type Output = i32;
    fn bitand(self, other: LoadMode) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for LoadMode {
    type Output = LoadMode;
    fn bitor(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for LoadMode {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<LoadMode> for i32 {
    type Output = i32;
    fn bitor(self, other: LoadMode) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for LoadMode {
    type Output = LoadMode;
    fn bitxor(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for LoadMode {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<LoadMode> for i32 {
    type Output = i32;
    fn bitxor(self, other: LoadMode) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for LoadMode {}

impl Ord for LoadMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    mode: Rc<RefCell<Option<LoadMode>>>,
    name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mode.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static modes: GoGlobal<[AnonymousStruct1; 3]> = GoGlobal::new();


fn __go_init_globals() {
    *modes.borrow_mut() = Some(std::array::from_fn(|_| Default::default()));
    *modes.borrow_mut() = Some((*Rc::new(RefCell::new(Some([AnonymousStruct1 { mode: Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some(NEED_NAME as i32))))))), name: Rc::new(RefCell::new(Some("NeedName".to_string()))), ..Default::default() }, AnonymousStruct1 { mode: Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some(NEED_FILES as i32))))))), name: Rc::new(RefCell::new(Some("NeedFiles".to_string()))), ..Default::default() }, AnonymousStruct1 { mode: Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some(NEED_IMPORTS as i32))))))), name: Rc::new(RefCell::new(Some("NeedImports".to_string()))), ..Default::default() }]))).borrow().as_ref().unwrap()).clone());
}


pub fn strip(mut mode: Rc<RefCell<Option<LoadMode>>>) -> Rc<RefCell<Option<String>>> {

    let mut out = Rc::new(RefCell::new(Some("".to_string())));
    { let __range_holder = modes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        if (LoadMode(Rc::new(RefCell::new(Some(((*(*mode.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) & (*(*item.mode.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()))))))) != LoadMode(Rc::new(RefCell::new(Some(0 as i32)))) {
        { let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ (*item.mode.borrow().as_ref().unwrap()).clone()); };
        if (*out.borrow().as_ref().unwrap()).clone() != "" {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&",".to_string()); };
    }
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&(*item.name.borrow().as_ref().unwrap())); };
    }
    } }
    if (*mode.borrow().as_ref().unwrap()) != LoadMode(Rc::new(RefCell::new(Some(0 as i32)))) {
        if (*out.borrow().as_ref().unwrap()).clone() != "" {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&",".to_string()); };
    }
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(format!("{}", format!("{:#x}", (*Rc::new(RefCell::new(Some((*(*mode.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())))))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
    }
    if (*out.borrow().as_ref().unwrap()).clone() == "" {
        return Rc::new(RefCell::new(Some("none".to_string())));
    }
    return out.clone();
}

fn main() {
    __go_init_all();
    println!("{}", (*strip(Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some((NEED_NAME | NEED_IMPORTS))))))))).borrow().as_ref().unwrap()));
    println!("{}", (*strip(Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some((NEED_FILES | 8 as i32))))))))).borrow().as_ref().unwrap()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
