use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

const UINT_SIZE: i64 = 32 << (^(*uint.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0)))) >> 63);


const MAX_SHIFT: i32 = UINT_SIZE - 4;


#[derive(Debug, Clone, Default)]
struct decimal {
    d: Rc<RefCell<Option<[u8; 800]>>>,
    nd: Rc<RefCell<Option<i32>>>,
    dp: Rc<RefCell<Option<i32>>>,
    neg: Rc<RefCell<Option<bool>>>,
    trunc: Rc<RefCell<Option<bool>>>,
}

impl std::fmt::Display for decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.d.borrow().as_ref().unwrap()), (*self.nd.borrow().as_ref().unwrap()), (*self.dp.borrow().as_ref().unwrap()), (*self.neg.borrow().as_ref().unwrap()), (*self.trunc.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct leftCheat {
    delta: Rc<RefCell<Option<i32>>>,
    cutoff: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for leftCheat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.delta.borrow().as_ref().unwrap()), (*self.cutoff.borrow().as_ref().unwrap()))
    }
}


impl decimal {
    pub fn string(&mut self) -> Rc<RefCell<Option<String>>> {
        let mut n = Rc::new(RefCell::new(Some(10 + (*self.nd.clone().borrow().as_ref().unwrap()))));
        if (*self.dp.clone().borrow().as_ref().unwrap()) > 0 {
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + self.dp.clone()); };
    }
        if (*self.dp.clone().borrow().as_ref().unwrap()) < 0 {
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + -self.dp.clone()); };
    }
        let mut buf = Rc::new(RefCell::new(Some(vec![0; (*n.borrow_mut().as_mut().unwrap())])));
        let mut w = Rc::new(RefCell::new(Some(0)));
        match true {
        true if (*self.nd.clone().borrow().as_ref().unwrap()) == 0 => {
            return Rc::new(RefCell::new(Some("0".to_string())));
        }
        true if (*self.dp.clone().borrow().as_ref().unwrap()) <= 0 => {
                        // zeros fill space between decimal point and digits
            (*buf.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = ('0' as i32);
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
            (*buf.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = ('.' as i32);
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*digitZero.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(*w.borrow_mut().as_mut().unwrap()) as usize..(*w.borrow_mut().as_mut().unwrap()) + (*-self.dp.clone().borrow().as_ref().unwrap()) as usize].to_vec())))))))); };
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + copy(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(*w.borrow_mut().as_mut().unwrap()) as usize..].to_vec())))))), Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*self.d.borrow().as_ref().unwrap())[0 as usize..self.nd.clone() as usize].to_vec())))))))); };
        }
        true if (*self.dp.clone().borrow().as_ref().unwrap()) < (*self.nd.clone().borrow().as_ref().unwrap()) => {
                        // decimal point in middle of digits
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + copy(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(*w.borrow_mut().as_mut().unwrap()) as usize..].to_vec())))))), Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*self.d.borrow().as_ref().unwrap())[0 as usize..self.dp.clone() as usize].to_vec())))))))); };
            (*buf.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = ('.' as i32);
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + copy(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(*w.borrow_mut().as_mut().unwrap()) as usize..].to_vec())))))), Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*self.d.borrow().as_ref().unwrap())[self.dp.clone() as usize..self.nd.clone() as usize].to_vec())))))))); };
        }
        _ => {
                        // zeros fill space between digits and decimal point
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + copy(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(*w.borrow_mut().as_mut().unwrap()) as usize..].to_vec())))))), Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*self.d.borrow().as_ref().unwrap())[0 as usize..self.nd.clone() as usize].to_vec())))))))); };
            { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*digitZero.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(*w.borrow_mut().as_mut().unwrap()) as usize..(*w.borrow_mut().as_mut().unwrap()) + (*self.dp.clone().borrow().as_ref().unwrap()) - (*self.nd.clone().borrow().as_ref().unwrap()) as usize].to_vec())))))))); };
        }
    }
                // zeros fill space between decimal point and digits
                // decimal point in middle of digits
                // zeros fill space between digits and decimal point
        return Rc::new(RefCell::new(Some((*string.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[0 as usize..(*w.borrow_mut().as_mut().unwrap()) as usize].to_vec()))))))))));
    }

    /// Assign v to a.
    pub fn assign(&mut self, v: Rc<RefCell<Option<u64>>>) {
        let mut buf: Rc<RefCell<Option<[u8; 24]>>> = Rc::new(RefCell::new(Some(Default::default())));
                // Write reversed decimal in buf.
        let mut n = Rc::new(RefCell::new(Some(0)));
        while (*v.borrow_mut().as_mut().unwrap()) > 0 {
        let mut v1 = Rc::new(RefCell::new(Some((*v.borrow_mut().as_mut().unwrap()) / 10)));
        { let mut guard = v.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 10 * (*v1.borrow_mut().as_mut().unwrap())); };
        (*buf.borrow_mut().as_mut().unwrap())[(*n.borrow_mut().as_mut().unwrap())] = (*(*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*v.borrow_mut().as_mut().unwrap()) + ('0' as i32))))).borrow().as_ref().unwrap());
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*v1.borrow_mut().as_mut().unwrap()); *v.borrow_mut() = Some(new_val); };
    }
                // Reverse again to produce forward decimal in a.d.
        { let new_val = 0; *self.nd.borrow_mut() = Some(new_val); };
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    while (*n.borrow_mut().as_mut().unwrap()) >= 0 {
        (*self.d.borrow_mut().as_mut().unwrap())[self.nd.clone()] = (*buf.borrow().as_ref().unwrap())[(*n.borrow_mut().as_mut().unwrap()) as usize].clone();
        { let mut guard = self.nd.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let new_val = self.nd.clone(); *self.dp.borrow_mut() = Some(new_val); };
        (*trim.borrow().as_ref().unwrap())(a.clone());
    }

    /// Binary shift left (k > 0) or right (k < 0).
    pub fn shift(&mut self, k: Rc<RefCell<Option<i32>>>) {
        match true {
        true if (*self.nd.clone().borrow().as_ref().unwrap()) == 0 => {
        }
        true if (*k.borrow_mut().as_mut().unwrap()) > 0 => {
            while (*k.borrow_mut().as_mut().unwrap()) > (*maxShift.borrow_mut().as_mut().unwrap()) {
        (*leftShift.borrow().as_ref().unwrap())(a.clone(), maxShift.clone());
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - (*maxShift.borrow_mut().as_mut().unwrap())); };
    }
            (*leftShift.borrow().as_ref().unwrap())(a.clone(), Rc::new(RefCell::new(Some((*uint.borrow().as_ref().unwrap())(k.clone())))));
        }
        true if (*k.borrow_mut().as_mut().unwrap()) < 0 => {
            while (*k.borrow_mut().as_mut().unwrap()) < (*-(*maxShift.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        (*rightShift.borrow().as_ref().unwrap())(a.clone(), maxShift.clone());
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*maxShift.borrow_mut().as_mut().unwrap())); };
    }
            (*rightShift.borrow().as_ref().unwrap())(a.clone(), Rc::new(RefCell::new(Some((*uint.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-(*k.borrow_mut().as_mut().unwrap())))))))));
        }
        _ => {}
    }
    }

    /// Round a to nd digits (or fewer).
    /// If nd is zero, it means we're rounding
    /// just to the left of the digits, as in
    /// 0.09 -> 0.1.
    pub fn round(&mut self, nd: Rc<RefCell<Option<i32>>>) {
        if (*nd.borrow_mut().as_mut().unwrap()) < 0 || (*nd.borrow_mut().as_mut().unwrap()) >= (*self.nd.clone().borrow().as_ref().unwrap()) {
        return;
    }
        if (*shouldRoundUp.borrow().as_ref().unwrap())(a.clone(), nd.clone()) {
        (*a.borrow_mut().as_mut().unwrap()).round_up(Rc::new(RefCell::new(Some((*nd.borrow_mut().as_mut().unwrap())))));
    } else {
        (*a.borrow_mut().as_mut().unwrap()).round_down(Rc::new(RefCell::new(Some((*nd.borrow_mut().as_mut().unwrap())))));
    }
    }

    /// Round a down to nd digits (or fewer).
    pub fn round_down(&mut self, nd: Rc<RefCell<Option<i32>>>) {
        if (*nd.borrow_mut().as_mut().unwrap()) < 0 || (*nd.borrow_mut().as_mut().unwrap()) >= (*self.nd.clone().borrow().as_ref().unwrap()) {
        return;
    }
        { let new_val = (*nd.borrow_mut().as_mut().unwrap()); *self.nd.borrow_mut() = Some(new_val); };
        (*trim.borrow().as_ref().unwrap())(a.clone());
    }

    /// Round a up to nd digits (or fewer).
    pub fn round_up(&mut self, nd: Rc<RefCell<Option<i32>>>) {
        if (*nd.borrow_mut().as_mut().unwrap()) < 0 || (*nd.borrow_mut().as_mut().unwrap()) >= (*self.nd.clone().borrow().as_ref().unwrap()) {
        return;
    }
                // round up
        let mut i = Rc::new(RefCell::new(Some((*nd.borrow_mut().as_mut().unwrap()) - 1)));
    while (*i.borrow_mut().as_mut().unwrap()) >= 0 {
        let mut c = Rc::new(RefCell::new(Some((*self.d.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone())));
        if (*c.borrow_mut().as_mut().unwrap()) < ('9' as i32) {
        { let mut guard = (*self.d.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*i.borrow_mut().as_mut().unwrap()) + 1; *self.nd.borrow_mut() = Some(new_val); };
        return;
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // can stop after this digit
                // Number is all 9s.
                // Change to single 1 with adjusted decimal point.
        (*self.d.borrow_mut().as_mut().unwrap())[0] = ('1' as i32);
        { let new_val = 1; *self.nd.borrow_mut() = Some(new_val); };
        { let mut guard = self.dp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// Extract integer part, rounded appropriately.
    /// No guarantees about overflow.
    pub fn rounded_integer(&mut self) -> Rc<RefCell<Option<u64>>> {
        if (*self.dp.clone().borrow().as_ref().unwrap()) > 20 {
        return Rc::new(RefCell::new(Some(0xFFFFFFFFFFFFFFFF)));
    }
        let mut i: Rc<RefCell<Option<i32>>> = 0;
        let mut n = (*uint64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0))));
        { let new_val = 0; *i.borrow_mut() = Some(new_val); };
    while (*i.borrow_mut().as_mut().unwrap()) < (*self.dp.clone().borrow().as_ref().unwrap()) && (*i.borrow_mut().as_mut().unwrap()) < (*self.nd.clone().borrow().as_ref().unwrap()) {
        { let new_val = (*n.borrow_mut().as_mut().unwrap()) * 10 + (*(*uint64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*self.d.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) - ('0' as i32))))).borrow().as_ref().unwrap()); *n.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow_mut().as_mut().unwrap()) < (*self.dp.clone().borrow().as_ref().unwrap()) {
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * 10); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*shouldRoundUp.borrow().as_ref().unwrap())(a.clone(), Rc::new(RefCell::new(Some(self.dp.clone())))) {
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return n.clone();
    }
}

