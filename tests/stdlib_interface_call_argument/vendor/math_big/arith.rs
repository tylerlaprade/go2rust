use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone};

use crate::accuracy_string::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
use crate::floatconv::*;
use crate::floatmarsh::*;
use crate::ftoa::*;
use crate::int::*;
use crate::intconv::*;
use crate::intmarsh::*;
use crate::nat::*;
use crate::natconv::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __S: i32 = __W / 8;
pub(crate) const __W: i32 = math_bits::UINT_SIZE as i32;
pub(crate) const __B: u128 = 1 << __W;
pub(crate) const __M: u128 = __B - 1;


/// A Word represents a single digit of a multi-precision unsigned integer.
#[derive(Debug, Clone, Default)]
pub struct Word(pub Arc<Mutex<Option<u64>>>);

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for Word {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for Word {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Word> for u64 {
    fn eq(&self, other: &Word) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Word> for u64 {
    fn partial_cmp(&self, other: &Word) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Word {
    type Output = Word;
    fn add(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for Word {
    type Output = Word;
    fn add(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Word> for u64 {
    type Output = Word;
    fn add(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Word {
    type Output = Word;
    fn sub(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for Word {
    type Output = Word;
    fn sub(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Word> for u64 {
    type Output = Word;
    fn sub(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Word {
    type Output = Word;
    fn mul(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for Word {
    type Output = Word;
    fn mul(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Word> for u64 {
    type Output = Word;
    fn mul(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Word {
    type Output = Word;
    fn div(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for Word {
    type Output = Word;
    fn div(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Word> for u64 {
    type Output = Word;
    fn div(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Word {
    type Output = Word;
    fn rem(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for Word {
    type Output = Word;
    fn rem(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Word> for u64 {
    type Output = Word;
    fn rem(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Word {
    type Output = Word;
    fn bitand(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for Word {
    type Output = Word;
    fn bitand(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Word> for u64 {
    type Output = Word;
    fn bitand(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Word {
    type Output = Word;
    fn bitor(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for Word {
    type Output = Word;
    fn bitor(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Word> for u64 {
    type Output = Word;
    fn bitor(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Word {
    type Output = Word;
    fn bitxor(self, other: Self) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for Word {
    type Output = Word;
    fn bitxor(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Word> for u64 {
    type Output = Word;
    fn bitxor(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Word {
    type Output = Word;
    fn not(self) -> Word {
        Word(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Word {
    type Output = Word;
    fn shl(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Word {
    type Output = Word;
    fn shl(self, other: i32) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Word {
    type Output = Word;
    fn shl(self, other: i8) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Word {
    type Output = Word;
    fn shl(self, other: i16) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Word {
    type Output = Word;
    fn shl(self, other: i64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Word {
    type Output = Word;
    fn shl(self, other: u32) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Word {
    type Output = Word;
    fn shl(self, other: u8) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Word {
    type Output = Word;
    fn shl(self, other: u16) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Word {
    type Output = Word;
    fn shl(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Word {
    type Output = Word;
    fn shl(self, other: usize) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Word {
    type Output = Word;
    fn shr(self, other: Word) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Word {
    type Output = Word;
    fn shr(self, other: i32) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Word {
    type Output = Word;
    fn shr(self, other: i8) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Word {
    type Output = Word;
    fn shr(self, other: i16) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Word {
    type Output = Word;
    fn shr(self, other: i64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Word {
    type Output = Word;
    fn shr(self, other: u32) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Word {
    type Output = Word;
    fn shr(self, other: u8) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Word {
    type Output = Word;
    fn shr(self, other: u16) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Word {
    type Output = Word;
    fn shr(self, other: u64) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Word {
    type Output = Word;
    fn shr(self, other: usize) -> Word {
        Word(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Word {}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// z1<<_W + z0 = x*y
pub fn mul_w_w(x: Arc<Mutex<Option<Word>>>, y: Arc<Mutex<Option<Word>>>) -> (Arc<Mutex<Option<Word>>>, Arc<Mutex<Option<Word>>>) {
    let mut z1: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut z0: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));

    let (mut hi, mut lo) = math_bits::mul(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))));
    (Arc::new(Mutex::new(Some(Word(Arc::new(Mutex::new(Some(hi as u64))))))), Arc::new(Mutex::new(Some(Word(Arc::new(Mutex::new(Some(lo as u64))))))))
}

/// nlz returns the number of leading zeros in x.
/// Wraps bits.LeadingZeros call for convenience.
pub fn nlz(x: Arc<Mutex<Option<Word>>>) -> u64 {
    (*Arc::new(Mutex::new(Some(math_bits::leading_zeros(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))) as u64))).lock().unwrap().as_ref().unwrap())
}

/// q = ( x1 << _W + x0 - r)/y. m = floor(( _B^2 - 1 ) / d - _B). Requiring x1<y.
/// An approximate reciprocal with a reference to "Improved Division by Invariant Integers
/// (IEEE Transactions on Computers, 11 Jun. 2010)"
pub fn div_w_w(mut x1: Arc<Mutex<Option<Word>>>, mut x0: Arc<Mutex<Option<Word>>>, mut y: Arc<Mutex<Option<Word>>>, m: Arc<Mutex<Option<Word>>>) -> (Arc<Mutex<Option<Word>>>, Arc<Mutex<Option<Word>>>) {
    let mut q: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut r: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));

    let mut s = nlz(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = s; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = Word(Arc::new(Mutex::new(Some((((*{ let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) << s) | ((*{ let __v = (*x0.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = __W as u64; let __tmp_y = s; __tmp_x - __tmp_y }))))))); *x1.lock().unwrap() = Some(new_val); };
        { let __rhs = s; let mut guard = x0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
        { let __rhs = s; let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
    }
    let mut d = Arc::new(Mutex::new(Some((*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)));

        // We know that
        //   m = ⎣(B^2-1)/d⎦-B
        //   ⎣(B^2-1)/d⎦ = m+B
        //   (B^2-1)/d = m+B+delta1    0 <= delta1 <= (d-1)/d
        //   B^2/d = m+B+delta2        0 <= delta2 <= 1
        // The quotient we're trying to compute is
        //   quotient = ⎣(x1*B+x0)/d⎦
        //            = ⎣(x1*B*(B^2/d)+x0*(B^2/d))/B^2⎦
        //            = ⎣(x1*B*(m+B+delta2)+x0*(m+B+delta2))/B^2⎦
        //            = ⎣(x1*m+x1*B+x0)/B + x0*m/B^2 + delta2*(x1*B+x0)/B^2⎦
        // The latter two terms of this three-term sum are between 0 and 1.
        // So we can compute just the first term, and we will be low by at most 2.
    let (mut t1, mut t0) = math_bits::mul(Arc::new(Mutex::new(Some((*{ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))));
    let (_, mut c) = math_bits::add(Arc::new(Mutex::new(Some(t0))), Arc::new(Mutex::new(Some((*{ let __v = (*x0.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some(0 as u64))));
    { let (__tmp_0, __tmp_1) = math_bits::add(Arc::new(Mutex::new(Some(t1))), Arc::new(Mutex::new(Some((*{ let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some(c)))); t1 = __tmp_0; };

        // The quotient is either t1, t1+1, or t1+2.
        // We'll try t1 and adjust if needed.
    let mut qq = Arc::new(Mutex::new(Some(t1)));

        // compute remainder r=x-d*q.
    let (mut dq1, mut dq0) = math_bits::mul(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = qq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut r0, mut b) = math_bits::sub(Arc::new(Mutex::new(Some((*{ let __v = (*x0.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some(dq0))), Arc::new(Mutex::new(Some(0 as u64))));
    let (mut r1, _) = math_bits::sub(Arc::new(Mutex::new(Some((*{ let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some(dq1))), Arc::new(Mutex::new(Some(b))));

        // The remainder we just computed is bounded above by B+d:
        // r = x1*B + x0 - d*q.
        //   = x1*B + x0 - d*⎣(x1*m+x1*B+x0)/B⎦
        //   = x1*B + x0 - d*((x1*m+x1*B+x0)/B-alpha)                                   0 <= alpha < 1
        //   = x1*B + x0 - x1*d/B*m                         - x1*d - x0*d/B + d*alpha
        //   = x1*B + x0 - x1*d/B*⎣(B^2-1)/d-B⎦             - x1*d - x0*d/B + d*alpha
        //   = x1*B + x0 - x1*d/B*⎣(B^2-1)/d-B⎦             - x1*d - x0*d/B + d*alpha
        //   = x1*B + x0 - x1*d/B*((B^2-1)/d-B-beta)        - x1*d - x0*d/B + d*alpha   0 <= beta < 1
        //   = x1*B + x0 - x1*B + x1/B + x1*d + x1*d/B*beta - x1*d - x0*d/B + d*alpha
        //   =        x0        + x1/B        + x1*d/B*beta        - x0*d/B + d*alpha
        //   = x0*(1-d/B) + x1*(1+d*beta)/B + d*alpha
        //   <  B*(1-d/B) +  d*B/B          + d          because x0<B (and 1-d/B>0), x1<d, 1+d*beta<=B, alpha<1
        //   =  B - d     +  d              + d
        //   = B+d
        // So r1 can only be 0 or 1. If r1 is 1, then we know q was too small.
        // Add 1 to q and subtract d from r. That guarantees that r is <B, so
        // we no longer need to keep track of r1.
    if { let __tmp_x = r1; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let mut guard = qq.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = (*d.lock().unwrap().as_ref().unwrap()); r0 = r0 - __rhs; };
    }

        // If the remainder is still too large, increment q one more time.
    if { let __tmp_x = r0; let __tmp_y = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let mut guard = qq.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = (*d.lock().unwrap().as_ref().unwrap()); r0 = r0 - __rhs; };
    }
    return (Arc::new(Mutex::new(Some(Word(Arc::new(Mutex::new(Some((*qq.lock().unwrap().as_ref().unwrap()) as u64))))))), Arc::new(Mutex::new(Some(Word(Arc::new(Mutex::new(Some({ let __tmp_x = r0; let __tmp_y = s; __tmp_x >> __tmp_y } as u64))))))));
}

/// reciprocalWord return the reciprocal of the divisor. rec = floor(( _B^2 - 1 ) / u - _B). u = d1 << nlz(d1).
pub fn reciprocal_word(d1: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<Word>>> {
    let mut u = Arc::new(Mutex::new(Some((((*{ let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) << nlz(Arc::new(Mutex::new(Some({ let __arg_holder = d1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))))) as u64)));
    let mut x1 = Arc::new(Mutex::new(Some(!(*u.lock().unwrap().as_ref().unwrap()))));
    let mut x0 = Arc::new(Mutex::new(Some(__M as u64)));
    let (mut rec, _) = math_bits::div(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    Arc::new(Mutex::new(Some(Word(Arc::new(Mutex::new(Some(rec as u64)))))))
}