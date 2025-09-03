use std::any::Any;
use std::cell::{RefCell};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Decimal represents a fixed-point decimal. It is immutable.
/// number = value * 10 ^ exp
#[derive(Debug, Clone, Default)]
struct Decimal {
    value: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>,
    exp: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.value.borrow().as_ref().unwrap()), (*self.exp.borrow().as_ref().unwrap()))
    }
}


/// NullDecimal represents a nullable decimal with compatibility for
/// scanning null values from the database.
#[derive(Debug, Clone, Default)]
struct NullDecimal {
    decimal: Rc<RefCell<Option<Decimal>>>,
    valid: Rc<RefCell<Option<bool>>>,
}

impl std::fmt::Display for NullDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.decimal.borrow().as_ref().unwrap()), (*self.valid.borrow().as_ref().unwrap()))
    }
}


impl Decimal {
    /// Copy returns a copy of decimal with the same value and exponent, but a different pointer to value.
    pub fn copy(&self) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some(((*self.value.borrow().as_ref().unwrap())).clone()))), exp: Rc::new(RefCell::new(Some(self.exp.clone()))) })));
    }

    /// rescale returns a rescaled version of the decimal. Returned
    /// decimal may be less precise if the given exponent is bigger
    /// than the initial exponent of the Decimal.
    /// NOTE: this will truncate, NOT round
    ///
    /// Example:
    ///
    /// 	d := New(12345, -4)
    ///	d2 := d.rescale(-1)
    ///	d3 := d2.rescale(-4)
    ///	println(d1)
    ///	println(d2)
    ///	println(d3)
    ///
    /// Output:
    ///
    ///	1.2345
    ///	1.2
    ///	1.2000
    ///
    pub fn rescale(&self, exp: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*self.exp.clone().borrow().as_ref().unwrap()) == (*exp.borrow_mut().as_mut().unwrap()) {
        return Rc::new(RefCell::new(Some(Decimal { ,  })));
    }
                // NOTE(vadim): must convert exps to float64 before - to prevent overflow
        let mut diff = (*math.borrow_mut().as_mut().unwrap()).abs(Rc::new(RefCell::new(Some((*(*float64.borrow().as_ref().unwrap())(exp.clone()).borrow().as_ref().unwrap()) - (*(*float64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone())))).borrow().as_ref().unwrap())))));
        let mut value = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set(Rc::new(RefCell::new(Some(self.value.clone()))));
        let mut expScale = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).exp(Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*int64.borrow().as_ref().unwrap())(diff.clone())))))))), Rc::new(RefCell::new(Some(None))));
        if (*exp.borrow_mut().as_mut().unwrap()) > (*self.exp.clone().borrow().as_ref().unwrap()) {
        { let new_val = (*value.borrow_mut().as_mut().unwrap()).quo(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*expScale.borrow_mut().as_mut().unwrap()))))); *value.borrow_mut() = Some(new_val); };
    } else if (*exp.borrow_mut().as_mut().unwrap()) < (*self.exp.clone().borrow().as_ref().unwrap()) {
        { let new_val = (*value.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*expScale.borrow_mut().as_mut().unwrap()))))); *value.borrow_mut() = Some(new_val); };
    }
        return Rc::new(RefCell::new(Some(Decimal { value: value.clone(), exp: exp.clone() })));
    }

    /// Abs returns the absolute value of the decimal.
    pub fn abs(&self) -> Rc<RefCell<Option<Decimal>>> {
        if !(*d.borrow_mut().as_mut().unwrap()).is_negative() {
        return d.clone();
    }
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        let mut d2Value = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).abs(Rc::new(RefCell::new(Some(self.value.clone()))));
        return Rc::new(RefCell::new(Some(Decimal { value: d2Value.clone(), exp: Rc::new(RefCell::new(Some(self.exp.clone()))) })));
    }

    /// Add returns d + d2.
    pub fn add(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<Decimal>>> {
        let (mut rd, mut rd2) = (*RescalePair.borrow().as_ref().unwrap())(d.clone(), d2.clone());
        let mut d3Value = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).add(Rc::new(RefCell::new(Some((*(*rd.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*(*rd2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))));
        return Rc::new(RefCell::new(Some(Decimal { value: d3Value.clone(), exp: Rc::new(RefCell::new(Some((*(*rd.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap())))) })));
    }

    /// Sub returns d - d2.
    pub fn sub(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<Decimal>>> {
        let (mut rd, mut rd2) = (*RescalePair.borrow().as_ref().unwrap())(d.clone(), d2.clone());
        let mut d3Value = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).sub(Rc::new(RefCell::new(Some((*(*rd.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*(*rd2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))));
        return Rc::new(RefCell::new(Some(Decimal { value: d3Value.clone(), exp: Rc::new(RefCell::new(Some((*(*rd.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap())))) })));
    }

    /// Neg returns -d.
    pub fn neg(&self) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        let mut val = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).neg(Rc::new(RefCell::new(Some(self.value.clone()))));
        return Rc::new(RefCell::new(Some(Decimal { value: val.clone(), exp: Rc::new(RefCell::new(Some(self.exp.clone()))) })));
    }

    /// Mul returns d * d2.
    pub fn mul(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        (*d2.borrow_mut().as_mut().unwrap()).ensure_initialized();
        let mut expInt64 = Rc::new(RefCell::new(Some((*(*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone())))).borrow().as_ref().unwrap()) + (*(*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d2.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()))));
        if (*expInt64.borrow_mut().as_mut().unwrap()) > (*(*math.borrow_mut().as_mut().unwrap())::max_int32.borrow().as_ref().unwrap()) || (*expInt64.borrow_mut().as_mut().unwrap()) < (*(*math.borrow_mut().as_mut().unwrap())::min_int32.borrow().as_ref().unwrap()) {
                // NOTE(vadim): better to panic than give incorrect results, as
                // Decimals are usually used for money
        panic!("{:?}", Rc::new(RefCell::new(Some(format!("exponent {} overflows an int32!", /* ERROR: Type information not available for print argument */ (*expInt64.borrow_mut().as_mut().unwrap()))))));
    }
                // NOTE(vadim): better to panic than give incorrect results, as
                // Decimals are usually used for money
        let mut d3Value = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).mul(Rc::new(RefCell::new(Some(self.value.clone()))), Rc::new(RefCell::new(Some((*(*d2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))));
        return Rc::new(RefCell::new(Some(Decimal { value: d3Value.clone(), exp: Rc::new(RefCell::new(Some((*int32.borrow().as_ref().unwrap())(expInt64.clone())))) })));
    }

    /// Shift shifts the decimal in base 10.
    /// It shifts left when shift is positive and right if shift is negative.
    /// In simpler terms, the given value for shift is added to the exponent
    /// of the decimal.
    pub fn shift(&self, shift: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set(Rc::new(RefCell::new(Some(self.value.clone()))))))), exp: Rc::new(RefCell::new(Some((*self.exp.clone().borrow().as_ref().unwrap()) + (*shift.borrow_mut().as_mut().unwrap())))) })));
    }

    /// Div returns d / d2. If it doesn't divide exactly, the result will have
    /// DivisionPrecision digits after the decimal point.
    pub fn div(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<Decimal>>> {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).div_round(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*int32.borrow().as_ref().unwrap())(DivisionPrecision.clone()))))))));
    }

    /// QuoRem does divsion with remainder
    /// d.QuoRem(d2,precision) returns quotient q and remainder r such that
    ///   d = d2 * q + r, q an integer multiple of 10^(-precision)
    ///   0 <= r < abs(d2) * 10 ^(-precision) if d>=0
    ///   0 >= r > -abs(d2) * 10 ^(-precision) if d<0
    /// Note that precision<0 is allowed as input.
    pub fn quo_rem(&self, d2: Rc<RefCell<Option<Decimal>>>, precision: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<Decimal>>>, Rc<RefCell<Option<Decimal>>>) {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        (*d2.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*(*(*(*d2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) == 0 {
        panic!("decimal division by 0");
    }
        let mut scale = Rc::new(RefCell::new(Some(-(*precision.borrow_mut().as_mut().unwrap()))));
        let mut e = (*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*self.exp.clone().borrow().as_ref().unwrap()) - (*(*(*d2.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) - (*scale.borrow_mut().as_mut().unwrap())))));
        if (*e.borrow_mut().as_mut().unwrap()) > (*(*math.borrow_mut().as_mut().unwrap())::max_int32.borrow().as_ref().unwrap()) || (*e.borrow_mut().as_mut().unwrap()) < (*(*math.borrow_mut().as_mut().unwrap())::min_int32.borrow().as_ref().unwrap()) {
        panic!("overflow in decimal QuoRem");
    }
        let mut aa: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;, let mut bb: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;, let mut expo: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;
        let mut scalerest: Rc<RefCell<Option<i32>>> = Default::default();
                // d = a 10^ea
                // d2 = b 10^eb
        if (*e.borrow_mut().as_mut().unwrap()) < 0 {
        { let new_val = (*self.value.borrow().as_ref().unwrap()); *aa.borrow_mut() = Some(new_val); };
        (*expo.borrow_mut().as_mut().unwrap()).set_int64(Rc::new(RefCell::new(Some(-(*e.borrow_mut().as_mut().unwrap())))));
        (*bb.borrow_mut().as_mut().unwrap()).exp(Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(expo.clone()))), Rc::new(RefCell::new(Some(None))));
        (*bb.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*(*d2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some(bb.clone()))));
        { let new_val = self.exp.clone(); *scalerest.borrow_mut() = Some(new_val); };
    } else {
        (*expo.borrow_mut().as_mut().unwrap()).set_int64(Rc::new(RefCell::new(Some((*e.borrow_mut().as_mut().unwrap())))));
        (*aa.borrow_mut().as_mut().unwrap()).exp(Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(expo.clone()))), Rc::new(RefCell::new(Some(None))));
        (*aa.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(self.value.clone()))), Rc::new(RefCell::new(Some(aa.clone()))));
        { let new_val = (*(*d2.borrow_mut().as_mut().unwrap()).value.borrow().as_ref().unwrap()); *bb.borrow_mut() = Some(new_val); };
        { let new_val = (*scale.borrow_mut().as_mut().unwrap()) + (*(*(*d2.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()); *scalerest.borrow_mut() = Some(new_val); };
    }
                // now aa = a
                //     bb = b 10^(scale + eb - ea)
                // now aa = a ^ (ea - eb - scale)
                //     bb = b
        let mut q: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;, let mut r: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;
        (*q.borrow_mut().as_mut().unwrap()).quo_rem(Rc::new(RefCell::new(Some(aa.clone()))), Rc::new(RefCell::new(Some(bb.clone()))), Rc::new(RefCell::new(Some(r.clone()))));
        let mut dq = Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some(q.clone()))), exp: scale.clone() })));
        let mut dr = Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some(r.clone()))), exp: scalerest.clone() })));
        return (dq.clone(), dr.clone());
    }

    /// DivRound divides and rounds to a given precision
    /// i.e. to an integer multiple of 10^(-precision)
    ///   for a positive quotient digit 5 is rounded up, away from 0
    ///   if the quotient is negative then digit 5 is rounded down, away from 0
    /// Note that precision<0 is allowed as input.
    pub fn div_round(&self, d2: Rc<RefCell<Option<Decimal>>>, precision: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
                // QuoRem already checks initialization
        let (mut q, mut r) = (*d.borrow_mut().as_mut().unwrap()).quo_rem(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*precision.borrow_mut().as_mut().unwrap())))));
                // the actual rounding decision is based on comparing r*10^precision and d2/2
                // instead compare 2 r 10 ^precision and d2
        let mut rv2: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;
        (*rv2.borrow_mut().as_mut().unwrap()).abs(Rc::new(RefCell::new(Some((*(*r.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))));
        (*rv2.borrow_mut().as_mut().unwrap()).lsh(Rc::new(RefCell::new(Some(rv2.clone()))), Rc::new(RefCell::new(Some(1))));
                // now rv2 = abs(r.value) * 2
        let mut r2 = Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some(rv2.clone()))), exp: Rc::new(RefCell::new(Some((*(*(*r.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) + (*precision.borrow_mut().as_mut().unwrap())))) })));
                // r2 is now 2 * r * 10 ^ precision
        let mut c = (*r2.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()).abs()))));
        if (*c.borrow_mut().as_mut().unwrap()) < 0 {
        return q.clone();
    }
        if (*(*self.value.clone().borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) * (*(*(*(*d2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        return Rc::new(RefCell::new(Some((*q.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(-(*precision.borrow_mut().as_mut().unwrap()))))))))))));
    }
        return Rc::new(RefCell::new(Some((*q.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(-(*precision.borrow_mut().as_mut().unwrap()))))))))))));
    }

    /// Mod returns d % d2.
    pub fn r#mod(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<Decimal>>> {
        let mut quo = (*d.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()))))).truncate(Rc::new(RefCell::new(Some(0))));
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*quo.borrow_mut().as_mut().unwrap()))))))))))));
    }

    /// Pow returns d to the power d2
    pub fn pow(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<Decimal>>> {
        let mut temp: Rc<RefCell<Option<Decimal>>> = Default::default();
        if (*(*d2.borrow_mut().as_mut().unwrap()).int_part().borrow().as_ref().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1)))))));
    }
        { let new_val = (*d.borrow_mut().as_mut().unwrap()).pow(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2)))))))))))); *temp.borrow_mut() = Some(new_val); };
        if (*(*d2.borrow_mut().as_mut().unwrap()).int_part().borrow().as_ref().unwrap()) % 2 == 0 {
        return Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()))))))));
    }
        if (*(*d2.borrow_mut().as_mut().unwrap()).int_part().borrow().as_ref().unwrap()) > 0 {
        return Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()))))).mul(Rc::new(RefCell::new(Some(self)))))));
    }
        return Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()))))).div(Rc::new(RefCell::new(Some(self)))))));
    }

    /// ExpHullAbrham calculates the natural exponent of decimal (e to the power of d) using Hull-Abraham algorithm.
    /// OverallPrecision argument specifies the overall precision of the result (integer part + decimal part).
    ///
    /// ExpHullAbrham is faster than ExpTaylor for small precision values, but it is much slower for large precision values.
    ///
    /// Example:
    ///
    ///     NewFromFloat(26.1).ExpHullAbrham(2).String()    // output: "220000000000"
    ///     NewFromFloat(26.1).ExpHullAbrham(20).String()   // output: "216314672147.05767284"
    ///
    pub fn exp_hull_abrham(&self, overallPrecision: Rc<RefCell<Option<u32>>>) -> (Rc<RefCell<Option<Decimal>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
                // Algorithm based on Variable precision exponential function.
                // ACM Transactions on Mathematical Software by T. E. Hull & A. Abrham.
        if (*d.borrow_mut().as_mut().unwrap()).is_zero() {
        return (Rc::new(RefCell::new(Some(Decimal { ,  }))), Rc::new(RefCell::new(None)));
    }
        let mut currentPrecision = Rc::new(RefCell::new(Some((*overallPrecision.borrow_mut().as_mut().unwrap()))));
                // Algorithm does not work if currentPrecision * 23 < |x|.
                // Precision is automatically increased in such cases, so the value can be calculated precisely.
                // If newly calculated precision is higher than ExpMaxIterations the currentPrecision will not be changed.
        let mut f = (*d.borrow_mut().as_mut().unwrap()).abs().inexact_float64();
        let mut ncp = Rc::new(RefCell::new(Some((*f.borrow_mut().as_mut().unwrap()) / 23)));
    if (*ncp.borrow_mut().as_mut().unwrap()) > (*(*float64.borrow().as_ref().unwrap())(currentPrecision.clone()).borrow().as_ref().unwrap()) && (*ncp.borrow_mut().as_mut().unwrap()) < (*(*float64.borrow().as_ref().unwrap())(ExpMaxIterations.clone()).borrow().as_ref().unwrap()) {
        { let new_val = (*uint32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*math.borrow_mut().as_mut().unwrap()).ceil(Rc::new(RefCell::new(Some((*ncp.borrow_mut().as_mut().unwrap()))))))))); *currentPrecision.borrow_mut() = Some(new_val); };
    }
                // fail if abs(d) beyond an over/underflow threshold
        let mut overflowThreshold = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(23 * (*(*int64.borrow().as_ref().unwrap())(currentPrecision.clone()).borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some(0))));
        if (*(*d.borrow_mut().as_mut().unwrap()).abs().cmp(Rc::new(RefCell::new(Some((*overflowThreshold.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) > 0 {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("over/underflow threshold, exp(x) cannot be calculated precisely")) as Box<dyn Error + Send + Sync>))));
    }
                // Return 1 if abs(d) small enough; this also avoids later over/underflow
        let mut overflowThreshold2 = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(9))), Rc::new(RefCell::new(Some((*-(*int32.borrow().as_ref().unwrap())(currentPrecision.clone()).borrow().as_ref().unwrap()) - 1))));
        if (*(*d.borrow_mut().as_mut().unwrap()).abs().cmp(Rc::new(RefCell::new(Some((*overflowThreshold2.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) <= 0 {
        return (Rc::new(RefCell::new(Some(Decimal { ,  }))), Rc::new(RefCell::new(None)));
    }
                // t is the smallest integer >= 0 such that the corresponding abs(d/k) < 1
        let mut t = Rc::new(RefCell::new(Some((*self.exp.clone().borrow().as_ref().unwrap()) + (*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).num_digits())))).borrow().as_ref().unwrap()))));
        if (*t.borrow_mut().as_mut().unwrap()) < 0 {
        { let new_val = 0; *t.borrow_mut() = Some(new_val); };
    }
        let mut k = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), t.clone());
        let mut r = Rc::new(RefCell::new(Some(Decimal { ,  })));
        let mut p = Rc::new(RefCell::new(Some((*(*int32.borrow().as_ref().unwrap())(currentPrecision.clone()).borrow().as_ref().unwrap()) + (*t.borrow_mut().as_mut().unwrap()) + 2)));
                // Determine n, the number of therms for calculating sum
                // use first Newton step (1.435p - 1.182) / log10(p/abs(r))
                // for solving appropriate equation, along with directed
                // roundings and simple rational bound for log10(p/abs(r))
        let mut rf = (*r.borrow_mut().as_mut().unwrap()).abs().inexact_float64();
        let mut pf = (*float64.borrow().as_ref().unwrap())(p.clone());
        let mut nf = (*math.borrow_mut().as_mut().unwrap()).ceil(Rc::new(RefCell::new(Some((1.453 * (*pf.borrow_mut().as_mut().unwrap()) - 1.182) / (*(*math.borrow_mut().as_mut().unwrap()).log10(Rc::new(RefCell::new(Some((*pf.borrow_mut().as_mut().unwrap()) / (*rf.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap())))));
        if (*nf.borrow_mut().as_mut().unwrap()) > (*(*float64.borrow().as_ref().unwrap())(ExpMaxIterations.clone()).borrow().as_ref().unwrap()) || (*(*math.borrow_mut().as_mut().unwrap()).is_na_n(Rc::new(RefCell::new(Some((*nf.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("exact value cannot be calculated in <=ExpMaxIterations iterations")) as Box<dyn Error + Send + Sync>))));
    }
        let mut n = (*int64.borrow().as_ref().unwrap())(nf.clone());
        let mut tmp = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(0))));
        let mut sum = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0))));
        let mut one = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0))));
        let mut i = Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 1)));
    while (*i.borrow_mut().as_mut().unwrap()) > 0 {
        (*(*(*tmp.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).set_int64(Rc::new(RefCell::new(Some((*i.borrow_mut().as_mut().unwrap())))));
        { let new_val = (*sum.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*r.borrow_mut().as_mut().unwrap()).div_round(Rc::new(RefCell::new(Some((*tmp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*p.borrow_mut().as_mut().unwrap()))))))))); *sum.borrow_mut() = Some(new_val); };
        { let new_val = (*sum.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*one.borrow_mut().as_mut().unwrap()))))); *sum.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let mut ki = (*k.borrow_mut().as_mut().unwrap()).int_part();
        let mut res = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0))));
        let mut i = Rc::new(RefCell::new(Some((*ki.borrow_mut().as_mut().unwrap()))));
    while (*i.borrow_mut().as_mut().unwrap()) > 0 {
        { let new_val = (*res.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*sum.borrow_mut().as_mut().unwrap()))))); *res.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let mut resNumDigits = (*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*res.borrow_mut().as_mut().unwrap()).num_digits()))));
        let mut roundDigits: Rc<RefCell<Option<i32>>> = Default::default();
        if (*resNumDigits.borrow_mut().as_mut().unwrap()) > (*(*abs.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*res.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) {
        { let new_val = (*(*int32.borrow().as_ref().unwrap())(currentPrecision.clone()).borrow().as_ref().unwrap()) - (*resNumDigits.borrow_mut().as_mut().unwrap()) - (*(*(*res.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()); *roundDigits.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*int32.borrow().as_ref().unwrap())(currentPrecision.clone()); *roundDigits.borrow_mut() = Some(new_val); };
    }
        { let new_val = (*res.borrow_mut().as_mut().unwrap()).round(Rc::new(RefCell::new(Some((*roundDigits.borrow_mut().as_mut().unwrap()))))); *res.borrow_mut() = Some(new_val); };
        return (res.clone(), Rc::new(RefCell::new(None)));
    }

    /// ExpTaylor calculates the natural exponent of decimal (e to the power of d) using Taylor series expansion.
    /// Precision argument specifies how precise the result must be (number of digits after decimal point).
    /// Negative precision is allowed.
    ///
    /// ExpTaylor is much faster for large precision values than ExpHullAbrham.
    ///
    /// Example:
    ///
    ///     d, err := NewFromFloat(26.1).ExpTaylor(2).String()
    ///     d.String()  // output: "216314672147.06"
    ///
    ///     NewFromFloat(26.1).ExpTaylor(20).String()
    ///     d.String()  // output: "216314672147.05767284062928674083"
    ///
    ///     NewFromFloat(26.1).ExpTaylor(-10).String()
    ///     d.String()  // output: "220000000000"
    ///
    pub fn exp_taylor(&self, precision: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<Decimal>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
                // Note(mwoss): Implementation can be optimized by exclusively using big.Int API only
        if (*d.borrow_mut().as_mut().unwrap()).is_zero() {
        return (Rc::new(RefCell::new(Some(Decimal { ,  }.round(Rc::new(RefCell::new(Some((*precision.borrow_mut().as_mut().unwrap())))))))), Rc::new(RefCell::new(None)));
    }
        let mut epsilon: Rc<RefCell<Option<Decimal>>> = Default::default();
        let mut divPrecision: Rc<RefCell<Option<i32>>> = Default::default();
        if (*precision.borrow_mut().as_mut().unwrap()) < 0 {
        { let new_val = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(-1)))); *epsilon.borrow_mut() = Some(new_val); };
        { let new_val = 8; *divPrecision.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some((*-(*precision.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) - 1)))); *epsilon.borrow_mut() = Some(new_val); };
        { let new_val = (*precision.borrow_mut().as_mut().unwrap()) + 1; *divPrecision.borrow_mut() = Some(new_val); };
    }
        let mut decAbs = (*d.borrow_mut().as_mut().unwrap()).abs();
        let mut pow = (*d.borrow_mut().as_mut().unwrap()).abs();
        let mut factorial = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0))));
        let mut result = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0))));
        let mut i = (*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))));
    while true {
        let mut step = (*pow.borrow_mut().as_mut().unwrap()).div_round(Rc::new(RefCell::new(Some((*factorial.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*divPrecision.borrow_mut().as_mut().unwrap())))));
        { let new_val = (*result.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*step.borrow_mut().as_mut().unwrap()))))); *result.borrow_mut() = Some(new_val); };

                // Stop Taylor series when current step is smaller than epsilon
        if (*(*step.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*epsilon.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) < 0 {
        break
    }

        { let new_val = (*pow.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*decAbs.borrow_mut().as_mut().unwrap()))))); *pow.borrow_mut() = Some(new_val); };

        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }

                // Calculate next factorial number or retrieve cached value
        if (*factorials.borrow().as_ref().unwrap()).len() >= (*(*int.borrow().as_ref().unwrap())(i.clone()).borrow().as_ref().unwrap()) && (*!(*factorials.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) - 1 as usize].clone().is_zero().borrow().as_ref().unwrap()) {
        { let new_val = (*factorials.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) - 1 as usize].clone(); *factorial.borrow_mut() = Some(new_val); };
    } else {
                // To avoid any race conditions, firstly the zero value is appended to a slice to create
                // a spot for newly calculated factorial. After that, the zero value is replaced by calculated
                // factorial using the index notation.
        { let new_val = (*factorials.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) - 2 as usize].clone().mul(Rc::new(RefCell::new(Some((*New.borrow().as_ref().unwrap())(i.clone(), Rc::new(RefCell::new(Some(0)))))))); *factorial.borrow_mut() = Some(new_val); };
        { let new_val = {(*factorials.borrow_mut().as_mut().unwrap()).push(ZERO); factorials.clone()}; *factorials.borrow_mut() = Some(new_val); };
        (*factorials.borrow_mut().as_mut().unwrap())[(*i.borrow_mut().as_mut().unwrap()) - 1] = (*factorial.borrow_mut().as_mut().unwrap());
    }
    }
                // Stop Taylor series when current step is smaller than epsilon
                // Calculate next factorial number or retrieve cached value
                // To avoid any race conditions, firstly the zero value is appended to a slice to create
                // a spot for newly calculated factorial. After that, the zero value is replaced by calculated
                // factorial using the index notation.
        if (*(*d.borrow_mut().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        { let new_val = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0)))).div_round(Rc::new(RefCell::new(Some((*result.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*precision.borrow_mut().as_mut().unwrap()) + 1)))); *result.borrow_mut() = Some(new_val); };
    }
        { let new_val = (*result.borrow_mut().as_mut().unwrap()).round(Rc::new(RefCell::new(Some((*precision.borrow_mut().as_mut().unwrap()))))); *result.borrow_mut() = Some(new_val); };
        return (result.clone(), Rc::new(RefCell::new(None)));
    }

    /// NumDigits returns the number of digits of the decimal coefficient (d.Value)
    /// Note: Current implementation is extremely slow for large decimals and/or decimals with large fractional part
    pub fn num_digits(&self) -> Rc<RefCell<Option<i32>>> {
                // Note(mwoss): It can be optimized, unnecessary cast of big.Int to string
        if (*d.borrow_mut().as_mut().unwrap()).is_negative() {
        return Rc::new(RefCell::new(Some((*(*self.value.clone().borrow().as_mut().unwrap()).string().borrow().as_ref().unwrap()).len() - 1)));
    }
        return Rc::new(RefCell::new(Some((*(*self.value.clone().borrow().as_mut().unwrap()).string().borrow().as_ref().unwrap()).len())));
    }

    /// IsInteger returns true when decimal can be represented as an integer value, otherwise, it returns false.
    pub fn is_integer(&self) -> Rc<RefCell<Option<bool>>> {
                // The most typical case, all decimal with exponent higher or equal 0 can be represented as integer
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= 0 {
        return Rc::new(RefCell::new(Some(true)));
    }
                // When the exponent is negative we have to check every number after the decimal place
                // If all of them are zeroes, we are sure that given decimal can be represented as an integer
        let mut r: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;
        let mut q = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set(Rc::new(RefCell::new(Some(self.value.clone()))));
        let mut z = (*abs.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))));
    while (*z.borrow_mut().as_mut().unwrap()) > 0 {
        (*q.borrow_mut().as_mut().unwrap()).quo_rem(Rc::new(RefCell::new(Some((*q.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(r.clone()))));
        if (*(*r.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*zeroInt.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) != 0 {
        return Rc::new(RefCell::new(Some(false)));
    }
        { let mut guard = z.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return Rc::new(RefCell::new(Some(true)));
    }

    /// Cmp compares the numbers represented by d and d2 and returns:
    ///
    ///     -1 if d <  d2
    ///      0 if d == d2
    ///     +1 if d >  d2
    ///
    pub fn cmp(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<i32>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        (*d2.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*self.exp.clone().borrow().as_ref().unwrap()) == (*(*(*d2.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*self.value.clone().borrow().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*(*d2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()))))))));
    }
        let (mut rd, mut rd2) = (*RescalePair.borrow().as_ref().unwrap())(d.clone(), d2.clone());
        return Rc::new(RefCell::new(Some((*(*(*rd.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*(*rd2.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()))))))));
    }

    /// Equal returns whether the numbers represented by d and d2 are equal.
    pub fn equal(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) == 0)));
    }

    /// Equals is deprecated, please use Equal method instead
    pub fn equals(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()))))))));
    }

    /// GreaterThan (GT) returns true when d is greater than d2.
    pub fn greater_than(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) == 1)));
    }

    /// GreaterThanOrEqual (GTE) returns true when d is greater than or equal to d2.
    pub fn greater_than_or_equal(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<bool>>> {
        let mut cmp = (*d.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some((*cmp.borrow_mut().as_mut().unwrap()) == 1 || (*cmp.borrow_mut().as_mut().unwrap()) == 0)));
    }

    /// LessThan (LT) returns true when d is less than d2.
    pub fn less_than(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) == (*-1.borrow().as_ref().unwrap()))));
    }

    /// LessThanOrEqual (LTE) returns true when d is less than or equal to d2.
    pub fn less_than_or_equal(&self, d2: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<bool>>> {
        let mut cmp = (*d.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some((*cmp.borrow_mut().as_mut().unwrap()) == (*-1.borrow().as_ref().unwrap()) || (*cmp.borrow_mut().as_mut().unwrap()) == 0)));
    }

    /// Sign returns:
    ///
    ///	-1 if d <  0
    ///	 0 if d == 0
    ///	+1 if d >  0
    ///
    pub fn sign(&self) -> Rc<RefCell<Option<i32>>> {
        if (*self.value.borrow()).is_none() {
        return Rc::new(RefCell::new(Some(0)));
    }
        return Rc::new(RefCell::new(Some((*self.value.clone().borrow().as_mut().unwrap()).sign())));
    }

    /// IsPositive return
    ///
    ///	true if d > 0
    ///	false if d == 0
    ///	false if d < 0
    pub fn is_positive(&self) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) == 1)));
    }

    /// IsNegative return
    ///
    ///	true if d < 0
    ///	false if d == 0
    ///	false if d > 0
    pub fn is_negative(&self) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) == (*-1.borrow().as_ref().unwrap()))));
    }

    /// IsZero return
    ///
    ///	true if d == 0
    ///	false if d > 0
    ///	false if d < 0
    pub fn is_zero(&self) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) == 0)));
    }

    /// Exponent returns the exponent, or scale component of the decimal.
    pub fn exponent(&self) -> Rc<RefCell<Option<i32>>> {
        return self.exp.clone();
    }

    /// Coefficient returns the coefficient of the decimal. It is scaled by 10^Exponent()
    pub fn coefficient(&self) -> Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
                // we copy the coefficient so that mutating the result does not mutate the Decimal.
        return Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set(Rc::new(RefCell::new(Some(self.value.clone())))))));
    }

    /// CoefficientInt64 returns the coefficient of the decimal as int64. It is scaled by 10^Exponent()
    /// If coefficient cannot be represented in an int64, the result will be undefined.
    pub fn coefficient_int64(&self) -> Rc<RefCell<Option<i64>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        return Rc::new(RefCell::new(Some((*self.value.clone().borrow().as_mut().unwrap()).int64())));
    }

    /// IntPart returns the integer component of the decimal.
    pub fn int_part(&self) -> Rc<RefCell<Option<i64>>> {
        let mut scaledD = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(0))));
        return Rc::new(RefCell::new(Some((*(*(*scaledD.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).int64())));
    }

    /// BigInt returns integer component of the decimal as a BigInt.
    pub fn big_int(&self) -> Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>> {
        let mut scaledD = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(0))));
        let mut i = Rc::new(RefCell::new(Some()));
        (*i.borrow_mut().as_mut().unwrap()).set_string(Rc::new(RefCell::new(Some((*scaledD.borrow_mut().as_mut().unwrap()).string()))), Rc::new(RefCell::new(Some(10))));
        return i.clone();
    }

    /// BigFloat returns decimal as BigFloat.
    /// Be aware that casting decimal to BigFloat might cause a loss of precision.
    pub fn big_float(&self) -> Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>> {
        let mut f = Rc::new(RefCell::new(Some()));
        (*f.borrow_mut().as_mut().unwrap()).set_string(Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).string()))));
        return f.clone();
    }

    /// Rat returns a rational number representation of the decimal.
    pub fn rat(&self) -> Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*self.exp.clone().borrow().as_ref().unwrap()) <= 0 {
                // NOTE(vadim): must negate after casting to prevent int32 overflow
        let mut denom = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).exp(Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some(-(*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))))))))))), Rc::new(RefCell::new(Some(None))));
        return Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set_frac(Rc::new(RefCell::new(Some(self.value.clone()))), Rc::new(RefCell::new(Some((*denom.borrow_mut().as_mut().unwrap()))))))));
    }
                // NOTE(vadim): must negate after casting to prevent int32 overflow
        let mut mul = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).exp(Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))))))))))), Rc::new(RefCell::new(Some(None))));
        let mut num = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).mul(Rc::new(RefCell::new(Some(self.value.clone()))), Rc::new(RefCell::new(Some((*mul.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set_frac(Rc::new(RefCell::new(Some((*num.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap()))))))));
    }

    /// Float64 returns the nearest float64 value for d and a bool indicating
    /// whether f represents d exactly.
    /// For more details, see the documentation for big.Rat.Float64
    pub fn float64(&self) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<bool>>>) {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).rat().float64())));
    }

    /// InexactFloat64 returns the nearest float64 value for d.
    /// It doesn't indicate if the returned value represents d exactly.
    pub fn inexact_float64(&self) -> Rc<RefCell<Option<f64>>> {
        let (mut f, _) = (*d.borrow_mut().as_mut().unwrap()).float64();
        return f.clone();
    }

    /// String returns the string representation of the decimal
    /// with the fixed point.
    ///
    /// Example:
    ///
    ///     d := New(-12345, -3)
    ///     println(d.String())
    ///
    /// Output:
    ///
    ///     -12.345
    ///
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).string(Rc::new(RefCell::new(Some(true)))))));
    }

    /// StringFixed returns a rounded fixed-point string with places digits after
    /// the decimal point.
    ///
    /// Example:
    ///
    /// 	   NewFromFloat(0).StringFixed(2) // output: "0.00"
    /// 	   NewFromFloat(0).StringFixed(0) // output: "0"
    /// 	   NewFromFloat(5.45).StringFixed(0) // output: "5"
    /// 	   NewFromFloat(5.45).StringFixed(1) // output: "5.5"
    /// 	   NewFromFloat(5.45).StringFixed(2) // output: "5.45"
    /// 	   NewFromFloat(5.45).StringFixed(3) // output: "5.450"
    /// 	   NewFromFloat(545).StringFixed(-1) // output: "550"
    ///
    pub fn string_fixed(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
        let mut rounded = (*d.borrow_mut().as_mut().unwrap()).round(Rc::new(RefCell::new(Some((*places.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some((*rounded.borrow_mut().as_mut().unwrap()).string(Rc::new(RefCell::new(Some(false)))))));
    }

    /// StringFixedBank returns a banker rounded fixed-point string with places digits
    /// after the decimal point.
    ///
    /// Example:
    ///
    /// 	   NewFromFloat(0).StringFixedBank(2) // output: "0.00"
    /// 	   NewFromFloat(0).StringFixedBank(0) // output: "0"
    /// 	   NewFromFloat(5.45).StringFixedBank(0) // output: "5"
    /// 	   NewFromFloat(5.45).StringFixedBank(1) // output: "5.4"
    /// 	   NewFromFloat(5.45).StringFixedBank(2) // output: "5.45"
    /// 	   NewFromFloat(5.45).StringFixedBank(3) // output: "5.450"
    /// 	   NewFromFloat(545).StringFixedBank(-1) // output: "540"
    ///
    pub fn string_fixed_bank(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
        let mut rounded = (*d.borrow_mut().as_mut().unwrap()).round_bank(Rc::new(RefCell::new(Some((*places.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some((*rounded.borrow_mut().as_mut().unwrap()).string(Rc::new(RefCell::new(Some(false)))))));
    }

    /// StringFixedCash returns a Swedish/Cash rounded fixed-point string. For
    /// more details see the documentation at function RoundCash.
    pub fn string_fixed_cash(&self, interval: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<String>>> {
        let mut rounded = (*d.borrow_mut().as_mut().unwrap()).round_cash(Rc::new(RefCell::new(Some((*interval.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some((*rounded.borrow_mut().as_mut().unwrap()).string(Rc::new(RefCell::new(Some(false)))))));
    }

    /// Round rounds the decimal to places decimal places.
    /// If places < 0, it will round the integer part to the nearest 10^(-places).
    ///
    /// Example:
    ///
    /// 	   NewFromFloat(5.45).Round(1).String() // output: "5.5"
    /// 	   NewFromFloat(545).Round(-1).String() // output: "550"
    ///
    pub fn round(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        if (*self.exp.clone().borrow().as_ref().unwrap()) == (*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        return d.clone();
    }
                // truncate to places + 1
        let mut ret = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some((*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) - 1))));
                // add sign(d) * 0.5
        if (*(*(*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        (*(*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*fiveInt.borrow_mut().as_mut().unwrap())))));
    } else {
        (*(*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*fiveInt.borrow_mut().as_mut().unwrap())))));
    }
                // floor for positive numbers, ceil for negative numbers
        let (_, mut m) = (*(*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).div_mod(Rc::new(RefCell::new(Some((*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*tenInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default())))))));
        { let mut guard = (*ret.borrow_mut().as_mut().unwrap()).exp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if (*(*(*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 && (*(*m.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*zeroInt.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) != 0 {
        (*(*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*(*ret.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    }
        return ret.clone();
    }

    /// RoundCeil rounds the decimal towards +infinity.
    ///
    /// Example:
    ///
    ///     NewFromFloat(545).RoundCeil(-2).String()   // output: "600"
    ///     NewFromFloat(500).RoundCeil(-2).String()   // output: "500"
    ///     NewFromFloat(1.1001).RoundCeil(2).String() // output: "1.11"
    ///     NewFromFloat(-1.454).RoundCeil(1).String() // output: "-1.5"
    ///
    pub fn round_ceil(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= (*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        return d.clone();
    }
        let mut rescaled = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(-(*places.borrow_mut().as_mut().unwrap())))));
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*rescaled.borrow_mut().as_mut().unwrap()))))) {
        return d.clone();
    }
        if (*(*self.value.clone().borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) > 0 {
        (*(*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    }
        return rescaled.clone();
    }

    /// RoundFloor rounds the decimal towards -infinity.
    ///
    /// Example:
    ///
    ///     NewFromFloat(545).RoundFloor(-2).String()   // output: "500"
    ///     NewFromFloat(-500).RoundFloor(-2).String()   // output: "-500"
    ///     NewFromFloat(1.1001).RoundFloor(2).String() // output: "1.1"
    ///     NewFromFloat(-1.454).RoundFloor(1).String() // output: "-1.4"
    ///
    pub fn round_floor(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= (*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        return d.clone();
    }
        let mut rescaled = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(-(*places.borrow_mut().as_mut().unwrap())))));
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*rescaled.borrow_mut().as_mut().unwrap()))))) {
        return d.clone();
    }
        if (*(*self.value.clone().borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        (*(*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    }
        return rescaled.clone();
    }

    /// RoundUp rounds the decimal away from zero.
    ///
    /// Example:
    ///
    ///     NewFromFloat(545).RoundUp(-2).String()   // output: "600"
    ///     NewFromFloat(500).RoundUp(-2).String()   // output: "500"
    ///     NewFromFloat(1.1001).RoundUp(2).String() // output: "1.11"
    ///     NewFromFloat(-1.454).RoundUp(1).String() // output: "-1.4"
    ///
    pub fn round_up(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= (*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        return d.clone();
    }
        let mut rescaled = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(-(*places.borrow_mut().as_mut().unwrap())))));
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*rescaled.borrow_mut().as_mut().unwrap()))))) {
        return d.clone();
    }
        if (*(*self.value.clone().borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) > 0 {
        (*(*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    } else if (*(*self.value.clone().borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        (*(*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*(*rescaled.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    }
        return rescaled.clone();
    }

    /// RoundDown rounds the decimal towards zero.
    ///
    /// Example:
    ///
    ///     NewFromFloat(545).RoundDown(-2).String()   // output: "500"
    ///     NewFromFloat(-500).RoundDown(-2).String()   // output: "-500"
    ///     NewFromFloat(1.1001).RoundDown(2).String() // output: "1.1"
    ///     NewFromFloat(-1.454).RoundDown(1).String() // output: "-1.5"
    ///
    pub fn round_down(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= (*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        return d.clone();
    }
        let mut rescaled = (*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(-(*places.borrow_mut().as_mut().unwrap())))));
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*rescaled.borrow_mut().as_mut().unwrap()))))) {
        return d.clone();
    }
        return rescaled.clone();
    }

    /// RoundBank rounds the decimal to places decimal places.
    /// If the final digit to round is equidistant from the nearest two integers the
    /// rounded value is taken as the even number
    ///
    /// If places < 0, it will round the integer part to the nearest 10^(-places).
    ///
    /// Examples:
    ///
    /// 	   NewFromFloat(5.45).RoundBank(1).String() // output: "5.4"
    /// 	   NewFromFloat(545).RoundBank(-1).String() // output: "540"
    /// 	   NewFromFloat(5.46).RoundBank(1).String() // output: "5.5"
    /// 	   NewFromFloat(546).RoundBank(-1).String() // output: "550"
    /// 	   NewFromFloat(5.55).RoundBank(1).String() // output: "5.6"
    /// 	   NewFromFloat(555).RoundBank(-1).String() // output: "560"
    ///
    pub fn round_bank(&self, places: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        let mut round = (*d.borrow_mut().as_mut().unwrap()).round(Rc::new(RefCell::new(Some((*places.borrow_mut().as_mut().unwrap())))));
        let mut remainder = (*d.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*round.borrow_mut().as_mut().unwrap()))))).abs();
        let mut half = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some((*-(*places.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) - 1))));
        if (*(*remainder.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*half.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) == 0 && (*(*(*(*round.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).bit(Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()) != 0 {
        if (*(*(*(*round.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        (*(*(*round.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*(*round.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    } else {
        (*(*(*round.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*(*round.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    }
    }
        return round.clone();
    }

    /// RoundCash aka Cash/Penny/öre rounding rounds decimal to a specific
    /// interval. The amount payable for a cash transaction is rounded to the nearest
    /// multiple of the minimum currency unit available. The following intervals are
    /// available: 5, 10, 25, 50 and 100; any other number throws a panic.
    ///	    5:   5 cent rounding 3.43 => 3.45
    /// 	   10:  10 cent rounding 3.45 => 3.50 (5 gets rounded up)
    /// 	   25:  25 cent rounding 3.41 => 3.50
    /// 	   50:  50 cent rounding 3.75 => 4.00
    /// 	  100: 100 cent rounding 3.50 => 4.00
    /// For more details: https://en.wikipedia.org/wiki/Cash_rounding
    pub fn round_cash(&self, interval: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<Decimal>>> {
        let mut iVal: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>> = Rc::new(RefCell::new(None));
        match (*interval.borrow_mut().as_mut().unwrap()) {
        5 => {
            { let new_val = (*twentyInt.borrow_mut().as_mut().unwrap()); *iVal.borrow_mut() = Some(new_val); };
        }
        10 => {
            { let new_val = (*tenInt.borrow_mut().as_mut().unwrap()); *iVal.borrow_mut() = Some(new_val); };
        }
        25 => {
            { let new_val = (*fourInt.borrow_mut().as_mut().unwrap()); *iVal.borrow_mut() = Some(new_val); };
        }
        50 => {
            { let new_val = (*twoInt.borrow_mut().as_mut().unwrap()); *iVal.borrow_mut() = Some(new_val); };
        }
        100 => {
            { let new_val = (*oneInt.borrow_mut().as_mut().unwrap()); *iVal.borrow_mut() = Some(new_val); };
        }
        _ => {
            panic!("{:?}", Rc::new(RefCell::new(Some(format!("Decimal does not support this Cash rounding interval `{}`. Supported: 5, 10, 25, 50, 100", /* ERROR: Type information not available for print argument */ (*interval.borrow_mut().as_mut().unwrap()))))));
        }
    }
        let mut dVal = Rc::new(RefCell::new(Some(Decimal { value: iVal.clone() })));
                // TODO: optimize those calculations to reduce the high allocations (~29 allocs).
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*dVal.borrow_mut().as_mut().unwrap()))))).round(Rc::new(RefCell::new(Some(0)))).div(Rc::new(RefCell::new(Some((*dVal.borrow_mut().as_mut().unwrap()))))).truncate(Rc::new(RefCell::new(Some(2)))))));
    }

    /// Floor returns the nearest integer value less than or equal to d.
    pub fn floor(&self) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= 0 {
        return d.clone();
    }
        let mut exp = (*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some(10))));
                // NOTE(vadim): must negate after casting to prevent int32 overflow
        (*exp.borrow_mut().as_mut().unwrap()).exp(Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some(-(*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))))))))))), Rc::new(RefCell::new(Some(None))));
        let mut z = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).div(Rc::new(RefCell::new(Some(self.value.clone()))), Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap())))));
        return Rc::new(RefCell::new(Some(Decimal { value: z.clone(), exp: Rc::new(RefCell::new(Some(0))) })));
    }

    /// Ceil returns the nearest integer value greater than or equal to d.
    pub fn ceil(&self) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= 0 {
        return d.clone();
    }
        let mut exp = (*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some(10))));
                // NOTE(vadim): must negate after casting to prevent int32 overflow
        (*exp.borrow_mut().as_mut().unwrap()).exp(Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some(-(*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))))))))))), Rc::new(RefCell::new(Some(None))));
        let (mut z, mut m) = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).div_mod(Rc::new(RefCell::new(Some(self.value.clone()))), Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default())))))));
        if (*(*m.borrow_mut().as_mut().unwrap()).cmp(Rc::new(RefCell::new(Some((*zeroInt.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) != 0 {
        (*z.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*oneInt.borrow_mut().as_mut().unwrap())))));
    }
        return Rc::new(RefCell::new(Some(Decimal { value: z.clone(), exp: Rc::new(RefCell::new(Some(0))) })));
    }

    /// Truncate truncates off digits from the number, without rounding.
    ///
    /// NOTE: precision is the last digit that will not be truncated (must be >= 0).
    ///
    /// Example:
    ///
    ///     decimal.NewFromString("123.456").Truncate(2).String() // "123.45"
    ///
    pub fn truncate(&self, precision: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {
        (*d.borrow_mut().as_mut().unwrap()).ensure_initialized();
        if (*precision.borrow_mut().as_mut().unwrap()) >= 0 && (*-(*precision.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) > (*self.exp.clone().borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(-(*precision.borrow_mut().as_mut().unwrap()))))))));
    }
        return d.clone();
    }

    /// UnmarshalJSON implements the json.Unmarshaler interface.
    pub fn unmarshal_j_s_o_n(&mut self, decimalBytes: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
        if (*(*string.borrow().as_ref().unwrap())(decimalBytes.clone()).borrow().as_ref().unwrap()) == "null".to_string() {
        return Rc::new(RefCell::new(None));
    }
        let (mut str, mut err) = (*unquoteIfQuoted.borrow().as_ref().unwrap())(decimalBytes.clone());
        if (*err.borrow()).is_some() {
        return Rc::new(RefCell::new(Some(Box::new(format!("error decoding string '{}': {}", (*decimalBytes.borrow_mut().as_mut().unwrap()), (*err.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>)));
    }
        let (mut decimal, mut err) = (*NewFromString.borrow().as_ref().unwrap())(str.clone());
        { let new_val = (*decimal.borrow_mut().as_mut().unwrap()); *self.borrow_mut() = Some(new_val); };
        if (*err.borrow()).is_some() {
        return Rc::new(RefCell::new(Some(Box::new(format!("error decoding string '{}': {}", (*str.borrow_mut().as_mut().unwrap()), (*err.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>)));
    }
        return Rc::new(RefCell::new(None));
    }

    /// MarshalJSON implements the json.Marshaler interface.
    pub fn marshal_j_s_o_n(&self) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        let mut str: Rc<RefCell<Option<String>>> = String::new();
        if MARSHAL_J_S_O_N_WITHOUT_QUOTES {
        { let new_val = (*d.borrow_mut().as_mut().unwrap()).string(); *str.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = format!("{}{}", format!("{}{}", "\"".to_string(), (*d.borrow_mut().as_mut().unwrap()).string()), "\"".to_string()); *str.borrow_mut() = Some(new_val); };
    }
        return (Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*str.borrow().as_ref().unwrap()).as_bytes().to_vec())))))), Rc::new(RefCell::new(None)));
    }

    /// UnmarshalBinary implements the encoding.BinaryUnmarshaler interface. As a string representation
    /// is already used when encoding to text, this method stores that string as []byte
    pub fn unmarshal_binary(&mut self, data: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
                // Verify we have at least 4 bytes for the exponent. The GOB encoded value
                // may be empty.
        if (*data.borrow().as_ref().unwrap()).len() < 4 {
        return Rc::new(RefCell::new(Some(Box::new(format!("error decoding binary {}: expected at least 4 bytes, got {}", (*data.borrow_mut().as_mut().unwrap()), (*data.borrow().as_ref().unwrap()).len())) as Box<dyn Error + Send + Sync>)));
    }
                // Extract the exponent
        { let new_val = (*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*binary.borrow_mut().as_mut().unwrap())::big_endian.borrow().as_mut().unwrap()).uint32(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[..4 as usize].to_vec()))))))))))); *self.exp.borrow_mut() = Some(new_val); };
                // Extract the value
        { let new_val = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))); *self.value.borrow_mut() = Some(new_val); };
        let mut err = (*self.value.clone().borrow().as_mut().unwrap()).gob_decode(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[4 as usize..].to_vec())))))));
    if (*err.borrow()).is_some() {
        return Rc::new(RefCell::new(Some(Box::new(format!("error decoding binary {}: {}", (*data.borrow_mut().as_mut().unwrap()), (*err.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>)));
    }
        return Rc::new(RefCell::new(None));
    }

    /// MarshalBinary implements the encoding.BinaryMarshaler interface.
    pub fn marshal_binary(&self) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
                // Write the exponent first since it's a fixed size
        let mut v1 = Rc::new(RefCell::new(Some(vec![0; 4])));
        (*(*binary.borrow_mut().as_mut().unwrap())::big_endian.borrow().as_mut().unwrap()).put_uint32(Rc::new(RefCell::new(Some((*v1.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*uint32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))))))));
                // Add the value
        let mut v2: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(Some(Default::default())));
        (v2, err) = (*self.value.clone().borrow().as_mut().unwrap()).gob_encode();
    if (*err.borrow()).is_some() {
        return (data, err);
    }
                // Return the byte array
        { let new_val = {(*v1.borrow_mut().as_mut().unwrap()).push((*v2.borrow_mut().as_mut().unwrap())); v1.clone()}; *data.borrow_mut() = Some(new_val); };
        return (data, err);
    }

    /// Scan implements the sql.Scanner interface for database deserialization.
    pub fn scan(&mut self, value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
                // first try to see if the data is stored in database as a Numeric datatype
        if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow_mut().as_mut().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<float32>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<float32>().unwrap()).clone())));
        { let new_val = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*float64.borrow().as_ref().unwrap())(v.clone()))))); *self.borrow_mut() = Some(new_val); };;
        return Rc::new(RefCell::new(None));;
    } else if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow_mut().as_mut().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<f64>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<f64>().unwrap()).clone())));
        { let new_val = (*NewFromFloat.borrow().as_ref().unwrap())(v.clone()); *self.borrow_mut() = Some(new_val); };;
        return Rc::new(RefCell::new(None));;
    } else if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow_mut().as_mut().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<int64>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<int64>().unwrap()).clone())));
        { let new_val = (*New.borrow().as_ref().unwrap())(v.clone(), Rc::new(RefCell::new(Some(0)))); *self.borrow_mut() = Some(new_val); };;
        return Rc::new(RefCell::new(None));;
    } else {
        let v = (*value.borrow_mut().as_mut().unwrap());
        let (mut str, mut err) = (*unquoteIfQuoted.borrow().as_ref().unwrap())(v.clone());;
        if (*err.borrow()).is_some() {
        return err.clone();
    };
        ((*self.borrow_mut().as_mut().unwrap()), err) = (*NewFromString.borrow().as_ref().unwrap())(str.clone());;
        return err.clone();;
    }
    }

    /// Value implements the driver.Valuer interface for database serialization.
    pub fn value(&self) -> (Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        return (Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).string()))), Rc::new(RefCell::new(None)));
    }

    /// UnmarshalText implements the encoding.TextUnmarshaler interface for XML
    /// deserialization.
    pub fn unmarshal_text(&mut self, text: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
        let mut str = (*string.borrow().as_ref().unwrap())(text.clone());
        let (mut dec, mut err) = (*NewFromString.borrow().as_ref().unwrap())(str.clone());
        { let new_val = (*dec.borrow_mut().as_mut().unwrap()); *self.borrow_mut() = Some(new_val); };
        if (*err.borrow()).is_some() {
        return Rc::new(RefCell::new(Some(Box::new(format!("error decoding string '{}': {}", (*str.borrow_mut().as_mut().unwrap()), (*err.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>)));
    }
        return Rc::new(RefCell::new(None));
    }

    /// MarshalText implements the encoding.TextMarshaler interface for XML
    /// serialization.
    pub fn marshal_text(&self) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        return (Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).string().borrow().as_ref().unwrap()).as_bytes().to_vec())))))), Rc::new(RefCell::new(None)));
    }

    /// GobEncode implements the gob.GobEncoder interface for gob serialization.
    pub fn gob_encode(&self) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).marshal_binary())));
    }

    /// GobDecode implements the gob.GobDecoder interface for gob serialization.
    pub fn gob_decode(&mut self, data: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
        return (*d.borrow_mut().as_mut().unwrap()).unmarshal_binary(Rc::new(RefCell::new(Some((*data.borrow_mut().as_mut().unwrap())))));
    }

    /// StringScaled first scales the decimal then calls .String() on it.
    /// NOTE: buggy, unintuitive, and DEPRECATED! Use StringFixed instead.
    pub fn string_scaled(&self, exp: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap()))))).string())));
    }

    pub fn string(&self, trimTrailingZeros: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<String>>> {
        if (*self.exp.clone().borrow().as_ref().unwrap()) >= 0 {
        return Rc::new(RefCell::new(Some((*(*(*d.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some(0)))).value.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).string())));
    }
        let mut abs = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).abs(Rc::new(RefCell::new(Some(self.value.clone()))));
        let mut str = (*abs.borrow_mut().as_mut().unwrap()).string();
        let mut intPart: Rc<RefCell<Option<String>>> = String::new();, let mut fractionalPart: Rc<RefCell<Option<String>>> = String::new();
                // NOTE(vadim): this cast to int will cause bugs if d.exp == INT_MIN
                // and you are on a 32-bit machine. Won't fix this super-edge case.
        let mut dExpInt = (*int.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(self.exp.clone()))));
        if (*str.borrow().as_ref().unwrap()).len() > (*-(*dExpInt.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) {
        { let new_val = Rc::new(RefCell::new(Some((*str.borrow().as_ref().unwrap())[..(*str.borrow().as_ref().unwrap()).len() + (*dExpInt.borrow_mut().as_mut().unwrap()) as usize].to_vec()))); *intPart.borrow_mut() = Some(new_val); };
        { let new_val = Rc::new(RefCell::new(Some((*str.borrow().as_ref().unwrap())[(*str.borrow().as_ref().unwrap()).len() + (*dExpInt.borrow_mut().as_mut().unwrap()) as usize..].to_vec()))); *fractionalPart.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = "0".to_string(); *intPart.borrow_mut() = Some(new_val); };
        let mut num0s = Rc::new(RefCell::new(Some((*-(*dExpInt.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()) - (*str.borrow().as_ref().unwrap()).len())));
        { let new_val = (*(*strings.borrow_mut().as_mut().unwrap()).repeat(Rc::new(RefCell::new(Some("0".to_string()))), Rc::new(RefCell::new(Some((*num0s.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) + (*str.borrow_mut().as_mut().unwrap()); *fractionalPart.borrow_mut() = Some(new_val); };
    }
        if (*trimTrailingZeros.borrow_mut().as_mut().unwrap()) {
        let mut i = Rc::new(RefCell::new(Some((*fractionalPart.borrow().as_ref().unwrap()).len() - 1)));
        while (*i.borrow_mut().as_mut().unwrap()) >= 0 {
        if (*(*fractionalPart.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) != ('0' as i32) {
        break
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let new_val = Rc::new(RefCell::new(Some((*fractionalPart.borrow().as_ref().unwrap())[..(*i.borrow_mut().as_mut().unwrap()) + 1 as usize].to_vec()))); *fractionalPart.borrow_mut() = Some(new_val); };
    }
        let mut number = Rc::new(RefCell::new(Some((*intPart.borrow_mut().as_mut().unwrap()))));
        if (*fractionalPart.borrow().as_ref().unwrap()).len() > 0 {
        { let mut guard = number.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + format!("{}{}", ".".to_string(), (*fractionalPart.borrow_mut().as_mut().unwrap()))); };
    }
        if (*(*self.value.clone().borrow().as_mut().unwrap()).sign().borrow().as_ref().unwrap()) < 0 {
        return {
            let __tmp_x = "-".to_string();
            let __tmp_y = (*number.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }
        return number.clone();
    }

    pub fn ensure_initialized(&mut self) {
        if (*self.value.borrow()).is_none() {
        { let new_val = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))); *self.value.borrow_mut() = Some(new_val); };
    }
    }

    /// Atan returns the arctangent, in radians, of x.
    pub fn atan(&self) -> Rc<RefCell<Option<Decimal>>> {
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        return d.clone();
    }
        if (*d.borrow_mut().as_mut().unwrap()).greater_than(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).satan())));
    }
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).neg().satan().neg())));
    }

    pub fn xatan(&self) -> Rc<RefCell<Option<Decimal>>> {
        let mut P0 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-8.750608600031904122785e-01))));
        let mut P1 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-1.615753718733365076637e+01))));
        let mut P2 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-7.500855792314704667340e+01))));
        let mut P3 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-1.228866684490136173410e+02))));
        let mut P4 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-6.485021904942025371773e+01))));
        let mut Q0 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2.485846490142306297962e+01))));
        let mut Q1 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.650270098316988542046e+02))));
        let mut Q2 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(4.328810604912902668951e+02))));
        let mut Q3 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(4.853903996359136964868e+02))));
        let mut Q4 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.945506571482613964425e+02))));
        let mut z = (*d.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(self))));
        let mut b1 = (*P0.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(P1)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(P2)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(P3)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(P4)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap())))));
        let mut b2 = (*z.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(Q0)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(Q1)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(Q2)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(Q3)))).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(Q4))));
        { let new_val = (*b1.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*b2.borrow_mut().as_mut().unwrap()))))); *z.borrow_mut() = Some(new_val); };
        { let new_val = (*d.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some(self)))); *z.borrow_mut() = Some(new_val); };
        return z.clone();
    }

    /// satan reduces its argument (known to be positive)
    /// to the range [0, 0.66] and calls xatan.
    pub fn satan(&self) -> Rc<RefCell<Option<Decimal>>> {
        let mut MOREBITS = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(6.123233995736765886130e-17))));
        let mut TAN3PIO8 = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2.41421356237309504880))));
        let mut pi = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3.14159265358979323846264338327950288419716939937510582097494459))));
        if (*d.borrow_mut().as_mut().unwrap()).less_than_or_equal(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.66)))))))) {
        return Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).xatan())));
    }
        if (*d.borrow_mut().as_mut().unwrap()).greater_than(Rc::new(RefCell::new(Some(TAN3PIO8)))) {
        return Rc::new(RefCell::new(Some((*pi.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2.0)))))))).sub(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))).div(Rc::new(RefCell::new(Some(self)))).xatan())))).add(Rc::new(RefCell::new(Some(MOREBITS)))))));
    }
        return Rc::new(RefCell::new(Some((*pi.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(4.0)))))))).add(Rc::new(RefCell::new(Some(((*d.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))))))).div(Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0))))))))))))).xatan())))).add(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.5)))).mul(Rc::new(RefCell::new(Some(MOREBITS)))))))))));
    }

    /// Sin returns the sine of the radian argument x.
    pub fn sin(&self) -> Rc<RefCell<Option<Decimal>>> {
        let mut P_I4_A = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(7.85398125648498535156e-1))));
        let mut P_I4_B = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3.77489470793079817668e-8))));
        let mut P_I4_C = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2.69515142907905952645e-15))));
        let mut M4_P_I = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.273239544735162542821171882678754627704620361328125))));
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        return d.clone();
    }
                // make argument positive but save the sign
        let mut sign = Rc::new(RefCell::new(Some(false)));
        if (*d.borrow_mut().as_mut().unwrap()).less_than(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        { let new_val = (*d.borrow_mut().as_mut().unwrap()).neg(); *self.borrow_mut() = Some(new_val); };
        { let new_val = true; *sign.borrow_mut() = Some(new_val); };
    }
        let mut j = (*d.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(M4_P_I)))).int_part();
        let mut y = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*float64.borrow().as_ref().unwrap())(j.clone())))));
                // map zeros to origin
        if (*j.borrow_mut().as_mut().unwrap()) & 1 == 1 {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*y.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))))))); *y.borrow_mut() = Some(new_val); };
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() & 7); };
                // reflect in x axis
        if (*j.borrow_mut().as_mut().unwrap()) > 3 {
        { let new_val = !(*sign.borrow_mut().as_mut().unwrap()); *sign.borrow_mut() = Some(new_val); };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 4); };
    }
        let mut z = (*d.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_A)))))))).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_B)))))))).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_C))))))));
        let mut zz = (*z.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap())))));
        if (*j.borrow_mut().as_mut().unwrap()) == 1 || (*j.borrow_mut().as_mut().unwrap()) == 2 {
        let mut w = (*zz.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).mul(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[0 as usize].clone().mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[1 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[2 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[3 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[4 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[5 as usize].clone()))))))));
        { let new_val = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))).sub(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.5)))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))))))).add(Rc::new(RefCell::new(Some((*w.borrow_mut().as_mut().unwrap()))))); *y.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*z.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).mul(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[0 as usize].clone().mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[1 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[2 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[3 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[4 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[5 as usize].clone())))))))))))); *y.borrow_mut() = Some(new_val); };
    }
        if (*sign.borrow_mut().as_mut().unwrap()) {
        { let new_val = (*y.borrow_mut().as_mut().unwrap()).neg(); *y.borrow_mut() = Some(new_val); };
    }
        return y.clone();
    }

    /// Cos returns the cosine of the radian argument x.
    pub fn cos(&self) -> Rc<RefCell<Option<Decimal>>> {
        let mut P_I4_A = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(7.85398125648498535156e-1))));
        let mut P_I4_B = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3.77489470793079817668e-8))));
        let mut P_I4_C = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2.69515142907905952645e-15))));
        let mut M4_P_I = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.273239544735162542821171882678754627704620361328125))));
                // make argument positive
        let mut sign = Rc::new(RefCell::new(Some(false)));
        if (*d.borrow_mut().as_mut().unwrap()).less_than(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        { let new_val = (*d.borrow_mut().as_mut().unwrap()).neg(); *self.borrow_mut() = Some(new_val); };
    }
        let mut j = (*d.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(M4_P_I)))).int_part();
        let mut y = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*float64.borrow().as_ref().unwrap())(j.clone())))));
                // map zeros to origin
        if (*j.borrow_mut().as_mut().unwrap()) & 1 == 1 {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*y.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))))))); *y.borrow_mut() = Some(new_val); };
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() & 7); };
                // reflect in x axis
        if (*j.borrow_mut().as_mut().unwrap()) > 3 {
        { let new_val = !(*sign.borrow_mut().as_mut().unwrap()); *sign.borrow_mut() = Some(new_val); };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 4); };
    }
        if (*j.borrow_mut().as_mut().unwrap()) > 1 {
        { let new_val = !(*sign.borrow_mut().as_mut().unwrap()); *sign.borrow_mut() = Some(new_val); };
    }
        let mut z = (*d.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_A)))))))).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_B)))))))).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_C))))))));
        let mut zz = (*z.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap())))));
        if (*j.borrow_mut().as_mut().unwrap()) == 1 || (*j.borrow_mut().as_mut().unwrap()) == 2 {
        { let new_val = (*z.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).mul(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[0 as usize].clone().mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[1 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[2 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[3 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[4 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_sin.borrow().as_ref().unwrap())[5 as usize].clone())))))))))))); *y.borrow_mut() = Some(new_val); };
    } else {
        let mut w = (*zz.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).mul(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[0 as usize].clone().mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[1 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[2 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[3 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[4 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_cos.borrow().as_ref().unwrap())[5 as usize].clone()))))))));
        { let new_val = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))).sub(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.5)))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))))))).add(Rc::new(RefCell::new(Some((*w.borrow_mut().as_mut().unwrap()))))); *y.borrow_mut() = Some(new_val); };
    }
        if (*sign.borrow_mut().as_mut().unwrap()) {
        { let new_val = (*y.borrow_mut().as_mut().unwrap()).neg(); *y.borrow_mut() = Some(new_val); };
    }
        return y.clone();
    }

    /// Tan returns the tangent of the radian argument x.
    pub fn tan(&self) -> Rc<RefCell<Option<Decimal>>> {
        let mut P_I4_A = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(7.85398125648498535156e-1))));
        let mut P_I4_B = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3.77489470793079817668e-8))));
        let mut P_I4_C = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2.69515142907905952645e-15))));
        let mut M4_P_I = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.273239544735162542821171882678754627704620361328125))));
        if (*d.borrow_mut().as_mut().unwrap()).equal(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        return d.clone();
    }
                // make argument positive but save the sign
        let mut sign = Rc::new(RefCell::new(Some(false)));
        if (*d.borrow_mut().as_mut().unwrap()).less_than(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0.0)))))))) {
        { let new_val = (*d.borrow_mut().as_mut().unwrap()).neg(); *self.borrow_mut() = Some(new_val); };
        { let new_val = true; *sign.borrow_mut() = Some(new_val); };
    }
        let mut j = (*d.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(M4_P_I)))).int_part();
        let mut y = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*float64.borrow().as_ref().unwrap())(j.clone())))));
                // map zeros to origin
        if (*j.borrow_mut().as_mut().unwrap()) & 1 == 1 {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*y.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1.0)))))))); *y.borrow_mut() = Some(new_val); };
    }
        let mut z = (*d.borrow_mut().as_mut().unwrap()).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_A)))))))).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_B)))))))).sub(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some(P_I4_C))))))));
        let mut zz = (*z.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap())))));
        if (*zz.borrow_mut().as_mut().unwrap()).greater_than(Rc::new(RefCell::new(Some((*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1e-14)))))))) {
        let mut w = (*zz.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*_tanP.borrow().as_ref().unwrap())[0 as usize].clone().mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_tanP.borrow().as_ref().unwrap())[1 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_tanP.borrow().as_ref().unwrap())[2 as usize].clone()))))))));
        let mut x = (*zz.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*_tanQ.borrow().as_ref().unwrap())[1 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_tanQ.borrow().as_ref().unwrap())[2 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_tanQ.borrow().as_ref().unwrap())[3 as usize].clone())))).mul(Rc::new(RefCell::new(Some((*zz.borrow_mut().as_mut().unwrap()))))).add(Rc::new(RefCell::new(Some((*_tanQ.borrow().as_ref().unwrap())[4 as usize].clone()))));
        { let new_val = (*z.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*z.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*w.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*x.borrow_mut().as_mut().unwrap()))))))))))))); *y.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*z.borrow_mut().as_mut().unwrap()); *y.borrow_mut() = Some(new_val); };
    }
        if (*j.borrow_mut().as_mut().unwrap()) & 2 == 2 {
        { let new_val = (*NewFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-1.0)))).div(Rc::new(RefCell::new(Some((*y.borrow_mut().as_mut().unwrap()))))); *y.borrow_mut() = Some(new_val); };
    }
        if (*sign.borrow_mut().as_mut().unwrap()) {
        { let new_val = (*y.borrow_mut().as_mut().unwrap()).neg(); *y.borrow_mut() = Some(new_val); };
    }
        return y.clone();
    }
}

