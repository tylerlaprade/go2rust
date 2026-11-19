use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Rc<RefCell<Option<i32>>>,
    pub next: Rc<RefCell<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            value: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            value: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct NodeSlice(pub Rc<RefCell<Option<Vec<Node>>>>);


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

pub(crate) static global: GoGlobal<Rc<RefCell<Option<Node>>>> = GoGlobal::new();


fn __go_init_globals() {
    *global.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *global.borrow_mut() = Some(Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(5))), ..Default::default() }))));
}


impl Node {
    pub fn link(&self) -> Rc<RefCell<Option<Node>>> {
        self.next.clone()
    }

    pub fn clear_receiver(&mut self) -> (bool, i32) {
        let mut __self = Rc::new(RefCell::new(Some(self.clone())));
        let mut count = Rc::new(RefCell::new(Some(0)));
        {
            let __tmp_0 = Rc::new(RefCell::new(None));
            let __tmp_1 = 5;
            __self = __tmp_0;
            *count.borrow_mut() = Some(__tmp_1);
        };
        return ({ let __self_guard = __self.borrow(); __self_guard.is_none() }, (*count.borrow().as_ref().unwrap()));
    }
}

fn main() {
    __go_init_all();
    let mut a = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(1))), ..Default::default() })));
    let mut b = a.clone();
    { let new_val = Rc::new(RefCell::new(None)); b = new_val; };
    println!("{} {}", format!("{}", { let __nil_result = (*a.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*b.borrow()).is_none(); __nil_result }));

    let mut shared = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut first = Rc::new(RefCell::new(Some(Node { next: shared.clone(), ..Default::default() })));
    let mut second = Rc::new(RefCell::new(Some(Node { next: shared.clone(), ..Default::default() })));
    { let new_val = Rc::new(RefCell::new(None)); (*first.borrow_mut().as_mut().unwrap()).next = new_val; };
    println!("{} {}", format!("{}", { let __nil_target = (*first.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*second.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }));

    let mut c = Rc::new(RefCell::new(Some(Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone(), ..Default::default() })));
    let mut p = (*c.borrow().as_ref().unwrap()).link();
    { let new_val = Rc::new(RefCell::new(None)); p = new_val; };
    println!("{} {}", format!("{}", { let __nil_target = (*c.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*p.borrow()).is_none(); __nil_result }));

    let mut parallel = a.clone();
    let mut count = Rc::new(RefCell::new(Some(0)));
    {
        let __tmp_0 = Rc::new(RefCell::new(None));
        let __tmp_1 = 2;
        parallel = __tmp_0.clone();
        *count.borrow_mut() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", { let __nil_result = (*a.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*parallel.borrow()).is_none(); __nil_result }), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));

    {
        let __tmp_0 = Rc::new(RefCell::new(None));
        let __tmp_1 = 3;
        (*second.borrow_mut().as_mut().unwrap()).next = __tmp_0.clone();
        *count.borrow_mut() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", { let __nil_result = (*shared.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_target = (*second.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));

    let mut captured = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(4))), ..Default::default() })));
    let mut captured_closure_clone = captured.clone(); let mut clear = Rc::new(RefCell::new(Some(Box::new(move || {
        *captured_closure_clone.borrow_mut() = None;
    }) as Box<dyn FnMut() -> ()>)));
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = clear.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{}", format!("{}", { let __nil_result = (*captured.borrow()).is_none(); __nil_result }));

    let mut observed = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(6))), ..Default::default() })));
    let observed_closure_clone = observed.clone(); let mut show = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", { let __nil_result = (*observed_closure_clone.borrow()).is_none(); __nil_result }));
    }) as Box<dyn FnMut() -> ()>)));
    *observed.borrow_mut() = None;
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = show.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };

    let mut previous = (*global.borrow().as_ref().unwrap()).clone();
    *global.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    println!("{} {}", format!("{}", { let __slot_guard = global.borrow(); let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr.borrow()).is_some()).unwrap_or(false); !__not_nil }), format!("{}", { let __nil_result = (*previous.borrow()).is_some(); __nil_result }));

    let mut addressed = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut address = Rc::new(RefCell::new(Some((addressed).clone())));
    *addressed.borrow_mut() = None;
    println!("{}", format!("{}", { let __nil_result = (*(*address.borrow_mut().as_mut().unwrap()).borrow()).is_none(); __nil_result }));

    let mut parallelAddressed = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(8))), ..Default::default() })));
    let mut parallelAddress = Rc::new(RefCell::new(Some(parallelAddressed.clone())));
    {
        let __tmp_0 = (None);
        let __tmp_1 = 4;
        *parallelAddressed.borrow_mut() = __tmp_0;
        *count.borrow_mut() = Some(__tmp_1);
    };
    println!("{} {}", format!("{}", { let __nil_result = (*(*parallelAddress.borrow_mut().as_mut().unwrap()).borrow()).is_none(); __nil_result }), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));

    let mut receiver = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(9))), ..Default::default() })));
    let (mut cleared, __tmp_1) = (*receiver.borrow_mut().as_mut().unwrap()).clear_receiver(); *count.borrow_mut() = Some(__tmp_1);;
    println!("{} {} {}", format!("{}", cleared), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*receiver.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));

    let mut nodes = Rc::new(RefCell::new(Some(vec![Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(10))), ..Default::default() }))).clone(), ..Default::default() }])));
    let mut sliceAlias = (*nodes.borrow().as_ref().unwrap())[(0) as usize].clone().next.clone();
    { let new_val = Rc::new(RefCell::new(None)); { let __sequence = (nodes).clone(); let __index_0 = (((*nodes.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; { let mut __sequence_guard = __sequence.borrow_mut(); let __element = &mut __sequence_guard.as_mut().unwrap()[__index_0]; __element.next = new_val; } } };
    println!("{} {}", format!("{}", { let __nil_target = (*nodes.borrow().as_ref().unwrap())[(0) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", (*(*sliceAlias.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));

    let mut array = Rc::new(RefCell::new(Some([Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(11))), ..Default::default() }))).clone(), ..Default::default() }])));
    let mut arrayAlias = (*array.borrow().as_ref().unwrap())[(0) as usize].clone().next.clone();
    {
        let __sequence_0 = (array).clone(); let __index_0_0 = (0) as usize;
        let __tmp_0 = Rc::new(RefCell::new(None));
        let __tmp_1 = 6;
        { let mut __sequence_guard = __sequence_0.borrow_mut(); let __element = &mut __sequence_guard.as_mut().unwrap()[__index_0_0]; __element.next = __tmp_0; }
        *count.borrow_mut() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", { let __nil_target = (*array.borrow().as_ref().unwrap())[(0) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", (*(*arrayAlias.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));

    let mut named = Rc::new(RefCell::new(Some(NodeSlice(Rc::new(RefCell::new(Some(vec![Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(12))), ..Default::default() }))).clone(), ..Default::default() }])))))));
    let mut namedAlias = { let __seq_holder = { let __named_slice = (*named.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.next.clone();
    { let new_val = Rc::new(RefCell::new(None)); { let __sequence = ({ let __named_slice = (*named.borrow().as_ref().unwrap()).0.clone(); __named_slice }).clone(); let __index_0 = (0) as usize; { let mut __sequence_guard = __sequence.borrow_mut(); let __element = &mut __sequence_guard.as_mut().unwrap()[__index_0]; __element.next = new_val; } } };
    println!("{} {}", format!("{}", { let __nil_target = { let __seq_holder = { let __named_slice = (*named.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", (*(*namedAlias.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));

    let mut sequence = Rc::new(RefCell::new(Some(vec![Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(13))), ..Default::default() }))).clone(), ..Default::default() }, Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(14))), ..Default::default() }))).clone(), ..Default::default() }])));
    let mut index = Rc::new(RefCell::new(Some(0)));
    let mut held = (*sequence.borrow().as_ref().unwrap())[((*index.borrow().as_ref().unwrap())) as usize].clone().next.clone();
    {
        let __sequence_1 = (sequence).clone(); let __index_1_0 = ((*index.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = 1;
        let __tmp_1 = Rc::new(RefCell::new(None));
        *index.borrow_mut() = Some(__tmp_0);
        { let mut __sequence_guard = __sequence_1.borrow_mut(); let __element = &mut __sequence_guard.as_mut().unwrap()[__index_1_0]; __element.next = __tmp_1; }
    };
    println!("{} {} {} {}", format!("{}", { let __v = (*index.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_target = (*sequence.borrow().as_ref().unwrap())[(0) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*sequence.borrow().as_ref().unwrap())[(1) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }), format!("{}", (*(*held.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));

    let mut nested = Rc::new(RefCell::new(Some(vec![vec![Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(15))), ..Default::default() }))).clone(), ..Default::default() }]])));
    let mut saved = (*nested.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone().next.clone();
    { let new_val = Rc::new(RefCell::new(None)); { let __sequence = (nested).clone(); let __index_0 = (((*nested.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __index_1 = (0) as usize; { let mut __sequence_guard = __sequence.borrow_mut(); let __element = &mut __sequence_guard.as_mut().unwrap()[__index_0][__index_1]; __element.next = new_val; } } };
    println!("{} {}", format!("{}", { let __nil_target = (*nested.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", (*(*saved.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));

    let mut matrix = Rc::new(RefCell::new(Some([[Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(16))), ..Default::default() }))).clone(), ..Default::default() }], [Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(17))), ..Default::default() }))).clone(), ..Default::default() }]])));
    let mut matrixAlias = (*matrix.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone().next.clone();
    let mut row = Rc::new(RefCell::new(Some(0)));
    {
        let __sequence_1 = (matrix).clone(); let __index_1_0 = ((*row.borrow().as_ref().unwrap())) as usize; let __index_1_1 = (0) as usize;
        let __tmp_0 = 1;
        let __tmp_1 = Rc::new(RefCell::new(None));
        *row.borrow_mut() = Some(__tmp_0);
        { let mut __sequence_guard = __sequence_1.borrow_mut(); let __element = &mut __sequence_guard.as_mut().unwrap()[__index_1_0][__index_1_1]; __element.next = __tmp_1; }
    };
    println!("{} {} {} {}", format!("{}", { let __v = (*row.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_target = (*matrix.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*matrix.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone().next.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }), format!("{}", (*(*matrixAlias.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
