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
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for ServerState {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<ServerState> for i32 {
    type Output = i32;
    fn add(self, other: ServerState) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for ServerState {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for ServerState {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<ServerState> for i32 {
    type Output = i32;
    fn sub(self, other: ServerState) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for ServerState {
    type Output = ServerState;
    fn bitand(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ServerState {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<ServerState> for i32 {
    type Output = i32;
    fn bitand(self, other: ServerState) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for ServerState {
    type Output = ServerState;
    fn bitor(self, other: Self) -> ServerState {
        ServerState(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ServerState {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<ServerState> for i32 {
    type Output = i32;
    fn bitor(self, other: ServerState) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
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

pub(crate) static stateName: GoGlobal<BTreeMap<i32, Rc<RefCell<Option<String>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *stateName.borrow_mut() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<i32, Rc<RefCell<Option<String>>>>::new();
        __go_map.insert(STATE_IDLE, Rc::new(RefCell::new(Some("idle".to_string()))));
        __go_map.insert(STATE_CONNECTED, Rc::new(RefCell::new(Some("connected".to_string()))));
        __go_map.insert(STATE_ERROR, Rc::new(RefCell::new(Some("error".to_string()))));
        __go_map.insert(STATE_RETRYING, Rc::new(RefCell::new(Some("retrying".to_string()))));
        *stateName.borrow_mut() = Some(__go_map);
    }
}


impl ServerState {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*stateName.borrow().as_ref().unwrap()).get(&(*self.0.borrow().as_ref().unwrap())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()))));
    }
}

fn main() {
    __go_init_all();
    let mut ns = transition(Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_IDLE as i32))))))));
    println!("{}", { let __v = (*ns.borrow().as_ref().unwrap()).clone(); __v });

    let mut ns2 = transition(Rc::new(RefCell::new(Some((*ns.borrow().as_ref().unwrap()).clone()))));
    println!("{}", { let __v = (*ns2.borrow().as_ref().unwrap()).clone(); __v });
}

pub fn transition(s: Rc<RefCell<Option<ServerState>>>) -> Rc<RefCell<Option<ServerState>>> {

    { let _switch_val = (*s.borrow().as_ref().unwrap()).clone();
    if _switch_val == (ServerState(Rc::new(RefCell::new(Some(STATE_IDLE as i32))))) {
            return Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_CONNECTED as i32)))))));
        } else if _switch_val == (ServerState(Rc::new(RefCell::new(Some(STATE_CONNECTED as i32))))) || _switch_val == (ServerState(Rc::new(RefCell::new(Some(STATE_RETRYING as i32))))) {
            return Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_IDLE as i32)))))));
        } else if _switch_val == (ServerState(Rc::new(RefCell::new(Some(STATE_ERROR as i32))))) {
            return Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_ERROR as i32)))))));
        } else {
            panic!("unknown state: {}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v });
        }
    }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