impl NullDecimal {
    /// Scan implements the sql.Scanner interface for database deserialization.
    pub fn scan(&mut self, value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
        if (*value.borrow()).is_none() {
        { let new_val = false; *self.valid.borrow_mut() = Some(new_val); };
        return Rc::new(RefCell::new(None));
    }
        { let new_val = true; *self.valid.borrow_mut() = Some(new_val); };
        return (*self.decimal.clone().borrow().as_mut().unwrap()).scan(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))));
    }

    /// Value implements the driver.Valuer interface for database serialization.
    pub fn value(&self) -> (Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        if !self.valid.clone() {
        return (Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)));
    }
        return Rc::new(RefCell::new(Some((*self.decimal.clone().borrow().as_mut().unwrap()).value())));
    }

    /// UnmarshalJSON implements the json.Unmarshaler interface.
    pub fn unmarshal_j_s_o_n(&mut self, decimalBytes: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
        if (*(*string.borrow().as_ref().unwrap())(decimalBytes.clone()).borrow().as_ref().unwrap()) == "null".to_string() {
        { let new_val = false; *self.valid.borrow_mut() = Some(new_val); };
        return Rc::new(RefCell::new(None));
    }
        { let new_val = true; *self.valid.borrow_mut() = Some(new_val); };
        return (*self.decimal.clone().borrow().as_mut().unwrap()).unmarshal_j_s_o_n(Rc::new(RefCell::new(Some((*decimalBytes.borrow_mut().as_mut().unwrap())))));
    }

    /// MarshalJSON implements the json.Marshaler interface.
    pub fn marshal_j_s_o_n(&self) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        if !self.valid.clone() {
        return (Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*"null".to_string().borrow().as_ref().unwrap()).as_bytes().to_vec())))))), Rc::new(RefCell::new(None)));
    }
        return Rc::new(RefCell::new(Some((*self.decimal.clone().borrow().as_mut().unwrap()).marshal_j_s_o_n())));
    }

    /// UnmarshalText implements the encoding.TextUnmarshaler interface for XML
    /// deserialization
    pub fn unmarshal_text(&mut self, text: Rc<RefCell<Option<Vec<u8>>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {
        let mut str = (*string.borrow().as_ref().unwrap())(text.clone());
                // check for empty XML or XML without body e.g., <tag></tag>
        if (*str.borrow_mut().as_mut().unwrap()) == "".to_string() {
        { let new_val = false; *self.valid.borrow_mut() = Some(new_val); };
        return Rc::new(RefCell::new(None));
    }
        let mut err = (*self.decimal.clone().borrow().as_mut().unwrap()).unmarshal_text(Rc::new(RefCell::new(Some((*text.borrow_mut().as_mut().unwrap())))));
    if (*err.borrow()).is_some() {
        { let new_val = false; *self.valid.borrow_mut() = Some(new_val); };
        return err.clone();
    }
        { let new_val = true; *self.valid.borrow_mut() = Some(new_val); };
        return Rc::new(RefCell::new(None));
    }

    /// MarshalText implements the encoding.TextMarshaler interface for XML
    /// serialization.
    pub fn marshal_text(&self) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
        if !self.valid.clone() {
        return (Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![])))))), Rc::new(RefCell::new(None)));
    }
        return Rc::new(RefCell::new(Some((*self.decimal.clone().borrow().as_mut().unwrap()).marshal_text())));
    }
}

