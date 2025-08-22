use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Integer compound assignments
    let mut x = Rc::new(RefCell::new(Some(10)));
    { let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 5); };
    print!("x += 5: {}\n", (*x.borrow_mut().as_mut().unwrap()));

    { let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 3); };
    print!("x -= 3: {}\n", (*x.borrow_mut().as_mut().unwrap()));

    { let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * 2); };
    print!("x *= 2: {}\n", (*x.borrow_mut().as_mut().unwrap()));

    { let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() / 4); };
    print!("x /= 4: {}\n", (*x.borrow_mut().as_mut().unwrap()));

    { let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() % 5); };
    print!("x %= 5: {}\n", (*x.borrow_mut().as_mut().unwrap()));

        // Bitwise compound assignments
    let mut y = Rc::new(RefCell::new(Some(0b1010)));
    { let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() & 0b1100); };
    print!("y &= 0b1100: {:b}\n", (*y.borrow_mut().as_mut().unwrap()));

    { let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | 0b0011); };
    print!("y |= 0b0011: {:b}\n", (*y.borrow_mut().as_mut().unwrap()));

    { let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() ^ 0b0101); };
    print!("y ^= 0b0101: {:b}\n", (*y.borrow_mut().as_mut().unwrap()));

    { let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() << 2); };
    print!("y <<= 2: {:b}\n", (*y.borrow_mut().as_mut().unwrap()));

    { let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() >> 1); };
    print!("y >>= 1: {:b}\n", (*y.borrow_mut().as_mut().unwrap()));

        // Float compound assignments
    let mut f = Rc::new(RefCell::new(Some(3.14)));
    { let mut guard = f.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 2.86); };
    print!("f += 2.86: {:.2}\n", (*f.borrow_mut().as_mut().unwrap()));

    { let mut guard = f.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * 2.0); };
    print!("f *= 2.0: {:.2}\n", (*f.borrow_mut().as_mut().unwrap()));

        // String compound assignment
    let mut s = Rc::new(RefCell::new(Some("Hello".to_string())));
    (*s.borrow_mut().as_mut().unwrap()).push_str(&" World".to_string());
    print!("s += \" World\": {}\n", (*s.borrow_mut().as_mut().unwrap()));
}