pub fn digit_zero(dst: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<i32>>> {

    for i in 0..(*dst.borrow_mut().as_mut().unwrap()).len() {
        (*dst.borrow_mut().as_mut().unwrap())[i] = ('0' as i32);
    }
    return Rc::new(RefCell::new(Some((*dst.borrow().as_ref().unwrap()).len())));
}

/// trim trailing zeros from number.
/// (They are meaningless; the decimal point is tracked
/// independent of the number of digits.)
pub fn trim(a: Rc<RefCell<Option<decimal>>>) {
    while (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) > 0 && (*(*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) - 1 as usize].clone().borrow().as_ref().unwrap()) == ('0' as i32) {
        { let mut guard = (*a.borrow_mut().as_mut().unwrap()).nd.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) == 0 {
        { let new_val = 0; *(*a.borrow_mut().as_mut().unwrap()).dp.borrow_mut() = Some(new_val); };
    }
}

/// Binary shift right (/ 2) by k bits.  k <= maxShift to avoid overflow.
pub fn right_shift(a: Rc<RefCell<Option<decimal>>>, k: Rc<RefCell<Option<u32>>>) {
    let mut r = Rc::new(RefCell::new(Some(0)));
    let mut w = Rc::new(RefCell::new(Some(0)));

        // Pick up enough leading digits to cover first shift.
    let mut n: Rc<RefCell<Option<u32>>> = Default::default();
    while (*n.borrow_mut().as_mut().unwrap()) >> (*k.borrow_mut().as_mut().unwrap()) == 0 {
        if (*r.borrow_mut().as_mut().unwrap()) >= (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        if (*n.borrow_mut().as_mut().unwrap()) == 0 {
                // a == 0; shouldn't get here, but handle anyway.
        { let new_val = 0; *(*a.borrow_mut().as_mut().unwrap()).nd.borrow_mut() = Some(new_val); };
        return;
    }
                // a == 0; shouldn't get here, but handle anyway.
        while (*n.borrow_mut().as_mut().unwrap()) >> (*k.borrow_mut().as_mut().unwrap()) == 0 {
        { let new_val = (*n.borrow_mut().as_mut().unwrap()) * 10; *n.borrow_mut() = Some(new_val); };
        { let mut guard = r.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        break
    }
                // a == 0; shouldn't get here, but handle anyway.
        let mut c = (*uint.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*r.borrow_mut().as_mut().unwrap()) as usize].clone()))));
        { let new_val = (*n.borrow_mut().as_mut().unwrap()) * 10 + (*c.borrow_mut().as_mut().unwrap()) - ('0' as i32); *n.borrow_mut() = Some(new_val); };
        { let mut guard = r.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // a == 0; shouldn't get here, but handle anyway.
    { let mut guard = (*a.borrow_mut().as_mut().unwrap()).dp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - (*r.borrow_mut().as_mut().unwrap()) - 1); };

    let mut mask: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some((1 << (*k.borrow_mut().as_mut().unwrap())) - 1)));

        // Pick up a digit, put down a digit.
    while (*r.borrow_mut().as_mut().unwrap()) < (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        let mut c = (*uint.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*r.borrow_mut().as_mut().unwrap()) as usize].clone()))));
        let mut dig = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) >> (*k.borrow_mut().as_mut().unwrap()))));
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() & (*mask.borrow_mut().as_mut().unwrap())); };
        (*(*a.borrow_mut().as_mut().unwrap()).d.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = (*(*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*dig.borrow_mut().as_mut().unwrap()) + ('0' as i32))))).borrow().as_ref().unwrap());
        { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*n.borrow_mut().as_mut().unwrap()) * 10 + (*c.borrow_mut().as_mut().unwrap()) - ('0' as i32); *n.borrow_mut() = Some(new_val); };
        { let mut guard = r.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Put down extra digits.
    while (*n.borrow_mut().as_mut().unwrap()) > 0 {
        let mut dig = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) >> (*k.borrow_mut().as_mut().unwrap()))));
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() & (*mask.borrow_mut().as_mut().unwrap())); };
        if (*w.borrow_mut().as_mut().unwrap()) < (*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap()).len() {
        (*(*a.borrow_mut().as_mut().unwrap()).d.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = (*(*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*dig.borrow_mut().as_mut().unwrap()) + ('0' as i32))))).borrow().as_ref().unwrap());
        { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else if (*dig.borrow_mut().as_mut().unwrap()) > 0 {
        { let new_val = true; *(*a.borrow_mut().as_mut().unwrap()).trunc.borrow_mut() = Some(new_val); };
    }
        { let new_val = (*n.borrow_mut().as_mut().unwrap()) * 10; *n.borrow_mut() = Some(new_val); };
    }

    { let new_val = (*w.borrow_mut().as_mut().unwrap()); *(*a.borrow_mut().as_mut().unwrap()).nd.borrow_mut() = Some(new_val); };
    (*trim.borrow().as_ref().unwrap())(a.clone());
}