/// New returns a new fixed-point decimal, value * 10 ^ exp.
pub fn new(value: Rc<RefCell<Option<i64>>>, exp: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {

    return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))))))), exp: exp.clone() })));
}

/// NewFromInt converts a int64 to Decimal.
///
/// Example:
///
///     NewFromInt(123).String() // output: "123"
///     NewFromInt(-10).String() // output: "-10"
pub fn new_from_int(value: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Decimal>>> {

    return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))))))), exp: Rc::new(RefCell::new(Some(0))) })));
}

/// NewFromInt32 converts a int32 to Decimal.
///
/// Example:
///
///     NewFromInt(123).String() // output: "123"
///     NewFromInt(-10).String() // output: "-10"
pub fn new_from_int32(value: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {

    return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*int64.borrow().as_ref().unwrap())(value.clone())))))))), exp: Rc::new(RefCell::new(Some(0))) })));
}

/// NewFromBigInt returns a new Decimal from a big.Int, value * 10 ^ exp
pub fn new_from_big_int(value: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>, exp: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {

    return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).set(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))))))), exp: exp.clone() })));
}

/// NewFromString returns a new Decimal from a string representation.
/// Trailing zeroes are not trimmed.
///
/// Example:
///
///     d, err := NewFromString("-123.45")
///     d2, err := NewFromString(".0001")
///     d3, err := NewFromString("1.47000")
///
pub fn new_from_string(value: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<Decimal>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    let mut originalInput = Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap()))));
    let mut intString: Rc<RefCell<Option<String>>> = String::new();
    let mut exp: Rc<RefCell<Option<i64>>> = Default::default();

        // Check if number is using scientific notation
    let mut eIndex = (*strings.borrow_mut().as_mut().unwrap()).index_any(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some("Ee".to_string()))));
    if (*eIndex.borrow_mut().as_mut().unwrap()) != (*-1.borrow().as_ref().unwrap()) {
        let (mut expInt, mut err) = (*strconv.borrow_mut().as_mut().unwrap()).parse_int(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap())[(*eIndex.borrow_mut().as_mut().unwrap()) + 1 as usize..].to_vec())))))), Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(32))));
        if (*err.borrow()).is_some() {
        let (mut e, mut ok) = ({
        let val = err.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) && (*(*(*e.borrow().as_ref().unwrap()).err.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) == (*(*strconv.borrow_mut().as_mut().unwrap())::err_range.borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("can't convert {} to decimal: fractional part too long", (*value.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("can't convert {} to decimal: exponent is not numeric", (*value.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }
        { let new_val = Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap())[..(*eIndex.borrow_mut().as_mut().unwrap()) as usize].to_vec()))); *value.borrow_mut() = Some(new_val); };
        { let new_val = (*expInt.borrow_mut().as_mut().unwrap()); *exp.borrow_mut() = Some(new_val); };
    }

    let mut pIndex = Rc::new(RefCell::new(Some(-1)));
    let mut vLen = (*value.borrow().as_ref().unwrap()).len();
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < (*vLen.borrow_mut().as_mut().unwrap()) {
        if (*(*value.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) == ('.' as i32) {
        if (*pIndex.borrow_mut().as_mut().unwrap()) > (*-1.borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("can't convert {} to decimal: too many .s", (*value.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }
        { let new_val = (*i.borrow_mut().as_mut().unwrap()); *pIndex.borrow_mut() = Some(new_val); };
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    if (*pIndex.borrow_mut().as_mut().unwrap()) == (*-1.borrow().as_ref().unwrap()) {
                // There is no decimal point, we can just parse the original string as
                // an int
        { let new_val = (*value.borrow_mut().as_mut().unwrap()); *intString.borrow_mut() = Some(new_val); };
    } else {
        if (*pIndex.borrow_mut().as_mut().unwrap()) + 1 < (*vLen.borrow_mut().as_mut().unwrap()) {
        { let new_val = (*Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap())[..(*pIndex.borrow_mut().as_mut().unwrap()) as usize].to_vec()))).borrow().as_ref().unwrap()) + (*Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap())[(*pIndex.borrow_mut().as_mut().unwrap()) + 1 as usize..].to_vec()))).borrow().as_ref().unwrap()); *intString.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap())[..(*pIndex.borrow_mut().as_mut().unwrap()) as usize].to_vec()))); *intString.borrow_mut() = Some(new_val); };
    }
        let mut expInt = Rc::new(RefCell::new(Some(-(*Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap())[(*pIndex.borrow_mut().as_mut().unwrap()) + 1 as usize..].to_vec()))).borrow().as_ref().unwrap()).len())));
        { let mut guard = exp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*int64.borrow().as_ref().unwrap())(expInt.clone())); };
    }

        // There is no decimal point, we can just parse the original string as
        // an int
    let mut dValue: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>> = Rc::new(RefCell::new(None));

        // strconv.ParseInt is faster than new(big.Int).SetString so this is just a shortcut for strings we know won't overflow
    if (*intString.borrow().as_ref().unwrap()).len() <= 18 {
        let (mut parsed64, mut err) = (*strconv.borrow_mut().as_mut().unwrap()).parse_int(Rc::new(RefCell::new(Some((*intString.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(64))));
        if (*err.borrow()).is_some() {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("can't convert {} to decimal", (*value.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }
        { let new_val = (*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*parsed64.borrow_mut().as_mut().unwrap()))))); *dValue.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))); *dValue.borrow_mut() = Some(new_val); };
        let (_, mut ok) = (*dValue.borrow_mut().as_mut().unwrap()).set_string(Rc::new(RefCell::new(Some((*intString.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(10))));
        if !(*ok.borrow_mut().as_mut().unwrap()) {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("can't convert {} to decimal", (*value.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }
    }

    if (*exp.borrow_mut().as_mut().unwrap()) < (*(*math.borrow_mut().as_mut().unwrap())::min_int32.borrow().as_ref().unwrap()) || (*exp.borrow_mut().as_mut().unwrap()) > (*(*math.borrow_mut().as_mut().unwrap())::max_int32.borrow().as_ref().unwrap()) {
                // NOTE(vadim): I doubt a string could realistically be this long
        return (Rc::new(RefCell::new(Some(Decimal {  }))), Rc::new(RefCell::new(Some(Box::new(format!("can't convert {} to decimal: fractional part too long", (*originalInput.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }

        // NOTE(vadim): I doubt a string could realistically be this long
    return (Rc::new(RefCell::new(Some(Decimal { value: dValue.clone(), exp: Rc::new(RefCell::new(Some((*int32.borrow().as_ref().unwrap())(exp.clone())))) }))), Rc::new(RefCell::new(None)));
}

/// NewFromFormattedString returns a new Decimal from a formatted string representation.
/// The second argument - replRegexp, is a regular expression that is used to find characters that should be
/// removed from given decimal string representation. All matched characters will be replaced with an empty string.
///
/// Example:
///
///     r := regexp.MustCompile("[$,]")
///     d1, err := NewFromFormattedString("$5,125.99", r)
///
///     r2 := regexp.MustCompile("[_]")
///     d2, err := NewFromFormattedString("1_000_000", r2)
///
///     r3 := regexp.MustCompile("[USD\\s]")
///     d3, err := NewFromFormattedString("5000 USD", r3)
///
pub fn new_from_formatted_string(value: Rc<RefCell<Option<String>>>, replRegexp: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>) -> (Rc<RefCell<Option<Decimal>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    let mut parsedValue = (*replRegexp.borrow_mut().as_mut().unwrap()).replace_all_string(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some("".to_string()))));
    let (mut d, mut err) = (*NewFromString.borrow().as_ref().unwrap())(parsedValue.clone());
    if (*err.borrow()).is_some() {
        return (Rc::new(RefCell::new(Some(Decimal {  }))), err.clone());
    }
    return (d.clone(), Rc::new(RefCell::new(None)));
}

/// RequireFromString returns a new Decimal from a string representation
/// or panics if NewFromString would have returned an error.
///
/// Example:
///
///     d := RequireFromString("-123.45")
///     d2 := RequireFromString(".0001")
///
pub fn require_from_string(value: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Decimal>>> {

    let (mut dec, mut err) = (*NewFromString.borrow().as_ref().unwrap())(value.clone());
    if (*err.borrow()).is_some() {
        panic!("{:?}", (*err.borrow_mut().as_mut().unwrap()));
    }
    return dec.clone();
}

/// NewFromFloat converts a float64 to Decimal.
///
/// The converted number will contain the number of significant digits that can be
/// represented in a float with reliable roundtrip.
/// This is typically 15 digits, but may be more in some cases.
/// See https://www.exploringbinary.com/decimal-precision-of-binary-floating-point-numbers/ for more information.
///
/// For slightly faster conversion, use NewFromFloatWithExponent where you can specify the precision in absolute terms.
///
/// NOTE: this will panic on NaN, +/-inf
pub fn new_from_float(value: Rc<RefCell<Option<f64>>>) -> Rc<RefCell<Option<Decimal>>> {

    if (*value.borrow_mut().as_mut().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some((*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(0)))))));
    }
    return Rc::new(RefCell::new(Some((*newFromFloat.borrow().as_ref().unwrap())(value.clone(), Rc::new(RefCell::new(Some((*math.borrow_mut().as_mut().unwrap()).float64bits(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))))))), Rc::new(RefCell::new(Some(float64info.clone())))))));
}

/// NewFromFloat32 converts a float32 to Decimal.
///
/// The converted number will contain the number of significant digits that can be
/// represented in a float with reliable roundtrip.
/// This is typically 6-8 digits depending on the input.
/// See https://www.exploringbinary.com/decimal-precision-of-binary-floating-point-numbers/ for more information.
///
/// For slightly faster conversion, use NewFromFloatWithExponent where you can specify the precision in absolute terms.
///
/// NOTE: this will panic on NaN, +/-inf
pub fn new_from_float32(value: Rc<RefCell<Option<f32>>>) -> Rc<RefCell<Option<Decimal>>> {

    if (*value.borrow_mut().as_mut().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some((*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(0)))))));
    }

        // XOR is workaround for https://github.com/golang/go/issues/26285
    let mut a = Rc::new(RefCell::new(Some((*(*math.borrow_mut().as_mut().unwrap()).float32bits(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) ^ 0x80808080)));
    return Rc::new(RefCell::new(Some((*newFromFloat.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*float64.borrow().as_ref().unwrap())(value.clone())))), Rc::new(RefCell::new(Some((*(*uint64.borrow().as_ref().unwrap())(a.clone()).borrow().as_ref().unwrap()) ^ 0x80808080))), Rc::new(RefCell::new(Some(float32info.clone())))))));
}

