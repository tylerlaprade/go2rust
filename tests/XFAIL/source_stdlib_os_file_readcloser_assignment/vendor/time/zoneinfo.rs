use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoByteSequence,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    r#mod::{INTERNAL_TO_ABSOLUTE, Month, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, UNIX_TO_INTERNAL, absDays, absSeconds, days_before, days_in, is_leap},
    zoneinfo_unix::{init_local},
};

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const ALPHA: i64 = -1 << 63;
pub(crate) const OMEGA: u64 = (((1 as u64) << (63 as u64)) - (1 as u64));


pub(crate) const RULE_JULIAN: i32 = 0;
pub(crate) const RULE_D_O_Y: i32 = 1;
pub(crate) const RULE_MONTH_WEEK_DAY: i32 = 2;


/// A Location maps time instants to the zone in use at that time.
/// Typically, the Location represents the collection of time offsets
/// in use in a geographical area. For many Locations the time offset varies
/// depending on whether daylight savings time is in use at the time instant.
///
/// Location is used to provide a time zone in a printed Time value and for
/// calculations involving intervals that may cross daylight savings time
/// boundaries.
#[derive(Debug, Clone)]
pub struct Location {
    pub name: Arc<Mutex<Option<String>>>,
    pub zone: Arc<Mutex<Option<Vec<zone>>>>,
    pub tx: Arc<Mutex<Option<Vec<zoneTrans>>>>,
    pub extend: Arc<Mutex<Option<String>>>,
    pub cache_start: Arc<Mutex<Option<i64>>>,
    pub cache_end: Arc<Mutex<Option<i64>>>,
    pub cache_zone: GoPtr<zone>,
}

impl Location {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.zone.clone();
        let __go_clone_2_0 = self.tx.clone();
        let __go_clone_3_0 = { let __guard = self.extend.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.cache_start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.cache_end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = self.cache_zone.clone();
        Self {
            name: __go_clone_0_0,
            zone: __go_clone_1_0,
            tx: __go_clone_2_0,
            extend: __go_clone_3_0,
            cache_start: __go_clone_4_0,
            cache_end: __go_clone_5_0,
            cache_zone: __go_clone_6_0,
        }
    }
}