/// Is the leading prefix of b lexicographically less than s?
pub fn prefix_is_less_than(b: Rc<RefCell<Option<Vec<u8>>>>, s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<bool>>> {

    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < (*s.borrow().as_ref().unwrap()).len() {
        if (*i.borrow_mut().as_mut().unwrap()) >= (*b.borrow().as_ref().unwrap()).len() {
        return Rc::new(RefCell::new(Some(true)));
    }
        if (*(*b.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) != (*(*s.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*(*b.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) < (*(*s.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()))));
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return Rc::new(RefCell::new(Some(false)));
}

/// Binary shift left (* 2) by k bits.  k <= maxShift to avoid overflow.
pub fn left_shift(a: Rc<RefCell<Option<decimal>>>, k: Rc<RefCell<Option<u32>>>) {
    let mut delta = Rc::new(RefCell::new(Some((*(*leftcheats.borrow().as_ref().unwrap())[(*k.borrow_mut().as_mut().unwrap()) as usize].clone().delta.borrow().as_ref().unwrap()))));
    if (*prefixIsLessThan.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[0 as usize..(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()) as usize].to_vec())))))), Rc::new(RefCell::new(Some((*(*leftcheats.borrow().as_ref().unwrap())[(*k.borrow_mut().as_mut().unwrap()) as usize].clone().cutoff.borrow().as_ref().unwrap()))))) {
        { let mut guard = delta.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

    let mut r = Rc::new(RefCell::new(Some((*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()))));
    let mut w = Rc::new(RefCell::new(Some((*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) + (*delta.borrow_mut().as_mut().unwrap()))));

        // Pick up a digit, put down a digit.
    let mut n: Rc<RefCell<Option<u32>>> = Default::default();
    { let mut guard = r.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    while (*r.borrow_mut().as_mut().unwrap()) >= 0 {
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + ((*(*uint.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*r.borrow_mut().as_mut().unwrap()) as usize].clone())))).borrow().as_ref().unwrap()) - ('0' as i32)) << (*k.borrow_mut().as_mut().unwrap())); };
        let mut quo = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) / 10)));
        let mut rem = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 10 * (*quo.borrow_mut().as_mut().unwrap()))));
        { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if (*w.borrow_mut().as_mut().unwrap()) < (*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap()).len() {
        (*(*a.borrow_mut().as_mut().unwrap()).d.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = (*(*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*rem.borrow_mut().as_mut().unwrap()) + ('0' as i32))))).borrow().as_ref().unwrap());
    } else if (*rem.borrow_mut().as_mut().unwrap()) != 0 {
        { let new_val = true; *(*a.borrow_mut().as_mut().unwrap()).trunc.borrow_mut() = Some(new_val); };
    }
        { let new_val = (*quo.borrow_mut().as_mut().unwrap()); *n.borrow_mut() = Some(new_val); };
        { let mut guard = r.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        // Put down extra digits.
    while (*n.borrow_mut().as_mut().unwrap()) > 0 {
        let mut quo = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) / 10)));
        let mut rem = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 10 * (*quo.borrow_mut().as_mut().unwrap()))));
        { let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if (*w.borrow_mut().as_mut().unwrap()) < (*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap()).len() {
        (*(*a.borrow_mut().as_mut().unwrap()).d.borrow_mut().as_mut().unwrap())[(*w.borrow_mut().as_mut().unwrap())] = (*(*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*rem.borrow_mut().as_mut().unwrap()) + ('0' as i32))))).borrow().as_ref().unwrap());
    } else if (*rem.borrow_mut().as_mut().unwrap()) != 0 {
        { let new_val = true; *(*a.borrow_mut().as_mut().unwrap()).trunc.borrow_mut() = Some(new_val); };
    }
        { let new_val = (*quo.borrow_mut().as_mut().unwrap()); *n.borrow_mut() = Some(new_val); };
    }

    { let mut guard = (*a.borrow_mut().as_mut().unwrap()).nd.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*delta.borrow_mut().as_mut().unwrap())); };
    if (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) >= (*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap()).len() {
        { let new_val = (*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap()).len(); *(*a.borrow_mut().as_mut().unwrap()).nd.borrow_mut() = Some(new_val); };
    }
    { let mut guard = (*a.borrow_mut().as_mut().unwrap()).dp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*delta.borrow_mut().as_mut().unwrap())); };
    (*trim.borrow().as_ref().unwrap())(a.clone());
}

/// If we chop a at nd digits, should we round up?
pub fn should_round_up(a: Rc<RefCell<Option<decimal>>>, nd: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>> {

    if (*nd.borrow_mut().as_mut().unwrap()) < 0 || (*nd.borrow_mut().as_mut().unwrap()) >= (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(false)));
    }
    if (*(*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*nd.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) == ('5' as i32) && (*nd.borrow_mut().as_mut().unwrap()) + 1 == (*(*(*a.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
                // if we truncated, a little higher than what's recorded - always round up
        if (*(*a.borrow().as_ref().unwrap()).trunc.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(true)));
    }
        return Rc::new(RefCell::new(Some((*nd.borrow_mut().as_mut().unwrap()) > 0 && ((*(*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*nd.borrow_mut().as_mut().unwrap()) - 1 as usize].clone().borrow().as_ref().unwrap()) - ('0' as i32)) % 2 != 0)));
    }

        // if we truncated, a little higher than what's recorded - always round up
        // not halfway - digit tells all
    return Rc::new(RefCell::new(Some((*(*(*a.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*nd.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) >= ('5' as i32))));
}