pub fn new_from_float(val: Rc<RefCell<Option<f64>>>, bits: Rc<RefCell<Option<u64>>>, flt: Rc<RefCell<Option<floatInfo>>>) -> Rc<RefCell<Option<Decimal>>> {

    if (*(*math.borrow_mut().as_mut().unwrap()).is_na_n(Rc::new(RefCell::new(Some((*val.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) || (*(*math.borrow_mut().as_mut().unwrap()).is_inf(Rc::new(RefCell::new(Some((*val.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()) {
        panic!("{:?}", Rc::new(RefCell::new(Some(format!("Cannot create a Decimal from {}", /* ERROR: Type information not available for print argument */ (*val.borrow_mut().as_mut().unwrap()))))));
    }
    let mut exp = Rc::new(RefCell::new(Some((*(*int.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*bits.borrow_mut().as_mut().unwrap()) >> (*(*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) & (1 << (*(*(*flt.borrow().as_ref().unwrap()).expbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) - 1))));
    let mut mant = Rc::new(RefCell::new(Some((*bits.borrow_mut().as_mut().unwrap()) & ((*(*uint64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap()) << (*(*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) - 1))));

    match (*exp.borrow_mut().as_mut().unwrap()) {
        0 => {
                        // denormalized
            { let mut guard = exp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        }
        _ => {
                        // add implicit top bit
            { let mut guard = mant.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | (*(*uint64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap()) << (*(*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap())); };
        }
    }
        // denormalized
        // add implicit top bit
    { let mut guard = exp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*(*flt.borrow().as_ref().unwrap()).bias.borrow().as_ref().unwrap())); };

    let mut d: Rc<RefCell<Option<decimal>>> = Default::default();
    (*d.borrow_mut().as_mut().unwrap()).assign(Rc::new(RefCell::new(Some((*mant.borrow_mut().as_mut().unwrap())))));
    (*d.borrow_mut().as_mut().unwrap()).shift(Rc::new(RefCell::new(Some((*exp.borrow_mut().as_mut().unwrap()) - (*(*int.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap())))));
    { let new_val = (*bits.borrow_mut().as_mut().unwrap()) >> ((*(*(*flt.borrow().as_ref().unwrap()).expbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) + (*(*(*flt.borrow().as_ref().unwrap()).mantbits.borrow().as_ref().unwrap()).borrow().as_ref().unwrap())) != 0; *(*d.borrow_mut().as_mut().unwrap()).neg.borrow_mut() = Some(new_val); };

    (*roundShortest.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(d.clone()))), mant.clone(), exp.clone(), flt.clone());

        // If less than 19 digits, we can do calculation in an int64.
    if (*(*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) < 19 {
        let mut tmp = (*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(0))));
        let mut m = (*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(1))));
        let mut i = Rc::new(RefCell::new(Some((*(*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow_mut().as_mut().unwrap()) >= 0 {
        { let mut guard = tmp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*m.borrow_mut().as_mut().unwrap()) * (*(*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*(*d.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[(*i.borrow_mut().as_mut().unwrap()) as usize].clone().borrow().as_ref().unwrap()) - ('0' as i32))))).borrow().as_ref().unwrap())); };
        { let mut guard = m.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * 10); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*(*d.borrow().as_ref().unwrap()).neg.borrow().as_ref().unwrap()) {
        { let mut guard = tmp.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * -1); };
    }
        return Rc::new(RefCell::new(Some(Decimal { value: Rc::new(RefCell::new(Some((*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*tmp.borrow_mut().as_mut().unwrap())))))))), exp: Rc::new(RefCell::new(Some((*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) - (*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap())))) })));
    }
    let mut dValue = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default())));
    let (mut dValue, mut ok) = (*dValue.borrow_mut().as_mut().unwrap()).set_string(Rc::new(RefCell::new(Some((*string.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*(*d.borrow_mut().as_mut().unwrap()).d.borrow().as_ref().unwrap())[..(*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()) as usize].to_vec())))))))))), Rc::new(RefCell::new(Some(10))));
    if (*ok.borrow_mut().as_mut().unwrap()) {
        return Rc::new(RefCell::new(Some(Decimal { value: dValue.clone(), exp: Rc::new(RefCell::new(Some((*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) - (*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap())))) })));
    }

    return Rc::new(RefCell::new(Some((*NewFromFloatWithExponent.borrow().as_ref().unwrap())(val.clone(), Rc::new(RefCell::new(Some((*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d.borrow().as_ref().unwrap()).dp.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()) - (*(*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d.borrow().as_ref().unwrap()).nd.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap()))))))));
}

