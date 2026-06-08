use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoByteSequence, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{format::{R_F_C3339_NANO, longDayNames, longMonthNames}, format_rfc3339::{parse_strict_r_f_c3339}, zoneinfo::{ALPHA, Local, Location, OMEGA, UTC, fixed_zone, localLoc, utcLoc, zone}};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const HAS_MONOTONIC: u64 = 1 << 63;
pub(crate) const MAX_WALL: i64 = ((WALL_TO_INTERNAL as i64) + (((1 as i64) << (33 as i64)) - (1 as i64)));
pub(crate) const MIN_WALL: i64 = WALL_TO_INTERNAL;
pub(crate) const NSEC_MASK: i32 = (((1 as i32) << (30 as i32)) - (1 as i32));
pub(crate) const NSEC_SHIFT: i32 = 30;


pub const JANUARY: i32 = 1 + 0;
pub const FEBRUARY: i32 = 1 + 1;
pub const MARCH: i32 = 1 + 2;
pub const APRIL: i32 = 1 + 3;
pub const MAY: i32 = 1 + 4;
pub const JUNE: i32 = 1 + 5;
pub const JULY: i32 = 1 + 6;
pub const AUGUST: i32 = 1 + 7;
pub const SEPTEMBER: i32 = 1 + 8;
pub const OCTOBER: i32 = 1 + 9;
pub const NOVEMBER: i32 = 1 + 10;
pub const DECEMBER: i32 = 1 + 11;


pub const SUNDAY: i32 = 0;
pub const MONDAY: i32 = 1;
pub const TUESDAY: i32 = 2;
pub const WEDNESDAY: i32 = 3;
pub const THURSDAY: i32 = 4;
pub const FRIDAY: i32 = 5;
pub const SATURDAY: i32 = 6;


pub(crate) const SECONDS_PER_MINUTE: i32 = 60;
pub(crate) const SECONDS_PER_HOUR: i32 = 60 * SECONDS_PER_MINUTE;
pub(crate) const SECONDS_PER_DAY: i32 = 24 * SECONDS_PER_HOUR;
pub(crate) const SECONDS_PER_WEEK: i32 = 7 * SECONDS_PER_DAY;
pub(crate) const DAYS_PER400_YEARS: i32 = 365 * 400 + 97;
pub(crate) const MARCH_THRU_DECEMBER: i32 = 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 + 31;
pub(crate) const ABSOLUTE_YEARS: i64 = 292277022400;
pub(crate) const INTERNAL_YEAR: i32 = 1;
pub(crate) const ABSOLUTE_TO_INTERNAL: i64 = ((-(1.06751990353932e+14 + 306.0) as i64) * (SECONDS_PER_DAY as i64));
pub(crate) const INTERNAL_TO_ABSOLUTE: i64 = -ABSOLUTE_TO_INTERNAL;
pub(crate) const UNIX_TO_INTERNAL: i64 = ((((((1969 as i64) * (365 as i64)) + ((1969 as i64) / (4 as i64))) - ((1969 as i64) / (100 as i64))) + ((1969 as i64) / (400 as i64))) * (SECONDS_PER_DAY as i64));
pub(crate) const INTERNAL_TO_UNIX: i64 = -UNIX_TO_INTERNAL;
pub(crate) const ABSOLUTE_TO_UNIX: i64 = ABSOLUTE_TO_INTERNAL as i64 + INTERNAL_TO_UNIX as i64;
pub(crate) const UNIX_TO_ABSOLUTE: i64 = UNIX_TO_INTERNAL as i64 + INTERNAL_TO_ABSOLUTE as i64;
pub(crate) const WALL_TO_INTERNAL: i64 = ((((((1884 as i64) * (365 as i64)) + ((1884 as i64) / (4 as i64))) - ((1884 as i64) / (100 as i64))) + ((1884 as i64) / (400 as i64))) * (SECONDS_PER_DAY as i64));


pub(crate) const MIN_DURATION: i64 = ((-1 as i64) << (63 as i64));
pub(crate) const MAX_DURATION: i64 = (((1 as u64) << (63 as u64)) - (1 as u64));


pub const NANOSECOND: i64 = 1;
pub const MICROSECOND: i64 = 1000 as i64 * NANOSECOND as i64;
pub const MILLISECOND: i64 = 1000 as i64 * MICROSECOND as i64;
pub const SECOND: i64 = 1000 as i64 * MILLISECOND as i64;
pub const MINUTE: i64 = 60 as i64 * SECOND as i64;
pub const HOUR: i64 = 60 as i64 * MINUTE as i64;


pub(crate) const TIME_BINARY_VERSION_V1: u8 = 0 + 1;
pub(crate) const TIME_BINARY_VERSION_V2: u8 = 1 + 1;


/// A Time represents an instant in time with nanosecond precision.
///
/// Programs using times should typically store and pass them as values,
/// not pointers. That is, time variables and struct fields should be of
/// type [time.Time], not *time.Time.
///
/// A Time value can be used by multiple goroutines simultaneously except
/// that the methods [Time.GobDecode], [Time.UnmarshalBinary], [Time.UnmarshalJSON] and
/// [Time.UnmarshalText] are not concurrency-safe.
///
/// Time instants can be compared using the [Time.Before], [Time.After], and [Time.Equal] methods.
/// The [Time.Sub] method subtracts two instants, producing a [Duration].
/// The [Time.Add] method adds a Time and a Duration, producing a Time.
///
/// The zero value of type Time is January 1, year 1, 00:00:00.000000000 UTC.
/// As this time is unlikely to come up in practice, the [Time.IsZero] method gives
/// a simple way of detecting a time that has not been initialized explicitly.
///
/// Each time has an associated [Location]. The methods [Time.Local], [Time.UTC], and Time.In return a
/// Time with a specific Location. Changing the Location of a Time value with
/// these methods does not change the actual instant it represents, only the time
/// zone in which to interpret it.
///
/// Representations of a Time value saved by the [Time.GobEncode], [Time.MarshalBinary], [Time.AppendBinary],
/// [Time.MarshalJSON], [Time.MarshalText] and [Time.AppendText] methods store the [Time.Location]'s offset,
/// but not the location name. They therefore lose information about Daylight Saving Time.
///
/// In addition to the required “wall clock” reading, a Time may contain an optional
/// reading of the current process's monotonic clock, to provide additional precision
/// for comparison or subtraction.
/// See the “Monotonic Clocks” section in the package documentation for details.
///
/// Note that the Go == operator compares not just the time instant but also the
/// Location and the monotonic clock reading. Therefore, Time values should not
/// be used as map or database keys without first guaranteeing that the
/// identical Location has been set for all values, which can be achieved
/// through use of the UTC or Local method, and that the monotonic clock reading
/// has been stripped by setting t = t.Round(0). In general, prefer t.Equal(u)
/// to t == u, since t.Equal uses the most accurate comparison available and
/// correctly handles the case when only one of its arguments has a monotonic
/// clock reading.
#[derive(Debug, Clone)]
pub struct Time {
    pub wall: Arc<Mutex<Option<u64>>>,
    pub ext: Arc<Mutex<Option<i64>>>,
    pub loc: Arc<Mutex<Option<Location>>>,
}

impl Time {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.wall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.loc.clone();
        Self {
            wall: __go_clone_0_0,
            ext: __go_clone_1_0,
            loc: __go_clone_2_0,
        }
    }
}


