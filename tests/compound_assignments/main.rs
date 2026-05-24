use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Integer compound assignments
    let mut x = Rc::new(RefCell::new(Some(10)));
    { let __rhs = 5; let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    print!("x += 5: {}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 3; let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    print!("x -= 3: {}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 2; let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    print!("x *= 2: {}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 4; let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    print!("x /= 4: {}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 5; let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() % __rhs); };
    print!("x %= 5: {}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

        // Bitwise compound assignments
    let mut y = Rc::new(RefCell::new(Some(0b1010)));
    { let __rhs = 0b1100; let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
    print!("y &= 0b1100: {:b}\n", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 0b0011; let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    print!("y |= 0b0011: {:b}\n", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 0b0101; let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
    print!("y ^= 0b0101: {:b}\n", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 2; let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    print!("y <<= 2: {:b}\n", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 1; let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    print!("y >>= 1: {:b}\n", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

        // Float compound assignments
    let mut f = Rc::new(RefCell::new(Some(3.14)));
    { let __rhs = 2.86; let mut guard = f.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    print!("f += 2.86: {:.2}\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v });

    { let __rhs = 2.0; let mut guard = f.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    print!("f *= 2.0: {:.2}\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v });

        // String compound assignment
    let mut s = Rc::new(RefCell::new(Some("Hello".to_string())));
    { (*s.borrow_mut().as_mut().unwrap()).push_str(&" World".to_string()); };
    print!("s += \" World\": {}\n", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v });
}