/// NewFromFloatWithExponent converts a float64 to Decimal, with an arbitrary
/// number of fractional digits.
///
/// Example:
///
///     NewFromFloatWithExponent(123.456, -2).String() // output: "123.46"
///
pub fn new_from_float_with_exponent(value: Rc<RefCell<Option<f64>>>, exp: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Decimal>>> {

    if (*(*math.borrow_mut().as_mut().unwrap()).is_na_n(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) || (*(*math.borrow_mut().as_mut().unwrap()).is_inf(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()) {
        panic!("{:?}", Rc::new(RefCell::new(Some(format!("Cannot create a Decimal from {}", /* ERROR: Type information not available for print argument */ (*value.borrow_mut().as_mut().unwrap()))))));
    }

    let mut bits = (*math.borrow_mut().as_mut().unwrap()).float64bits(Rc::new(RefCell::new(Some((*value.borrow_mut().as_mut().unwrap())))));
    let mut mant = Rc::new(RefCell::new(Some((*bits.borrow_mut().as_mut().unwrap()) & (1 << 52 - 1))));
    let mut exp2 = (*int32.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(((*bits.borrow_mut().as_mut().unwrap()) >> 52) & (1 << 11 - 1)))));
    let mut sign = Rc::new(RefCell::new(Some((*bits.borrow_mut().as_mut().unwrap()) >> 63)));

    if (*exp2.borrow_mut().as_mut().unwrap()) == 0 {
                // specials
        if (*mant.borrow_mut().as_mut().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some(Decimal {  })));
    }
                // subnormal
        { let mut guard = exp2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else {
                // normal
        { let mut guard = mant.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | 1 << 52); };
    }

        // specials
        // subnormal
        // normal
    { let mut guard = exp2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1023 + 52); };

        // normalizing base-2 values
    while (*mant.borrow_mut().as_mut().unwrap()) & 1 == 0 {
        { let new_val = (*mant.borrow_mut().as_mut().unwrap()) >> 1; *mant.borrow_mut() = Some(new_val); };
        { let mut guard = exp2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // maximum number of fractional base-10 digits to represent 2^N exactly cannot be more than -N if N<0
    if (*exp.borrow_mut().as_mut().unwrap()) < 0 && (*exp.borrow_mut().as_mut().unwrap()) < (*exp2.borrow_mut().as_mut().unwrap()) {
        if (*exp2.borrow_mut().as_mut().unwrap()) < 0 {
        { let new_val = (*exp2.borrow_mut().as_mut().unwrap()); *exp.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = 0; *exp.borrow_mut() = Some(new_val); };
    }
    }

        // representing 10^M * 2^N as 5^M * 2^(M+N)
    { let mut guard = exp2.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - (*exp.borrow_mut().as_mut().unwrap())); };

    let mut temp = (*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some(1))));
    let mut dMant = (*big.borrow_mut().as_mut().unwrap()).new_int(Rc::new(RefCell::new(Some((*int64.borrow().as_ref().unwrap())(mant.clone())))));

        // applying 5^M
    if (*exp.borrow_mut().as_mut().unwrap()) > 0 {
        { let new_val = (*temp.borrow_mut().as_mut().unwrap()).set_int64(Rc::new(RefCell::new(Some((*int64.borrow().as_ref().unwrap())(exp.clone()))))); *temp.borrow_mut() = Some(new_val); };
        { let new_val = (*temp.borrow_mut().as_mut().unwrap()).exp(Rc::new(RefCell::new(Some((*fiveInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(None)))); *temp.borrow_mut() = Some(new_val); };
    } else if (*exp.borrow_mut().as_mut().unwrap()) < 0 {
        { let new_val = (*temp.borrow_mut().as_mut().unwrap()).set_int64(Rc::new(RefCell::new(Some(-(*int64.borrow().as_ref().unwrap())(exp.clone()))))); *temp.borrow_mut() = Some(new_val); };
        { let new_val = (*temp.borrow_mut().as_mut().unwrap()).exp(Rc::new(RefCell::new(Some((*fiveInt.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(None)))); *temp.borrow_mut() = Some(new_val); };
        { let new_val = (*dMant.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*dMant.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()))))); *dMant.borrow_mut() = Some(new_val); };
        { let new_val = (*temp.borrow_mut().as_mut().unwrap()).set_uint64(Rc::new(RefCell::new(Some(1)))); *temp.borrow_mut() = Some(new_val); };
    }

        // applying 2^(M+N)
    if (*exp2.borrow_mut().as_mut().unwrap()) > 0 {
        { let new_val = (*dMant.borrow_mut().as_mut().unwrap()).lsh(Rc::new(RefCell::new(Some((*dMant.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*uint.borrow().as_ref().unwrap())(exp2.clone()))))); *dMant.borrow_mut() = Some(new_val); };
    } else if (*exp2.borrow_mut().as_mut().unwrap()) < 0 {
        { let new_val = (*temp.borrow_mut().as_mut().unwrap()).lsh(Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*uint.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(-(*exp2.borrow_mut().as_mut().unwrap()))))))))); *temp.borrow_mut() = Some(new_val); };
    }

        // rounding and downscaling
    if (*exp.borrow_mut().as_mut().unwrap()) > 0 || (*exp2.borrow_mut().as_mut().unwrap()) < 0 {
        let mut halfDown = Rc::new(RefCell::new(Some(Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>::default()))).rsh(Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(1))));
        { let new_val = (*dMant.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some((*dMant.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*halfDown.borrow_mut().as_mut().unwrap()))))); *dMant.borrow_mut() = Some(new_val); };
        { let new_val = (*dMant.borrow_mut().as_mut().unwrap()).quo(Rc::new(RefCell::new(Some((*dMant.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some((*temp.borrow_mut().as_mut().unwrap()))))); *dMant.borrow_mut() = Some(new_val); };
    }

    if (*sign.borrow_mut().as_mut().unwrap()) == 1 {
        { let new_val = (*dMant.borrow_mut().as_mut().unwrap()).neg(Rc::new(RefCell::new(Some((*dMant.borrow_mut().as_mut().unwrap()))))); *dMant.borrow_mut() = Some(new_val); };
    }

    return Rc::new(RefCell::new(Some(Decimal { value: dMant.clone(), exp: exp.clone() })));
}

