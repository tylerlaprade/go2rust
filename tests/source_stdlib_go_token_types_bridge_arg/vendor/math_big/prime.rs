use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
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
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: GoMutex,
    pub table: Arc<Mutex<Option<[divisor; 64]>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), table: { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: GoMutex::new(), table: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.table))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type cacheBase10 = AnonymousStruct1;


impl crate::int::Int {
    /// ProbablyPrime reports whether x is probably prime,
    /// applying the Miller-Rabin test with n pseudorandomly chosen bases
    /// as well as a Baillie-PSW test.
    ///
    /// If x is prime, ProbablyPrime returns true.
    /// If x is chosen randomly and not prime, ProbablyPrime probably returns false.
    /// The probability of returning true for a randomly chosen non-prime is at most ¼ⁿ.
    ///
    /// ProbablyPrime is 100% accurate for inputs less than 2⁶⁴.
    /// See Menezes et al., Handbook of Applied Cryptography, 1997, pp. 145-149,
    /// and FIPS 186-4 Appendix F for further discussion of the error probabilities.
    ///
    /// ProbablyPrime is not suitable for judging primes that an adversary may
    /// have crafted to fool the test.
    ///
    /// As of Go 1.8, ProbablyPrime(0) is allowed and applies only a Baillie-PSW test.
    /// Before Go 1.8, ProbablyPrime applied only the Miller-Rabin tests, and ProbablyPrime(0) panicked.
    pub fn probably_prime(&self, n: Arc<Mutex<Option<i32>>>) -> bool {
                // Note regarding the doc comment above:
                // It would be more precise to say that the Baillie-PSW test uses the
                // extra strong Lucas test as its Lucas test, but since no one knows
                // how to tell any of the Lucas tests apart inside a Baillie-PSW test
                // (they all work equally well empirically), that detail need not be
                // documented or implicitly guaranteed.
                // The comment does avoid saying "the" Baillie-PSW test
                // because of this general ambiguity.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        panic!("negative n for ProbablyPrime");
    }
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) || { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
                // primeBitMask records the primes < 64.
        const primeBitMask: u64 = (((((((((((((((((((1 as u64) << (2 as u64)) | ((1 as u64) << (3 as u64))) | ((1 as u64) << (5 as u64))) | ((1 as u64) << (7 as u64))) | ((1 as u64) << (11 as u64))) | ((1 as u64) << (13 as u64))) | ((1 as u64) << (17 as u64))) | ((1 as u64) << (19 as u64))) | ((1 as u64) << (23 as u64))) | ((1 as u64) << (29 as u64))) | ((1 as u64) << (31 as u64))) | ((1 as u64) << (37 as u64))) | ((1 as u64) << (41 as u64))) | ((1 as u64) << (43 as u64))) | ((1 as u64) << (47 as u64))) | ((1 as u64) << (53 as u64))) | ((1 as u64) << (59 as u64))) | ((1 as u64) << (61 as u64)));

        let mut w = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = (*w.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(64 as u64)))); __tmp_x < __tmp_y } {
        return { let __tmp_x = { let __tmp_x = primeBitMask as u64; let __tmp_y = ({ let __tmp_x = (1 as u64); let __tmp_y = (*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y };
    }
        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        return false;
    }
                // x is even
        const primesA: i64 = 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 37;

        const primesB: i64 = 29 * 31 * 41 * 43 * 47 * 53;

        let mut rA: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut rB: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = __W;
    if _switch_val == (32) {
            { let new_val = Arc::new(Mutex::new(Some((*(*(*self.abs.lock().unwrap().as_ref().unwrap()).mod_w(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(primesA as u64)))))))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rA.lock().unwrap() = __moved_val; };
            { let new_val = Arc::new(Mutex::new(Some((*(*(*self.abs.lock().unwrap().as_ref().unwrap()).mod_w(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(primesB as u64)))))))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rB.lock().unwrap() = __moved_val; };
        } else if _switch_val == (64) {
            let mut r = (*self.abs.lock().unwrap().as_ref().unwrap()).mod_w(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((((primesA as u64) * (primesB as u64)) & (__M as u64)) as u64))))))));
            { let new_val = Arc::new(Mutex::new(Some((((*{ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) % primesA as u64)) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rA.lock().unwrap() = __moved_val; };
            { let new_val = Arc::new(Mutex::new(Some((((*{ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) % primesB as u64)) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rB.lock().unwrap() = __moved_val; };
        } else {
            panic!("math/big: invalid word size");
        }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 11 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 13 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 17 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 19 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 23 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 37 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 29 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 41 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 43 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 47 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*rB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 53 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return false;
    }
        (*self.abs.lock().unwrap().as_ref().unwrap()).probably_prime_miller_rabin(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(true)))) && (*self.abs.lock().unwrap().as_ref().unwrap()).probably_prime_lucas()
    }
}

impl crate::nat::nat {
    /// probablyPrimeMillerRabin reports whether n passes reps rounds of the
    /// Miller-Rabin primality test, using pseudo-randomly chosen bases.
    /// If force2 is true, one of the rounds is forced to use base 2.
    /// See Handbook of Applied Cryptography, p. 139, Algorithm 4.24.
    /// The number n is known to be non-zero.
    pub fn probably_prime_miller_rabin(&self, reps: Arc<Mutex<Option<i32>>>, force2: Arc<Mutex<Option<bool>>>) -> bool {
        let mut nm1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub(Arc::new(Mutex::new(Some(self.clone()))), natOne.clone());
                // determine q, k such that nm1 = q << k
        let mut k = (*nm1.lock().unwrap().as_ref().unwrap()).trailing_zero_bits();
        let mut q = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shr(nm1.clone(), Arc::new(Mutex::new(Some(k))));
        let mut nm3 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub(nm1.clone(), natTwo.clone());
        let mut rand = rand::new(rand::new_source(Arc::new(Mutex::new(Some((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as i64)))));
        let mut x: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut y: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut quotient: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut nm3Len = (*nm3.lock().unwrap().as_ref().unwrap()).bit_len();
        let mut i = Arc::new(Mutex::new(Some(0)));
    'next_random: while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*reps.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*reps.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; __tmp_x == __tmp_y } && { let __v = (*force2.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = (*x.lock().unwrap().as_ref().unwrap()).set(natTwo.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*x.lock().unwrap().as_ref().unwrap()).random(rand.clone(), nm3.clone(), Arc::new(Mutex::new(Some(nm3Len)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
        { let new_val = (*x.lock().unwrap().as_ref().unwrap()).add(x.clone(), natTwo.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    }
        { let new_val = (*y.lock().unwrap().as_ref().unwrap()).exp_n_n(x.clone(), q.clone(), Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some(false)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).cmp(natOne.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).cmp(nm1.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let mut j = Arc::new(Mutex::new(Some(1 as u64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = k; __tmp_x < __tmp_y } {
        { let new_val = (*y.lock().unwrap().as_ref().unwrap()).sqr(y.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = (*quotient.lock().unwrap().as_ref().unwrap()).div(y.clone(), y.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *quotient.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_tmp_1; };
        if { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).cmp(nm1.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'next_random
    }
        if { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).cmp(natOne.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return false;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        true
    }

    /// probablyPrimeLucas reports whether n passes the "almost extra strong" Lucas probable prime test,
    /// using Baillie-OEIS parameter selection. This corresponds to "AESLPSP" on Jacobsen's tables (link below).
    /// The combination of this test and a Miller-Rabin/Fermat test with base 2 gives a Baillie-PSW test.
    ///
    /// References:
    ///
    /// Baillie and Wagstaff, "Lucas Pseudoprimes", Mathematics of Computation 35(152),
    /// October 1980, pp. 1391-1417, especially page 1401.
    /// https://www.ams.org/journals/mcom/1980-35-152/S0025-5718-1980-0583518-6/S0025-5718-1980-0583518-6.pdf
    ///
    /// Grantham, "Frobenius Pseudoprimes", Mathematics of Computation 70(234),
    /// March 2000, pp. 873-891.
    /// https://www.ams.org/journals/mcom/2001-70-234/S0025-5718-00-01197-2/S0025-5718-00-01197-2.pdf
    ///
    /// Baillie, "Extra strong Lucas pseudoprimes", OEIS A217719, https://oeis.org/A217719.
    ///
    /// Jacobsen, "Pseudoprime Statistics, Tables, and Data", http://ntheory.org/pseudoprimes.html.
    ///
    /// Nicely, "The Baillie-PSW Primality Test", https://web.archive.org/web/20191121062007/http://www.trnicely.net/misc/bpsw.html.
    /// (Note that Nicely's definition of the "extra strong" test gives the wrong Jacobi condition,
    /// as pointed out by Jacobsen.)
    ///
    /// Crandall and Pomerance, Prime Numbers: A Computational Perspective, 2nd ed.
    /// Springer, 2005.
    pub fn probably_prime_lucas(&self) -> bool {
                // Discard 0, 1.
        if { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = self.cmp(natOne.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
                // Two is the only even prime.
                // Already checked by caller, but here to allow testing in isolation.
        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        return { let __tmp_x = self.cmp(natTwo.clone()); let __tmp_y = 0; __tmp_x == __tmp_y };
    }
                // Baillie-OEIS "method C" for choosing D, P, Q,
                // as in https://oeis.org/A217719/a217719.txt:
                // try increasing P ≥ 3 such that D = P² - 4 (so Q = 1)
                // until Jacobi(D, n) = -1.
                // The search is expected to succeed for non-square n after just a few trials.
                // After more than expected failures, check whether n is square
                // (which would cause Jacobi(D, n) = 1 for all D not dividing n).
        let mut p = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(3 as u64)))))));
        let mut d = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))])))))));
        let mut t1 = Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))))));
        let mut intD = Arc::new(Mutex::new(Some(Int { abs: d.clone(), ..Default::default() })));
        let mut intN = Arc::new(Mutex::new(Some(Int { abs: Arc::new(Mutex::new(Some(self.clone()))), ..Default::default() })));
        loop {
        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(10000 as u64)))); __tmp_x > __tmp_y } {
                // This is widely believed to be impossible.
                // If we get a report, we'll want the exact number n.
        panic!("{}", format!("{}{}", "math/big: internal error: cannot find (D/n) = -1 for ".to_string(), (*{ let __recv = intN.clone(); let __recv_ptr: *const crate::int::Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::int::Int }; let __result = unsafe { &*__recv_ptr }.string(); __result }.lock().unwrap().as_ref().unwrap())));
    }
                // This is widely believed to be impossible.
                // If we get a report, we'll want the exact number n.
        (*{ let __named_slice = (*d.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * (*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) - 4)))));
        let mut j = jacobi(intD.clone(), intN.clone());
        if { let __tmp_x = j; let __tmp_y = -1; __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = j; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // d = p²-4 = (p-2)(p+2).
                // If (d/n) == 0 then d shares a prime factor with n.
                // Since the loop proceeds in increasing p and starts with p-2==1,
                // the shared prime factor must be p+2.
                // If p+2 == n, then n is prime; otherwise p+2 is a proper factor of n.
        return { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(2 as u64)))); __tmp_x + __tmp_y }; __tmp_x == __tmp_y };
    }
                // d = p²-4 = (p-2)(p+2).
                // If (d/n) == 0 then d shares a prime factor with n.
                // Since the loop proceeds in increasing p and starts with p-2==1,
                // the shared prime factor must be p+2.
                // If p+2 == n, then n is prime; otherwise p+2 is a proper factor of n.
        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(40 as u64)))); __tmp_x == __tmp_y } {
                // We'll never find (d/n) = -1 if n is a square.
                // If n is a non-square we expect to find a d in just a few attempts on average.
                // After 40 attempts, take a moment to check if n is indeed a square.
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sqrt(Arc::new(Mutex::new(Some(self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sqr(t1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*t1.lock().unwrap().as_ref().unwrap()).cmp(Arc::new(Mutex::new(Some(self.clone())))); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
    }
        { let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }
                // This is widely believed to be impossible.
                // If we get a report, we'll want the exact number n.
                // d = p²-4 = (p-2)(p+2).
                // If (d/n) == 0 then d shares a prime factor with n.
                // Since the loop proceeds in increasing p and starts with p-2==1,
                // the shared prime factor must be p+2.
                // If p+2 == n, then n is prime; otherwise p+2 is a proper factor of n.
                // We'll never find (d/n) = -1 if n is a square.
                // If n is a non-square we expect to find a d in just a few attempts on average.
                // After 40 attempts, take a moment to check if n is indeed a square.
                // Grantham definition of "extra strong Lucas pseudoprime", after Thm 2.3 on p. 876
                // (D, P, Q above have become Δ, b, 1):
                //
                // Let U_n = U_n(b, 1), V_n = V_n(b, 1), and Δ = b²-4.
                // An extra strong Lucas pseudoprime to base b is a composite n = 2^r s + Jacobi(Δ, n),
                // where s is odd and gcd(n, 2*Δ) = 1, such that either (i) U_s ≡ 0 mod n and V_s ≡ ±2 mod n,
                // or (ii) V_{2^t s} ≡ 0 mod n for some 0 ≤ t < r-1.
                //
                // We know gcd(n, Δ) = 1 or else we'd have found Jacobi(d, n) == 0 above.
                // We know gcd(n, 2) = 1 because n is odd.
                //
                // Arrange s = (n - Jacobi(Δ, n)) / 2^r = (n+1) / 2^r.
        let mut s = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).add(Arc::new(Mutex::new(Some(self.clone()))), natOne.clone());
        let mut r = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).trailing_zero_bits() as i32)));
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).shr(s.clone(), Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        let mut nm2 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub(Arc::new(Mutex::new(Some(self.clone()))), natTwo.clone());
                // We apply the "almost extra strong" test, which checks the above conditions
                // except for U_s ≡ 0 mod n, which allows us to avoid computing any U_k values.
                // Jacobsen points out that maybe we should just do the full extra strong test:
                // "It is also possible to recover U_n using Crandall and Pomerance equation 3.13:
                // U_n = D^-1 (2V_{n+1} - PV_n) allowing us to run the full extra-strong test
                // at the cost of a single modular inversion. This computation is easy and fast in GMP,
                // so we can get the full extra-strong test at essentially the same performance as the
                // almost extra strong test."
                // Compute Lucas sequence V_s(b, 1), where:
                //
                //	V(0) = 2
                //	V(1) = P
                //	V(k) = P V(k-1) - Q V(k-2).
                //
                // (Remember that due to method C above, P = b, Q = 1.)
                //
                // In general V(k) = α^k + β^k, where α and β are roots of x² - Px + Q.
                // Crandall and Pomerance (p.147) observe that for 0 ≤ j ≤ k,
                //
                //	V(j+k) = V(j)V(k) - V(k-j).
                //
                // So in particular, to quickly double the subscript:
                //
                //	V(2k) = V(k)² - 2
                //	V(2k+1) = V(k) V(k+1) - P
                //
                // We can therefore start with k=0 and build up to k=s in log₂(s) steps.
        let mut natP = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut vk = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(2 as u64))))))));
        let mut vk1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut t2 = Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))))));
        let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).bit_len() as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).bit(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64)))); let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // k' = 2k+1
                // V(k') = V(2k+1) = V(k) V(k+1) - P.
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).mul(vk.clone(), vk1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).add(t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sub(t1.clone(), natP.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = (*t2.lock().unwrap().as_ref().unwrap()).div(vk.clone(), t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t2.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *vk.lock().unwrap() = __moved_tmp_1; };
                // V(k'+1) = V(2k+2) = V(k+1)² - 2.
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sqr(vk1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).add(t1.clone(), nm2.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = (*t2.lock().unwrap().as_ref().unwrap()).div(vk1.clone(), t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t2.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *vk1.lock().unwrap() = __moved_tmp_1; };
    } else {
                // k' = 2k
                // V(k'+1) = V(2k+1) = V(k) V(k+1) - P.
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).mul(vk.clone(), vk1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).add(t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sub(t1.clone(), natP.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = (*t2.lock().unwrap().as_ref().unwrap()).div(vk1.clone(), t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t2.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *vk1.lock().unwrap() = __moved_tmp_1; };
                // V(k') = V(2k) = V(k)² - 2
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sqr(vk.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).add(t1.clone(), nm2.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = (*t2.lock().unwrap().as_ref().unwrap()).div(vk.clone(), t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t2.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *vk.lock().unwrap() = __moved_tmp_1; };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // k' = 2k+1
                // V(k') = V(2k+1) = V(k) V(k+1) - P.
                // V(k'+1) = V(2k+2) = V(k+1)² - 2.
                // k' = 2k
                // V(k'+1) = V(2k+1) = V(k) V(k+1) - P.
                // V(k') = V(2k) = V(k)² - 2
                // Now k=s, so vk = V(s). Check V(s) ≡ ±2 (mod n).
        if { let __tmp_x = (*vk.lock().unwrap().as_ref().unwrap()).cmp(natTwo.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (*vk.lock().unwrap().as_ref().unwrap()).cmp(nm2.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Check U(s) ≡ 0.
                // As suggested by Jacobsen, apply Crandall and Pomerance equation 3.13:
                //
                //	U(k) = D⁻¹ (2 V(k+1) - P V(k))
                //
                // Since we are checking for U(k) == 0 it suffices to check 2 V(k+1) == P V(k) mod n,
                // or P V(k) - 2 V(k+1) == 0 mod n.
        let mut t1 = (*t1.lock().unwrap().as_ref().unwrap()).mul(vk.clone(), natP.clone());
        let mut t2 = (*t2.lock().unwrap().as_ref().unwrap()).shl(vk1.clone(), Arc::new(Mutex::new(Some(1 as u64))));
        if { let __tmp_x = (*t1.lock().unwrap().as_ref().unwrap()).cmp(t2.clone()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __tmp_0 = t2.clone(); let __tmp_1 = t1.clone(); *t1.lock().unwrap() = __tmp_0.lock().unwrap().take(); *t2.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sub(t1.clone(), t2.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        let mut t3 = Arc::new(Mutex::new(Some({ let __v = (*vk1.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        *vk1.lock().unwrap() = None;
        let _ = { let __v = (*vk1.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let (__tmp_0, __tmp_1) = (*t2.lock().unwrap().as_ref().unwrap()).div(t3.clone(), t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t2.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *t3.lock().unwrap() = __moved_tmp_1; };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*t3.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return true;
    }
    }
                // Check U(s) ≡ 0.
                // As suggested by Jacobsen, apply Crandall and Pomerance equation 3.13:
                //
                //	U(k) = D⁻¹ (2 V(k+1) - P V(k))
                //
                // Since we are checking for U(k) == 0 it suffices to check 2 V(k+1) == P V(k) mod n,
                // or P V(k) - 2 V(k+1) == 0 mod n.
                // steal vk1, no longer needed below
                // Check V(2^t s) ≡ 0 mod n for some 0 ≤ t < r-1.
        let mut t = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*vk.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return true;
    }

                // Optimization: V(k) = 2 is a fixed point for V(k') = V(k)² - 2,
                // so if V(k) = 2, we can stop: we will never find a future V(k) == 0.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*vk.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq_holder = { let __named_slice = (*vk.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(2 as u64)))); __tmp_x == __tmp_y } {
        return false;
    }

                // k' = 2k
                // V(k') = V(2k) = V(k)² - 2
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sqr(vk.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let new_val = (*t1.lock().unwrap().as_ref().unwrap()).sub(t1.clone(), natTwo.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t1.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = (*t2.lock().unwrap().as_ref().unwrap()).div(vk.clone(), t1.clone(), Arc::new(Mutex::new(Some(self.clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t2.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *vk.lock().unwrap() = __moved_tmp_1; };
        { let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // vk == 0
                // Optimization: V(k) = 2 is a fixed point for V(k') = V(k)² - 2,
                // so if V(k) = 2, we can stop: we will never find a future V(k) == 0.
                // vk == 2
                // k' = 2k
                // V(k') = V(2k) = V(k)² - 2
        false
    }
}