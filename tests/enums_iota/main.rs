use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const STATE_IDLE: i32 = 0;
pub const STATE_CONNECTED: i32 = 1;
pub const STATE_ERROR: i32 = 2;
pub const STATE_RETRYING: i32 = 3;


#[derive(Debug, Clone, Default)]
pub struct ServerState(pub Rc<RefCell<Option<i32>>>);

impl Display for ServerState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}

impl PartialEq for ServerState {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for ServerState {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ServerState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for ServerState {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ServerState> for i32 {
    fn eq(&self, other: &ServerState) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<ServerState> for i32 {
    fn partial_cmp(&self, other: &ServerState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for ServerState {
    type Output = ServerState;
    fn add(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ServerState {
    type Output = ServerState;
    fn add(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ServerState> for i32 {
    type Output = ServerState;
    fn add(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ServerState {
    type Output = ServerState;
    fn sub(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ServerState {
    type Output = ServerState;
    fn sub(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ServerState> for i32 {
    type Output = ServerState;
    fn sub(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ServerState {
    type Output = ServerState;
    fn bitand(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ServerState {
    type Output = ServerState;
    fn bitand(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ServerState> for i32 {
    type Output = ServerState;
    fn bitand(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ServerState {
    type Output = ServerState;
    fn bitor(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ServerState {
    type Output = ServerState;
    fn bitor(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ServerState> for i32 {
    type Output = ServerState;
    fn bitor(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ServerState {
    type Output = ServerState;
    fn bitxor(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ServerState {
    type Output = ServerState;
    fn bitxor(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ServerState> for i32 {
    type Output = ServerState;
    fn bitxor(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ServerState {
    type Output = ServerState;
    fn not(self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ServerState {
    type Output = ServerState;
    fn shl(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ServerState {
    type Output = ServerState;
    fn shl(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ServerState {
    type Output = ServerState;
    fn shl(self, other: i8) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ServerState {
    type Output = ServerState;
    fn shl(self, other: i16) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ServerState {
    type Output = ServerState;
    fn shl(self, other: i64) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ServerState {
    type Output = ServerState;
    fn shl(self, other: u32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ServerState {
    type Output = ServerState;
    fn shl(self, other: u8) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ServerState {
    type Output = ServerState;
    fn shl(self, other: u16) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ServerState {
    type Output = ServerState;
    fn shl(self, other: u64) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ServerState {
    type Output = ServerState;
    fn shl(self, other: usize) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ServerState {
    type Output = ServerState;
    fn shr(self, other: ServerState) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ServerState {
    type Output = ServerState;
    fn shr(self, other: i32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ServerState {
    type Output = ServerState;
    fn shr(self, other: i8) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ServerState {
    type Output = ServerState;
    fn shr(self, other: i16) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ServerState {
    type Output = ServerState;
    fn shr(self, other: i64) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ServerState {
    type Output = ServerState;
    fn shr(self, other: u32) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ServerState {
    type Output = ServerState;
    fn shr(self, other: u8) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ServerState {
    type Output = ServerState;
    fn shr(self, other: u16) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ServerState {
    type Output = ServerState;
    fn shr(self, other: u64) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ServerState {
    type Output = ServerState;
    fn shr(self, other: usize) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for ServerState {}

impl Ord for ServerState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
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

pub(crate) static stateName: GoGlobal<BTreeMap<ServerState, Rc<RefCell<Option<String>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *stateName.borrow_mut() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<ServerState, Rc<RefCell<Option<String>>>>::new();
        __go_map.insert(STATE_IDLE, Rc::new(RefCell::new(Some("idle".to_string()))));
        __go_map.insert(STATE_CONNECTED, Rc::new(RefCell::new(Some("connected".to_string()))));
        __go_map.insert(STATE_ERROR, Rc::new(RefCell::new(Some("error".to_string()))));
        __go_map.insert(STATE_RETRYING, Rc::new(RefCell::new(Some("retrying".to_string()))));
        *stateName.borrow_mut() = Some(__go_map);
    }
}


impl ServerState {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*stateName.borrow().as_ref().unwrap()).get(&self.clone()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()))))
    }
}

fn main() {
    __go_init_all();
    let mut ns = transition(Rc::new(RefCell::new(Some(STATE_IDLE))));
    println!("{}", format!("{}", { let __v = (*ns.borrow().as_ref().unwrap()).clone(); __v }));

    let mut ns2 = transition(Rc::new(RefCell::new(Some((*ns.borrow().as_ref().unwrap()).clone()))));
    println!("{}", format!("{}", { let __v = (*ns2.borrow().as_ref().unwrap()).clone(); __v }));
}

pub fn transition(s: Rc<RefCell<Option<ServerState>>>) -> Rc<RefCell<Option<ServerState>>> {
    { let _switch_val = (*s.borrow().as_ref().unwrap()).clone();
    if _switch_val == (STATE_IDLE) {
            return Rc::new(RefCell::new(Some(STATE_CONNECTED)));
        } else if _switch_val == (STATE_CONNECTED) || _switch_val == (STATE_RETRYING) {
            return Rc::new(RefCell::new(Some(STATE_IDLE)));
        } else if _switch_val == (STATE_ERROR) {
            return Rc::new(RefCell::new(Some(STATE_ERROR)));
        } else {
            panic!("unknown state: {}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v });
        }
    }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