/// Abs calculates absolute value of any int32. Used for calculating absolute value of decimal's exponent.
pub fn abs(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*n.borrow_mut().as_mut().unwrap()) < 0 {
        return Rc::new(RefCell::new(Some(-(*n.borrow_mut().as_mut().unwrap()))));
    }
    return n.clone();
}

/// Min returns the smallest Decimal that was passed in the arguments.
///
/// To call this function with an array, you must do:
///
///     Min(arr[0], arr[1:]...)
///
/// This makes it harder to accidentally call Min with 0 arguments.
pub fn min(first: Rc<RefCell<Option<Decimal>>>, rest: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<Decimal>>> {

    let mut ans = Rc::new(RefCell::new(Some((*first.borrow_mut().as_mut().unwrap()))));
    for item in &(*rest.borrow_mut().as_mut().unwrap()) {
        if (*item.cmp(Rc::new(RefCell::new(Some((*ans.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) < 0 {
        { let new_val = item; *ans.borrow_mut() = Some(new_val); };
    }
    }
    return ans.clone();
}

/// Max returns the largest Decimal that was passed in the arguments.
///
/// To call this function with an array, you must do:
///
///     Max(arr[0], arr[1:]...)
///
/// This makes it harder to accidentally call Max with 0 arguments.
pub fn max(first: Rc<RefCell<Option<Decimal>>>, rest: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<Decimal>>> {

    let mut ans = Rc::new(RefCell::new(Some((*first.borrow_mut().as_mut().unwrap()))));
    for item in &(*rest.borrow_mut().as_mut().unwrap()) {
        if (*item.cmp(Rc::new(RefCell::new(Some((*ans.borrow_mut().as_mut().unwrap()))))).borrow().as_ref().unwrap()) > 0 {
        { let new_val = item; *ans.borrow_mut() = Some(new_val); };
    }
    }
    return ans.clone();
}

/// Sum returns the combined total of the provided first and rest Decimals
pub fn sum(first: Rc<RefCell<Option<Decimal>>>, rest: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<Decimal>>> {

    let mut total = Rc::new(RefCell::new(Some((*first.borrow_mut().as_mut().unwrap()))));
    for item in &(*rest.borrow_mut().as_mut().unwrap()) {
        { let new_val = (*total.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(item)))); *total.borrow_mut() = Some(new_val); };
    }

    return total.clone();
}

/// Avg returns the average value of the provided first and rest Decimals
pub fn avg(first: Rc<RefCell<Option<Decimal>>>, rest: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<Decimal>>> {

    let mut count = (*New.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*int64.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*rest.borrow().as_ref().unwrap()).len() + 1))))))), Rc::new(RefCell::new(Some(0))));
    let mut sum = (*Sum.borrow().as_ref().unwrap())(first.clone(), rest.clone());
    return Rc::new(RefCell::new(Some((*sum.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*count.borrow_mut().as_mut().unwrap()))))))));
}