impl Default for Time {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            wall: __go_default_0_0,
            ext: __go_default_1_0,
            loc: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Time {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Month specifies a month of the year (January = 1, ...).
#[derive(Debug, Clone, Default)]
pub struct Month(pub Arc<Mutex<Option<i32>>>);

impl Display for Month {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Month {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Month {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Month {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Month {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Month> for i32 {
    fn eq(&self, other: &Month) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Month> for i32 {
    fn partial_cmp(&self, other: &Month) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Month {
    type Output = Month;
    fn add(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Month {
    type Output = Month;
    fn add(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Month> for i32 {
    type Output = Month;
    fn add(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Month {
    type Output = Month;
    fn sub(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Month {
    type Output = Month;
    fn sub(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Month> for i32 {
    type Output = Month;
    fn sub(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Month {
    type Output = Month;
    fn mul(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Month {
    type Output = Month;
    fn mul(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Month> for i32 {
    type Output = Month;
    fn mul(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Month {
    type Output = Month;
    fn div(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Month {
    type Output = Month;
    fn div(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Month> for i32 {
    type Output = Month;
    fn div(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Month {
    type Output = Month;
    fn neg(self) -> Month {
        Month(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Month {
    type Output = Month;
    fn rem(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Month {
    type Output = Month;
    fn rem(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Month> for i32 {
    type Output = Month;
    fn rem(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Month {
    type Output = Month;
    fn bitand(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Month {
    type Output = Month;
    fn bitand(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Month> for i32 {
    type Output = Month;
    fn bitand(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Month {
    type Output = Month;
    fn bitor(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Month {
    type Output = Month;
    fn bitor(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Month> for i32 {
    type Output = Month;
    fn bitor(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Month {
    type Output = Month;
    fn bitxor(self, other: Self) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Month {
    type Output = Month;
    fn bitxor(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Month> for i32 {
    type Output = Month;
    fn bitxor(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Month {
    type Output = Month;
    fn not(self) -> Month {
        Month(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Month {
    type Output = Month;
    fn shl(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Month {
    type Output = Month;
    fn shl(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Month {
    type Output = Month;
    fn shl(self, other: i8) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Month {
    type Output = Month;
    fn shl(self, other: i16) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Month {
    type Output = Month;
    fn shl(self, other: i64) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Month {
    type Output = Month;
    fn shl(self, other: u32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Month {
    type Output = Month;
    fn shl(self, other: u8) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Month {
    type Output = Month;
    fn shl(self, other: u16) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Month {
    type Output = Month;
    fn shl(self, other: u64) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Month {
    type Output = Month;
    fn shl(self, other: usize) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Month {
    type Output = Month;
    fn shr(self, other: Month) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Month {
    type Output = Month;
    fn shr(self, other: i32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Month {
    type Output = Month;
    fn shr(self, other: i8) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Month {
    type Output = Month;
    fn shr(self, other: i16) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Month {
    type Output = Month;
    fn shr(self, other: i64) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Month {
    type Output = Month;
    fn shr(self, other: u32) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Month {
    type Output = Month;
    fn shr(self, other: u8) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Month {
    type Output = Month;
    fn shr(self, other: u16) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Month {
    type Output = Month;
    fn shr(self, other: u64) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Month {
    type Output = Month;
    fn shr(self, other: usize) -> Month {
        Month(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Month {}

impl Ord for Month {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Weekday specifies a day of the week (Sunday = 0, ...).
#[derive(Debug, Clone, Default)]
pub struct Weekday(pub Arc<Mutex<Option<i32>>>);

impl Display for Weekday {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Weekday {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Weekday {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Weekday {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Weekday {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Weekday> for i32 {
    fn eq(&self, other: &Weekday) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Weekday> for i32 {
    fn partial_cmp(&self, other: &Weekday) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Weekday {
    type Output = Weekday;
    fn add(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Weekday {
    type Output = Weekday;
    fn add(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Weekday> for i32 {
    type Output = Weekday;
    fn add(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Weekday {
    type Output = Weekday;
    fn sub(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Weekday {
    type Output = Weekday;
    fn sub(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Weekday> for i32 {
    type Output = Weekday;
    fn sub(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Weekday {
    type Output = Weekday;
    fn mul(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Weekday {
    type Output = Weekday;
    fn mul(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Weekday> for i32 {
    type Output = Weekday;
    fn mul(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Weekday {
    type Output = Weekday;
    fn div(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Weekday {
    type Output = Weekday;
    fn div(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Weekday> for i32 {
    type Output = Weekday;
    fn div(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Weekday {
    type Output = Weekday;
    fn neg(self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Weekday {
    type Output = Weekday;
    fn rem(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Weekday {
    type Output = Weekday;
    fn rem(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Weekday> for i32 {
    type Output = Weekday;
    fn rem(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Weekday {
    type Output = Weekday;
    fn bitand(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Weekday {
    type Output = Weekday;
    fn bitand(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Weekday> for i32 {
    type Output = Weekday;
    fn bitand(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Weekday {
    type Output = Weekday;
    fn bitor(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Weekday {
    type Output = Weekday;
    fn bitor(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Weekday> for i32 {
    type Output = Weekday;
    fn bitor(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Weekday {
    type Output = Weekday;
    fn bitxor(self, other: Self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Weekday {
    type Output = Weekday;
    fn bitxor(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Weekday> for i32 {
    type Output = Weekday;
    fn bitxor(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Weekday {
    type Output = Weekday;
    fn not(self) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Weekday {
    type Output = Weekday;
    fn shl(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Weekday {
    type Output = Weekday;
    fn shl(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Weekday {
    type Output = Weekday;
    fn shl(self, other: i8) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Weekday {
    type Output = Weekday;
    fn shl(self, other: i16) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Weekday {
    type Output = Weekday;
    fn shl(self, other: i64) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Weekday {
    type Output = Weekday;
    fn shl(self, other: u32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Weekday {
    type Output = Weekday;
    fn shl(self, other: u8) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Weekday {
    type Output = Weekday;
    fn shl(self, other: u16) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Weekday {
    type Output = Weekday;
    fn shl(self, other: u64) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Weekday {
    type Output = Weekday;
    fn shl(self, other: usize) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Weekday {
    type Output = Weekday;
    fn shr(self, other: Weekday) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Weekday {
    type Output = Weekday;
    fn shr(self, other: i32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Weekday {
    type Output = Weekday;
    fn shr(self, other: i8) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Weekday {
    type Output = Weekday;
    fn shr(self, other: i16) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Weekday {
    type Output = Weekday;
    fn shr(self, other: i64) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Weekday {
    type Output = Weekday;
    fn shr(self, other: u32) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Weekday {
    type Output = Weekday;
    fn shr(self, other: u8) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Weekday {
    type Output = Weekday;
    fn shr(self, other: u16) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Weekday {
    type Output = Weekday;
    fn shr(self, other: u64) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Weekday {
    type Output = Weekday;
    fn shr(self, other: usize) -> Weekday {
        Weekday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Weekday {}

impl Ord for Weekday {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absSeconds counts the number of seconds since the absolute zero instant.
#[derive(Debug, Clone, Default)]
pub struct absSeconds(pub Arc<Mutex<Option<u64>>>);

impl Display for absSeconds {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absSeconds {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for absSeconds {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absSeconds {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for absSeconds {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absSeconds> for u64 {
    fn eq(&self, other: &absSeconds) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absSeconds> for u64 {
    fn partial_cmp(&self, other: &absSeconds) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absSeconds {
    type Output = absSeconds;
    fn add(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for absSeconds {
    type Output = absSeconds;
    fn add(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absSeconds> for u64 {
    type Output = absSeconds;
    fn add(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absSeconds {
    type Output = absSeconds;
    fn sub(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for absSeconds {
    type Output = absSeconds;
    fn sub(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absSeconds> for u64 {
    type Output = absSeconds;
    fn sub(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absSeconds {
    type Output = absSeconds;
    fn mul(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for absSeconds {
    type Output = absSeconds;
    fn mul(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absSeconds> for u64 {
    type Output = absSeconds;
    fn mul(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absSeconds {
    type Output = absSeconds;
    fn div(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for absSeconds {
    type Output = absSeconds;
    fn div(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absSeconds> for u64 {
    type Output = absSeconds;
    fn div(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absSeconds {
    type Output = absSeconds;
    fn rem(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for absSeconds {
    type Output = absSeconds;
    fn rem(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absSeconds> for u64 {
    type Output = absSeconds;
    fn rem(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absSeconds {
    type Output = absSeconds;
    fn bitand(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for absSeconds {
    type Output = absSeconds;
    fn bitand(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absSeconds> for u64 {
    type Output = absSeconds;
    fn bitand(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absSeconds {
    type Output = absSeconds;
    fn bitor(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for absSeconds {
    type Output = absSeconds;
    fn bitor(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absSeconds> for u64 {
    type Output = absSeconds;
    fn bitor(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absSeconds {
    type Output = absSeconds;
    fn bitxor(self, other: Self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for absSeconds {
    type Output = absSeconds;
    fn bitxor(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absSeconds> for u64 {
    type Output = absSeconds;
    fn bitxor(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absSeconds {
    type Output = absSeconds;
    fn not(self) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: i32) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: i8) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: i16) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: i64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: u32) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: u8) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: u16) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absSeconds {
    type Output = absSeconds;
    fn shl(self, other: usize) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: absSeconds) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: i32) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: i8) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: i16) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: i64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: u32) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: u8) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: u16) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: u64) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absSeconds {
    type Output = absSeconds;
    fn shr(self, other: usize) -> absSeconds {
        absSeconds(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absSeconds {}

impl Ord for absSeconds {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absDays counts the number of days since the absolute zero instant.
#[derive(Debug, Clone, Default)]
pub struct absDays(pub Arc<Mutex<Option<u64>>>);

impl Display for absDays {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absDays {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for absDays {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absDays {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for absDays {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absDays> for u64 {
    fn eq(&self, other: &absDays) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absDays> for u64 {
    fn partial_cmp(&self, other: &absDays) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absDays {
    type Output = absDays;
    fn add(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for absDays {
    type Output = absDays;
    fn add(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absDays> for u64 {
    type Output = absDays;
    fn add(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absDays {
    type Output = absDays;
    fn sub(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for absDays {
    type Output = absDays;
    fn sub(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absDays> for u64 {
    type Output = absDays;
    fn sub(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absDays {
    type Output = absDays;
    fn mul(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for absDays {
    type Output = absDays;
    fn mul(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absDays> for u64 {
    type Output = absDays;
    fn mul(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absDays {
    type Output = absDays;
    fn div(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for absDays {
    type Output = absDays;
    fn div(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absDays> for u64 {
    type Output = absDays;
    fn div(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absDays {
    type Output = absDays;
    fn rem(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for absDays {
    type Output = absDays;
    fn rem(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absDays> for u64 {
    type Output = absDays;
    fn rem(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absDays {
    type Output = absDays;
    fn bitand(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for absDays {
    type Output = absDays;
    fn bitand(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absDays> for u64 {
    type Output = absDays;
    fn bitand(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absDays {
    type Output = absDays;
    fn bitor(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for absDays {
    type Output = absDays;
    fn bitor(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absDays> for u64 {
    type Output = absDays;
    fn bitor(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absDays {
    type Output = absDays;
    fn bitxor(self, other: Self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for absDays {
    type Output = absDays;
    fn bitxor(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absDays> for u64 {
    type Output = absDays;
    fn bitxor(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absDays {
    type Output = absDays;
    fn not(self) -> absDays {
        absDays(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absDays {
    type Output = absDays;
    fn shl(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absDays {
    type Output = absDays;
    fn shl(self, other: i32) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absDays {
    type Output = absDays;
    fn shl(self, other: i8) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absDays {
    type Output = absDays;
    fn shl(self, other: i16) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absDays {
    type Output = absDays;
    fn shl(self, other: i64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absDays {
    type Output = absDays;
    fn shl(self, other: u32) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absDays {
    type Output = absDays;
    fn shl(self, other: u8) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absDays {
    type Output = absDays;
    fn shl(self, other: u16) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absDays {
    type Output = absDays;
    fn shl(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absDays {
    type Output = absDays;
    fn shl(self, other: usize) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absDays {
    type Output = absDays;
    fn shr(self, other: absDays) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absDays {
    type Output = absDays;
    fn shr(self, other: i32) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absDays {
    type Output = absDays;
    fn shr(self, other: i8) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absDays {
    type Output = absDays;
    fn shr(self, other: i16) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absDays {
    type Output = absDays;
    fn shr(self, other: i64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absDays {
    type Output = absDays;
    fn shr(self, other: u32) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absDays {
    type Output = absDays;
    fn shr(self, other: u8) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absDays {
    type Output = absDays;
    fn shr(self, other: u16) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absDays {
    type Output = absDays;
    fn shr(self, other: u64) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absDays {
    type Output = absDays;
    fn shr(self, other: usize) -> absDays {
        absDays(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absDays {}

impl Ord for absDays {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absCentury counts the number of centuries since the absolute zero instant.
#[derive(Debug, Clone, Default)]
pub struct absCentury(pub Arc<Mutex<Option<u64>>>);

impl Display for absCentury {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absCentury {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for absCentury {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absCentury {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for absCentury {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absCentury> for u64 {
    fn eq(&self, other: &absCentury) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absCentury> for u64 {
    fn partial_cmp(&self, other: &absCentury) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absCentury {
    type Output = absCentury;
    fn add(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for absCentury {
    type Output = absCentury;
    fn add(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absCentury> for u64 {
    type Output = absCentury;
    fn add(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absCentury {
    type Output = absCentury;
    fn sub(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for absCentury {
    type Output = absCentury;
    fn sub(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absCentury> for u64 {
    type Output = absCentury;
    fn sub(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absCentury {
    type Output = absCentury;
    fn mul(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for absCentury {
    type Output = absCentury;
    fn mul(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absCentury> for u64 {
    type Output = absCentury;
    fn mul(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absCentury {
    type Output = absCentury;
    fn div(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for absCentury {
    type Output = absCentury;
    fn div(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absCentury> for u64 {
    type Output = absCentury;
    fn div(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absCentury {
    type Output = absCentury;
    fn rem(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for absCentury {
    type Output = absCentury;
    fn rem(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absCentury> for u64 {
    type Output = absCentury;
    fn rem(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absCentury {
    type Output = absCentury;
    fn bitand(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for absCentury {
    type Output = absCentury;
    fn bitand(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absCentury> for u64 {
    type Output = absCentury;
    fn bitand(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absCentury {
    type Output = absCentury;
    fn bitor(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for absCentury {
    type Output = absCentury;
    fn bitor(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absCentury> for u64 {
    type Output = absCentury;
    fn bitor(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absCentury {
    type Output = absCentury;
    fn bitxor(self, other: Self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for absCentury {
    type Output = absCentury;
    fn bitxor(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absCentury> for u64 {
    type Output = absCentury;
    fn bitxor(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absCentury {
    type Output = absCentury;
    fn not(self) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absCentury {
    type Output = absCentury;
    fn shl(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absCentury {
    type Output = absCentury;
    fn shl(self, other: i32) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absCentury {
    type Output = absCentury;
    fn shl(self, other: i8) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absCentury {
    type Output = absCentury;
    fn shl(self, other: i16) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absCentury {
    type Output = absCentury;
    fn shl(self, other: i64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absCentury {
    type Output = absCentury;
    fn shl(self, other: u32) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absCentury {
    type Output = absCentury;
    fn shl(self, other: u8) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absCentury {
    type Output = absCentury;
    fn shl(self, other: u16) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absCentury {
    type Output = absCentury;
    fn shl(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absCentury {
    type Output = absCentury;
    fn shl(self, other: usize) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absCentury {
    type Output = absCentury;
    fn shr(self, other: absCentury) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absCentury {
    type Output = absCentury;
    fn shr(self, other: i32) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absCentury {
    type Output = absCentury;
    fn shr(self, other: i8) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absCentury {
    type Output = absCentury;
    fn shr(self, other: i16) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absCentury {
    type Output = absCentury;
    fn shr(self, other: i64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absCentury {
    type Output = absCentury;
    fn shr(self, other: u32) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absCentury {
    type Output = absCentury;
    fn shr(self, other: u8) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absCentury {
    type Output = absCentury;
    fn shr(self, other: u16) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absCentury {
    type Output = absCentury;
    fn shr(self, other: u64) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absCentury {
    type Output = absCentury;
    fn shr(self, other: usize) -> absCentury {
        absCentury(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absCentury {}

impl Ord for absCentury {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absCyear counts the number of years since the start of a century.
#[derive(Debug, Clone, Default)]
pub struct absCyear(pub Arc<Mutex<Option<i32>>>);

impl Display for absCyear {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absCyear {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for absCyear {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absCyear {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for absCyear {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absCyear> for i32 {
    fn eq(&self, other: &absCyear) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absCyear> for i32 {
    fn partial_cmp(&self, other: &absCyear) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absCyear {
    type Output = absCyear;
    fn add(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for absCyear {
    type Output = absCyear;
    fn add(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absCyear> for i32 {
    type Output = absCyear;
    fn add(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absCyear {
    type Output = absCyear;
    fn sub(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for absCyear {
    type Output = absCyear;
    fn sub(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absCyear> for i32 {
    type Output = absCyear;
    fn sub(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absCyear {
    type Output = absCyear;
    fn mul(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for absCyear {
    type Output = absCyear;
    fn mul(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absCyear> for i32 {
    type Output = absCyear;
    fn mul(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absCyear {
    type Output = absCyear;
    fn div(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for absCyear {
    type Output = absCyear;
    fn div(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absCyear> for i32 {
    type Output = absCyear;
    fn div(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for absCyear {
    type Output = absCyear;
    fn neg(self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absCyear {
    type Output = absCyear;
    fn rem(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for absCyear {
    type Output = absCyear;
    fn rem(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absCyear> for i32 {
    type Output = absCyear;
    fn rem(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absCyear {
    type Output = absCyear;
    fn bitand(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for absCyear {
    type Output = absCyear;
    fn bitand(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absCyear> for i32 {
    type Output = absCyear;
    fn bitand(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absCyear {
    type Output = absCyear;
    fn bitor(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for absCyear {
    type Output = absCyear;
    fn bitor(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absCyear> for i32 {
    type Output = absCyear;
    fn bitor(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absCyear {
    type Output = absCyear;
    fn bitxor(self, other: Self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for absCyear {
    type Output = absCyear;
    fn bitxor(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absCyear> for i32 {
    type Output = absCyear;
    fn bitxor(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absCyear {
    type Output = absCyear;
    fn not(self) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absCyear {
    type Output = absCyear;
    fn shl(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absCyear {
    type Output = absCyear;
    fn shl(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absCyear {
    type Output = absCyear;
    fn shl(self, other: i8) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absCyear {
    type Output = absCyear;
    fn shl(self, other: i16) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absCyear {
    type Output = absCyear;
    fn shl(self, other: i64) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absCyear {
    type Output = absCyear;
    fn shl(self, other: u32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absCyear {
    type Output = absCyear;
    fn shl(self, other: u8) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absCyear {
    type Output = absCyear;
    fn shl(self, other: u16) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absCyear {
    type Output = absCyear;
    fn shl(self, other: u64) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absCyear {
    type Output = absCyear;
    fn shl(self, other: usize) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absCyear {
    type Output = absCyear;
    fn shr(self, other: absCyear) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absCyear {
    type Output = absCyear;
    fn shr(self, other: i32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absCyear {
    type Output = absCyear;
    fn shr(self, other: i8) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absCyear {
    type Output = absCyear;
    fn shr(self, other: i16) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absCyear {
    type Output = absCyear;
    fn shr(self, other: i64) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absCyear {
    type Output = absCyear;
    fn shr(self, other: u32) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absCyear {
    type Output = absCyear;
    fn shr(self, other: u8) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absCyear {
    type Output = absCyear;
    fn shr(self, other: u16) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absCyear {
    type Output = absCyear;
    fn shr(self, other: u64) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absCyear {
    type Output = absCyear;
    fn shr(self, other: usize) -> absCyear {
        absCyear(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absCyear {}

impl Ord for absCyear {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absYday counts the number of days since the start of a year.
/// Note that absolute years start on March 1.
#[derive(Debug, Clone, Default)]
pub struct absYday(pub Arc<Mutex<Option<i32>>>);

impl Display for absYday {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absYday {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for absYday {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absYday {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for absYday {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absYday> for i32 {
    fn eq(&self, other: &absYday) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absYday> for i32 {
    fn partial_cmp(&self, other: &absYday) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absYday {
    type Output = absYday;
    fn add(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for absYday {
    type Output = absYday;
    fn add(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absYday> for i32 {
    type Output = absYday;
    fn add(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absYday {
    type Output = absYday;
    fn sub(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for absYday {
    type Output = absYday;
    fn sub(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absYday> for i32 {
    type Output = absYday;
    fn sub(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absYday {
    type Output = absYday;
    fn mul(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for absYday {
    type Output = absYday;
    fn mul(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absYday> for i32 {
    type Output = absYday;
    fn mul(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absYday {
    type Output = absYday;
    fn div(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for absYday {
    type Output = absYday;
    fn div(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absYday> for i32 {
    type Output = absYday;
    fn div(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for absYday {
    type Output = absYday;
    fn neg(self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absYday {
    type Output = absYday;
    fn rem(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for absYday {
    type Output = absYday;
    fn rem(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absYday> for i32 {
    type Output = absYday;
    fn rem(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absYday {
    type Output = absYday;
    fn bitand(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for absYday {
    type Output = absYday;
    fn bitand(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absYday> for i32 {
    type Output = absYday;
    fn bitand(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absYday {
    type Output = absYday;
    fn bitor(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for absYday {
    type Output = absYday;
    fn bitor(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absYday> for i32 {
    type Output = absYday;
    fn bitor(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absYday {
    type Output = absYday;
    fn bitxor(self, other: Self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for absYday {
    type Output = absYday;
    fn bitxor(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absYday> for i32 {
    type Output = absYday;
    fn bitxor(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absYday {
    type Output = absYday;
    fn not(self) -> absYday {
        absYday(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absYday {
    type Output = absYday;
    fn shl(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absYday {
    type Output = absYday;
    fn shl(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absYday {
    type Output = absYday;
    fn shl(self, other: i8) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absYday {
    type Output = absYday;
    fn shl(self, other: i16) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absYday {
    type Output = absYday;
    fn shl(self, other: i64) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absYday {
    type Output = absYday;
    fn shl(self, other: u32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absYday {
    type Output = absYday;
    fn shl(self, other: u8) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absYday {
    type Output = absYday;
    fn shl(self, other: u16) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absYday {
    type Output = absYday;
    fn shl(self, other: u64) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absYday {
    type Output = absYday;
    fn shl(self, other: usize) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absYday {
    type Output = absYday;
    fn shr(self, other: absYday) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absYday {
    type Output = absYday;
    fn shr(self, other: i32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absYday {
    type Output = absYday;
    fn shr(self, other: i8) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absYday {
    type Output = absYday;
    fn shr(self, other: i16) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absYday {
    type Output = absYday;
    fn shr(self, other: i64) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absYday {
    type Output = absYday;
    fn shr(self, other: u32) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absYday {
    type Output = absYday;
    fn shr(self, other: u8) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absYday {
    type Output = absYday;
    fn shr(self, other: u16) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absYday {
    type Output = absYday;
    fn shr(self, other: u64) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absYday {
    type Output = absYday;
    fn shr(self, other: usize) -> absYday {
        absYday(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absYday {}

impl Ord for absYday {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absMonth counts the number of months since the start of a year.
/// absMonth=0 denotes March.
#[derive(Debug, Clone, Default)]
pub struct absMonth(pub Arc<Mutex<Option<i32>>>);

impl Display for absMonth {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absMonth {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for absMonth {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absMonth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for absMonth {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absMonth> for i32 {
    fn eq(&self, other: &absMonth) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absMonth> for i32 {
    fn partial_cmp(&self, other: &absMonth) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absMonth {
    type Output = absMonth;
    fn add(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for absMonth {
    type Output = absMonth;
    fn add(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absMonth> for i32 {
    type Output = absMonth;
    fn add(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absMonth {
    type Output = absMonth;
    fn sub(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for absMonth {
    type Output = absMonth;
    fn sub(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absMonth> for i32 {
    type Output = absMonth;
    fn sub(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absMonth {
    type Output = absMonth;
    fn mul(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for absMonth {
    type Output = absMonth;
    fn mul(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absMonth> for i32 {
    type Output = absMonth;
    fn mul(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absMonth {
    type Output = absMonth;
    fn div(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for absMonth {
    type Output = absMonth;
    fn div(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absMonth> for i32 {
    type Output = absMonth;
    fn div(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for absMonth {
    type Output = absMonth;
    fn neg(self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absMonth {
    type Output = absMonth;
    fn rem(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for absMonth {
    type Output = absMonth;
    fn rem(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absMonth> for i32 {
    type Output = absMonth;
    fn rem(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absMonth {
    type Output = absMonth;
    fn bitand(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for absMonth {
    type Output = absMonth;
    fn bitand(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absMonth> for i32 {
    type Output = absMonth;
    fn bitand(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absMonth {
    type Output = absMonth;
    fn bitor(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for absMonth {
    type Output = absMonth;
    fn bitor(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absMonth> for i32 {
    type Output = absMonth;
    fn bitor(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absMonth {
    type Output = absMonth;
    fn bitxor(self, other: Self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for absMonth {
    type Output = absMonth;
    fn bitxor(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absMonth> for i32 {
    type Output = absMonth;
    fn bitxor(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absMonth {
    type Output = absMonth;
    fn not(self) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absMonth {
    type Output = absMonth;
    fn shl(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absMonth {
    type Output = absMonth;
    fn shl(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absMonth {
    type Output = absMonth;
    fn shl(self, other: i8) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absMonth {
    type Output = absMonth;
    fn shl(self, other: i16) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absMonth {
    type Output = absMonth;
    fn shl(self, other: i64) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absMonth {
    type Output = absMonth;
    fn shl(self, other: u32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absMonth {
    type Output = absMonth;
    fn shl(self, other: u8) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absMonth {
    type Output = absMonth;
    fn shl(self, other: u16) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absMonth {
    type Output = absMonth;
    fn shl(self, other: u64) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absMonth {
    type Output = absMonth;
    fn shl(self, other: usize) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absMonth {
    type Output = absMonth;
    fn shr(self, other: absMonth) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absMonth {
    type Output = absMonth;
    fn shr(self, other: i32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absMonth {
    type Output = absMonth;
    fn shr(self, other: i8) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absMonth {
    type Output = absMonth;
    fn shr(self, other: i16) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absMonth {
    type Output = absMonth;
    fn shr(self, other: i64) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absMonth {
    type Output = absMonth;
    fn shr(self, other: u32) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absMonth {
    type Output = absMonth;
    fn shr(self, other: u8) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absMonth {
    type Output = absMonth;
    fn shr(self, other: u16) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absMonth {
    type Output = absMonth;
    fn shr(self, other: u64) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absMonth {
    type Output = absMonth;
    fn shr(self, other: usize) -> absMonth {
        absMonth(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absMonth {}

impl Ord for absMonth {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absLeap is a single bit (0 or 1) denoting whether a given year is a leap year.
#[derive(Debug, Clone, Default)]
pub struct absLeap(pub Arc<Mutex<Option<i32>>>);

impl Display for absLeap {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absLeap {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for absLeap {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absLeap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for absLeap {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absLeap> for i32 {
    fn eq(&self, other: &absLeap) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absLeap> for i32 {
    fn partial_cmp(&self, other: &absLeap) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absLeap {
    type Output = absLeap;
    fn add(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for absLeap {
    type Output = absLeap;
    fn add(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absLeap> for i32 {
    type Output = absLeap;
    fn add(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absLeap {
    type Output = absLeap;
    fn sub(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for absLeap {
    type Output = absLeap;
    fn sub(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absLeap> for i32 {
    type Output = absLeap;
    fn sub(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absLeap {
    type Output = absLeap;
    fn mul(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for absLeap {
    type Output = absLeap;
    fn mul(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absLeap> for i32 {
    type Output = absLeap;
    fn mul(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absLeap {
    type Output = absLeap;
    fn div(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for absLeap {
    type Output = absLeap;
    fn div(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absLeap> for i32 {
    type Output = absLeap;
    fn div(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for absLeap {
    type Output = absLeap;
    fn neg(self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absLeap {
    type Output = absLeap;
    fn rem(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for absLeap {
    type Output = absLeap;
    fn rem(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absLeap> for i32 {
    type Output = absLeap;
    fn rem(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absLeap {
    type Output = absLeap;
    fn bitand(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for absLeap {
    type Output = absLeap;
    fn bitand(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absLeap> for i32 {
    type Output = absLeap;
    fn bitand(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absLeap {
    type Output = absLeap;
    fn bitor(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for absLeap {
    type Output = absLeap;
    fn bitor(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absLeap> for i32 {
    type Output = absLeap;
    fn bitor(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absLeap {
    type Output = absLeap;
    fn bitxor(self, other: Self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for absLeap {
    type Output = absLeap;
    fn bitxor(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absLeap> for i32 {
    type Output = absLeap;
    fn bitxor(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absLeap {
    type Output = absLeap;
    fn not(self) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absLeap {
    type Output = absLeap;
    fn shl(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absLeap {
    type Output = absLeap;
    fn shl(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absLeap {
    type Output = absLeap;
    fn shl(self, other: i8) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absLeap {
    type Output = absLeap;
    fn shl(self, other: i16) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absLeap {
    type Output = absLeap;
    fn shl(self, other: i64) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absLeap {
    type Output = absLeap;
    fn shl(self, other: u32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absLeap {
    type Output = absLeap;
    fn shl(self, other: u8) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absLeap {
    type Output = absLeap;
    fn shl(self, other: u16) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absLeap {
    type Output = absLeap;
    fn shl(self, other: u64) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absLeap {
    type Output = absLeap;
    fn shl(self, other: usize) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absLeap {
    type Output = absLeap;
    fn shr(self, other: absLeap) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absLeap {
    type Output = absLeap;
    fn shr(self, other: i32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absLeap {
    type Output = absLeap;
    fn shr(self, other: i8) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absLeap {
    type Output = absLeap;
    fn shr(self, other: i16) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absLeap {
    type Output = absLeap;
    fn shr(self, other: i64) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absLeap {
    type Output = absLeap;
    fn shr(self, other: u32) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absLeap {
    type Output = absLeap;
    fn shr(self, other: u8) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absLeap {
    type Output = absLeap;
    fn shr(self, other: u16) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absLeap {
    type Output = absLeap;
    fn shr(self, other: u64) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absLeap {
    type Output = absLeap;
    fn shr(self, other: usize) -> absLeap {
        absLeap(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absLeap {}

impl Ord for absLeap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An absJanFeb is a single bit (0 or 1) denoting whether a given day falls in January or February.
/// That is a special case because the absolute years start in March (unlike normal calendar years).
#[derive(Debug, Clone, Default)]
pub struct absJanFeb(pub Arc<Mutex<Option<i32>>>);

impl Display for absJanFeb {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for absJanFeb {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for absJanFeb {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for absJanFeb {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for absJanFeb {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<absJanFeb> for i32 {
    fn eq(&self, other: &absJanFeb) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<absJanFeb> for i32 {
    fn partial_cmp(&self, other: &absJanFeb) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for absJanFeb {
    type Output = absJanFeb;
    fn add(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for absJanFeb {
    type Output = absJanFeb;
    fn add(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn add(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for absJanFeb {
    type Output = absJanFeb;
    fn sub(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for absJanFeb {
    type Output = absJanFeb;
    fn sub(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn sub(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for absJanFeb {
    type Output = absJanFeb;
    fn mul(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for absJanFeb {
    type Output = absJanFeb;
    fn mul(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn mul(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for absJanFeb {
    type Output = absJanFeb;
    fn div(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for absJanFeb {
    type Output = absJanFeb;
    fn div(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn div(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for absJanFeb {
    type Output = absJanFeb;
    fn neg(self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for absJanFeb {
    type Output = absJanFeb;
    fn rem(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for absJanFeb {
    type Output = absJanFeb;
    fn rem(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn rem(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for absJanFeb {
    type Output = absJanFeb;
    fn bitand(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for absJanFeb {
    type Output = absJanFeb;
    fn bitand(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn bitand(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for absJanFeb {
    type Output = absJanFeb;
    fn bitor(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for absJanFeb {
    type Output = absJanFeb;
    fn bitor(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn bitor(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for absJanFeb {
    type Output = absJanFeb;
    fn bitxor(self, other: Self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for absJanFeb {
    type Output = absJanFeb;
    fn bitxor(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<absJanFeb> for i32 {
    type Output = absJanFeb;
    fn bitxor(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for absJanFeb {
    type Output = absJanFeb;
    fn not(self) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: i8) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: i16) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: i64) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: u32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: u8) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: u16) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: u64) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for absJanFeb {
    type Output = absJanFeb;
    fn shl(self, other: usize) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: absJanFeb) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: i32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: i8) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: i16) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: i64) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: u32) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: u8) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: u16) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: u64) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for absJanFeb {
    type Output = absJanFeb;
    fn shr(self, other: usize) -> absJanFeb {
        absJanFeb(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for absJanFeb {}

impl Ord for absJanFeb {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Duration represents the elapsed time between two instants
/// as an int64 nanosecond count. The representation limits the
/// largest representable duration to approximately 290 years.
#[derive(Debug, Clone, Default)]
pub struct Duration(pub Arc<Mutex<Option<i64>>>);

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Duration {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Duration {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Duration> for i64 {
    fn eq(&self, other: &Duration) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Duration> for i64 {
    fn partial_cmp(&self, other: &Duration) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Duration {
    type Output = Duration;
    fn add(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Duration {
    type Output = Duration;
    fn add(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Duration> for i64 {
    type Output = Duration;
    fn add(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Duration {
    type Output = Duration;
    fn sub(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Duration {
    type Output = Duration;
    fn sub(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Duration> for i64 {
    type Output = Duration;
    fn sub(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Duration {
    type Output = Duration;
    fn mul(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Duration {
    type Output = Duration;
    fn mul(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Duration> for i64 {
    type Output = Duration;
    fn mul(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Duration {
    type Output = Duration;
    fn div(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Duration {
    type Output = Duration;
    fn div(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Duration> for i64 {
    type Output = Duration;
    fn div(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Duration {
    type Output = Duration;
    fn neg(self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Duration {
    type Output = Duration;
    fn rem(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Duration {
    type Output = Duration;
    fn rem(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Duration> for i64 {
    type Output = Duration;
    fn rem(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Duration {
    type Output = Duration;
    fn bitand(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Duration {
    type Output = Duration;
    fn bitand(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Duration> for i64 {
    type Output = Duration;
    fn bitand(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Duration {
    type Output = Duration;
    fn bitor(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Duration {
    type Output = Duration;
    fn bitor(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Duration> for i64 {
    type Output = Duration;
    fn bitor(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Duration {
    type Output = Duration;
    fn bitxor(self, other: Self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Duration {
    type Output = Duration;
    fn bitxor(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Duration> for i64 {
    type Output = Duration;
    fn bitxor(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Duration {
    type Output = Duration;
    fn not(self) -> Duration {
        Duration(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Duration {
    type Output = Duration;
    fn shl(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Duration {
    type Output = Duration;
    fn shl(self, other: i32) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Duration {
    type Output = Duration;
    fn shl(self, other: i8) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Duration {
    type Output = Duration;
    fn shl(self, other: i16) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Duration {
    type Output = Duration;
    fn shl(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Duration {
    type Output = Duration;
    fn shl(self, other: u32) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Duration {
    type Output = Duration;
    fn shl(self, other: u8) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Duration {
    type Output = Duration;
    fn shl(self, other: u16) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Duration {
    type Output = Duration;
    fn shl(self, other: u64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Duration {
    type Output = Duration;
    fn shl(self, other: usize) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Duration {
    type Output = Duration;
    fn shr(self, other: Duration) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Duration {
    type Output = Duration;
    fn shr(self, other: i32) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Duration {
    type Output = Duration;
    fn shr(self, other: i8) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Duration {
    type Output = Duration;
    fn shr(self, other: i16) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Duration {
    type Output = Duration;
    fn shr(self, other: i64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Duration {
    type Output = Duration;
    fn shr(self, other: u32) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Duration {
    type Output = Duration;
    fn shr(self, other: u8) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Duration {
    type Output = Duration;
    fn shr(self, other: u16) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Duration {
    type Output = Duration;
    fn shr(self, other: u64) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Duration {
    type Output = Duration;
    fn shr(self, other: usize) -> Duration {
        Duration(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Duration {}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static startNano: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *startNano.lock().unwrap() = Some(0);
    *startNano.lock().unwrap() = Some({ let __tmp_x = runtime_nano(); let __tmp_y = 1 as i64; __tmp_x - __tmp_y });
}


pub(crate) fn __go_zero_globals() {
    *startNano.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_10() {
    *startNano.lock().unwrap() = Some({ let __tmp_x = runtime_nano(); let __tmp_y = 1 as i64; __tmp_x - __tmp_y });
}


impl Time {
    /// nsec returns the time's nanoseconds.
    pub fn nsec(&self) -> i32 {
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = NSEC_MASK as u64; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
    }

    /// sec returns the time's seconds since Jan 1 year 1.
    pub fn sec(&self) -> i64 {
        if { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return { let __tmp_x = WALL_TO_INTERNAL as i64; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }; let __tmp_y = ({ let __tmp_x = NSEC_SHIFT; let __tmp_y = 1; __tmp_x + __tmp_y }); __tmp_x >> __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
    }
        return (*self.ext.lock().unwrap().as_ref().unwrap());
    }

    /// unixSec returns the time's seconds since Jan 1 1970 (Unix time).
    pub fn unix_sec(&self) -> i64 {
        return { let __tmp_x = self.sec(); let __tmp_y = INTERNAL_TO_UNIX as i64; __tmp_x + __tmp_y };
    }

    /// addSec adds d seconds to the time.
    pub fn add_sec(&mut self, d: Arc<Mutex<Option<i64>>>) {
        if { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        let mut sec = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }; let __tmp_y = ({ let __tmp_x = NSEC_SHIFT; let __tmp_y = 1; __tmp_x + __tmp_y }); __tmp_x >> __tmp_y }) as i64)));
        let mut dsec = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        if { let __tmp_x = 0 as i64; let __tmp_y = { let __v = (*dsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*dsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as i64) << (33 as i64)) - (1 as i64)) as i64; __tmp_x <= __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = NSEC_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*dsec.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = NSEC_SHIFT; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = HAS_MONOTONIC as u64; __tmp_x | __tmp_y }; *self.wall.lock().unwrap() = Some(new_val); };
        return;
    }
                // Wall second now out of range for packed field.
                // Move to ext.
        self.strip_mono();
    }
                // Wall second now out of range for packed field.
                // Move to ext.
                // Check if the sum of t.ext and d overflows and handle it properly.
        let mut sum = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        if { let __tmp_x = ({ let __tmp_x = { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.ext.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y }); __tmp_x == __tmp_y } {
        { let new_val = sum.lock().unwrap().as_ref().unwrap().clone(); *self.ext.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let new_val = (((1 as u64) << (63 as u64)) - (1 as u64)) as i64; *self.ext.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = -((((1 as i128) << (63 as i128)) - (1 as i128))) as i64; *self.ext.lock().unwrap() = Some(new_val); };
    }
    }

    /// setLoc sets the location associated with the time.
    pub fn set_loc(&mut self, mut loc: Arc<Mutex<Option<Location>>>) {
        if { let __left = loc.clone(); let __right = utcLoc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        *loc.lock().unwrap() = None;
    }
        self.strip_mono();
        { let new_val = loc.clone(); self.loc = new_val; };
    }

    /// stripMono strips the monotonic clock reading in t.
    pub fn strip_mono(&mut self) {
        if { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = self.sec(); *self.ext.lock().unwrap() = Some(new_val); };
        { let __target = self.wall.clone(); let __rhs = NSEC_MASK as u64; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
    }
    }

    /// setMono sets the monotonic clock reading in t.
    /// If t cannot hold a monotonic clock reading,
    /// because its wall time is too large,
    /// setMono is a no-op.
    pub fn set_mono(&mut self, m: Arc<Mutex<Option<i64>>>) {
        if { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        let mut sec = Arc::new(Mutex::new(Some({ let __selector_holder = self.ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_WALL as i64; __tmp_x < __tmp_y } || { let __tmp_x = MAX_WALL as i64; let __tmp_y = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return;
    }
        { let __target = self.wall.clone(); let __rhs = { let __tmp_x = HAS_MONOTONIC as u64; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_WALL as i64; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = NSEC_SHIFT; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *self.ext.lock().unwrap() = Some(new_val); };
    }

    /// mono returns t's monotonic clock reading.
    /// It returns 0 for a missing reading.
    /// This function is used only for testing,
    /// so it's OK that technically 0 is a valid
    /// monotonic clock reading as well.
    pub fn mono(&self) -> i64 {
        if { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return 0;
    }
        return (*self.ext.lock().unwrap().as_ref().unwrap());
    }

    /// IsZero reports whether t represents the zero time instant,
    /// January 1, year 1, 00:00:00 UTC.
    pub fn is_zero(&self) -> bool {
        return { let __tmp_x = self.sec(); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } && { let __tmp_x = self.nsec(); let __tmp_y = 0 as i32; __tmp_x == __tmp_y };
    }

    /// After reports whether the time instant t is after u.
    pub fn after(&self, u: Arc<Mutex<Option<Time>>>) -> bool {
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).wall.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return { let __tmp_x = (*self.ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).ext.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y };
    }
        let mut ts = self.sec();
        let mut us = (*u.lock().unwrap().as_ref().unwrap()).sec();
        return { let __tmp_x = ts; let __tmp_y = us; __tmp_x > __tmp_y } || { let __tmp_x = ts; let __tmp_y = us; __tmp_x == __tmp_y } && { let __tmp_x = self.nsec(); let __tmp_y = (*u.lock().unwrap().as_ref().unwrap()).nsec(); __tmp_x > __tmp_y };
    }

    /// Before reports whether the time instant t is before u.
    pub fn before(&self, u: Arc<Mutex<Option<Time>>>) -> bool {
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).wall.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return { let __tmp_x = (*self.ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).ext.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
    }
        let mut ts = self.sec();
        let mut us = (*u.lock().unwrap().as_ref().unwrap()).sec();
        return { let __tmp_x = ts; let __tmp_y = us; __tmp_x < __tmp_y } || { let __tmp_x = ts; let __tmp_y = us; __tmp_x == __tmp_y } && { let __tmp_x = self.nsec(); let __tmp_y = (*u.lock().unwrap().as_ref().unwrap()).nsec(); __tmp_x < __tmp_y };
    }

    /// Compare compares the time instant t with u. If t is before u, it returns -1;
    /// if t is after u, it returns +1; if they're the same, it returns 0.
    pub fn compare(&self, u: Arc<Mutex<Option<Time>>>) -> i32 {
        let mut tc: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut uc: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).wall.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __tmp_0 = { let __selector_holder = self.ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *tc.lock().unwrap() = Some(__tmp_0); *uc.lock().unwrap() = Some(__tmp_1); };
    } else {
        { let __tmp_0 = self.sec(); let __tmp_1 = (*u.lock().unwrap().as_ref().unwrap()).sec(); *tc.lock().unwrap() = Some(__tmp_0); *uc.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*tc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*uc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let __tmp_0 = Arc::new(Mutex::new(Some(self.nsec() as i64))); let __tmp_1 = Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()).nsec() as i64))); *tc.lock().unwrap() = __tmp_0.lock().unwrap().take(); *uc.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
    }
        if { let __tmp_x = { let __v = (*tc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*uc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            return -(1);
        } else if { let __tmp_x = { let __v = (*tc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*uc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            return 1;
        }
        0
    }

    /// Equal reports whether t and u represent the same time instant.
    /// Two times can be equal even if they are in different locations.
    /// For example, 6:00 +0200 and 4:00 UTC are Equal.
    /// See the documentation on the Time type for the pitfalls of using == with
    /// Time values; most code should use Equal instead.
    pub fn equal(&self, u: Arc<Mutex<Option<Time>>>) -> bool {
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).wall.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return { let __tmp_x = (*self.ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).ext.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
    }
        return { let __tmp_x = self.sec(); let __tmp_y = (*u.lock().unwrap().as_ref().unwrap()).sec(); __tmp_x == __tmp_y } && { let __tmp_x = self.nsec(); let __tmp_y = (*u.lock().unwrap().as_ref().unwrap()).nsec(); __tmp_x == __tmp_y };
    }

    /// absSec returns the time t as an absolute seconds, adjusted by the zone offset.
    /// It is called when computing a presentation property like Month or Hour.
    /// We'd rather call it abs, but there are linknames to abs that make that problematic.
    /// See timeAbs below.
    pub fn abs_sec(&self) -> Arc<Mutex<Option<absSeconds>>> {
        let mut l = self.loc.clone();
                // Avoid function calls when possible.
        if { let __nil_result = (*l.lock().unwrap()).is_none(); __nil_result } || { let __left = l.clone(); let __right = localLoc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = { let __recv = l.clone(); let __recv_ptr: *const crate::zoneinfo::Location = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::zoneinfo::Location }; let __result = unsafe { &*__recv_ptr }.get(); __result }.clone(); l = new_val; };
    }
        let mut sec = self.unix_sec();
        if { let __left = l.clone(); let __right = utcLoc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        if { let __ptr_field = (*l.lock().unwrap().as_ref().unwrap()).cache_zone.clone(); !__ptr_field.is_nil() } && { let __tmp_x = (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).cache_start.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = sec; __tmp_x <= __tmp_y } && { let __tmp_x = sec; let __tmp_y = (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).cache_end.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = (*l.lock().unwrap().as_ref().unwrap()).cache_zone.with_mut(|__ptr_value| __ptr_value.offset.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); sec = sec + __rhs; };
    } else {
        let (_, mut offset, _, _, _) = { let __recv = l.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some(sec)))); __result };
        { let __rhs = (*Arc::new(Mutex::new(Some(offset as i64))).lock().unwrap().as_ref().unwrap()); sec = sec + __rhs; };
    }
    }
        Arc::new(Mutex::new(Some(absSeconds(Arc::new(Mutex::new(Some({ let __tmp_x = sec; let __tmp_y = ({ let __tmp_x = UNIX_TO_INTERNAL as i64; let __tmp_y = INTERNAL_TO_ABSOLUTE as i64; __tmp_x + __tmp_y }) as i64; __tmp_x + __tmp_y } as u64)))))))
    }

    /// locabs is a combination of the Zone and abs methods,
    /// extracting both return values from a single zone lookup.
    pub fn locabs(&self) -> (Arc<Mutex<Option<String>>>, i32, Arc<Mutex<Option<absSeconds>>>) {
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut offset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut abs: Arc<Mutex<Option<absSeconds>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut l = self.loc.clone();
        if { let __nil_result = (*l.lock().unwrap()).is_none(); __nil_result } || { let __left = l.clone(); let __right = localLoc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = { let __recv = l.clone(); let __recv_ptr: *const crate::zoneinfo::Location = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::zoneinfo::Location }; let __result = unsafe { &*__recv_ptr }.get(); __result }.clone(); l = new_val; };
    }
                // Avoid function call if we hit the local time cache.
        let mut sec = self.unix_sec();
        if { let __left = l.clone(); let __right = utcLoc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        if { let __ptr_field = (*l.lock().unwrap().as_ref().unwrap()).cache_zone.clone(); !__ptr_field.is_nil() } && { let __tmp_x = (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).cache_start.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = sec; __tmp_x <= __tmp_y } && { let __tmp_x = sec; let __tmp_y = (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).cache_end.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = { let __ptr_value = (*l.lock().unwrap().as_ref().unwrap()).cache_zone.with_mut(|__ptr_value| __ptr_value.name.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = { let __ptr_value = (*l.lock().unwrap().as_ref().unwrap()).cache_zone.with_mut(|__ptr_value| __ptr_value.offset.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *offset.lock().unwrap() = Some(new_val); };
    } else {
        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3, __tmp_4) = { let __recv = l.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some(sec)))); __result }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_tmp_0; *offset.lock().unwrap() = Some(__tmp_1); };
    }
        { let __rhs = (*Arc::new(Mutex::new(Some((*offset.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); sec = sec + __rhs; };
    } else {
        { let new_val = "UTC".to_string(); *name.lock().unwrap() = Some(new_val); };
    }
        { let new_val = absSeconds(Arc::new(Mutex::new(Some({ let __tmp_x = sec; let __tmp_y = ({ let __tmp_x = UNIX_TO_INTERNAL as i64; let __tmp_y = INTERNAL_TO_ABSOLUTE as i64; __tmp_x + __tmp_y }) as i64; __tmp_x + __tmp_y } as u64)))); *abs.lock().unwrap() = Some(new_val); };
        return (name.clone(), (*offset.lock().unwrap().as_ref().unwrap()), abs.clone());
    }

    /// Date returns the year, month, and day in which t occurs.
    pub fn date(&self) -> (i32, Arc<Mutex<Option<Month>>>, i32) {
    let mut year: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut month: Arc<Mutex<Option<Month>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut day: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        absDays::date(&(*absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))
    }

    /// Year returns the year in which t occurs.
    pub fn year(&self) -> i32 {
        let (mut century, mut cyear, mut ayday) = absDays::split(&(*absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
        let mut janFeb = absYday::jan_feb(&(*ayday.lock().unwrap().as_ref().unwrap()));
        return absCentury::year(&(*century.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = cyear.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = janFeb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Month returns the month of the year specified by t.
    pub fn month(&self) -> Arc<Mutex<Option<Month>>> {
        let (_, _, mut ayday) = absDays::split(&(*absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
        let (mut amonth, _) = absYday::split(&(*ayday.lock().unwrap().as_ref().unwrap()));
        return absMonth::month(&(*amonth.lock().unwrap().as_ref().unwrap()), absYday::jan_feb(&(*ayday.lock().unwrap().as_ref().unwrap())));
    }

    /// Day returns the day of the month specified by t.
    pub fn day(&self) -> i32 {
        let (_, _, mut ayday) = absDays::split(&(*absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
        let (_, mut day) = absYday::split(&(*ayday.lock().unwrap().as_ref().unwrap()));
        day
    }

    /// Weekday returns the day of the week specified by t.
    pub fn weekday(&self) -> Arc<Mutex<Option<Weekday>>> {
        absDays::weekday(&(*absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))
    }

    /// ISOWeek returns the ISO 8601 year and week number in which t occurs.
    /// Week ranges from 1 to 53. Jan 01 to Jan 03 of year n might belong to
    /// week 52 or 53 of year n-1, and Dec 29 to Dec 31 might belong to week 1
    /// of year n+1.
    pub fn i_s_o_week(&self) -> (i32, i32) {
    let mut year: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut week: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

                // According to the rule that the first calendar week of a calendar year is
                // the week including the first Thursday of that year, and that the last one is
                // the week immediately preceding the first calendar week of the next calendar year.
                // See https://www.iso.org/obp/ui#iso:std:iso:8601:-1:ed-1:v1:en:term:3.1.1.23 for details.
                // weeks start with Monday
                // Monday Tuesday Wednesday Thursday Friday Saturday Sunday
                // 1      2       3         4        5      6        7
                // +3     +2      +1        0        -1     -2       -3
                // the offset to Thursday
        let mut days = absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap()));
        let mut thu = Arc::new(Mutex::new(Some(absDays(Arc::new(Mutex::new(Some(((*{ let __v = (*days.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + (THURSDAY as i32 - (((*(*({ let __tmp_x = (*days.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = absDays(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x - __tmp_y }).weekday().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))) as u64))))))));
        let (__tmp_0, mut yday) = absDays::year_yday(&(*thu.lock().unwrap().as_ref().unwrap())); *year.lock().unwrap() = Some(__tmp_0);;
        return ({ let __v = (*year.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __tmp_x = ({ let __tmp_x = yday; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 7; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y });
    }

    /// Clock returns the hour, minute, and second within the day specified by t.
    pub fn clock(&self) -> (i32, i32, i32) {
    let mut hour: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut min: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut sec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        absSeconds::clock(&(*self.abs_sec().lock().unwrap().as_ref().unwrap()))
    }

    /// Hour returns the hour within the day specified by t, in the range [0, 23].
    pub fn hour(&self) -> i32 {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((((*(*self.abs_sec().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) % SECONDS_PER_DAY as u64)) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3600; __tmp_x / __tmp_y };
    }

    /// Minute returns the minute offset within the hour specified by t, in the range [0, 59].
    pub fn minute(&self) -> i32 {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((((*(*self.abs_sec().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) % SECONDS_PER_HOUR as u64)) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 60; __tmp_x / __tmp_y };
    }

    /// Second returns the second offset within the minute specified by t, in the range [0, 59].
    pub fn second(&self) -> i32 {
        (*Arc::new(Mutex::new(Some((((*(*self.abs_sec().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) % SECONDS_PER_MINUTE as u64)) as i32))).lock().unwrap().as_ref().unwrap())
    }

    /// Nanosecond returns the nanosecond offset within the second specified by t,
    /// in the range [0, 999999999].
    pub fn nanosecond(&self) -> i32 {
        (*Arc::new(Mutex::new(Some(self.nsec() as i32))).lock().unwrap().as_ref().unwrap())
    }

    /// YearDay returns the day of the year specified by t, in the range [1,365] for non-leap years,
    /// and [1,366] in leap years.
    pub fn year_day(&self) -> i32 {
        let (_, mut yday) = absDays::year_yday(&(*absSeconds::days(&(*self.abs_sec().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
        yday
    }

    /// Add returns the time t+d.
    pub fn add(&self, d: Arc<Mutex<Option<Duration>>>) -> Arc<Mutex<Option<Time>>> {
        let mut __self = self.clone();
        let mut dsec = Arc::new(Mutex::new(Some(({ let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(1e9 as i64)))); __tmp_x / __tmp_y }).as_nanos() as i64)));
        let mut nsec = Arc::new(Mutex::new(Some({ let __tmp_x = __self.nsec(); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(1e9 as i64)))); __tmp_x % __tmp_y }).as_nanos() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as i32; __tmp_x >= __tmp_y } {
        { let mut guard = dsec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = 1e9 as i32; let mut guard = nsec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    } else if { let __tmp_x = { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let mut guard = dsec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __rhs = 1e9 as i32; let mut guard = nsec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let new_val = { let __tmp_x = { let __tmp_x = (*__self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = NSEC_MASK as u64; __tmp_x & ! __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*nsec.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }; *__self.wall.lock().unwrap() = Some(new_val); };
        __self.add_sec(Arc::new(Mutex::new(Some({ let __arg_holder = dsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __tmp_x = (*__self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        let mut te = Arc::new(Mutex::new(Some({ let __tmp_x = (*__self.ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*te.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.ext.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } || { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*te.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.ext.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // Monotonic clock reading now out of range; degrade to wall-only.
        __self.strip_mono();
    } else {
        { let new_val = te.lock().unwrap().as_ref().unwrap().clone(); *__self.ext.lock().unwrap() = Some(new_val); };
    }
    }
                // Monotonic clock reading now out of range; degrade to wall-only.
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// Sub returns the duration t-u. If the result exceeds the maximum (or minimum)
    /// value that can be stored in a [Duration], the maximum (or minimum) duration
    /// will be returned.
    /// To compute t-d for a duration d, use t.Add(-d).
    pub fn sub(&self, u: Arc<Mutex<Option<Time>>>) -> Arc<Mutex<Option<Duration>>> {
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).wall.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return sub_mono(Arc::new(Mutex::new(Some({ let __selector_holder = self.ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = std::time::Duration::from_nanos({ let __tmp_x = self.sec(); let __tmp_y = (*u.lock().unwrap().as_ref().unwrap()).sec(); __tmp_x - __tmp_y } as u64); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x * __tmp_y }; let __tmp_y = std::time::Duration::from_nanos({ let __tmp_x = self.nsec(); let __tmp_y = (*u.lock().unwrap().as_ref().unwrap()).nsec(); __tmp_x - __tmp_y } as u64); __tmp_x + __tmp_y })));
                // Check for overflow or underflow.
        if { let __recv = (*u.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).equal(Arc::new(Mutex::new(Some(self.clone())))); __result } {
            return { let __owned = d.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        } else if self.before(Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MIN_DURATION as u64))));
        } else {
            return Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MAX_DURATION as u64))));
        }
    }

    /// AddDate returns the time corresponding to adding the
    /// given number of years, months, and days to t.
    /// For example, AddDate(-1, 2, 3) applied to January 1, 2011
    /// returns March 4, 2010.
    ///
    /// Note that dates are fundamentally coupled to timezones, and calendrical
    /// periods like days don't have fixed durations. AddDate uses the Location of
    /// the Time value to determine these durations. That means that the same
    /// AddDate arguments can produce a different shift in absolute time depending on
    /// the base Time value and its Location. For example, AddDate(0, 0, 1) applied
    /// to 12:00 on March 27 always returns 12:00 on March 28. At some locations and
    /// in some years this is a 24 hour shift. In others it's a 23 hour shift due to
    /// daylight savings time transitions.
    ///
    /// AddDate normalizes its result in the same way that Date does,
    /// so, for example, adding one month to October 31 yields
    /// December 1, the normalized form for November 31.
    pub fn add_date(&self, years: Arc<Mutex<Option<i32>>>, months: Arc<Mutex<Option<i32>>>, days: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Time>>> {
        let (mut year, mut month, mut day) = self.date();
        let (mut hour, mut min, mut sec) = self.clock();
        return date(Arc::new(Mutex::new(Some({ let __tmp_x = year; let __tmp_y = { let __v = (*years.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(Month(Arc::new(Mutex::new(Some(((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + (*months.lock().unwrap().as_ref().unwrap()) as i32)))))))), Arc::new(Mutex::new(Some({ let __tmp_x = day; let __tmp_y = { let __v = (*days.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(hour))), Arc::new(Mutex::new(Some(min))), Arc::new(Mutex::new(Some(sec))), Arc::new(Mutex::new(Some(self.nsec() as i32))), self.location());
    }

    /// UTC returns t with the location set to UTC.
    pub fn u_t_c(&self) -> Arc<Mutex<Option<Time>>> {
        let mut __self = self.clone();
        __self.set_loc(utcLoc.clone());
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// Local returns t with the location set to local time.
    pub fn local(&self) -> Arc<Mutex<Option<Time>>> {
        let mut __self = self.clone();
        __self.set_loc({ let __arg_holder = Local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// In returns a copy of t representing the same time instant, but
    /// with the copy's location information set to loc for display
    /// purposes.
    ///
    /// In panics if loc is nil.
    pub fn r#in(&self, loc: Arc<Mutex<Option<Location>>>) -> Arc<Mutex<Option<Time>>> {
        let mut __self = self.clone();
        if { let __nil_result = (*loc.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("time: missing Location in call to Time.In".to_string()) as Box<dyn Any + Send + Sync>);
    }
        __self.set_loc(loc.clone());
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// Location returns the time zone information associated with t.
    pub fn location(&self) -> Arc<Mutex<Option<crate::zoneinfo::Location>>> {
        let mut l = self.loc.clone();
        if { let __nil_result = (*l.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = (*UTC.lock().unwrap().as_ref().unwrap()).clone(); l = new_val; };
    }
        return l.clone();
    }

    /// Zone computes the time zone in effect at time t, returning the abbreviated
    /// name of the zone (such as "CET") and its offset in seconds east of UTC.
    pub fn zone(&self) -> (Arc<Mutex<Option<String>>>, i32) {
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut offset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3, __tmp_4) = (*self.loc.lock().unwrap().as_mut().unwrap()).lookup(Arc::new(Mutex::new(Some(self.unix_sec())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_tmp_0; *offset.lock().unwrap() = Some(__tmp_1); };
        return (name.clone(), (*offset.lock().unwrap().as_ref().unwrap()));
    }

    /// ZoneBounds returns the bounds of the time zone in effect at time t.
    /// The zone begins at start and the next zone begins at end.
    /// If the zone begins at the beginning of time, start will be returned as a zero Time.
    /// If the zone goes on forever, end will be returned as a zero Time.
    /// The Location of the returned times will be the same as t.
    pub fn zone_bounds(&self) -> (Arc<Mutex<Option<Time>>>, Arc<Mutex<Option<Time>>>) {
    let mut start: Arc<Mutex<Option<Time>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut end: Arc<Mutex<Option<Time>>> = Arc::new(Mutex::new(Some(Default::default())));

        let (_, _, mut startSec, mut endSec, _) = (*self.loc.lock().unwrap().as_mut().unwrap()).lookup(Arc::new(Mutex::new(Some(self.unix_sec()))));
        if { let __tmp_x = startSec; let __tmp_y = ALPHA as i64; __tmp_x != __tmp_y } {
        { let new_val = unix_time(Arc::new(Mutex::new(Some(startSec))), Arc::new(Mutex::new(Some(0 as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *start.lock().unwrap() = __moved_val; };
        (*start.lock().unwrap().as_mut().unwrap()).set_loc({ let __field = self.loc.clone(); __field });
    }
        if { let __tmp_x = endSec; let __tmp_y = OMEGA as i64; __tmp_x != __tmp_y } {
        { let new_val = unix_time(Arc::new(Mutex::new(Some(endSec))), Arc::new(Mutex::new(Some(0 as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *end.lock().unwrap() = __moved_val; };
        (*end.lock().unwrap().as_mut().unwrap()).set_loc({ let __field = self.loc.clone(); __field });
    }
        (start.clone(), end.clone())
    }

    /// Unix returns t as a Unix time, the number of seconds elapsed
    /// since January 1, 1970 UTC. The result does not depend on the
    /// location associated with t.
    /// Unix-like operating systems often record time as a 32-bit
    /// count of seconds, but since the method here returns a 64-bit
    /// value it is valid for billions of years into the past or future.
    pub fn unix(&self) -> i64 {
        self.unix_sec()
    }

    /// UnixMilli returns t as a Unix time, the number of milliseconds elapsed since
    /// January 1, 1970 UTC. The result is undefined if the Unix time in
    /// milliseconds cannot be represented by an int64 (a date more than 292 million
    /// years before or after 1970). The result does not depend on the
    /// location associated with t.
    pub fn unix_milli(&self) -> i64 {
        return { let __tmp_x = { let __tmp_x = self.unix_sec(); let __tmp_y = 1e3 as i64; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(self.nsec() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e6 as i64; __tmp_x / __tmp_y }; __tmp_x + __tmp_y };
    }

    /// UnixMicro returns t as a Unix time, the number of microseconds elapsed since
    /// January 1, 1970 UTC. The result is undefined if the Unix time in
    /// microseconds cannot be represented by an int64 (a date before year -290307 or
    /// after year 294246). The result does not depend on the location associated
    /// with t.
    pub fn unix_micro(&self) -> i64 {
        return { let __tmp_x = { let __tmp_x = self.unix_sec(); let __tmp_y = 1e6 as i64; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(self.nsec() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e3 as i64; __tmp_x / __tmp_y }; __tmp_x + __tmp_y };
    }

    /// UnixNano returns t as a Unix time, the number of nanoseconds elapsed
    /// since January 1, 1970 UTC. The result is undefined if the Unix time
    /// in nanoseconds cannot be represented by an int64 (a date before the year
    /// 1678 or after 2262). Note that this means the result of calling UnixNano
    /// on the zero Time is undefined. The result does not depend on the
    /// location associated with t.
    pub fn unix_nano(&self) -> i64 {
        return { let __tmp_x = { let __tmp_x = (self.unix_sec()); let __tmp_y = 1e9 as i64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(self.nsec() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
    }

    /// AppendBinary implements the [encoding.BinaryAppender] interface.
    pub fn append_binary(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut offsetMin: Arc<Mutex<Option<i16>>> = Arc::new(Mutex::new(Some(0)));
        let mut offsetSec: Arc<Mutex<Option<i8>>> = Arc::new(Mutex::new(Some(0)));
        let mut version = Arc::new(Mutex::new(Some(TIME_BINARY_VERSION_V1)));
        if { let __left = self.location(); let __right = (*UTC.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = -1 as i16; *offsetMin.lock().unwrap() = Some(new_val); };
    } else {
        let (_, mut offset) = self.zone();
        if { let __tmp_x = { let __tmp_x = offset; let __tmp_y = 60; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let new_val = TIME_BINARY_VERSION_V2 as u8; *version.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = offset; let __tmp_y = 60; __tmp_x % __tmp_y }) as i8))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *offsetSec.lock().unwrap() = __moved_val; };
    }
        { let __rhs = 60; offset = offset / __rhs; };
        if { let __tmp_x = offset; let __tmp_y = -32768; __tmp_x < __tmp_y } || { let __tmp_x = offset; let __tmp_y = -1; __tmp_x == __tmp_y } || { let __tmp_x = offset; let __tmp_y = 32767; __tmp_x > __tmp_y } {
        return (b.clone(), errors::new(Arc::new(Mutex::new(Some("Time.MarshalBinary: unexpected zone offset".to_string())))));
    }
        { let new_val = Arc::new(Mutex::new(Some(offset as i16))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *offsetMin.lock().unwrap() = __moved_val; };
    }
        let mut sec = self.sec();
        let mut nsec = self.nsec();
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![(*version.lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 56; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 48; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 40; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 32; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 24; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 16; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(sec as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = nsec; let __tmp_y = 24; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = nsec; let __tmp_y = 16; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = nsec; let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(nsec as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*offsetMin.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(), (*Arc::new(Mutex::new(Some((*offsetMin.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone()]); __append_target.clone() }; b = new_val; };
                // byte 0 : version
                // bytes 1-8: seconds
                // bytes 9-12: nanoseconds
                // bytes 13-14: zone offset in minutes
        if { let __tmp_x = { let __v = (*version.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_BINARY_VERSION_V2 as u8; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some((*offsetSec.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; b = new_val; };
    }
        return (b.clone(), Arc::new(Mutex::new(None)));
    }

    /// MarshalBinary implements the [encoding.BinaryMarshaler] interface.
    pub fn marshal_binary(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let (mut b, mut err) = self.append_binary(Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity((16) as usize)))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
        return (b.clone(), Arc::new(Mutex::new(None)));
    }

    /// UnmarshalBinary implements the [encoding.BinaryUnmarshaler] interface.
    pub fn unmarshal_binary(&mut self, data: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut buf = Arc::new(Mutex::new(Some({ let __v = (*data.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return errors::new(Arc::new(Mutex::new(Some("Time.UnmarshalBinary: no data".to_string()))));
    }
        let mut version = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        if { let __tmp_x = { let __v = (*version.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_BINARY_VERSION_V1 as u8; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*version.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_BINARY_VERSION_V2 as u8; __tmp_x != __tmp_y } {
        return errors::new(Arc::new(Mutex::new(Some("Time.UnmarshalBinary: unsupported version".to_string()))));
    }
        let mut wantLen = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = 1; let __tmp_y = 8; __tmp_x + __tmp_y }; let __tmp_y = 4; __tmp_x + __tmp_y }; let __tmp_y = 2; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*version.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_BINARY_VERSION_V2 as u8; __tmp_x == __tmp_y } {
        { let mut guard = wantLen.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*wantLen.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x != __tmp_y } {
        return errors::new(Arc::new(Mutex::new(Some("Time.UnmarshalBinary: invalid length".to_string()))));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); buf = new_val; };
        let mut sec = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(7) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(6) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(5) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 16; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 24; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 40; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 48; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 56; __tmp_x << __tmp_y }; __tmp_x | __tmp_y })));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (8) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); buf = new_val; };
        let mut nsec = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 16; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 24; __tmp_x << __tmp_y }; __tmp_x | __tmp_y })));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (4) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); buf = new_val; };
        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as i16))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i16))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 60; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*version.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_BINARY_VERSION_V2 as u8; __tmp_x == __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = offset.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let new_val = Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }; *self = new_val; };
        { let new_val = Arc::new(Mutex::new(Some((*nsec.lock().unwrap().as_ref().unwrap()) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.wall.lock().unwrap() = __moved_val; };
        { let new_val = sec.lock().unwrap().as_ref().unwrap().clone(); *self.ext.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -60; __tmp_x == __tmp_y } {
        self.set_loc(utcLoc.clone());
    } else {
        let (_, mut localoff, _, _, _) = { let __recv_holder = (*Local.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).lookup(Arc::new(Mutex::new(Some(self.unix_sec())))); __result };;
        if { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = localoff; __tmp_x == __tmp_y } {
            self.set_loc({ let __arg_holder = Local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });;
        } else {
            self.set_loc(fixed_zone(Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// GobEncode implements the gob.GobEncoder interface.
    pub fn gob_encode(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.marshal_binary()
    }

    /// GobDecode implements the gob.GobDecoder interface.
    pub fn gob_decode(&mut self, data: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.unmarshal_binary(data.clone())
    }

    /// MarshalJSON implements the [encoding/json.Marshaler] interface.
    /// The time is a quoted string in the RFC 3339 format with sub-second precision.
    /// If the timestamp cannot be represented as valid RFC 3339
    /// (e.g., the year is out of range), then an error is reported.
    pub fn marshal_j_s_o_n(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut b = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = 35; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize))));
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('"' as i32) as u8); __append_target.clone() }; b = new_val; };
        let (__tmp_0, mut err) = self.append_strict_r_f_c3339(b.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *b.lock().unwrap() = __moved_tmp_0;;
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('"' as i32) as u8); __append_target.clone() }; b = new_val; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", "Time.MarshalJSON: ".to_string(), (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())))))));
    }
        return (b.clone(), Arc::new(Mutex::new(None)));
    }

    /// UnmarshalJSON implements the [encoding/json.Unmarshaler] interface.
    /// The time must be a quoted string in the RFC 3339 format.
    pub fn unmarshal_j_s_o_n(&mut self, mut data: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*data.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "null".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
                // TODO(https://go.dev/issue/47353): Properly unescape a JSON string.
        if { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x < __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = ('"' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('"' as i32) as u8; __tmp_x != __tmp_y } {
        return errors::new(Arc::new(Mutex::new(Some("Time.UnmarshalJSON: input is not a JSON string".to_string()))));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ("\"".len()) as usize; let __high = ({ let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let (__tmp_0, __tmp_1) = parse_strict_r_f_c3339(data.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        return err.clone();
    }

    pub fn append_to(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>, errPrefix: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let (__tmp_0, mut err) = self.append_strict_r_f_c3339(b.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *b.lock().unwrap() = __moved_tmp_0;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*errPrefix.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())))))));
    }
        return (b.clone(), Arc::new(Mutex::new(None)));
    }

    /// AppendText implements the [encoding.TextAppender] interface.
    /// The time is formatted in RFC 3339 format with sub-second precision.
    /// If the timestamp cannot be represented as valid RFC 3339
    /// (e.g., the year is out of range), then an error is returned.
    pub fn append_text(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.append_to(b.clone(), Arc::new(Mutex::new(Some("Time.AppendText: ".to_string()))))
    }

    /// MarshalText implements the [encoding.TextMarshaler] interface. The output
    /// matches that of calling the [Time.AppendText] method.
    ///
    /// See [Time.AppendText] for more information.
    pub fn marshal_text(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.append_to(Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity((R_F_C3339_NANO.len()) as usize)))), Arc::new(Mutex::new(Some("Time.MarshalText: ".to_string()))))
    }

    /// UnmarshalText implements the [encoding.TextUnmarshaler] interface.
    /// The time must be in the RFC 3339 format.
    pub fn unmarshal_text(&mut self, data: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let (__tmp_0, __tmp_1) = parse_strict_r_f_c3339(data.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        return err.clone();
    }

    /// IsDST reports whether the time in the configured location is in Daylight Savings Time.
    pub fn is_d_s_t(&self) -> bool {
        let (_, _, _, _, mut isDST) = (*self.loc.lock().unwrap().as_mut().unwrap()).lookup(Arc::new(Mutex::new(Some(self.unix()))));
        isDST
    }

    /// Truncate returns the result of rounding t down to a multiple of d (since the zero time).
    /// If d <= 0, Truncate returns t stripped of any monotonic clock reading but otherwise unchanged.
    ///
    /// Truncate operates on the time as an absolute duration since the
    /// zero time; it does not operate on the presentation form of the
    /// time. Thus, Truncate(Hour) may return a time with a non-zero
    /// minute, depending on the time's Location.
    pub fn truncate(&self, d: Arc<Mutex<Option<Duration>>>) -> Arc<Mutex<Option<Time>>> {
        let mut __self = self.clone();
        __self.strip_mono();
        if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        let (_, mut r) = div(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return __self.add(Arc::new(Mutex::new(Some(-((*r.lock().unwrap().as_ref().unwrap()).clone())))));
    }

    /// Round returns the result of rounding t to the nearest multiple of d (since the zero time).
    /// The rounding behavior for halfway values is to round up.
    /// If d <= 0, Round returns t stripped of any monotonic clock reading but otherwise unchanged.
    ///
    /// Round operates on the time as an absolute duration since the
    /// zero time; it does not operate on the presentation form of the
    /// time. Thus, Round(Hour) may return a time with a non-zero
    /// minute, depending on the time's Location.
    pub fn round(&self, d: Arc<Mutex<Option<Duration>>>) -> Arc<Mutex<Option<Time>>> {
        let mut __self = self.clone();
        __self.strip_mono();
        if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        let (_, mut r) = div(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if less_than_half(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return __self.add(Arc::new(Mutex::new(Some(-((*r.lock().unwrap().as_ref().unwrap()).clone())))));
    }
        return __self.add(Arc::new(Mutex::new(Some({ let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*r.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x - __tmp_y }))));
    }
}

impl Month {
    /// String returns the English name of the month ("January", "February", ...).
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = Month(Arc::new(Mutex::new(Some(JANUARY as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Month(Arc::new(Mutex::new(Some(DECEMBER as i32)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = longMonthNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*self.0.lock().unwrap().as_ref().unwrap()) - 1) as usize].clone() })));
    }
        let mut buf = Arc::new(Mutex::new(Some(vec![0; (20) as usize])));
        let mut n = fmt_int(buf.clone(), Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))));
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "%!Month(".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (n) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
    }
}

impl Weekday {
    /// String returns the English name of the day ("Sunday", "Monday", ...).
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = Weekday(Arc::new(Mutex::new(Some(SUNDAY as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Weekday(Arc::new(Mutex::new(Some(SATURDAY as i32)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = longDayNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
    }
        let mut buf = Arc::new(Mutex::new(Some(vec![0; (20) as usize])));
        let mut n = fmt_int(buf.clone(), Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))));
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "%!Weekday(".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (n) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
    }
}

impl absSeconds {
    /// days converts absolute seconds to absolute days.
    pub fn days(&self) -> Arc<Mutex<Option<absDays>>> {
        Arc::new(Mutex::new(Some(absDays(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) / SECONDS_PER_DAY as u64) as u64)))))))
    }

    /// clock returns the hour, minute, and second within the day specified by abs.
    pub fn clock(&self) -> (i32, i32, i32) {
    let mut hour: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut min: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut sec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        { let new_val = Arc::new(Mutex::new(Some((((*self.0.lock().unwrap().as_ref().unwrap()) % SECONDS_PER_DAY as u64)) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *sec.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3600; __tmp_x / __tmp_y }; *hour.lock().unwrap() = Some(new_val); };
        { let __rhs = { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3600; __tmp_x * __tmp_y }; let mut guard = sec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let new_val = { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x / __tmp_y }; *min.lock().unwrap() = Some(new_val); };
        { let __rhs = { let __tmp_x = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x * __tmp_y }; let mut guard = sec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        return ((*hour.lock().unwrap().as_ref().unwrap()), (*min.lock().unwrap().as_ref().unwrap()), (*sec.lock().unwrap().as_ref().unwrap()));
    }
}

impl absDays {
    /// split splits days into century, cyear, ayday.
    pub fn split(&self) -> (Arc<Mutex<Option<absCentury>>>, Arc<Mutex<Option<absCyear>>>, Arc<Mutex<Option<absYday>>>) {
    let mut century: Arc<Mutex<Option<absCentury>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut cyear: Arc<Mutex<Option<absCyear>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut ayday: Arc<Mutex<Option<absYday>>> = Arc::new(Mutex::new(Some(Default::default())));

                // See “Computations on Times” comment above.
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 4 as u64; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = 3 as u64; __tmp_x + __tmp_y })));
        { let new_val = absCentury(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 146097 as u64; __tmp_x / __tmp_y } as u64)))); *century.lock().unwrap() = Some(new_val); };
                // This should be
                //	cday := uint32(d % 146097) / 4
                //	cd := 4*cday + 3
                // which is to say
                //	cday := uint32(d % 146097) >> 2
                //	cd := cday<<2 + 3
                // but of course (x>>2<<2)+3 == x|3,
                // so do that instead.
        let mut cd = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 146097 as u64; __tmp_x % __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as u32; __tmp_x | __tmp_y })));
                // For cdays in the range [0,146097] (100 years), we want:
                //
                //	cyear := (4 cdays + 3) / 1461
                //	yday := (4 cdays + 3) % 1461 / 4
                //
                // (See the “Computations on Times” comment above
                // as well as Neri and Schneider, section 7.)
                //
                // That is equivalent to:
                //
                //	cyear := (2939745 cdays) >> 32
                //	yday := (2939745 cdays) & 0xFFFFFFFF / 2939745 / 4
                //
                // so do that instead, saving a few cycles.
                // See Neri and Schneider, section 8.3
                // for more about this optimization.
        let (mut hi, mut lo) = math_bits::mul32(Arc::new(Mutex::new(Some(2939745 as u32))), Arc::new(Mutex::new(Some((*cd.lock().unwrap().as_ref().unwrap()) as u32))));
        { let new_val = absCyear(Arc::new(Mutex::new(Some(hi as i32)))); *cyear.lock().unwrap() = Some(new_val); };
        { let new_val = absYday(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = lo; let __tmp_y = 2939745 as u32; __tmp_x / __tmp_y }; let __tmp_y = 4 as u32; __tmp_x / __tmp_y } as i32)))); *ayday.lock().unwrap() = Some(new_val); };
        (century.clone(), cyear.clone(), ayday.clone())
    }

    /// date converts days into standard year, month, day.
    pub fn date(&self) -> (i32, Arc<Mutex<Option<Month>>>, i32) {
    let mut year: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut month: Arc<Mutex<Option<Month>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut day: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        let (mut century, mut cyear, mut ayday) = absDays::split(self);
        let (mut amonth, __tmp_1) = absYday::split(&(*ayday.lock().unwrap().as_ref().unwrap())); *day.lock().unwrap() = Some(__tmp_1);;
        let mut janFeb = absYday::jan_feb(&(*ayday.lock().unwrap().as_ref().unwrap()));
        { let new_val = absCentury::year(&(*century.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = cyear.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = janFeb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *year.lock().unwrap() = Some(new_val); };
        { let new_val = absMonth::month(&(*amonth.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = janFeb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *month.lock().unwrap() = __moved_val; };
        return ((*year.lock().unwrap().as_ref().unwrap()), month.clone(), (*day.lock().unwrap().as_ref().unwrap()));
    }

    /// yearYday converts days into the standard year and 1-based yday.
    pub fn year_yday(&self) -> (i32, i32) {
    let mut year: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut yday: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        let (mut century, mut cyear, mut ayday) = absDays::split(self);
        let mut janFeb = absYday::jan_feb(&(*ayday.lock().unwrap().as_ref().unwrap()));
        { let new_val = absCentury::year(&(*century.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = cyear.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = janFeb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *year.lock().unwrap() = Some(new_val); };
        { let new_val = absYday::yday(&(*ayday.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = janFeb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), absCentury::leap(&(*century.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = cyear.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); *yday.lock().unwrap() = Some(new_val); };
        return ((*year.lock().unwrap().as_ref().unwrap()), (*yday.lock().unwrap().as_ref().unwrap()));
    }

    /// weekday returns the day of the week specified by days.
    pub fn weekday(&self) -> Arc<Mutex<Option<Weekday>>> {
                // March 1 of the absolute year, like March 1 of 2000, was a Wednesday.
        Arc::new(Mutex::new(Some(Weekday(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(WEDNESDAY as i32 as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x + __tmp_y }); let __tmp_y = 7 as u64; __tmp_x % __tmp_y } as i32)))))))
    }
}

impl absYday {
    /// split splits ayday into absolute month and standard (1-based) day-in-month.
    pub fn split(&self) -> (Arc<Mutex<Option<absMonth>>>, i32) {
    let mut m: Arc<Mutex<Option<absMonth>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut mday: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

                // See “Computations on Times” comment above.
                //
                // For yday in the range [0,366],
                //
                //	amonth := (5 yday + 461) / 153
                //	mday := (5 yday + 461) % 153 / 5
                //
                // is equivalent to:
                //
                //	amonth = (2141 yday + 197913) >> 16
                //	mday = (2141 yday + 197913) & 0xFFFF / 2141
                //
                // so do that instead, saving a few cycles.
                // See Neri and Schneider, section 8.3.
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2141 as u32; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = 197913 as u32; __tmp_x + __tmp_y })));
        return (Arc::new(Mutex::new(Some(absMonth(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y } as i32))))))), { let __tmp_x = 1; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFFFF as u32; __tmp_x & __tmp_y }); let __tmp_y = 2141 as u32; __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y });
    }

    /// janFeb returns 1 if the March 1-based ayday is in January or February, 0 otherwise.
    pub fn jan_feb(&self) -> Arc<Mutex<Option<absJanFeb>>> {
                // See “Computations on Times” comment above.
        let mut jf = Arc::new(Mutex::new(Some(absJanFeb(Arc::new(Mutex::new(Some(0 as i32)))))));
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = absYday(Arc::new(Mutex::new(Some(MARCH_THRU_DECEMBER as i32)))); __tmp_x >= __tmp_y } {
        { let new_val = absJanFeb(Arc::new(Mutex::new(Some(1 as i32)))); *jf.lock().unwrap() = Some(new_val); };
    }
        return { let __owned = jf.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// yday returns the standard 1-based yday for (ayday, janFeb, leap).
    pub fn yday(&self, janFeb: Arc<Mutex<Option<absJanFeb>>>, leap: Arc<Mutex<Option<absLeap>>>) -> i32 {
                // See “Computations on Times” comment above.
        return { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 60; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*leap.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*janFeb.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x & ! __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = 365; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*janFeb.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x - __tmp_y };
    }
}

impl absMonth {
    /// month returns the standard Month for (m, janFeb)
    pub fn month(&self, janFeb: Arc<Mutex<Option<absJanFeb>>>) -> Arc<Mutex<Option<Month>>> {
                // See “Computations on Times” comment above.
        return Arc::new(Mutex::new(Some(Month(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) as i32 - ((*{ let __v = (*janFeb.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32 * 12)))))))));
    }
}

impl absCentury {
    /// leap returns 1 if (century, cyear) is a leap year, 0 otherwise.
    pub fn leap(&self, cyear: Arc<Mutex<Option<absCyear>>>) -> Arc<Mutex<Option<absLeap>>> {
                // See “Computations on Times” comment above.
        let mut y4ok = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __tmp_x = (*cyear.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = absCyear(Arc::new(Mutex::new(Some(4 as i32)))); __tmp_x % __tmp_y }; let __tmp_y = absCyear(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } {
        { let new_val = 1; *y4ok.lock().unwrap() = Some(new_val); };
    }
        let mut y100ok = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = (*cyear.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = absCyear(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        { let new_val = 1; *y100ok.lock().unwrap() = Some(new_val); };
    }
        let mut y400ok = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = absCentury(Arc::new(Mutex::new(Some(4 as u64)))); __tmp_x % __tmp_y }; let __tmp_y = absCentury(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let new_val = 1; *y400ok.lock().unwrap() = Some(new_val); };
    }
        return Arc::new(Mutex::new(Some(absLeap(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y4ok.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*y100ok.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y400ok.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }); __tmp_x & __tmp_y } as i32)))))));
    }

    /// year returns the standard year for (century, cyear, janFeb).
    pub fn year(&self, cyear: Arc<Mutex<Option<absCyear>>>, janFeb: Arc<Mutex<Option<absJanFeb>>>) -> i32 {
                // See “Computations on Times” comment above.
        return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 100 as u64; __tmp_x * __tmp_y }; let __tmp_y = ABSOLUTE_YEARS as u64; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*cyear.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*janFeb.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
    }
}

impl Duration {
    /// String returns a string representing the duration in the form "72h3m0.5s".
    /// Leading zero units are omitted. As a special case, durations less than one
    /// second format use a smaller unit (milli-, micro-, or nanoseconds) to ensure
    /// that the leading digit is non-zero. The zero duration formats as 0s.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
                // This is inlinable to take advantage of "function outlining".
                // Thus, the caller can decide whether a string must be heap allocated.
        let mut arr: Arc<Mutex<Option<[u8; 32]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let mut n = Duration::format(self, arr.clone());
        return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (n) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    /// format formats the representation of d into the end of buf and
    /// returns the offset of the first character.
    pub fn format(&self, buf: Arc<Mutex<Option<[u8; 32]>>>) -> i32 {
                // Largest time is 2540400h10m10.000000000s
        let mut w = Arc::new(Mutex::new(Some(32 as i32)));
        let mut u = Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap())).as_nanos() as u64)));
        let mut neg = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x < __tmp_y })));
        if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = ((*u.lock().unwrap().as_ref().unwrap())).wrapping_neg(); *u.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((SECOND).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x < __tmp_y } {
                // Special case: if duration is smaller than a second,
                // use smaller units, like 1.2ms
        let mut prec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('s' as i32) as u8;
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
            (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('0' as i32) as u8;
            return { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x < __tmp_y } {
                        // print nanoseconds
            { let new_val = 0; *prec.lock().unwrap() = Some(new_val); };
            (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('n' as i32) as u8;
        } else if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((MILLISECOND).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x < __tmp_y } {
                        // print microseconds
            { let new_val = 3; *prec.lock().unwrap() = Some(new_val); };
                        // U+00B5 'µ' micro sign == 0xC2 0xB5
            { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
            { let _dst_start = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (*buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = "\u{b5}".to_string().as_bytes().to_vec(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        } else {
                        // print milliseconds
            { let new_val = 6; *prec.lock().unwrap() = Some(new_val); };
            (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('m' as i32) as u8;
        }
                // print nanoseconds
                // print microseconds
                // U+00B5 'µ' micro sign == 0xC2 0xB5
                // Need room for two bytes.
                // print milliseconds
        { let (__tmp_0, __tmp_1) = fmt_frac(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *w.lock().unwrap() = Some(__tmp_0); *u.lock().unwrap() = Some(__tmp_1); };
        { let new_val = fmt_int(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *w.lock().unwrap() = Some(new_val); };
    } else {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('s' as i32) as u8;
        { let (__tmp_0, __tmp_1) = fmt_frac(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(9)))); *w.lock().unwrap() = Some(__tmp_0); *u.lock().unwrap() = Some(__tmp_1); };
                // u is now integer seconds
        { let new_val = fmt_int(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60 as u64; __tmp_x % __tmp_y })))); *w.lock().unwrap() = Some(new_val); };
        { let __rhs = 60 as u64; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
                // u is now integer minutes
        if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('m' as i32) as u8;
        { let new_val = fmt_int(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60 as u64; __tmp_x % __tmp_y })))); *w.lock().unwrap() = Some(new_val); };
        { let __rhs = 60 as u64; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
                // u is now integer hours
                // Stop at hours because days can be different lengths.
        if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('h' as i32) as u8;
        { let new_val = fmt_int(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *w.lock().unwrap() = Some(new_val); };
    }
    }
    }
                // Special case: if duration is smaller than a second,
                // use smaller units, like 1.2ms
                // print nanoseconds
                // print microseconds
                // U+00B5 'µ' micro sign == 0xC2 0xB5
                // Need room for two bytes.
                // print milliseconds
                // u is now integer seconds
                // u is now integer minutes
                // u is now integer hours
                // Stop at hours because days can be different lengths.
        if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('-' as i32) as u8;
    }
        return { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// Nanoseconds returns the duration as an integer nanosecond count.
    pub fn nanoseconds(&self) -> i64 {
        (*Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap())).as_nanos() as i64))).lock().unwrap().as_ref().unwrap())
    }

    /// Microseconds returns the duration as an integer microsecond count.
    pub fn microseconds(&self) -> i64 {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap())).as_nanos() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e3 as i64; __tmp_x / __tmp_y };
    }

    /// Milliseconds returns the duration as an integer millisecond count.
    pub fn milliseconds(&self) -> i64 {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap())).as_nanos() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e6 as i64; __tmp_x / __tmp_y };
    }

    /// Seconds returns the duration as a floating point number of seconds.
    pub fn seconds(&self) -> f64 {
        let mut sec = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x / __tmp_y })));
        let mut nsec = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x % __tmp_y })));
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e+09; __tmp_x / __tmp_y }; __tmp_x + __tmp_y };
    }

    /// Minutes returns the duration as a floating point number of minutes.
    pub fn minutes(&self) -> f64 {
        let mut min = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(MINUTE as i64)))); __tmp_x / __tmp_y })));
        let mut nsec = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(MINUTE as i64)))); __tmp_x % __tmp_y })));
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 6e+10; __tmp_x / __tmp_y }; __tmp_x + __tmp_y };
    }

    /// Hours returns the duration as a floating point number of hours.
    pub fn hours(&self) -> f64 {
        let mut hour = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(HOUR as i64)))); __tmp_x / __tmp_y })));
        let mut nsec = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(HOUR as i64)))); __tmp_x % __tmp_y })));
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3.6e+12; __tmp_x / __tmp_y }; __tmp_x + __tmp_y };
    }

    /// Truncate returns the result of rounding d toward zero to a multiple of m.
    /// If m <= 0, Truncate returns d unchanged.
    pub fn truncate(&self, m: Arc<Mutex<Option<Duration>>>) -> Arc<Mutex<Option<Duration>>> {
        if { let __tmp_x = (*m.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        return Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*m.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x % __tmp_y }; __tmp_x - __tmp_y })));
    }

    /// Round returns the result of rounding d to the nearest multiple of m.
    /// The rounding behavior for halfway values is to round away from zero.
    /// If the result exceeds the maximum (or minimum)
    /// value that can be stored in a [Duration],
    /// Round returns the maximum (or minimum) duration.
    /// If m <= 0, Round returns d unchanged.
    pub fn round(&self, m: Arc<Mutex<Option<Duration>>>) -> Arc<Mutex<Option<Duration>>> {
        if { let __tmp_x = (*m.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        let mut r = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*m.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x % __tmp_y })));
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x < __tmp_y } {
        { let new_val = -((*r.lock().unwrap().as_ref().unwrap()).clone()); *r.lock().unwrap() = Some(new_val); };
        if less_than_half(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*r.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x + __tmp_y })));
    }
        {
        let mut d1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*m.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x - __tmp_y }; let __tmp_y = (*r.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x + __tmp_y })));;
        if { let __tmp_x = (*d1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
            return { let __owned = d1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
        return Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MIN_DURATION as u64))));
    }
                // overflow
        if less_than_half(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some({ let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*r.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x - __tmp_y })));
    }
        {
        let mut d1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*m.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x + __tmp_y }; let __tmp_y = (*r.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x - __tmp_y })));;
        if { let __tmp_x = (*d1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y } {
            return { let __owned = d1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
        Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MAX_DURATION as u64))))
    }

    /// Abs returns the absolute value of d.
    /// As a special case, Duration([math.MinInt64]) is converted to Duration([math.MaxInt64]),
    /// reducing its magnitude by 1 nanosecond.
    pub fn abs(&self) -> Arc<Mutex<Option<Duration>>> {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x >= __tmp_y } {
            return Arc::new(Mutex::new(Some(self.clone())));
        } else if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(MIN_DURATION as i64)))); __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MAX_DURATION as u64))));
        } else {
            return Arc::new(Mutex::new(Some(-((*self.0.lock().unwrap().as_ref().unwrap()).clone()))));
        }
    }
}

/// dateToAbsDays takes a standard year/month/day and returns the
/// number of days from the absolute epoch to that day.
/// The days argument can be out of range and in particular can be negative.
pub fn date_to_abs_days(year: Arc<Mutex<Option<i64>>>, month: Arc<Mutex<Option<Month>>>, day: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<absDays>>> {
        // See “Computations on Times” comment above.
    let mut amonth = Arc::new(Mutex::new(Some((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32)));
    let mut janFeb = Arc::new(Mutex::new(Some(0 as u32)));
    if { let __tmp_x = { let __v = (*amonth.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as u32; __tmp_x < __tmp_y } {
        { let new_val = 1 as u32; *janFeb.lock().unwrap() = Some(new_val); };
    }
    { let __rhs = { let __tmp_x = 12 as u32; let __tmp_y = { let __v = (*janFeb.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let mut guard = amonth.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut y = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*year.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*janFeb.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = ABSOLUTE_YEARS as u64; __tmp_x + __tmp_y })));

        // For amonth is in the range [3,14], we want:
        //
        //	ayday := (153*amonth - 457) / 5
        //
        // (See the “Computations on Times” comment above
        // as well as Neri and Schneider, section 7.)
        //
        // That is equivalent to:
        //
        //	ayday := (979*amonth - 2919) >> 5
        //
        // and the latter form uses a couple fewer instructions,
        // so use it, saving a few cycles.
        // See Neri and Schneider, section 8.3
        // for more about this optimization.
        //
        // (Note that there is no saved division, because the compiler
        // implements / 5 without division in all cases.)
    let mut ayday = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = 979 as u32; let __tmp_y = { let __v = (*amonth.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 2919 as u32; __tmp_x - __tmp_y }); let __tmp_y = 5; __tmp_x >> __tmp_y })));

    let mut century = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x / __tmp_y })));
    let mut cyear = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x % __tmp_y }) as u32)));
    let mut cday = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1461 as u32; let __tmp_y = { let __v = (*cyear.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 4 as u32; __tmp_x / __tmp_y })));
    let mut centurydays = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 146097 as u64; let __tmp_y = { let __v = (*century.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 4 as u64; __tmp_x / __tmp_y })));

    return Arc::new(Mutex::new(Some(absDays(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*centurydays.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*cday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ayday.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*day.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y } as u64)))))));
}

/// fmtFrac formats the fraction of v/10**prec (e.g., ".12345") into the
/// tail of buf, omitting trailing zeros. It omits the decimal
/// point too when the fraction is 0. It returns the index where the
/// output bytes begin and the value v/10**prec.
pub fn fmt_frac(buf: Arc<Mutex<Option<Vec<u8>>>>, mut v: Arc<Mutex<Option<u64>>>, prec: Arc<Mutex<Option<i32>>>) -> (i32, u64) {
    let mut nw: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut nv: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        // Omit trailing zeros up to and including decimal point.
    let mut w = Arc::new(Mutex::new(Some((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    let mut print = Arc::new(Mutex::new(Some(false)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut digit = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x % __tmp_y })));
        { let new_val = { let __v = (*print.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*digit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y }; *print.lock().unwrap() = Some(new_val); };
        if { let __v = (*print.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = (*Arc::new(Mutex::new(Some((*digit.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ('0' as i32) as u8; __tmp_x + __tmp_y };
    }
        { let __rhs = 10 as u64; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __v = (*print.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('.' as i32) as u8;
    }
    return ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// fmtInt formats v into the tail of buf.
/// It returns the index where the output begins.
pub fn fmt_int(buf: Arc<Mutex<Option<Vec<u8>>>>, mut v: Arc<Mutex<Option<u64>>>) -> i32 {
    let mut w = Arc::new(Mutex::new(Some((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('0' as i32) as u8;
    } else {
        while { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x % __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ('0' as i32) as u8; __tmp_x + __tmp_y };
        { let __rhs = 10 as u64; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    }
    }
    return { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// lessThanHalf reports whether x+x < y but avoids overflow,
/// assuming x and y are both positive (Duration is signed).
pub fn less_than_half(x: Arc<Mutex<Option<Duration>>>, y: Arc<Mutex<Option<Duration>>>) -> bool {
    return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
}

pub fn sub_mono(t: Arc<Mutex<Option<i64>>>, u: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Duration>>> {
    let mut d = Arc::new(Mutex::new(Some(std::time::Duration::from_nanos({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y } as u64))));
    if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MAX_DURATION as u64))));
    }
        // t - u is positive out of range
    if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(std::time::Duration::from_nanos(MIN_DURATION as u64))));
    }
        // t - u is negative out of range
    return { let __owned = d.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// Until returns the duration until t.
/// It is shorthand for t.Sub(time.Now()).
pub fn until(t: Arc<Mutex<Option<Time>>>) -> Arc<Mutex<Option<Duration>>> {
    if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).wall.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // Common case optimization: if t has monotonic time, then Sub will use only it.
        return sub_mono(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = runtime_nano(); let __tmp_y = (*startNano.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))));
    }
        // Common case optimization: if t has monotonic time, then Sub will use only it.
    (*t.lock().unwrap().as_ref().unwrap()).sub(now())
}

/// daysBefore returns the number of days in a non-leap year before month m.
/// daysBefore(December+1) returns 365.
pub fn days_before(m: Arc<Mutex<Option<Month>>>) -> i32 {
    let mut adj = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*m.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Month(Arc::new(Mutex::new(Some(MARCH as i32)))); __tmp_x >= __tmp_y } {
        { let new_val = -2; *adj.lock().unwrap() = Some(new_val); };
    }

        // With the -2 adjustment after February,
        // we need to compute the running sum of:
        //	0  31  30  31  30  31  30  31  31  30  31  30  31
        // which is:
        //	0  31  61  92 122 153 183 214 245 275 306 336 367
        // This is almost exactly 367/12×(m-1) except for the
        // occasonal off-by-one suggesting there may be an
        // integer approximation of the form (a×m + b)/c.
        // A brute force search over small a, b, c finds that
        // (214×m - 211) / 7 computes the function perfectly.
    return { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = 214; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = 211; __tmp_x - __tmp_y }); let __tmp_y = 7; __tmp_x / __tmp_y }; let __tmp_y = { let __v = (*adj.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
}

pub fn days_in(m: Arc<Mutex<Option<Month>>>, year: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = (*m.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Month(Arc::new(Mutex::new(Some(FEBRUARY as i32)))); __tmp_x == __tmp_y } {
        if is_leap(Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return 29;
    }
        return 28;
    }

        // With the special case of February eliminated, the pattern is
        //	31 30 31 30 31 30 31 31 30 31 30 31
        // Adding m&1 produces the basic alternation;
        // adding (m>>3)&1 inverts the alternation starting in August.
    return { let __tmp_x = 30; let __tmp_y = (*Arc::new(Mutex::new(Some((((((*{ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + ((*{ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> 3i32))) & 1)) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
}

/// runtimeNow returns the current time.
/// When called within a synctest.Run bubble, it returns the group's fake clock.
///
///go:linkname runtimeNow
pub fn runtime_now() -> (i64, i32, i64) {
    unimplemented!("Go function declaration has no body");
}


/// runtimeNano returns the current value of the runtime clock in nanoseconds.
/// When called within a synctest.Run bubble, it returns the group's fake clock.
///
///go:linkname runtimeNano
pub fn runtime_nano() -> i64 {
    unimplemented!("Go function declaration has no body");
}


/// Now returns the current local time.
pub fn now() -> Arc<Mutex<Option<Time>>> {
    let (mut sec, mut nsec, mut mono) = runtime_now();
    if { let __tmp_x = mono; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(nsec as u64))), ext: Arc::new(Mutex::new(Some({ let __tmp_x = sec; let __tmp_y = UNIX_TO_INTERNAL as i64; __tmp_x + __tmp_y }))), loc: (*Local.lock().unwrap().as_ref().unwrap()).clone(), ..Default::default() })));
    }
    { let __rhs = (*startNano.lock().unwrap().as_ref().unwrap()); mono = mono - __rhs; };
    { let __rhs = { let __tmp_x = UNIX_TO_INTERNAL as i64; let __tmp_y = MIN_WALL as i64; __tmp_x - __tmp_y } as i64; sec = sec + __rhs; };
    if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(sec as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 33; __tmp_x >> __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // Seconds field overflowed the 33 bits available when
                // storing a monotonic time. This will be true after
                // March 16, 2157.
        return Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(nsec as u64))), ext: Arc::new(Mutex::new(Some({ let __tmp_x = sec; let __tmp_y = MIN_WALL as i64; __tmp_x + __tmp_y }))), loc: (*Local.lock().unwrap().as_ref().unwrap()).clone(), ..Default::default() })));
    }
        // Seconds field overflowed the 33 bits available when
        // storing a monotonic time. This will be true after
        // March 16, 2157.
    Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = HAS_MONOTONIC as u64; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(sec as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = NSEC_SHIFT; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(nsec as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }))), ext: Arc::new(Mutex::new(Some(mono))), loc: (*Local.lock().unwrap().as_ref().unwrap()).clone(), ..Default::default() })))
}

pub fn unix_time(sec: Arc<Mutex<Option<i64>>>, nsec: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Time>>> {
    Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some((*nsec.lock().unwrap().as_ref().unwrap()) as u64))), ext: Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = UNIX_TO_INTERNAL as i64; __tmp_x + __tmp_y }))), loc: (*Local.lock().unwrap().as_ref().unwrap()).clone(), ..Default::default() })))
}

/// Unix returns the local Time corresponding to the given Unix time,
/// sec seconds and nsec nanoseconds since January 1, 1970 UTC.
/// It is valid to pass nsec outside the range [0, 999999999].
/// Not all sec values have a corresponding time value. One such
/// value is 1<<63-1 (the largest int64 value).
pub fn unix(mut sec: Arc<Mutex<Option<i64>>>, mut nsec: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Time>>> {
    if { let __tmp_x = { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as i64; __tmp_x >= __tmp_y } {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as i64; __tmp_x / __tmp_y })));
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = sec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as i64; __tmp_x * __tmp_y }; let mut guard = nsec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __v = (*nsec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let __rhs = 1e9 as i64; let mut guard = nsec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = sec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
    unix_time(Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*nsec.lock().unwrap().as_ref().unwrap()) as i32))))
}

pub fn is_leap(year: Arc<Mutex<Option<i32>>>) -> bool {
        // year%4 == 0 && (year%100 != 0 || year%400 == 0)
        // Bottom 2 bits must be clear.
        // For multiples of 25, bottom 4 bits must be clear.
        // Thanks to Cassio Neri for this trick.
    let mut mask = Arc::new(Mutex::new(Some(0xf)));
    if { let __tmp_x = { let __tmp_x = { let __v = (*year.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 25; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let new_val = 3; *mask.lock().unwrap() = Some(new_val); };
    }
    return { let __tmp_x = { let __tmp_x = { let __v = (*year.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y };
}

/// norm returns nhi, nlo such that
///
///	hi * base + lo == nhi * base + nlo
///	0 <= nlo < base
pub fn norm(mut hi: Arc<Mutex<Option<i32>>>, mut lo: Arc<Mutex<Option<i32>>>, base: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    let mut nhi: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut nlo: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = -((*lo.lock().unwrap().as_ref().unwrap())); let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = hi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let __rhs = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let mut guard = lo.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    if { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y })));
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = hi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let mut guard = lo.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
    return ({ let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// Date returns the Time corresponding to
///
///	yyyy-mm-dd hh:mm:ss + nsec nanoseconds
///
/// in the appropriate zone for that time in the given location.
///
/// The month, day, hour, min, sec, and nsec values may be outside
/// their usual ranges and will be normalized during the conversion.
/// For example, October 32 converts to November 1.
///
/// A daylight savings time transition skips or repeats times.
/// For example, in the United States, March 13, 2011 2:15am never occurred,
/// while November 6, 2011 1:15am occurred twice. In such cases, the
/// choice of time zone, and therefore the time, is not well-defined.
/// Date returns a time that is correct in one of the two zones involved
/// in the transition, but it does not guarantee which.
///
/// Date panics if loc is nil.
pub fn date(mut year: Arc<Mutex<Option<i32>>>, mut month: Arc<Mutex<Option<Month>>>, mut day: Arc<Mutex<Option<i32>>>, mut hour: Arc<Mutex<Option<i32>>>, mut min: Arc<Mutex<Option<i32>>>, mut sec: Arc<Mutex<Option<i32>>>, mut nsec: Arc<Mutex<Option<i32>>>, loc: Arc<Mutex<Option<Location>>>) -> Arc<Mutex<Option<Time>>> {
    if { let __nil_result = (*loc.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("time: missing Location in call to Date".to_string()) as Box<dyn Any + Send + Sync>);
    }

        // Normalize month, overflowing into year.
    let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y })));
    { let (__tmp_0, __tmp_1) = norm(Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(12)))); *year.lock().unwrap() = Some(__tmp_0); *m.lock().unwrap() = Some(__tmp_1); };
    { let new_val = Month(Arc::new(Mutex::new(Some(((*m.lock().unwrap().as_ref().unwrap()) as i32 + 1))))); *month.lock().unwrap() = Some(new_val); };

        // Normalize nsec, sec, min, hour, overflowing into day.
    { let (__tmp_0, __tmp_1) = norm(Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = nsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1000000000)))); *sec.lock().unwrap() = Some(__tmp_0); *nsec.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = norm(Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(60)))); *min.lock().unwrap() = Some(__tmp_0); *sec.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = norm(Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(60)))); *hour.lock().unwrap() = Some(__tmp_0); *min.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = norm(Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(24)))); *day.lock().unwrap() = Some(__tmp_0); *hour.lock().unwrap() = Some(__tmp_1); };

        // Convert to absolute time and then Unix time.
    let mut unix = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*(*date_to_abs_days(Arc::new(Mutex::new(Some((*year.lock().unwrap().as_ref().unwrap()) as i64))), Arc::new(Mutex::new(Some({ let __arg_holder = month.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SECONDS_PER_DAY as i64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3600; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = ABSOLUTE_TO_UNIX as i64; __tmp_x + __tmp_y })));

        // Look for zone offset for expected time, so we can adjust to UTC.
        // The lookup function expects UTC, so first we pass unix in the
        // hope that it will not be too close to a zone transition,
        // and then adjust if it is.
    let (_, mut offset, mut start, mut end, _) = { let __recv = loc.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = unix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    if { let __tmp_x = offset; let __tmp_y = 0; __tmp_x != __tmp_y } {
        let mut utc = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*unix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(offset as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
                // If utc is valid for the time zone we found, then we have the right offset.
                // If not, we get the correct offset by looking up utc in the location.
        if { let __tmp_x = { let __v = (*utc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = start; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*utc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = end; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3, __tmp_4) = { let __recv = loc.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = utc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; offset = __tmp_1; };
    }
        { let __rhs = (*Arc::new(Mutex::new(Some(offset as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = unix.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }

        // If utc is valid for the time zone we found, then we have the right offset.
        // If not, we get the correct offset by looking up utc in the location.
    let mut t = unix_time(Arc::new(Mutex::new(Some({ let __arg_holder = unix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*nsec.lock().unwrap().as_ref().unwrap()) as i32))));
    (*t.lock().unwrap().as_mut().unwrap()).set_loc(loc.clone());
    return { let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// div divides t by d and returns the quotient parity and remainder.
/// We don't use the quotient parity anymore (round half up instead of round to even)
/// but it's still here in case we change our minds.
pub fn div(t: Arc<Mutex<Option<Time>>>, d: Arc<Mutex<Option<Duration>>>) -> (i32, Arc<Mutex<Option<Duration>>>) {
    let mut qmod2: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut r: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(Some(Default::default())));

    let mut neg = Arc::new(Mutex::new(Some(false)));
    let mut nsec = (*t.lock().unwrap().as_ref().unwrap()).nsec();
    let mut sec = (*t.lock().unwrap().as_ref().unwrap()).sec();
    if { let __tmp_x = sec; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // Operate on absolute value.
        { let new_val = true; *neg.lock().unwrap() = Some(new_val); };
        { let new_val = -(sec); sec = new_val; };
        { let new_val = -(nsec); nsec = new_val; };
        if { let __tmp_x = nsec; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let __rhs = 1e9 as i32; nsec = nsec + __rhs; };
        { sec -= 1; }
    }
    }

        // Operate on absolute value.
        // sec >= 1 before the -- so safe
    if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x < __tmp_y } && { let __tmp_x = { let __tmp_x = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); let __tmp_y = ({ let __bin_d = (*d.lock().unwrap().as_ref().unwrap()).clone(); __bin_d + __bin_d }); __tmp_x % __tmp_y }; let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x == __tmp_y } {
            { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = nsec; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x & __tmp_y }; *qmod2.lock().unwrap() = Some(new_val); };
            { let new_val = std::time::Duration::from_nanos({ let __tmp_x = nsec; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y } as u64); *r.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x % __tmp_y }; let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x == __tmp_y } {
            let mut d1 = Arc::new(Mutex::new(Some(({ let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x / __tmp_y }).as_nanos() as i64)));
            { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = sec; let __tmp_y = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x & __tmp_y }; *qmod2.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = { let __tmp_x = std::time::Duration::from_nanos({ let __tmp_x = sec; let __tmp_y = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y } as u64); let __tmp_y = Duration(Arc::new(Mutex::new(Some(SECOND as i64)))); __tmp_x * __tmp_y }; let __tmp_y = std::time::Duration::from_nanos(nsec as u64); __tmp_x + __tmp_y }; *r.lock().unwrap() = Some(new_val); };
        } else {
                        // Compute nanoseconds as 128-bit number.
            let mut sec = Arc::new(Mutex::new(Some((*sec.lock().unwrap().as_ref().unwrap()) as u64)));
            let mut tmp = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = sec; let __tmp_y = 32; __tmp_x >> __tmp_y }); let __tmp_y = 1e9 as u64; __tmp_x * __tmp_y })));
            let mut u1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*tmp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y })));
            let mut u0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*tmp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x << __tmp_y })));
            { let new_val = { let __tmp_x = ({ let __tmp_x = sec; let __tmp_y = 0xFFFFFFFF as u64; __tmp_x & __tmp_y }); let __tmp_y = 1e9 as u64; __tmp_x * __tmp_y }; *tmp.lock().unwrap() = Some(new_val); };
            let (mut u0x, __tmp_1) = ({ let __owned = u0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tmp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *u0.lock().unwrap() = __moved_tmp_1;;
            if { let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*u0x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let mut guard = u1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let __tmp_0 = (*u0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = { let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(nsec as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *u0x.lock().unwrap() = Some(__tmp_0); *u0.lock().unwrap() = Some(__tmp_1); };
            if { let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*u0x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let mut guard = u1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                        // Compute remainder by subtracting r<<k for decreasing k.
                        // Quotient parity is whether we subtract on last round.
            let mut d1 = Arc::new(Mutex::new(Some(({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as u64)));
            while { let __tmp_x = { let __tmp_x = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63; __tmp_x >> __tmp_y }; let __tmp_y = 1 as u64; __tmp_x != __tmp_y } {
        { let __rhs = 1 as u64; let mut guard = d1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    }
            let mut d0 = Arc::new(Mutex::new(Some(0 as u64)));
            loop {
        { let new_val = 0; *qmod2.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*u1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*u1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*d0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // subtract
        { let new_val = 1; *qmod2.lock().unwrap() = Some(new_val); };
        { let __tmp_0 = (*u0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = { let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*d0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *u0x.lock().unwrap() = Some(__tmp_0); *u0.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*u0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*u0x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let mut guard = u1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let __rhs = (*d1.lock().unwrap().as_ref().unwrap()); let mut guard = u1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // subtract
        if { let __tmp_x = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*d0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_nanos() as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        break
    }
        { let __rhs = 1 as u64; let mut guard = d0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = { let __tmp_x = ({ let __tmp_x = { let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }); let __tmp_y = 63; __tmp_x << __tmp_y }; let mut guard = d0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = 1 as u64; let mut guard = d1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
                        // subtract
            { let new_val = std::time::Duration::from_nanos((*u0.lock().unwrap().as_ref().unwrap()) as u64); *r.lock().unwrap() = Some(new_val); };
        }

        // Special case: 2d divides 1 second.
        // Special case: d is a multiple of 1 second.
        // General case.
        // This could be faster if more cleverness were applied,
        // but it's really only here to avoid special case restrictions in the API.
        // No one will care about these cases.
        // Compute nanoseconds as 128-bit number.
        // Compute remainder by subtracting r<<k for decreasing k.
        // Quotient parity is whether we subtract on last round.
        // subtract
    if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*r.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Duration(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x != __tmp_y } {
                // If input was negative and not an exact multiple of d, we computed q, r such that
                //	q*d + r = -t
                // But the right answers are given by -(q-1), d-r:
                //	q*d + r = -t
                //	-q*d - r = t
                //	-(q-1)*d + (d - r) = t
        { let __rhs = 1; let mut guard = qmod2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let new_val = { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*r.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x - __tmp_y }; *r.lock().unwrap() = Some(new_val); };
    }
        // If input was negative and not an exact multiple of d, we computed q, r such that
        //	q*d + r = -t
        // But the right answers are given by -(q-1), d-r:
        //	q*d + r = -t
        //	-q*d - r = t
        //	-(q-1)*d + (d - r) = t
    return ((*qmod2.lock().unwrap().as_ref().unwrap()), r.clone());
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Time {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