impl Default for Location {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = GoPtr::nil();
        Self {
            name: __go_default_0_0,
            zone: __go_default_1_0,
            tx: __go_default_2_0,
            extend: __go_default_3_0,
            cache_start: __go_default_4_0,
            cache_end: __go_default_5_0,
            cache_zone: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Location {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A zone represents a single time zone such as CET.
#[derive(Debug, Clone)]
pub struct zone {
    pub name: Arc<Mutex<Option<String>>>,
    pub offset: Arc<Mutex<Option<i32>>>,
    pub is_d_s_t: Arc<Mutex<Option<bool>>>,
}

impl zone {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.is_d_s_t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            offset: __go_clone_1_0,
            is_d_s_t: __go_clone_2_0,
        }
    }
}


impl Default for zone {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            name: __go_default_0_0,
            offset: __go_default_1_0,
            is_d_s_t: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for zone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.offset.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.is_d_s_t.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for zone {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A zoneTrans represents a single time zone transition.
#[derive(Debug, Clone)]
pub struct zoneTrans {
    pub when: Arc<Mutex<Option<i64>>>,
    pub index: Arc<Mutex<Option<u8>>>,
    pub isstd: Arc<Mutex<Option<bool>>>,
    pub isutc: Arc<Mutex<Option<bool>>>,
}

impl zoneTrans {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.when.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.isstd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_1 = { let __guard = self.isutc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            when: __go_clone_0_0,
            index: __go_clone_1_0,
            isstd: __go_clone_2_0,
            isutc: __go_clone_2_1,
        }
    }
}


impl Default for zoneTrans {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_1 = Arc::new(Mutex::new(Some(false)));
        Self {
            when: __go_default_0_0,
            index: __go_default_1_0,
            isstd: __go_default_2_0,
            isutc: __go_default_2_1,
        }
    }
}

impl std::fmt::Display for zoneTrans {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.when.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.index.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.isstd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.isutc.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for zoneTrans {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// ruleKind is the kinds of rules that can be seen in a tzset string.
#[derive(Debug, Clone, Default)]
pub struct ruleKind(pub Arc<Mutex<Option<i32>>>);

impl Display for ruleKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ruleKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for ruleKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ruleKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for ruleKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ruleKind> for i32 {
    fn eq(&self, other: &ruleKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ruleKind> for i32 {
    fn partial_cmp(&self, other: &ruleKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ruleKind {
    type Output = ruleKind;
    fn add(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ruleKind {
    type Output = ruleKind;
    fn add(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ruleKind> for i32 {
    type Output = ruleKind;
    fn add(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ruleKind {
    type Output = ruleKind;
    fn sub(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ruleKind {
    type Output = ruleKind;
    fn sub(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ruleKind> for i32 {
    type Output = ruleKind;
    fn sub(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ruleKind {
    type Output = ruleKind;
    fn mul(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for ruleKind {
    type Output = ruleKind;
    fn mul(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ruleKind> for i32 {
    type Output = ruleKind;
    fn mul(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ruleKind {
    type Output = ruleKind;
    fn div(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for ruleKind {
    type Output = ruleKind;
    fn div(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ruleKind> for i32 {
    type Output = ruleKind;
    fn div(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ruleKind {
    type Output = ruleKind;
    fn neg(self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ruleKind {
    type Output = ruleKind;
    fn rem(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ruleKind {
    type Output = ruleKind;
    fn rem(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ruleKind> for i32 {
    type Output = ruleKind;
    fn rem(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ruleKind {
    type Output = ruleKind;
    fn bitand(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ruleKind {
    type Output = ruleKind;
    fn bitand(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ruleKind> for i32 {
    type Output = ruleKind;
    fn bitand(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ruleKind {
    type Output = ruleKind;
    fn bitor(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ruleKind {
    type Output = ruleKind;
    fn bitor(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ruleKind> for i32 {
    type Output = ruleKind;
    fn bitor(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ruleKind {
    type Output = ruleKind;
    fn bitxor(self, other: Self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ruleKind {
    type Output = ruleKind;
    fn bitxor(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ruleKind> for i32 {
    type Output = ruleKind;
    fn bitxor(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ruleKind {
    type Output = ruleKind;
    fn not(self) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: i8) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: i16) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: i64) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: u32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: u8) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: u16) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: u64) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ruleKind {
    type Output = ruleKind;
    fn shl(self, other: usize) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: ruleKind) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: i32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: i8) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: i16) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: i64) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: u32) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: u8) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: u16) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: u64) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ruleKind {
    type Output = ruleKind;
    fn shr(self, other: usize) -> ruleKind {
        ruleKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ruleKind {}

impl Ord for ruleKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// rule is a rule read from a tzset string.
#[derive(Debug, Clone)]
pub struct rule {
    pub kind: Arc<Mutex<Option<ruleKind>>>,
    pub day: Arc<Mutex<Option<i32>>>,
    pub week: Arc<Mutex<Option<i32>>>,
    pub mon: Arc<Mutex<Option<i32>>>,
    pub time: Arc<Mutex<Option<i32>>>,
}

impl rule {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.day.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.week.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.mon.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            kind: __go_clone_0_0,
            day: __go_clone_1_0,
            week: __go_clone_2_0,
            mon: __go_clone_3_0,
            time: __go_clone_4_0,
        }
    }
}


impl Default for rule {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            kind: __go_default_0_0,
            day: __go_default_1_0,
            week: __go_default_2_0,
            mon: __go_default_3_0,
            time: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.kind.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.day.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.week.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.mon.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.time.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for rule {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub static UTC: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Location>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static utcLoc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Location>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Local: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Location>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static localLoc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Location>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static localOnce: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::once::Once>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static unnamedFixedZones: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<Location>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static unnamedFixedZonesOnce: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::once::Once>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errLocation: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zoneinfo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<String>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zoneinfoOnce: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::once::Once>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *UTC.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *utcLoc.lock().unwrap() = Some(Default::default());
    *Local.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *localLoc.lock().unwrap() = Some(Default::default());
    *localOnce.lock().unwrap() = Some(Default::default());
    *unnamedFixedZones.lock().unwrap() = Some(vec![]);
    *unnamedFixedZonesOnce.lock().unwrap() = Some(Default::default());
    *errLocation.lock().unwrap() = None;
    *zoneinfo.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *zoneinfoOnce.lock().unwrap() = Some(Default::default());
    *utcLoc.lock().unwrap() = Some(Location { name: Arc::new(Mutex::new(Some("UTC".to_string()))), ..Default::default() });
    *UTC.lock().unwrap() = Some(utcLoc.clone());
    *Local.lock().unwrap() = Some(localLoc.clone());
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("time: invalid location name".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errLocation.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *UTC.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *utcLoc.lock().unwrap() = Some(Default::default());
    *Local.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *localLoc.lock().unwrap() = Some(Default::default());
    *localOnce.lock().unwrap() = Some(Default::default());
    *unnamedFixedZones.lock().unwrap() = Some(vec![]);
    *unnamedFixedZonesOnce.lock().unwrap() = Some(Default::default());
    *errLocation.lock().unwrap() = None;
    *zoneinfo.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *zoneinfoOnce.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_11() {
    *utcLoc.lock().unwrap() = Some(Location { name: Arc::new(Mutex::new(Some("UTC".to_string()))), ..Default::default() });
}


pub(crate) fn __go_init_order_12() {
    *UTC.lock().unwrap() = Some(utcLoc.clone());
}


pub(crate) fn __go_init_order_13() {
    *Local.lock().unwrap() = Some(localLoc.clone());
}


pub(crate) fn __go_init_order_14() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("time: invalid location name".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errLocation.lock().unwrap() = new_val; }
}


impl Location {
    pub fn get(&self) -> Arc<Mutex<Option<Location>>> {
        if false {
        return utcLoc.clone();
    }
        if { let __peer = localLoc.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } {
        { let __once = (*localOnce.lock().unwrap().as_ref().unwrap()).clone(); __once.r#do(Arc::new(Mutex::new(Some(Box::new(move || { init_local() }) as Box<dyn FnMut() -> () + Send + Sync>)))) };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// String returns a descriptive name for the time zone information,
    /// corresponding to the name argument to [LoadLocation] or [FixedZone].
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = (*self.get().lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    /// lookup returns information about the time zone in use at an
    /// instant in time expressed as seconds since January 1, 1970 00:00:00 UTC.
    ///
    /// The returned information gives the name of the zone (such as "CET"),
    /// the start and end times bracketing sec when that zone is in effect,
    /// the offset in seconds east of UTC (such as -5*60*60), and whether
    /// the daylight savings is being observed at that time.
    pub fn lookup(&mut self, sec: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<String>>>, i32, i64, i64, bool) {
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut offset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut start: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut end: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut isDST: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut __self = self.clone();
        { let new_val = __self.get(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        if { let __tmp_x = (({ let __len_target = { let __field = __self.zone.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = "UTC".to_string(); *name.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *offset.lock().unwrap() = Some(new_val); };
        { let new_val = ALPHA as i64; *start.lock().unwrap() = Some(new_val); };
        { let new_val = OMEGA as i64; *end.lock().unwrap() = Some(new_val); };
        { let new_val = false; *isDST.lock().unwrap() = Some(new_val); };
        return (name.clone(), (*offset.lock().unwrap().as_ref().unwrap()), (*start.lock().unwrap().as_ref().unwrap()), (*end.lock().unwrap().as_ref().unwrap()), (*isDST.lock().unwrap().as_ref().unwrap()));
    }
        {
        let mut zone: GoPtr<zone> = __self.cache_zone.clone();;
        if !zone.is_nil() && { let __tmp_x = (*__self.cache_start.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.cache_end.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
            { let new_val = { let __selector_holder = { let __ptr_value = zone.with_mut(|__ptr_value| __ptr_value.name.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = { let __ptr_value = zone.with_mut(|__ptr_value| __ptr_value.offset.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *offset.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = __self.cache_start.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *start.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = __self.cache_end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *end.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = { let __ptr_value = zone.with_mut(|__ptr_value| __ptr_value.is_d_s_t.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *isDST.lock().unwrap() = Some(new_val); };;
            return (name.clone(), (*offset.lock().unwrap().as_ref().unwrap()), (*start.lock().unwrap().as_ref().unwrap()), (*end.lock().unwrap().as_ref().unwrap()), (*isDST.lock().unwrap().as_ref().unwrap()));;
        }
    }
        if {
            let __go_cond_0 = { let __tmp_x = (({ let __len_target = { let __field = __self.tx.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v };
                    let __tmp_y = (*{ let __seq = { let __seq_holder = __self.tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.when.lock().unwrap().as_ref().unwrap());
                    __tmp_x < __tmp_y
                };
                __go_cond_1
            }
        } {
        let mut zone: Option<GoSliceElemPtr<zone>> = Some(GoSliceElemPtr::new(__self.zone.clone(), (__self.lookup_first_zone()) as usize));
        { let new_val = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *offset.lock().unwrap() = Some(new_val); };
        { let new_val = ALPHA as i64; *start.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (({ let __len_target = { let __field = __self.tx.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = __self.tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *end.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = OMEGA as i64; *end.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).is_d_s_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *isDST.lock().unwrap() = Some(new_val); };
        return (name.clone(), (*offset.lock().unwrap().as_ref().unwrap()), (*start.lock().unwrap().as_ref().unwrap()), (*end.lock().unwrap().as_ref().unwrap()), (*isDST.lock().unwrap().as_ref().unwrap()));
    }
                // Binary search for entry with largest time <= sec.
                // Not using sort.Search to avoid dependencies.
        let mut tx = __self.tx.clone();
        { let new_val = OMEGA as i64; *end.lock().unwrap() = Some(new_val); };
        let mut lo = Arc::new(Mutex::new(Some(0)));
        let mut hi = Arc::new(Mutex::new(Some((*tx.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        while { let __tmp_x = { let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 1; __tmp_x > __tmp_y } {
        let mut m = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));
        let mut lim = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*lim.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = lim.lock().unwrap().as_ref().unwrap().clone(); *end.lock().unwrap() = Some(new_val); };
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *hi.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *lo.lock().unwrap() = Some(new_val); };
    }
    }
        let mut zone: Option<GoSliceElemPtr<zone>> = Some(GoSliceElemPtr::new(__self.zone.clone(), ((*{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.index.lock().unwrap().as_ref().unwrap())) as usize));
        { let new_val = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *offset.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *start.lock().unwrap() = Some(new_val); };
                // end = maintained during the search
        { let new_val = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).is_d_s_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *isDST.lock().unwrap() = Some(new_val); };
                // If we're at the end of the known zone transitions,
                // try the extend string.
        if { let __tmp_x = ({ let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = ((*tx.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } && { let __tmp_x = (*__self.extend.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        {
        let (mut ename, mut eoffset, mut estart, mut eend, mut eisDST, mut ok) = tzset(Arc::new(Mutex::new(Some({ let __selector_holder = __self.extend.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if ok {
            return ({ let __owned = ename.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, eoffset, estart, eend, eisDST);;
        }
    }
    }
        return (name.clone(), (*offset.lock().unwrap().as_ref().unwrap()), (*start.lock().unwrap().as_ref().unwrap()), (*end.lock().unwrap().as_ref().unwrap()), (*isDST.lock().unwrap().as_ref().unwrap()));
    }

    /// lookupFirstZone returns the index of the time zone to use for times
    /// before the first transition time, or when there are no transition
    /// times.
    ///
    /// The reference implementation in localtime.c from
    /// https://www.iana.org/time-zones/repository/releases/tzcode2013g.tar.gz
    /// implements the following algorithm for these cases:
    ///  1. If the first zone is unused by the transitions, use it.
    ///  2. Otherwise, if there are transition times, and the first
    ///     transition is to a zone in daylight time, find the first
    ///     non-daylight-time zone before and closest to the first transition
    ///     zone.
    ///  3. Otherwise, use the first zone that is not daylight time, if
    ///     there is one.
    ///  4. Otherwise, use the first zone.
    pub fn lookup_first_zone(&self) -> i32 {
                // Case 1.
        if !self.first_zone_used() {
        return 0;
    }
                // Case 2.
        if {
            let __go_cond_0 = { let __tmp_x = (({ let __len_target = { let __field = self.tx.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = (*{ let __seq = { let __seq_holder = self.zone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*{ let __seq = { let __seq_holder = self.tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.index.lock().unwrap().as_ref().unwrap())) as usize].clone() }.is_d_s_t.lock().unwrap().as_ref().unwrap());
                __go_cond_1
            } else {
                false
            }
        } {
        let mut zi = Arc::new(Mutex::new(Some({
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = self.tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.index.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = 1;
            __tmp_x - __tmp_y
        })));
    while { let __tmp_x = { let __v = (*zi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        if !(*{ let __seq = { let __seq_holder = self.zone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*zi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.is_d_s_t.lock().unwrap().as_ref().unwrap()) {
        return { let __v = (*zi.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = zi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
                // Case 3.
        for zi in 0..(({ let __range_holder = self.zone.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if !(*{ let __seq = { let __seq_holder = self.zone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(zi) as usize].clone() }.is_d_s_t.lock().unwrap().as_ref().unwrap()) {
        return zi as i32;
    }
    }
                // Case 4.
        0
    }

    /// firstZoneUsed reports whether the first zone is used by some
    /// transition.
    pub fn first_zone_used(&self) -> bool {
        { let __range_holder = self.tx.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tx in __range_values.iter() {
        if { let __tmp_x = (*tx.index.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return true;
    }
    } }
        false
    }

    /// lookupName returns information about the time zone with
    /// the given name (such as "EST") at the given pseudo-Unix time
    /// (what the given time of day would be in UTC).
    pub fn lookup_name(&mut self, name: Arc<Mutex<Option<String>>>, unix: Arc<Mutex<Option<i64>>>) -> (i32, bool) {
    let mut offset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut __self = self.clone();
        { let new_val = __self.get(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
                // First try for a zone with the right name that was actually
                // in effect at the given time. (In Sydney, Australia, both standard
                // and daylight-savings time are abbreviated "EST". Using the
                // offset helps us pick the right one for the given time.
                // It's not perfect: during the backward transition we might pick
                // either one.)
        for i in 0..(({ let __range_holder = __self.zone.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut zone: Option<GoSliceElemPtr<zone>> = Some(GoSliceElemPtr::new(__self.zone.clone(), (i) as usize));
        if { let __tmp_x = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        let (mut nam, mut offset, _, _, _) = __self.lookup(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*unix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))));
        if { let __tmp_x = (*nam.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y } {
        return (offset, true);
    }
    }
    }
                // Otherwise fall back to an ordinary name match.
        for i in 0..(({ let __range_holder = __self.zone.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut zone: Option<GoSliceElemPtr<zone>> = Some(GoSliceElemPtr::new(__self.zone.clone(), (i) as usize));
        if { let __tmp_x = { let __selector_holder = (*zone.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return ((*(*zone.as_ref().unwrap().borrow().as_ref().unwrap()).offset.lock().unwrap().as_ref().unwrap()), true);
    }
    }
                // Otherwise, give up.
        return ((*offset.lock().unwrap().as_ref().unwrap()), (*ok.lock().unwrap().as_ref().unwrap()));
    }
}

/// FixedZone returns a [Location] that always uses
/// the given zone name and offset (seconds east of UTC).
pub fn fixed_zone(name: Arc<Mutex<Option<String>>>, offset: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Location>>> {
        // Most calls to FixedZone have an unnamed zone with an offset by the hour.
        // Optimize for that case by returning the same *Location for a given hour.
    const hoursBeforeUTC: i32 = 12;

    const hoursAfterUTC: i32 = 14;

    let mut hour = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x / __tmp_y }; let __tmp_y = 60; __tmp_x / __tmp_y })));
    if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = -12; let __tmp_y = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 14; __tmp_x <= __tmp_y } && { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x * __tmp_y }; let __tmp_y = 60; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let __once = (*unnamedFixedZonesOnce.lock().unwrap().as_ref().unwrap()).clone(); __once.r#do(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = { let __collection_holder = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __tmp_x = { let __tmp_x = hoursBeforeUTC; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = hoursAfterUTC; __tmp_x + __tmp_y }) as usize]))).clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *unnamedFixedZones.lock().unwrap() = new_val; };
        let mut hr = Arc::new(Mutex::new(Some(-(hoursBeforeUTC))));
    while { let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 14; __tmp_x <= __tmp_y } {
        (*unnamedFixedZones.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x + __tmp_y }) as usize] = fixed_zone_1(Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x * __tmp_y }; let __tmp_y = 60; __tmp_x * __tmp_y }))));
        { let mut guard = hr.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>)))) };
        return { let __seq = { let __seq_holder = unnamedFixedZones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x + __tmp_y }) as usize].clone() };
    }
    fixed_zone_1(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn fixed_zone_1(name: Arc<Mutex<Option<String>>>, offset: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Location>>> {
    let mut l = Arc::new(Mutex::new(Some(Location {
        name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        zone: Arc::new(Mutex::new(Some(vec![zone { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), offset: Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), is_d_s_t: Arc::new(Mutex::new(Some(false))), ..Default::default() }]))),
        tx: Arc::new(Mutex::new(Some(vec![zoneTrans { when: Arc::new(Mutex::new(Some(ALPHA as i64))), index: Arc::new(Mutex::new(Some(0 as u8))), isstd: Arc::new(Mutex::new(Some(false))), isutc: Arc::new(Mutex::new(Some(false))), ..Default::default() }]))),
        cache_start: Arc::new(Mutex::new(Some(ALPHA as i64))),
        cache_end: Arc::new(Mutex::new(Some(OMEGA as i64))),
        ..Default::default()
    })));
    { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new((*l.lock().unwrap().as_ref().unwrap()).zone.clone(), (0) as usize)); (*l.lock().unwrap().as_mut().unwrap()).cache_zone = new_val; };
    return l.clone();
}

/// tzset takes a timezone string like the one found in the TZ environment
/// variable, the time of the last time zone transition expressed as seconds
/// since January 1, 1970 00:00:00 UTC, and a time expressed the same way.
/// We call this a tzset string since in C the function tzset reads TZ.
/// The return values are as for lookup, plus ok which reports whether the
/// parse succeeded.
pub fn tzset(mut s: Arc<Mutex<Option<String>>>, lastTxSec: Arc<Mutex<Option<i64>>>, sec: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<String>>>, i32, i64, i64, bool, bool) {
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut offset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut start: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut end: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut isDST: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut stdName: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut dstName: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut stdOffset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut dstOffset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_name(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *stdName.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_offset(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *stdOffset.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    }
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0, 0, 0, false, false);
    }

        // The numbers in the tzset string are added to local time to get UTC,
        // but our offsets are added to UTC to get local time,
        // so we negate the number we see here.
    { let new_val = -((*stdOffset.lock().unwrap().as_ref().unwrap())); *stdOffset.lock().unwrap() = Some(new_val); };

    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (',' as i32) as u8; __tmp_x == __tmp_y } {
                // No daylight savings time.
        return ({ let __owned = stdName.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*stdOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*lastTxSec.lock().unwrap().as_ref().unwrap()).clone(); __v }, OMEGA as i64, false, true);
    }

        // No daylight savings time.
    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_name(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *dstName.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (',' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*stdOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3600; __tmp_x + __tmp_y }; *dstOffset.lock().unwrap() = Some(new_val); };
    } else {
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_offset(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *dstOffset.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        { let new_val = -((*dstOffset.lock().unwrap().as_ref().unwrap())); *dstOffset.lock().unwrap() = Some(new_val); };
    }
    }
        // as with stdOffset, above
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0, 0, 0, false, false);
    }

    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Default DST rules per tzcode.
        { let new_val = ",M3.2.0,M11.1.0".to_string(); *s.lock().unwrap() = Some(new_val); };
    }

        // Default DST rules per tzcode.
        // The TZ definition does not mention ';' here but tzcode accepts it.
    if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (',' as i32) as u8; __tmp_x != __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (';' as i32) as u8; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0, 0, 0, false, false);
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };

    let mut startRule: Arc<Mutex<Option<rule>>> = Arc::new(Mutex::new(Some(Default::default())));let mut endRule: Arc<Mutex<Option<rule>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_rule(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *startRule.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (',' as i32) as u8; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0, 0, 0, false, false);
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_rule(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *endRule.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0, 0, 0, false, false);
    }

        // Compute start of year in seconds since Unix epoch,
        // and seconds since then to get to sec.
    let (mut year, mut yday) = crate::r#mod::absDays::year_yday(&(*crate::r#mod::absSeconds::days(&(crate::r#mod::absSeconds(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = UNIX_TO_INTERNAL as i64; __tmp_x + __tmp_y }; let __tmp_y = INTERNAL_TO_ABSOLUTE as i64; __tmp_x + __tmp_y } as u64)))))).lock().unwrap().as_ref().unwrap()));
    let mut ysec = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = yday; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 86400; __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SECONDS_PER_DAY as i64; __tmp_x % __tmp_y }; __tmp_x + __tmp_y })));
    let mut ystart = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ysec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));

    let mut startSec = Arc::new(Mutex::new(Some(tzrule_time(Arc::new(Mutex::new(Some(year))), Arc::new(Mutex::new(Some({ let __arg_holder = startRule.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = stdOffset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as i64)));
    let mut endSec = Arc::new(Mutex::new(Some(tzrule_time(Arc::new(Mutex::new(Some(year))), Arc::new(Mutex::new(Some({ let __arg_holder = endRule.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dstOffset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as i64)));
    let (mut dstIsDST, mut stdIsDST) = (Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(false))));

        // Note: this is a flipping of "DST" and "STD" while retaining the labels
        // This happens in southern hemispheres. The labelling here thus is a little
        // inconsistent with the goal.
    if { let __tmp_x = { let __v = (*endSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*startSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __tmp_0 = (*endSec.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*startSec.lock().unwrap().as_ref().unwrap()).clone();
            *startSec.lock().unwrap() = Some(__tmp_0);
            *endSec.lock().unwrap() = Some(__tmp_1);
        };
        {
            let __tmp_0 = (*dstName.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*stdName.lock().unwrap().as_ref().unwrap()).clone();
            *stdName.lock().unwrap() = Some(__tmp_0);
            *dstName.lock().unwrap() = Some(__tmp_1);
        };
        {
            let __tmp_0 = (*dstOffset.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*stdOffset.lock().unwrap().as_ref().unwrap()).clone();
            *stdOffset.lock().unwrap() = Some(__tmp_0);
            *dstOffset.lock().unwrap() = Some(__tmp_1);
        };
        {
            let __tmp_0 = (*dstIsDST.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*stdIsDST.lock().unwrap().as_ref().unwrap()).clone();
            *stdIsDST.lock().unwrap() = Some(__tmp_0);
            *dstIsDST.lock().unwrap() = Some(__tmp_1);
        };
    }

        // The start and end values that we return are accurate
        // close to a daylight savings transition, but are otherwise
        // just the start and end of the year. That suffices for
        // the only caller that cares, which is Date.
    if { let __tmp_x = { let __v = (*ysec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*startSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return ({ let __owned = stdName.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*stdOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*ystart.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*startSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ystart.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }, { let __v = (*stdIsDST.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    } else if { let __tmp_x = { let __v = (*ysec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*endSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        return ({ let __owned = stdName.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*stdOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*endSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ystart.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }, { let __tmp_x = { let __v = (*ystart.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((365 as i64) * (SECONDS_PER_DAY as i64)) as i64; __tmp_x + __tmp_y }, { let __v = (*stdIsDST.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    } else {
        return ({ let __owned = dstName.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*dstOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*startSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ystart.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }, { let __tmp_x = { let __v = (*endSec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ystart.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }, { let __v = (*dstIsDST.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
}

/// tzsetName returns the timezone name at the start of the tzset string s,
/// and the remainder of s, and reports whether the parsing is OK.
pub fn tzset_name(s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>, bool) {
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('<' as i32) as u8; __tmp_x != __tmp_y } {
        for (i, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        { let _switch_val = r;
    if _switch_val == ('0') || _switch_val == ('1') || _switch_val == ('2') || _switch_val == ('3') || _switch_val == ('4') || _switch_val == ('5') || _switch_val == ('6') || _switch_val == ('7') || _switch_val == ('8') || _switch_val == ('9') || _switch_val == (',') || _switch_val == ('-') || _switch_val == ('+') {
            if { let __tmp_x = i as i32; let __tmp_y = 3; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
            return (Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() }))), true);
        }
    }
    }
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        return ({ let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some("".to_string()))), true);
    } else {
        for (i, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = r; let __tmp_y = '>'; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = (i) as usize; __s[__low..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))), true);
    }
    }
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
}

/// tzsetOffset returns the timezone offset at the start of the tzset string s,
/// and the remainder of s, and reports whether the parsing is OK.
/// The timezone offset is returned as a number of seconds.
pub fn tzset_offset(mut s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<String>>>, bool) {
    let mut offset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut rest: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    let mut neg = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    } else if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        { let new_val = true; *neg.lock().unwrap() = Some(new_val); };
    }

        // The tzdata code permits values up to 24 * 7 here,
        // although POSIX does not.
    let mut hours: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
        Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some(168)))
    ); *hours.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hours.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3600; __tmp_x * __tmp_y })));
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (':' as i32) as u8; __tmp_x != __tmp_y } {
        if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = -((*off.lock().unwrap().as_ref().unwrap())); *off.lock().unwrap() = Some(new_val); };
    }
        return ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
    }

    let mut mins: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
        Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some(59)))
    ); *mins.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    { let __rhs = { let __tmp_x = { let __v = (*mins.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x * __tmp_y }; let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (':' as i32) as u8; __tmp_x != __tmp_y } {
        if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = -((*off.lock().unwrap().as_ref().unwrap())); *off.lock().unwrap() = Some(new_val); };
    }
        return ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
    }

    let mut secs: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
        Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some(59)))
    ); *secs.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    { let __rhs = (*secs.lock().unwrap().as_ref().unwrap()); let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

    if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = -((*off.lock().unwrap().as_ref().unwrap())); *off.lock().unwrap() = Some(new_val); };
    }
    return ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
}

/// tzsetRule parses a rule from a tzset string.
/// It returns the rule, and the remainder of the string, and reports success.
pub fn tzset_rule(mut s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<rule>>>, Arc<Mutex<Option<String>>>, bool) {
    let mut r: Arc<Mutex<Option<rule>>> = Arc::new(Mutex::new(Some(Default::default())));
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    let mut ok = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('J' as i32) as u8; __tmp_x == __tmp_y } {
        let mut jday: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
            Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))),
            Arc::new(Mutex::new(Some(1))),
            Arc::new(Mutex::new(Some(365)))
        ); *jday.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        { let new_val = ruleKind(Arc::new(Mutex::new(Some(RULE_JULIAN as i32)))); *(*r.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
        { let new_val = jday.lock().unwrap().as_ref().unwrap().clone(); *(*r.lock().unwrap().as_ref().unwrap()).day.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('M' as i32) as u8; __tmp_x == __tmp_y } {
        let mut mon: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
            Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))),
            Arc::new(Mutex::new(Some(1))),
            Arc::new(Mutex::new(Some(12)))
        ); *mon.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        let mut week: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
            Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))),
            Arc::new(Mutex::new(Some(1))),
            Arc::new(Mutex::new(Some(5)))
        ); *week.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        let mut day: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(
            Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))),
            Arc::new(Mutex::new(Some(0))),
            Arc::new(Mutex::new(Some(6)))
        ); *day.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        { let new_val = ruleKind(Arc::new(Mutex::new(Some(RULE_MONTH_WEEK_DAY as i32)))); *(*r.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
        { let new_val = day.lock().unwrap().as_ref().unwrap().clone(); *(*r.lock().unwrap().as_ref().unwrap()).day.lock().unwrap() = Some(new_val); };
        { let new_val = week.lock().unwrap().as_ref().unwrap().clone(); *(*r.lock().unwrap().as_ref().unwrap()).week.lock().unwrap() = Some(new_val); };
        { let new_val = mon.lock().unwrap().as_ref().unwrap().clone(); *(*r.lock().unwrap().as_ref().unwrap()).mon.lock().unwrap() = Some(new_val); };
    } else {
        let mut day: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = tzset_num(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(365)))); *day.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        { let new_val = ruleKind(Arc::new(Mutex::new(Some(RULE_D_O_Y as i32)))); *(*r.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
        { let new_val = day.lock().unwrap().as_ref().unwrap().clone(); *(*r.lock().unwrap().as_ref().unwrap()).day.lock().unwrap() = Some(new_val); };
    }

    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('/' as i32) as u8; __tmp_x != __tmp_y } {
        { let new_val = 7200; *(*r.lock().unwrap().as_ref().unwrap()).time.lock().unwrap() = Some(new_val); };
        return ({ let __owned = r.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
    }

        // 2am is the default
    let (mut offset, __tmp_1, __tmp_2) = tzset_offset(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2);;
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some(rule { kind: Arc::new(Mutex::new(Some(ruleKind(Arc::new(Mutex::new(Some(0))))))), day: Arc::new(Mutex::new(Some(0))), week: Arc::new(Mutex::new(Some(0))), mon: Arc::new(Mutex::new(Some(0))), time: Arc::new(Mutex::new(Some(0))) }))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    { let new_val = offset; *(*r.lock().unwrap().as_ref().unwrap()).time.lock().unwrap() = Some(new_val); };

    return ({ let __owned = r.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
}

/// tzsetNum parses a number from a tzset string.
/// It returns the number, and the remainder of the string, and reports success.
/// The number must be between min and max.
pub fn tzset_num(s: Arc<Mutex<Option<String>>>, min: Arc<Mutex<Option<i32>>>, max: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<String>>>, bool) {
    let mut num: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut rest: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    { let new_val = 0; *num.lock().unwrap() = Some(new_val); };
    for (i, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = r; let __tmp_y = '0'; __tmp_x < __tmp_y } || { let __tmp_x = r; let __tmp_y = '9'; __tmp_x > __tmp_y } {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
        return ({ let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() }))), true);
    }
        { let __rhs = 10; let mut guard = num.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some((r as i32) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ('0' as i32); __tmp_x - __tmp_y }; let mut guard = num.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    }
    if { let __tmp_x = { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (0, Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    return ({ let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some("".to_string()))), true);
}

/// tzruleTime takes a year, a rule, and a timezone offset,
/// and returns the number of seconds since the start of the year
/// that the rule takes effect.
pub fn tzrule_time(year: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<rule>>>, off: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut s: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let _switch_val = { let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (ruleKind(Arc::new(Mutex::new(Some(RULE_JULIAN as i32))))) {
            { let new_val = { let __tmp_x = ({ let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).day.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 86400; __tmp_x * __tmp_y }; *s.lock().unwrap() = Some(new_val); };
            if is_leap(Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).day.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 60; __tmp_x >= __tmp_y } {
        { let __rhs = 86400; let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        } else if _switch_val == (ruleKind(Arc::new(Mutex::new(Some(RULE_D_O_Y as i32))))) {
            { let new_val = { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).day.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 86400; __tmp_x * __tmp_y }; *s.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (ruleKind(Arc::new(Mutex::new(Some(RULE_MONTH_WEEK_DAY as i32))))) {
                        // Zeller's Congruence.
            let mut m1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).mon.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 9; __tmp_x + __tmp_y }); let __tmp_y = 12; __tmp_x % __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
            let mut yy0 = { let __owned = year.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
            if { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).mon.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x <= __tmp_y } {
        { let mut guard = yy0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
            let mut yy1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*yy0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x / __tmp_y })));
            let mut yy2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*yy0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x % __tmp_y })));
            let __go_binary_0 = 26;
let __go_binary_1 = (*m1.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = __go_binary_0 * __go_binary_1;
let __go_binary_3 = 2;
let __go_binary_4 = __go_binary_2 - __go_binary_3;
let __go_binary_5 = 10;
let __go_binary_6 = __go_binary_4 / __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 + __go_binary_7;
let __go_binary_9 = (*yy2.lock().unwrap().as_ref().unwrap());
let __go_binary_10 = __go_binary_8 + __go_binary_9;
let __go_binary_11 = (*yy2.lock().unwrap().as_ref().unwrap());
let __go_binary_12 = 4;
let __go_binary_13 = __go_binary_11 / __go_binary_12;
let __go_binary_14 = __go_binary_10 + __go_binary_13;
let __go_binary_15 = (*yy1.lock().unwrap().as_ref().unwrap());
let __go_binary_16 = 4;
let __go_binary_17 = __go_binary_15 / __go_binary_16;
let __go_binary_18 = __go_binary_14 + __go_binary_17;
let __go_binary_19 = 2;
let __go_binary_20 = (*yy1.lock().unwrap().as_ref().unwrap());
let __go_binary_21 = __go_binary_19 * __go_binary_20;
let __go_binary_22 = __go_binary_18 - __go_binary_21;
let __go_binary_23 = 7;
let __go_binary_24 = __go_binary_22 % __go_binary_23;
let mut dow = Arc::new(Mutex::new(Some(__go_binary_24)));
            if { let __tmp_x = { let __v = (*dow.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __rhs = 7; let mut guard = dow.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                        // Now dow is the day-of-week of the first day of r.mon.
                        // Get the day-of-month of the first "dow" day.
            let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).day.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dow.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
            if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __rhs = 7; let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
            let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).week.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        if {
            let __tmp_x = { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y };
            let __tmp_y = days_in(Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).mon.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            __tmp_x >= __tmp_y
        } {
        break
    }
        { let __rhs = 7; let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let __rhs = (*Arc::new(Mutex::new(Some(days_before(Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).mon.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32)))))))) as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
            if is_leap(Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).mon.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x > __tmp_y } {
        { let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let new_val = { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 86400; __tmp_x * __tmp_y }; *s.lock().unwrap() = Some(new_val); };
        }
    }

        // Zeller's Congruence.
        // Now dow is the day-of-week of the first day of r.mon.
        // Get the day-of-month of the first "dow" day.
    return { let __tmp_x = { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).time.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Location {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for zone {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for zoneTrans {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for rule {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