/// RescalePair rescales two decimals to common exponential value (minimal exp of both decimals)
pub fn rescale_pair(d1: Rc<RefCell<Option<Decimal>>>, d2: Rc<RefCell<Option<Decimal>>>) -> (Rc<RefCell<Option<Decimal>>>, Rc<RefCell<Option<Decimal>>>) {

    (*d1.borrow_mut().as_mut().unwrap()).ensure_initialized();
    (*d2.borrow_mut().as_mut().unwrap()).ensure_initialized();

    if (*(*(*d1.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) == (*(*(*d2.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        return (d1.clone(), d2.clone());
    }

    let mut baseScale = (*min.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some((*(*d1.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*(*d2.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap())))));
    if (*baseScale.borrow_mut().as_mut().unwrap()) != (*(*(*d1.borrow().as_ref().unwrap()).exp.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some((*d1.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some((*baseScale.borrow_mut().as_mut().unwrap())))))))), d2.clone());
    }
    return (d1.clone(), Rc::new(RefCell::new(Some((*d2.borrow_mut().as_mut().unwrap()).rescale(Rc::new(RefCell::new(Some((*baseScale.borrow_mut().as_mut().unwrap())))))))));
}

pub fn min(x: Rc<RefCell<Option<i32>>>, y: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*x.borrow_mut().as_mut().unwrap()) >= (*y.borrow_mut().as_mut().unwrap()) {
        return y.clone();
    }
    return x.clone();
}

