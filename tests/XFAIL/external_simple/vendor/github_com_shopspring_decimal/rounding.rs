use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct floatInfo {
    mantbits: Rc<RefCell<Option<u32>>>,
    expbits: Rc<RefCell<Option<u32>>>,
    bias: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for floatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.mantbits.borrow().as_ref().unwrap()), (*self.expbits.borrow().as_ref().unwrap()), (*self.bias.borrow().as_ref().unwrap()))
    }
}


/// roundShortest rounds d (= mant * 2^exp) to the shortest number of digits
/// that will let the original floating point value be precisely reconstructed.
pub fn round_shortest(d: Rc<RefCell<Option<decimal>>>, mant: Rc<RefCell<Option<u64>>>, exp: Rc<RefCell<Option<i32>>>, flt: Rc<RefCell<Option<floatInfo>>>) {
        // If mantissa is zero, the number is zero; stop now.
    if (*mant.borrow_mut().as_mut().unwrap()) == 0 {
        { let new_val = 0; *(*d.borrow_mut().as_mut().unwrap()).nd.borrow_mut() = Some(new_val); };
        return;
    }

        // Compute upper and lower such that any decimal number
        // between upper and lower (possibly inclusive)
        // will round to the original floating point number.
        // We may see at once that the number is already shortest.
        //
        // Suppose d is not denormal, so that 2^exp <= d < 10^dp.
        // The closest shorter number is at least 10^(dp-nd) away.
        // The lower/upper bounds computed below are at distance
        // at most 2^(exp-mantbits).
        //
        // So the number is already shortest if 10^(dp-nd) > 2^(exp-mantbits),
        // or equivalently log2(10)*(dp-nd) > exp-mantbits.
        // It is true if 332/100*(dp-nd) >= exp-mantbits (log2(10) > 3.32).
    let mut minexp = Rc::new(RefCell::new(Some((*(*(*flt.borrow().as_ref().unwrap()).bias.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) + 1)));
    if (*exp.borrow_mut().as_mut().unwrap()) > (*minexp.borrow_mut().as_mut().unwrap()) && 332 * ((*(*(*d.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) - (*(*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap())) >= 100 * ((*exp.borrow_mut().as_mut().unwrap()) - (*(*int.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap())) {
                // The number is already shortest.
        return;
    }

        // The number is already shortest.
        // d = mant << (exp - mantbits)
        // Next highest floating point number is mant+1 << exp-mantbits.
        // Our upper bound is halfway between, mant*2+1 << exp-mantbits-1.
    let mut upper = Rc::new(RefCell::new(Some(Rc<RefCell<Option<decimal>>>::default())));
    (*upper.borrow_mut().as_mut().unwrap()).assign(Rc::new(RefCell::new(Some((*mant.borrow_mut().as_mut().unwrap()) * 2 + 1))));
    (*upper.borrow_mut().as_mut().unwrap()).shift(Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap()) - (*(*int.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) - 1))));

        // d = mant << (exp - mantbits)
        // Next lowest floating point number is mant-1 << exp-mantbits,
        // unless mant-1 drops the significant bit and exp is not the minimum exp,
        // in which case the next lowest is mant*2-1 << exp-mantbits-1.
        // Either way, call it mantlo << explo-mantbits.
        // Our lower bound is halfway between, mantlo*2+1 << explo-mantbits-1.
    let mut mantlo: Rc<RefCell<Option<u64>>> = Default::default();
    let mut explo: Rc<RefCell<Option<i32>>> = 0;
    if (*mant.borrow_mut().as_mut().unwrap()) > 1 << (*(*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) || (*exp.borrow_mut().as_mut().unwrap()) == (*minexp.borrow_mut().as_mut().unwrap()) {
        { let new_val = (*mant.borrow_mut().as_mut().unwrap()) - 1; *mantlo.borrow_mut() = Some(new_val); };
        { let new_val = (*exp.borrow_mut().as_mut().unwrap()); *explo.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*mant.borrow_mut().as_mut().unwrap()) * 2 - 1; *mantlo.borrow_mut() = Some(new_val); };
        { let new_val = (*exp.borrow_mut().as_mut().unwrap()) - 1; *explo.borrow_mut() = Some(new_val); };
    }
    let mut lower = Rc::new(RefCell::new(Some(Rc<RefCell<Option<decimal>>>::default())));
    (*lower.borrow_mut().as_mut().unwrap()).assign(Rc::new(RefCell::new(Some((*mantlo.borrow_mut().as_mut().unwrap()) * 2 + 1))));
    (*lower.borrow_mut().as_mut().unwrap()).shift(Rc::new(RefCell::new(Some((*explo.borrow_mut().as_mut().unwrap()) - (*(*int.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) - 1))));

        // The upper and lower bounds are possible outputs only if
        // the original mantissa is even, so that IEEE round-to-even
        // would round to the original mantissa and not the neighbors.
    let mut inclusive = Rc::new(RefCell::new(Some((*mant.borrow_mut().as_mut().unwrap()) % 2 == 0)));

        // As we walk the digits we want to know whether rounding up would fall
        // within the upper bound. This is tracked by upperdelta:
        //
        // If upperdelta == 0, the digits of d and upper are the same so far.
        //
        // If upperdelta == 1, we saw a difference of 1 between d and upper on a
        // previous digit and subsequently only 9s for d and 0s for upper.
        // (Thus rounding up may fall outside the bound, if it is exclusive.)
        //
        // If upperdelta == 2, then the difference is greater than 1
        // and we know that rounding up falls within the bound.
    let mut upperdelta: Rc<RefCell<Option<u8>>> = Default::default();

        // Now we can figure out the minimum number of digits required.
        // Walk along until d has distinguished itself from upper and lower.
    let mut ui = Rc::new(RefCell::new(Some(0)));
    while true {
                // lower, d, and upper may have the decimal points at different
                // places. In this case upper is the longest, so we iterate from
                // ui==0 and start li and mi at (possibly) -1.
        let mut mi = Rc::new(RefCell::new(Some((*ui.borrow_mut().as_mut().unwrap()) - (*(*(*upper.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) + (*(*(*d.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()))));
        if (*mi.borrow_mut().as_mut().unwrap()) >= (*(*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        break
    }
        let mut li = Rc::new(RefCell::new(Some((*ui.borrow_mut().as_mut().unwrap()) - (*(*(*upper.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) + (*(*(*lower.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()))));
        let mut l = (*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(('0' as i32)))));
        if (*li.borrow_mut().as_mut().unwrap()) >= 0 && (*li.borrow_mut().as_mut().unwrap()) < (*(*(*lower.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        { let new_val = (*(*lower.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*li.borrow_mut().as_mut().unwrap()) as usize].clone(); *l.borrow_mut() = Some(new_val); };
    }
        let mut m = (*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(('0' as i32)))));
        if (*mi.borrow_mut().as_mut().unwrap()) >= 0 {
        { let new_val = (*(*d.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*mi.borrow_mut().as_mut().unwrap()) as usize].clone(); *m.borrow_mut() = Some(new_val); };
    }
        let mut u = (*byte.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(('0' as i32)))));
        if (*ui.borrow_mut().as_mut().unwrap()) < (*(*(*upper.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        { let new_val = (*(*upper.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*ui.borrow_mut().as_mut().unwrap()) as usize].clone(); *u.borrow_mut() = Some(new_val); };
    }

                // Okay to round down (truncate) if lower has a different digit
                // or if lower is inclusive and is exactly the result of rounding
                // down (i.e., and we have reached the final digit of lower).
        let mut okdown = Rc::new(RefCell::new(Some((*l.borrow_mut().as_mut().unwrap()) != (*m.borrow_mut().as_mut().unwrap()) || (*inclusive.borrow_mut().as_mut().unwrap()) && (*li.borrow_mut().as_mut().unwrap()) + 1 == (*(*(*lower.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()))));

        match true {
        true if (*upperdelta.borrow_mut().as_mut().unwrap()) == 0 && (*m.borrow_mut().as_mut().unwrap()) + 1 < (*u.borrow_mut().as_mut().unwrap()) => {
                        // Example:
                        // m = 12345xxx
                        // u = 12347xxx
            { let new_val = 2; *upperdelta.borrow_mut() = Some(new_val); };
        }
        true if (*upperdelta.borrow_mut().as_mut().unwrap()) == 0 && (*m.borrow_mut().as_mut().unwrap()) != (*u.borrow_mut().as_mut().unwrap()) => {
                        // Example:
                        // m = 12345xxx
                        // u = 12346xxx
            { let new_val = 1; *upperdelta.borrow_mut() = Some(new_val); };
        }
        true if (*upperdelta.borrow_mut().as_mut().unwrap()) == 1 && ((*m.borrow_mut().as_mut().unwrap()) != ('9' as i32) || (*u.borrow_mut().as_mut().unwrap()) != ('0' as i32)) => {
                        // Example:
                        // m = 1234598x
                        // u = 1234600x
            { let new_val = 2; *upperdelta.borrow_mut() = Some(new_val); };
        }
        _ => {}
    }

                // Example:
                // m = 12345xxx
                // u = 12347xxx
                // Example:
                // m = 12345xxx
                // u = 12346xxx
                // Example:
                // m = 1234598x
                // u = 1234600x
                // Okay to round up if upper has a different digit and either upper
                // is inclusive or upper is bigger than the result of rounding up.
        let mut okup = Rc::new(RefCell::new(Some((*upperdelta.borrow_mut().as_mut().unwrap()) > 0 && ((*inclusive.borrow_mut().as_mut().unwrap()) || (*upperdelta.borrow_mut().as_mut().unwrap()) > 1 || (*ui.borrow_mut().as_mut().unwrap()) + 1 < (*(*(*upper.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap())))));

                // If it's okay to do either, then round to the nearest one.
                // If it's okay to do only one, do it.
        match true {
        true if (*okdown.borrow_mut().as_mut().unwrap()) && (*okup.borrow_mut().as_mut().unwrap()) => {
            (*d.borrow_mut().as_mut().unwrap()).round(Rc::new(RefCell::new(Some((*mi.borrow_mut().as_mut().unwrap()) + 1))));
            return;
        }
        true if (*okdown.borrow_mut().as_mut().unwrap()) => {
            (*d.borrow_mut().as_mut().unwrap()).round_down(Rc::new(RefCell::new(Some((*mi.borrow_mut().as_mut().unwrap()) + 1))));
            return;
        }
        true if (*okup.borrow_mut().as_mut().unwrap()) => {
            (*d.borrow_mut().as_mut().unwrap()).round_up(Rc::new(RefCell::new(Some((*mi.borrow_mut().as_mut().unwrap()) + 1))));
            return;
        }
        _ => {}
    }
        { let mut guard = ui.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}