pub fn unquote_if_quoted(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    let mut bytes: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(Some(Default::default())));

    if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow_mut().as_mut().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<String>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<String>().unwrap()).clone())));
        { let new_val = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).as_bytes().to_vec()))); *bytes.borrow_mut() = Some(new_val); };;
    } else if let Some(v) = (|| -> Option<Box<dyn Any>> {
        let val = (*value.borrow_mut().as_mut().unwrap());
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(val) = any_val.downcast_ref::<>() {
                return Some(Box::new(val.clone()) as Box<dyn Any>);
            }
        }
        None
    })() {
        let v = Rc::new(RefCell::new(Some((*v.downcast_ref::<>().unwrap()).clone())));
        { let new_val = (*v.borrow_mut().as_mut().unwrap()); *bytes.borrow_mut() = Some(new_val); };;
    } else {
        let v = (*value.borrow_mut().as_mut().unwrap());
        return (Rc::new(RefCell::new(Some("".to_string()))), Rc::new(RefCell::new(Some(Box::new(format!("could not convert value '%+v' to byte array of type '<type>'", (*value.borrow_mut().as_mut().unwrap()), (*value.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));;
    }

        // If the amount is quoted, strip the quotes
    if (*bytes.borrow().as_ref().unwrap()).len() > 2 && (*(*bytes.borrow().as_ref().unwrap())[0 as usize].clone().borrow().as_ref().unwrap()) == ('"' as i32) && (*(*bytes.borrow().as_ref().unwrap())[(*bytes.borrow().as_ref().unwrap()).len() - 1 as usize].clone().borrow().as_ref().unwrap()) == ('"' as i32) {
        { let new_val = Rc::new(RefCell::new(Some((*bytes.borrow().as_ref().unwrap())[1 as usize..(*bytes.borrow().as_ref().unwrap()).len() - 1 as usize].to_vec()))); *bytes.borrow_mut() = Some(new_val); };
    }
    return (Rc::new(RefCell::new(Some((*string.borrow().as_ref().unwrap())(bytes.clone())))), Rc::new(RefCell::new(None)));
}

pub fn new_null_decimal(d: Rc<RefCell<Option<Decimal>>>) -> Rc<RefCell<Option<NullDecimal>>> {

    return Rc::new(RefCell::new(Some(NullDecimal { decimal: d.clone(), valid: Rc::new(RefCell::new(Some(true))) })));
}