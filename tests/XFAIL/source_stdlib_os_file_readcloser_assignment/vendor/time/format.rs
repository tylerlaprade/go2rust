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
    format_rfc3339::{parse_r_f_c3339},
    r#mod::{DECEMBER, FEBRUARY, HAS_MONOTONIC, HOUR, JANUARY, MICROSECOND, MILLISECOND, MINUTE, Month, NANOSECOND, SECOND, Time, Weekday, absDays, absSeconds, date, days_before, days_in, is_leap},
    zoneinfo::{Local, Location, UTC, fixed_zone},
};

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const LAYOUT: &'static str = "01/02 03:04:05PM '06 -0700";
pub const A_N_S_I_C: &'static str = "Mon Jan _2 15:04:05 2006";
pub const UNIX_DATE: &'static str = "Mon Jan _2 15:04:05 MST 2006";
pub const RUBY_DATE: &'static str = "Mon Jan 02 15:04:05 -0700 2006";
pub const R_F_C822: &'static str = "02 Jan 06 15:04 MST";
pub const R_F_C822_Z: &'static str = "02 Jan 06 15:04 -0700";
pub const R_F_C850: &'static str = "Monday, 02-Jan-06 15:04:05 MST";
pub const R_F_C1123: &'static str = "Mon, 02 Jan 2006 15:04:05 MST";
pub const R_F_C1123_Z: &'static str = "Mon, 02 Jan 2006 15:04:05 -0700";
pub const R_F_C3339: &'static str = "2006-01-02T15:04:05Z07:00";
pub const R_F_C3339_NANO: &'static str = "2006-01-02T15:04:05.999999999Z07:00";
pub const KITCHEN: &'static str = "3:04PM";
pub const STAMP: &'static str = "Jan _2 15:04:05";
pub const STAMP_MILLI: &'static str = "Jan _2 15:04:05.000";
pub const STAMP_MICRO: &'static str = "Jan _2 15:04:05.000000";
pub const STAMP_NANO: &'static str = "Jan _2 15:04:05.000000000";
pub const DATE_TIME: &'static str = "2006-01-02 15:04:05";
pub const DATE_ONLY: &'static str = "2006-01-02";
pub const TIME_ONLY: &'static str = "15:04:05";


pub(crate) const STD_LONG_MONTH: i32 = 1 + STD_NEED_DATE;
pub(crate) const STD_MONTH: i32 = 2 + STD_NEED_DATE;
pub(crate) const STD_NUM_MONTH: i32 = 3 + STD_NEED_DATE;
pub(crate) const STD_ZERO_MONTH: i32 = 4 + STD_NEED_DATE;
pub(crate) const STD_LONG_WEEK_DAY: i32 = 5 + STD_NEED_DATE;
pub(crate) const STD_WEEK_DAY: i32 = 6 + STD_NEED_DATE;
pub(crate) const STD_DAY: i32 = 7 + STD_NEED_DATE;
pub(crate) const STD_UNDER_DAY: i32 = 8 + STD_NEED_DATE;
pub(crate) const STD_ZERO_DAY: i32 = 9 + STD_NEED_DATE;
pub(crate) const STD_UNDER_YEAR_DAY: i32 = 10 + STD_NEED_YDAY;
pub(crate) const STD_ZERO_YEAR_DAY: i32 = 11 + STD_NEED_YDAY;
pub(crate) const STD_HOUR: i32 = 12 + STD_NEED_CLOCK;
pub(crate) const STD_HOUR12: i32 = 13 + STD_NEED_CLOCK;
pub(crate) const STD_ZERO_HOUR12: i32 = 14 + STD_NEED_CLOCK;
pub(crate) const STD_MINUTE: i32 = 15 + STD_NEED_CLOCK;
pub(crate) const STD_ZERO_MINUTE: i32 = 16 + STD_NEED_CLOCK;
pub(crate) const STD_SECOND: i32 = 17 + STD_NEED_CLOCK;
pub(crate) const STD_ZERO_SECOND: i32 = 18 + STD_NEED_CLOCK;
pub(crate) const STD_LONG_YEAR: i32 = 19 + STD_NEED_DATE;
pub(crate) const STD_YEAR: i32 = 20 + STD_NEED_DATE;
pub(crate) const STD_P_M: i32 = 21 + STD_NEED_CLOCK;
pub(crate) const STDPM: i32 = 22 + STD_NEED_CLOCK;
pub(crate) const STD_T_Z: i32 = 23;
pub(crate) const STD_I_S_O8601_T_Z: i32 = 24;
pub(crate) const STD_I_S_O8601_SECONDS_T_Z: i32 = 25;
pub(crate) const STD_I_S_O8601_SHORT_T_Z: i32 = 26;
pub(crate) const STD_I_S_O8601_COLON_T_Z: i32 = 27;
pub(crate) const STD_I_S_O8601_COLON_SECONDS_T_Z: i32 = 28;
pub(crate) const STD_NUM_T_Z: i32 = 29;
pub(crate) const STD_NUM_SECONDS_TZ: i32 = 30;
pub(crate) const STD_NUM_SHORT_T_Z: i32 = 31;
pub(crate) const STD_NUM_COLON_T_Z: i32 = 32;
pub(crate) const STD_NUM_COLON_SECONDS_T_Z: i32 = 33;
pub(crate) const STD_FRAC_SECOND0: i32 = 34;
pub(crate) const STD_FRAC_SECOND9: i32 = 35;
pub(crate) const STD_NEED_DATE: i32 = 1 << 8;
pub(crate) const STD_NEED_YDAY: i32 = 1 << 9;
pub(crate) const STD_NEED_CLOCK: i32 = 1 << 10;
pub(crate) const STD_ARG_SHIFT: i32 = 16;
pub(crate) const STD_SEPARATOR_SHIFT: i32 = 28;
pub(crate) const STD_MASK: i32 = (((1 as i32) << (STD_ARG_SHIFT as i32)) - (1 as i32));


pub(crate) const LOWERHEX: &'static str = "0123456789abcdef";
pub(crate) const RUNE_SELF: i32 = 0x80;
pub(crate) const RUNE_ERROR: i32 = ('\u{fffd}' as i32);


/// ParseError describes a problem parsing a time string.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub layout: Arc<Mutex<Option<String>>>,
    pub value: Arc<Mutex<Option<String>>>,
    pub layout_elem: Arc<Mutex<Option<String>>>,
    pub value_elem: Arc<Mutex<Option<String>>>,
    pub message: Arc<Mutex<Option<String>>>,
}

impl ParseError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.layout.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.layout_elem.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.value_elem.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.message.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            layout: __go_clone_0_0,
            value: __go_clone_1_0,
            layout_elem: __go_clone_2_0,
            value_elem: __go_clone_3_0,
            message: __go_clone_4_0,
        }
    }
}


impl Default for ParseError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            layout: __go_default_0_0,
            value: __go_default_1_0,
            layout_elem: __go_default_2_0,
            value_elem: __go_default_3_0,
            message: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ParseError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Layout") {
            out.layout = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Value") {
            out.value = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("LayoutElem") {
            out.layout_elem = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ValueElem") {
            out.value_elem = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Message") {
            out.message = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) static std0x: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[i32; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static longDayNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static shortDayNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static shortMonthNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static longMonthNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errAtoi: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errBad: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errLeadingInt: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static unitMap: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<u64>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *std0x.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *longDayNames.lock().unwrap() = Some(vec![]);
    *shortDayNames.lock().unwrap() = Some(vec![]);
    *shortMonthNames.lock().unwrap() = Some(vec![]);
    *longMonthNames.lock().unwrap() = Some(vec![]);
    *errAtoi.lock().unwrap() = None;
    *errBad.lock().unwrap() = None;
    *errLeadingInt.lock().unwrap() = None;
    *unitMap.lock().unwrap() = Some(BTreeMap::new());
    {
        let mut __go_array = Vec::<i32>::with_capacity(6);
        __go_array.push(260);
        __go_array.push(265);
        __go_array.push(1038);
        __go_array.push(1040);
        __go_array.push(1042);
        __go_array.push(276);
        let __go_array: [i32; 6] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *std0x.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_slice = Vec::<String>::with_capacity(7);
        __go_slice.push("Sunday".to_string());
        __go_slice.push("Monday".to_string());
        __go_slice.push("Tuesday".to_string());
        __go_slice.push("Wednesday".to_string());
        __go_slice.push("Thursday".to_string());
        __go_slice.push("Friday".to_string());
        __go_slice.push("Saturday".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *longDayNames.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<String>::with_capacity(7);
        __go_slice.push("Sun".to_string());
        __go_slice.push("Mon".to_string());
        __go_slice.push("Tue".to_string());
        __go_slice.push("Wed".to_string());
        __go_slice.push("Thu".to_string());
        __go_slice.push("Fri".to_string());
        __go_slice.push("Sat".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *shortDayNames.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<String>::with_capacity(12);
        __go_slice.push("Jan".to_string());
        __go_slice.push("Feb".to_string());
        __go_slice.push("Mar".to_string());
        __go_slice.push("Apr".to_string());
        __go_slice.push("May".to_string());
        __go_slice.push("Jun".to_string());
        __go_slice.push("Jul".to_string());
        __go_slice.push("Aug".to_string());
        __go_slice.push("Sep".to_string());
        __go_slice.push("Oct".to_string());
        __go_slice.push("Nov".to_string());
        __go_slice.push("Dec".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *shortMonthNames.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<String>::with_capacity(12);
        __go_slice.push("January".to_string());
        __go_slice.push("February".to_string());
        __go_slice.push("March".to_string());
        __go_slice.push("April".to_string());
        __go_slice.push("May".to_string());
        __go_slice.push("June".to_string());
        __go_slice.push("July".to_string());
        __go_slice.push("August".to_string());
        __go_slice.push("September".to_string());
        __go_slice.push("October".to_string());
        __go_slice.push("November".to_string());
        __go_slice.push("December".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *longMonthNames.lock().unwrap() = Some(__go_slice);
    }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("time: invalid number".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errAtoi.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("bad value for field".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errBad.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("time: bad [0-9]*".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errLeadingInt.lock().unwrap() = new_val; }
    {
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<u64>>>>::new();
        __go_map.insert("ns".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((NANOSECOND).as_nanos() as u64)))))));
        __go_map.insert("us".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64)))))));
        __go_map.insert("\u{b5}s".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64)))))));
        __go_map.insert("\u{3bc}s".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64)))))));
        __go_map.insert("ms".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MILLISECOND).as_nanos() as u64)))))));
        __go_map.insert("s".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((SECOND).as_nanos() as u64)))))));
        __go_map.insert("m".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MINUTE).as_nanos() as u64)))))));
        __go_map.insert("h".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((HOUR).as_nanos() as u64)))))));
        *unitMap.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_zero_globals() {
    *std0x.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *longDayNames.lock().unwrap() = Some(vec![]);
    *shortDayNames.lock().unwrap() = Some(vec![]);
    *shortMonthNames.lock().unwrap() = Some(vec![]);
    *longMonthNames.lock().unwrap() = Some(vec![]);
    *errAtoi.lock().unwrap() = None;
    *errBad.lock().unwrap() = None;
    *errLeadingInt.lock().unwrap() = None;
    *unitMap.lock().unwrap() = Some(BTreeMap::new());
}


pub(crate) fn __go_init_order_0() {
    {
        let mut __go_array = Vec::<i32>::with_capacity(6);
        __go_array.push(260);
        __go_array.push(265);
        __go_array.push(1038);
        __go_array.push(1040);
        __go_array.push(1042);
        __go_array.push(276);
        let __go_array: [i32; 6] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *std0x.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_1() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(7);
        __go_slice.push("Sunday".to_string());
        __go_slice.push("Monday".to_string());
        __go_slice.push("Tuesday".to_string());
        __go_slice.push("Wednesday".to_string());
        __go_slice.push("Thursday".to_string());
        __go_slice.push("Friday".to_string());
        __go_slice.push("Saturday".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *longDayNames.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_2() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(7);
        __go_slice.push("Sun".to_string());
        __go_slice.push("Mon".to_string());
        __go_slice.push("Tue".to_string());
        __go_slice.push("Wed".to_string());
        __go_slice.push("Thu".to_string());
        __go_slice.push("Fri".to_string());
        __go_slice.push("Sat".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *shortDayNames.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_3() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(12);
        __go_slice.push("Jan".to_string());
        __go_slice.push("Feb".to_string());
        __go_slice.push("Mar".to_string());
        __go_slice.push("Apr".to_string());
        __go_slice.push("May".to_string());
        __go_slice.push("Jun".to_string());
        __go_slice.push("Jul".to_string());
        __go_slice.push("Aug".to_string());
        __go_slice.push("Sep".to_string());
        __go_slice.push("Oct".to_string());
        __go_slice.push("Nov".to_string());
        __go_slice.push("Dec".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *shortMonthNames.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_4() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(12);
        __go_slice.push("January".to_string());
        __go_slice.push("February".to_string());
        __go_slice.push("March".to_string());
        __go_slice.push("April".to_string());
        __go_slice.push("May".to_string());
        __go_slice.push("June".to_string());
        __go_slice.push("July".to_string());
        __go_slice.push("August".to_string());
        __go_slice.push("September".to_string());
        __go_slice.push("October".to_string());
        __go_slice.push("November".to_string());
        __go_slice.push("December".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *longMonthNames.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("time: invalid number".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errAtoi.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_6() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("bad value for field".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errBad.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_7() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("time: bad [0-9]*".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errLeadingInt.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_8() {
    {
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<u64>>>>::new();
        __go_map.insert("ns".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((NANOSECOND).as_nanos() as u64)))))));
        __go_map.insert("us".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64)))))));
        __go_map.insert("\u{b5}s".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64)))))));
        __go_map.insert("\u{3bc}s".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MICROSECOND).as_nanos() as u64)))))));
        __go_map.insert("ms".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MILLISECOND).as_nanos() as u64)))))));
        __go_map.insert("s".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((SECOND).as_nanos() as u64)))))));
        __go_map.insert("m".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((MINUTE).as_nanos() as u64)))))));
        __go_map.insert("h".to_string(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((HOUR).as_nanos() as u64)))))));
        *unitMap.lock().unwrap() = Some(__go_map);
    }
}


impl crate::r#mod::Time {
    /// String returns the time formatted using the format string
    ///
    ///	"2006-01-02 15:04:05.999999999 -0700 MST"
    ///
    /// If the time has a monotonic clock reading, the returned string
    /// includes a final field "m=±<value>", where value is the monotonic
    /// clock reading formatted as a decimal number of seconds.
    ///
    /// The returned string is meant for debugging; for a stable serialized
    /// representation, use t.MarshalText, t.MarshalBinary, or t.Format
    /// with an explicit format string.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = self.format(Arc::new(Mutex::new(Some("2006-01-02 15:04:05.999999999 -0700 MST".to_string()))));
                // Format monotonic clock reading as m=±ddd.nnnnnnnnn.
        if { let __tmp_x = { let __tmp_x = (*self.wall.lock().unwrap().as_ref().unwrap()); let __tmp_y = HAS_MONOTONIC as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        let mut m2 = Arc::new(Mutex::new(Some({ let __selector_holder = self.ext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)));
        let mut sign = Arc::new(Mutex::new(Some(('+' as i32) as u8)));
        if { let __tmp_x = (*self.ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = ('-' as i32) as u8; *sign.lock().unwrap() = Some(new_val); };
        { let new_val = ((*m2.lock().unwrap().as_ref().unwrap())).wrapping_neg(); *m2.lock().unwrap() = Some(new_val); };
    }
        let (mut m1, __tmp_1) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x % __tmp_y })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *m2.lock().unwrap() = __moved_tmp_1;;
        let (mut m0, __tmp_1) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x % __tmp_y })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *m1.lock().unwrap() = __moved_tmp_1;;
        let mut buf = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity((24) as usize))));
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(" m=".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*sign.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
        let mut wid = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __v = (*m0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = append_int(
            buf.clone(),
            Arc::new(Mutex::new(Some((*m0.lock().unwrap().as_ref().unwrap()) as i32))),
            Arc::new(Mutex::new(Some(0)))
        ); buf = new_val; };
        { let new_val = 9; *wid.lock().unwrap() = Some(new_val); };
    }
        { let new_val = append_int(
            buf.clone(),
            Arc::new(Mutex::new(Some((*m1.lock().unwrap().as_ref().unwrap()) as i32))),
            Arc::new(Mutex::new(Some({ let __arg_holder = wid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        ); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('.' as i32) as u8); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(
            buf.clone(),
            Arc::new(Mutex::new(Some((*m2.lock().unwrap().as_ref().unwrap()) as i32))),
            Arc::new(Mutex::new(Some(9)))
        ); buf = new_val; };
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// GoString implements [fmt.GoStringer] and formats t to be printed in Go source
    /// code.
    pub fn go_string(&self) -> Arc<Mutex<Option<String>>> {
        let mut abs = self.abs_sec();
        let (mut year, mut month, mut day) = crate::r#mod::absDays::date(&(*crate::r#mod::absSeconds::days(&(*abs.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
        let (mut hour, mut minute, mut second) = crate::r#mod::absSeconds::clock(&(*abs.lock().unwrap().as_ref().unwrap()));
        let mut buf = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(("time.Date(9999, time.September, 31, 23, 59, 59, 999999999, time.Local)".len()) as usize))));
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("time.Date(".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(buf.clone(), Arc::new(Mutex::new(Some(year))), Arc::new(Mutex::new(Some(0)))); buf = new_val; };
        if { let __tmp_x = crate::r#mod::Month(Arc::new(Mutex::new(Some(JANUARY as i32)))); let __tmp_y = (*month.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*month.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::r#mod::Month(Arc::new(Mutex::new(Some(DECEMBER as i32)))); __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", time.".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __seq = { let __seq_holder = longMonthNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) - 1) as usize].clone() }.as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
    } else {
                // It's difficult to construct a time.Time with a date outside the
                // standard range but we might as well try to handle the case.
        { let new_val = append_int(
            buf.clone(),
            Arc::new(Mutex::new(Some((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))),
            Arc::new(Mutex::new(Some(0)))
        ); buf = new_val; };
    }
                // It's difficult to construct a time.Time with a date outside the
                // standard range but we might as well try to handle the case.
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", ".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(buf.clone(), Arc::new(Mutex::new(Some(day))), Arc::new(Mutex::new(Some(0)))); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", ".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(buf.clone(), Arc::new(Mutex::new(Some(hour))), Arc::new(Mutex::new(Some(0)))); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", ".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(buf.clone(), Arc::new(Mutex::new(Some(minute))), Arc::new(Mutex::new(Some(0)))); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", ".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(buf.clone(), Arc::new(Mutex::new(Some(second))), Arc::new(Mutex::new(Some(0)))); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", ".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = append_int(
            buf.clone(),
            Arc::new(Mutex::new(Some(self.nanosecond()))),
            Arc::new(Mutex::new(Some(0)))
        ); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(", ".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        let mut loc = self.location();
    if { let __switch_val = loc.clone(); { let __case = (*UTC.lock().unwrap().as_ref().unwrap()).clone(); let __switch_guard = __switch_val.lock().unwrap(); let __case_guard = __case.lock().unwrap(); let __both_nil = (*__switch_guard).is_none() && (*__case_guard).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__switch_val, &__case); __eq } } || { let __switch_val = loc.clone(); (*__switch_val.lock().unwrap()).is_none() } {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("time.UTC".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if { let __switch_val = loc.clone(); { let __case = (*Local.lock().unwrap().as_ref().unwrap()).clone(); let __switch_guard = __switch_val.lock().unwrap(); let __case_guard = __case.lock().unwrap(); let __both_nil = (*__switch_guard).is_none() && (*__case_guard).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__switch_val, &__case); __eq } } {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("time.Local".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else {
                        // there are several options for how we could display this, none of
                        // which are great:
                        //
                        // - use Location(loc.name), which is not technically valid syntax
                        // - use LoadLocation(loc.name), which will cause a syntax error when
                        // embedded and also would require us to escape the string without
                        // importing fmt or strconv
                        // - try to use FixedZone, which would also require escaping the name
                        // and would represent e.g. "America/Los_Angeles" daylight saving time
                        // shifts inaccurately
                        // - use the pointer format, which is no worse than you'd get with the
                        // old fmt.Sprintf("%#v", t) format.
                        //
                        // Of these, Location(loc.name) is the least disruptive. This is an edge
                        // case we hope not to hit too often.
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("time.Location(".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*quote(Arc::new(Mutex::new(Some({ let __selector_holder = (*loc.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((')' as i32) as u8); __append_target.clone() }; buf = new_val; };
        }
                // there are several options for how we could display this, none of
                // which are great:
                //
                // - use Location(loc.name), which is not technically valid syntax
                // - use LoadLocation(loc.name), which will cause a syntax error when
                // embedded and also would require us to escape the string without
                // importing fmt or strconv
                // - try to use FixedZone, which would also require escaping the name
                // and would represent e.g. "America/Los_Angeles" daylight saving time
                // shifts inaccurately
                // - use the pointer format, which is no worse than you'd get with the
                // old fmt.Sprintf("%#v", t) format.
                //
                // Of these, Location(loc.name) is the least disruptive. This is an edge
                // case we hope not to hit too often.
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((')' as i32) as u8); __append_target.clone() }; buf = new_val; };
        return Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    /// Format returns a textual representation of the time value formatted according
    /// to the layout defined by the argument. See the documentation for the
    /// constant called [Layout] to see how to represent the layout format.
    ///
    /// The executable example for [Time.Format] demonstrates the working
    /// of the layout string in detail and is a good reference.
    pub fn format(&self, layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        const bufSize: i32 = 64;

        let mut b: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
        let mut max = Arc::new(Mutex::new(Some({ let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 10; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } {
        let mut buf: Arc<Mutex<Option<[u8; 64]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)))); b = new_val; };
    }
        { let new_val = self.append_format(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); b = new_val; };
        return Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    /// AppendFormat is like [Time.Format] but appends the textual
    /// representation to b and returns the extended buffer.
    pub fn append_format(&self, b: Arc<Mutex<Option<Vec<u8>>>>, layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
                // Optimize for RFC3339 as it accounts for over half of all representations.
        { let _switch_val = (*layout.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("2006-01-02T15:04:05Z07:00".to_string()) {
            return self.append_format_r_f_c3339(b.clone(), Arc::new(Mutex::new(Some(false))));
        } else if _switch_val == ("2006-01-02T15:04:05.999999999Z07:00".to_string()) {
            return self.append_format_r_f_c3339(b.clone(), Arc::new(Mutex::new(Some(true))));
        } else {
            return self.append_format_1(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        }
    }
    }

    pub fn append_format_1(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>, mut layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        let (mut name, mut offset, mut abs) = self.locabs();
        let mut days = crate::r#mod::absSeconds::days(&(*abs.lock().unwrap().as_ref().unwrap()));
        let mut year: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut month: Arc<Mutex<Option<Month>>> = Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some(0)))))));let mut day: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut yday: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut hour: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut min: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut sec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
                // Each iteration generates one std value.
        while { let __tmp_x = (*layout.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let (mut prefix, mut std, mut suffix) = next_std_chunk(Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*prefix.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    }
        if { let __tmp_x = std; let __tmp_y = 0; __tmp_x == __tmp_y } {
        break
    }
        { let new_val = suffix.lock().unwrap().as_ref().unwrap().clone(); *layout.lock().unwrap() = Some(new_val); };

                // Compute year, month, day if needed.
        if { let __tmp_x = { let __v = (*year.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } && { let __tmp_x = { let __tmp_x = std; let __tmp_y = 256; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = crate::r#mod::absDays::date(&(*days.lock().unwrap().as_ref().unwrap())); *year.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *month.lock().unwrap() = __moved_tmp_1; *day.lock().unwrap() = Some(__tmp_2); };
    }
        if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } && { let __tmp_x = { let __tmp_x = std; let __tmp_y = 512; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = crate::r#mod::absDays::year_yday(&(*days.lock().unwrap().as_ref().unwrap())); *yday.lock().unwrap() = Some(__tmp_1); };
    }

                // Compute hour, minute, second if needed.
        if { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } && { let __tmp_x = { let __tmp_x = std; let __tmp_y = 1024; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = crate::r#mod::absSeconds::clock(&(*abs.lock().unwrap().as_ref().unwrap())); *hour.lock().unwrap() = Some(__tmp_0); *min.lock().unwrap() = Some(__tmp_1); *sec.lock().unwrap() = Some(__tmp_2); };
    }

        '__go_switch_1: loop {
        { let _switch_val = { let __tmp_x = std; let __tmp_y = 65535; __tmp_x & __tmp_y };
    if _switch_val == (276) {
            let mut y = { let __owned = year.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
            if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = -((*y.lock().unwrap().as_ref().unwrap())); *y.lock().unwrap() = Some(new_val); };
    }
            { let new_val = append_int(
                b.clone(),
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x % __tmp_y }))),
                Arc::new(Mutex::new(Some(2)))
            ); b = new_val; };
        } else if _switch_val == (275) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(4)))); b = new_val; };
        } else if _switch_val == (258) {
            { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*Arc::new(Mutex::new(Some({ let __s = &((*crate::r#mod::Month::string(&(*month.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).clone()); let __high = (3) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
        } else if _switch_val == (257) {
            let mut m = crate::r#mod::Month::string(&(*month.lock().unwrap().as_ref().unwrap()));
            { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*m.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
        } else if _switch_val == (259) {
            { let new_val = append_int(
                b.clone(),
                Arc::new(Mutex::new(Some((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))),
                Arc::new(Mutex::new(Some(0)))
            ); b = new_val; };
        } else if _switch_val == (260) {
            { let new_val = append_int(
                b.clone(),
                Arc::new(Mutex::new(Some((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))),
                Arc::new(Mutex::new(Some(2)))
            ); b = new_val; };
        } else if _switch_val == (262) {
            { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*Arc::new(Mutex::new(Some({ let __s = &((*crate::r#mod::Weekday::string(&(*crate::r#mod::absDays::weekday(&(*days.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).clone()); let __high = (3) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
        } else if _switch_val == (261) {
            let mut s = crate::r#mod::Weekday::string(&(*crate::r#mod::absDays::weekday(&(*days.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
            { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
        } else if _switch_val == (263) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); b = new_val; };
        } else if _switch_val == (264) {
            if { let __tmp_x = { let __v = (*day.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((' ' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); b = new_val; };
        } else if _switch_val == (265) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        } else if _switch_val == (522) {
            if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((' ' as i32) as u8); __append_target.clone() }; b = new_val; };
        if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((' ' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
    }
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = yday.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); b = new_val; };
        } else if _switch_val == (523) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = yday.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(3)))); b = new_val; };
        } else if _switch_val == (1036) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        } else if _switch_val == (1037) {
                        // Noon is 12PM, midnight is 12AM.
            let mut hr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x % __tmp_y })));
            if { let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 12; *hr.lock().unwrap() = Some(new_val); };
    }
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = hr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); b = new_val; };
        } else if _switch_val == (1038) {
                        // Noon is 12PM, midnight is 12AM.
            let mut hr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x % __tmp_y })));
            if { let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 12; *hr.lock().unwrap() = Some(new_val); };
    }
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = hr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        } else if _switch_val == (1039) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); b = new_val; };
        } else if _switch_val == (1040) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        } else if _switch_val == (1041) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); b = new_val; };
        } else if _switch_val == (1042) {
            { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        } else if _switch_val == (1045) {
            if { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("PM".to_string().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    } else {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("AM".to_string().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    }
        } else if _switch_val == (1046) {
            if { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("pm".to_string().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    } else {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("am".to_string().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    }
        } else if _switch_val == (24) || _switch_val == (27) || _switch_val == (25) || _switch_val == (26) || _switch_val == (28) || _switch_val == (29) || _switch_val == (32) || _switch_val == (30) || _switch_val == (31) || _switch_val == (33) {
                        // Ugly special case. We cheat and take the "Z" variants
                        // to mean "the time zone as formatted for ISO 8601".
            if {
                let __go_cond_0 = { let __tmp_x = offset; let __tmp_y = 0; __tmp_x == __tmp_y };
                if __go_cond_0 {
                    let __go_cond_1 = {
                        let __go_cond_2 = {
                            let __go_cond_3 = {
                                let __go_cond_4 = {
                                    let __go_cond_5 = { let __tmp_x = std; let __tmp_y = 24; __tmp_x == __tmp_y };
                                    if __go_cond_5 {
                                        true
                                    } else {
                                        let __go_cond_6 = { let __tmp_x = std; let __tmp_y = 27; __tmp_x == __tmp_y };
                                        __go_cond_6
                                    }
                                };
                                if __go_cond_4 {
                                    true
                                } else {
                                    let __go_cond_7 = { let __tmp_x = std; let __tmp_y = 25; __tmp_x == __tmp_y };
                                    __go_cond_7
                                }
                            };
                            if __go_cond_3 {
                                true
                            } else {
                                let __go_cond_8 = { let __tmp_x = std; let __tmp_y = 26; __tmp_x == __tmp_y };
                                __go_cond_8
                            }
                        };
                        if __go_cond_2 {
                            true
                        } else {
                            let __go_cond_9 = { let __tmp_x = std; let __tmp_y = 28; __tmp_x == __tmp_y };
                            __go_cond_9
                        }
                    };
                    __go_cond_1
                } else {
                    false
                }
            } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('Z' as i32) as u8); __append_target.clone() }; b = new_val; };
        break '__go_switch_1
    }
            let mut zone = Arc::new(Mutex::new(Some({ let __tmp_x = offset; let __tmp_y = 60; __tmp_x / __tmp_y })));
            let mut absoffset = Arc::new(Mutex::new(Some(offset)));
            if { let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = -((*zone.lock().unwrap().as_ref().unwrap())); *zone.lock().unwrap() = Some(new_val); };
        { let new_val = -((*absoffset.lock().unwrap().as_ref().unwrap())); *absoffset.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('+' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
            { let new_val = append_int(
                b.clone(),
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x / __tmp_y }))),
                Arc::new(Mutex::new(Some(2)))
            ); b = new_val; };
            if { let __tmp_x = std; let __tmp_y = 27; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 32; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 28; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 33; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((':' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
            if { let __tmp_x = std; let __tmp_y = 31; __tmp_x != __tmp_y } && { let __tmp_x = std; let __tmp_y = 26; __tmp_x != __tmp_y } {
        { let new_val = append_int(
            b.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x % __tmp_y }))),
            Arc::new(Mutex::new(Some(2)))
        ); b = new_val; };
    }
                        // append seconds if appropriate
            if { let __tmp_x = std; let __tmp_y = 25; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 30; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 33; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 28; __tmp_x == __tmp_y } {
        if { let __tmp_x = std; let __tmp_y = 33; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 28; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((':' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
        { let new_val = append_int(
            b.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*absoffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x % __tmp_y }))),
            Arc::new(Mutex::new(Some(2)))
        ); b = new_val; };
    }
        } else if _switch_val == (23) {
            if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*name.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
        break '__go_switch_1
    }
                        // No time zone known for this time, but we must print one.
                        // Use the -0700 format.
            let mut zone = Arc::new(Mutex::new(Some({ let __tmp_x = offset; let __tmp_y = 60; __tmp_x / __tmp_y })));
            if { let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = -((*zone.lock().unwrap().as_ref().unwrap())); *zone.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('+' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
            { let new_val = append_int(
                b.clone(),
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x / __tmp_y }))),
                Arc::new(Mutex::new(Some(2)))
            ); b = new_val; };
            { let new_val = append_int(
                b.clone(),
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x % __tmp_y }))),
                Arc::new(Mutex::new(Some(2)))
            ); b = new_val; };
        } else if _switch_val == (34) || _switch_val == (35) {
            { let new_val = append_nano(
                b.clone(),
                Arc::new(Mutex::new(Some(self.nanosecond()))),
                Arc::new(Mutex::new(Some(std)))
            ); b = new_val; };
        }
    };
        break;
    }
    }
                // Compute year, month, day if needed.
                // Compute hour, minute, second if needed.
                // Noon is 12PM, midnight is 12AM.
                // Noon is 12PM, midnight is 12AM.
                // Ugly special case. We cheat and take the "Z" variants
                // to mean "the time zone as formatted for ISO 8601".
                // convert to minutes
                // append seconds if appropriate
                // No time zone known for this time, but we must print one.
                // Use the -0700 format.
                // convert to minutes
        return b.clone();
    }
}

impl ParseError {
    /// Error returns the string representation of a ParseError.
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.message.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", "parsing time ".to_string()));
            __s.push_str(&format!("{}", (*quote(Arc::new(Mutex::new(Some({ let __selector_holder = self.value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", " as ".to_string()));
            __s.push_str(&format!("{}", (*quote(Arc::new(Mutex::new(Some({ let __selector_holder = self.layout.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ": cannot parse ".to_string()));
            __s.push_str(&format!("{}", (*quote(Arc::new(Mutex::new(Some({ let __selector_holder = self.value_elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", " as ".to_string()));
            __s.push_str(&format!("{}", (*quote(Arc::new(Mutex::new(Some({ let __selector_holder = self.layout_elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", "parsing time ".to_string()));
            __s.push_str(&format!("{}", (*quote(Arc::new(Mutex::new(Some({ let __selector_holder = self.value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", (*self.message.clone().lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }
}

impl StdError for ParseError {}


/// startsWithLowerCase reports whether the string has a lower-case letter at the beginning.
/// Its purpose is to prevent matching strings like "Month" when looking for "Mon".
pub fn starts_with_lower_case(str: Arc<Mutex<Option<String>>>) -> bool {
    if { let __tmp_x = ((*str.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
    let mut c = Arc::new(Mutex::new(Some({ let __s = &((*str.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
    return { let __tmp_x = ('a' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('z' as i32) as u8; __tmp_x <= __tmp_y };
}

/// nextStdChunk finds the first occurrence of a std string in
/// layout and returns the text before, the std string, and the text after.
///
/// nextStdChunk should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/searKing/golang/go
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname nextStdChunk
pub fn next_std_chunk(layout: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, i32, Arc<Mutex<Option<String>>>) {
    let mut prefix: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut std: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut suffix: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32)));
    { let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('J' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Jan".to_string(); __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "January".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 257, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        if !starts_with_lower_case(Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })))) {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 258, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
    }
        } else if _switch_val == (('M' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } {
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Mon".to_string(); __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Monday".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 261, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        if !starts_with_lower_case(Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })))) {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 262, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "MST".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 23, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
    }
        } else if _switch_val == (('0' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = ('1' as i32) as u8; let __tmp_y = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('6' as i32) as u8; __tmp_x <= __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), { let __seq = { let __seq_holder = std0x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('1' as i32) as u8; __tmp_x - __tmp_y }) as usize].clone() }, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('2' as i32) as u8; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 523, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        } else if _switch_val == (('1' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('5' as i32) as u8; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 1036, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 259, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
        } else if _switch_val == (('2' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "2006".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 275, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 263, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
        } else if _switch_val == (('_' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('2' as i32) as u8; __tmp_x == __tmp_y } {
                // _2006 is really a literal _, followed by stdLongYear
        if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "2006".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))), 275, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 264, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
                        // _2006 is really a literal _, followed by stdLongYear
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('2' as i32) as u8; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 522, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        } else if _switch_val == (('3' as i32)) {
            return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 1037, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
        } else if _switch_val == (('4' as i32)) {
            return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 1039, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
        } else if _switch_val == (('5' as i32)) {
            return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 1041, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
        } else if _switch_val == (('P' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('M' as i32) as u8; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 1045, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        } else if _switch_val == (('p' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('m' as i32) as u8; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 1046, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        } else if _switch_val == (('-' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "-070000".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 30, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "-07:00:00".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 33, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "-0700".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 29, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "-07:00".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 32, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "-07".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 31, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        } else if _switch_val == (('Z' as i32)) {
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Z070000".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 25, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Z07:00:00".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 28, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Z0700".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 24, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Z07:00".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 27, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
            if { let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y } as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Z07".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), 26, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
    }
        } else if _switch_val == (('.' as i32)) || _switch_val == ((',' as i32)) {
            if { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && ({ let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('9' as i32) as u8; __tmp_x == __tmp_y }) {
        let mut ch = Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] })));
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // String of digits must end here - only fractional second is all digits.
        if !is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        let mut code = Arc::new(Mutex::new(Some(STD_FRAC_SECOND0)));
        if { let __tmp_x = { let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('9' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = 35; *code.lock().unwrap() = Some(new_val); };
    }
        let mut std = std_frac_second(
            Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }); __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        return (Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), std, Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))));
    }
    }
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // January, Jan
        // Monday, Mon, MST
        // 01, 02, 03, 04, 05, 06, 002
        // 15, 1
        // 2006, 2
        // _2, _2006, __2
        // _2006 is really a literal _, followed by stdLongYear
        // PM
        // pm
        // -070000, -07:00:00, -0700, -07:00, -07
        // Z070000, Z07:00:00, Z0700, Z07:00,
        // ,000, or .000, or ,999, or .999 - repeated digits for fractional seconds.
        // String of digits must end here - only fractional second is all digits.
    return ({ let __owned = layout.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, 0, Arc::new(Mutex::new(Some("".to_string()))));
}

/// match reports whether s1 and s2 match ignoring case.
/// It is assumed s1 and s2 are the same length.
pub fn r#match(s1: Arc<Mutex<Option<String>>>, s2: Arc<Mutex<Option<String>>>) -> bool {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s1.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut c1 = Arc::new(Mutex::new(Some({ let __s = &((*s1.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        let mut c2 = Arc::new(Mutex::new(Some({ let __s = &((*s2.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if { let __tmp_x = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // Switch to lower-case; 'a'-'A' is known to be a single bit.
        { let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y } as u8; let mut guard = c1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y } as u8; let mut guard = c2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('a' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('z' as i32) as u8; __tmp_x > __tmp_y } {
        return false;
    }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Switch to lower-case; 'a'-'A' is known to be a single bit.
    true
}

pub fn lookup(tab: Arc<Mutex<Option<Vec<String>>>>, val: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    { let __range_holder = tab.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        if { let __tmp_x = ((*val.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (v.len() as i32); __tmp_x >= __tmp_y } && r#match(Arc::new(Mutex::new(Some({ let __s = &((*val.lock().unwrap().as_ref().unwrap()).clone()); let __high = (v.len()) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some((*v).clone())))) {
        return (i as i32, Arc::new(Mutex::new(Some({ let __s = &((*val.lock().unwrap().as_ref().unwrap()).clone()); let __low = (v.len()) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
    }
    } }
    (-(1), { let __owned = val.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, errBad.clone())
}

/// appendInt appends the decimal form of x to b and returns the result.
/// If the decimal form (excluding sign) is shorter than width, the result is padded with leading 0's.
/// Duplicates functionality in strconv, but avoids dependency.
pub fn append_int(mut b: Arc<Mutex<Option<Vec<u8>>>>, x: Arc<Mutex<Option<i32>>>, width: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut u = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64)));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(-((*x.lock().unwrap().as_ref().unwrap())) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *u.lock().unwrap() = __moved_val; };
    }

        // 2-digit and 4-digit fields are the most common in time formats.
    let mut utod = Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<u64>>>| -> u8 {
        return { let __tmp_x = ('0' as i32) as u8; let __tmp_y = (*Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync>)));
    if { let __tmp_x = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e2 as u64; __tmp_x < __tmp_y } {
            return { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e1 as u64; __tmp_x / __tmp_y })))) }, { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e1 as u64; __tmp_x % __tmp_y })))) }]); __append_target.clone() };
        } else if { let __tmp_x = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e4 as u64; __tmp_x < __tmp_y } {
            return { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![
                { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e3 as u64; __tmp_x / __tmp_y })))) },
                { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e2 as u64; __tmp_x / __tmp_y }; let __tmp_y = 1e1 as u64; __tmp_x % __tmp_y })))) },
                { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e1 as u64; __tmp_x / __tmp_y }; let __tmp_y = 1e1 as u64; __tmp_x % __tmp_y })))) },
                { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e1 as u64; __tmp_x % __tmp_y })))) },
            ]); __append_target.clone() };
        }

        // Compute the number of decimal digits.
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = 1; *n.lock().unwrap() = Some(new_val); };
    }
    let mut u2 = { let __owned = u.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*u2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = 10 as u64; let mut guard = u2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    }

        // Add 0-padding.
    let mut pad = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*pad.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('0' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let mut guard = pad.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        // Ensure capacity.
    if { let __tmp_x = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); __tmp_x <= __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    } else {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; b = new_val; };
    }

        // Assemble decimal in reverse order.
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut q = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x / __tmp_y })));
        (*b.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })))) };
        { let new_val = q.lock().unwrap().as_ref().unwrap().clone(); *u.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    (*b.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> = { let mut __f_guard = utod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(u.clone()) };
    return b.clone();
}

/// Duplicates functionality in strconv, but avoids dependency.
pub fn atoi<bytes: GoByteSequence + Clone + Send + Sync + 'static>(mut s: Arc<Mutex<Option<bytes>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut neg = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && ({ let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y }) {
        { let new_val = { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y }; *neg.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((1) as usize, None)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
    let (mut q, mut rem, __tmp_2) = leading_int::<bytes>(s.clone()); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2;;
    { let new_val = Arc::new(Mutex::new(Some(q as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = ((*rem.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return (0, errAtoi.clone());
    }
    if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = -((*x.lock().unwrap().as_ref().unwrap())); *x.lock().unwrap() = Some(new_val); };
    }
    return ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
}

/// The "std" value passed to appendNano contains two packed fields: the number of
/// digits after the decimal and the separator character (period or comma).
/// These functions pack and unpack that variable.
pub fn std_frac_second(code: Arc<Mutex<Option<i32>>>, n: Arc<Mutex<Option<i32>>>, c: Arc<Mutex<Option<i32>>>) -> i32 {
        // Use 0xfff to make the failure case even more absurd.
    if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('.' as i32); __tmp_x == __tmp_y } {
        return { let __tmp_x = { let __v = (*code.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4095; __tmp_x & __tmp_y }); let __tmp_y = STD_ARG_SHIFT; __tmp_x << __tmp_y }); __tmp_x | __tmp_y };
    }
    return {
            let __go_binary_0 = (*code.lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*n.lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = 4095;
            let __go_binary_3 = __go_binary_1 & __go_binary_2;
            let __go_binary_4 = STD_ARG_SHIFT;
            let __go_binary_5 = __go_binary_3 << __go_binary_4;
            let __go_binary_6 = __go_binary_0 | __go_binary_5;
            let __go_binary_7 = 1;
            let __go_binary_8 = STD_SEPARATOR_SHIFT;
            let __go_binary_9 = __go_binary_7 << __go_binary_8;
            let __go_binary_10 = __go_binary_6 | __go_binary_9;
            __go_binary_10
        };
}

pub fn digits_len(std: Arc<Mutex<Option<i32>>>) -> i32 {
    return { let __tmp_x = ({ let __tmp_x = { let __v = (*std.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = STD_ARG_SHIFT; __tmp_x >> __tmp_y }); let __tmp_y = 4095; __tmp_x & __tmp_y };
}

pub fn separator(std: Arc<Mutex<Option<i32>>>) -> u8 {
    if { let __tmp_x = ({ let __tmp_x = { let __v = (*std.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = STD_SEPARATOR_SHIFT; __tmp_x >> __tmp_y }); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return ('.' as u8);
    }
    (',' as u8)
}

/// appendNano appends a fractional second, as nanoseconds, to b
/// and returns the result. The nanosec must be within [0, 999999999].
pub fn append_nano(mut b: Arc<Mutex<Option<Vec<u8>>>>, nanosec: Arc<Mutex<Option<i32>>>, std: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut trim = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*std.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 65535; __tmp_x & __tmp_y }; let __tmp_y = 35; __tmp_x == __tmp_y })));
    let mut n = digits_len(Arc::new(Mutex::new(Some({ let __arg_holder = std.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __v = (*trim.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*nanosec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y }) {
        return b.clone();
    }
    let mut dot = separator(Arc::new(Mutex::new(Some({ let __arg_holder = std.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(dot); __append_target.clone() }; b = new_val; };
    { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = nanosec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(9)))); b = new_val; };
    if { let __tmp_x = n; let __tmp_y = 9; __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 9; __tmp_x - __tmp_y } as i32); let __tmp_y = (n as i32); __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    }
    if { let __v = (*trim.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        while { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    }
        if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = dot; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    }
    }
    return b.clone();
}

/// newParseError creates a new ParseError.
/// The provided value and valueElem are cloned to avoid escaping their values.
pub fn new_parse_error(layout: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>, layoutElem: Arc<Mutex<Option<String>>>, valueElem: Arc<Mutex<Option<String>>>, message: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<ParseError>>> {
    let mut valueCopy = internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut valueElemCopy = internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = valueElem.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return Arc::new(Mutex::new(Some(ParseError { layout: Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: Arc::new(Mutex::new(Some({ let __arg_holder = valueCopy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), layout_elem: Arc::new(Mutex::new(Some({ let __arg_holder = layoutElem.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value_elem: Arc::new(Mutex::new(Some({ let __arg_holder = valueElemCopy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), message: Arc::new(Mutex::new(Some({ let __arg_holder = message.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
}

pub fn quote(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut buf = Arc::new(Mutex::new(Some({ let mut v = Vec::with_capacity(({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x + __tmp_y }) as usize); v.resize((1) as usize, 0); v })));
    (*buf.lock().unwrap().as_mut().unwrap())[(0) as usize] = ('"' as i32) as u8;
    for (i, c) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = (c as i32); let __tmp_y = RUNE_SELF as i32; __tmp_x >= __tmp_y } || { let __tmp_x = c; let __tmp_y = ' '; __tmp_x < __tmp_y } {
                // This means you are asking us to parse a time.Duration or
                // time.Location with unprintable or non-ASCII characters in it.
                // We don't expect to hit this case very often. We could try to
                // reproduce strconv.Quote's behavior with full fidelity but
                // given how rarely we expect to hit these edge cases, speed and
                // conciseness are better.
        let mut width: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = (c as i32); let __tmp_y = RUNE_ERROR as i32; __tmp_x == __tmp_y } {
        { let new_val = 1; *width.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ({ let __tmp_x = i as i32; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; let __high = ({ let __tmp_x = i as i32; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "\u{fffd}".to_string(); __tmp_x == __tmp_y } {
        { let new_val = 3; *width.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = (*Arc::new(Mutex::new(Some(char::from_u32(((c as i32)) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap()).len() as i32; *width.lock().unwrap() = Some(new_val); };
    }
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\x".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = i as i32; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = 4; __tmp_x >> __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = i as i32; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = 0xF as u8; __tmp_x & __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } else {
        if { let __tmp_x = c; let __tmp_y = '"'; __tmp_x == __tmp_y } || { let __tmp_x = c; let __tmp_y = '\\'; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\\' as i32) as u8); __append_target.clone() }; buf = new_val; };
    }
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*Arc::new(Mutex::new(Some(char::from_u32(((c as i32)) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
    }
    }
        // This means you are asking us to parse a time.Duration or
        // time.Location with unprintable or non-ASCII characters in it.
        // We don't expect to hit this case very often. We could try to
        // reproduce strconv.Quote's behavior with full fidelity but
        // given how rarely we expect to hit these edge cases, speed and
        // conciseness are better.
    { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('"' as i32) as u8); __append_target.clone() }; buf = new_val; };
    return Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
}

/// isDigit reports whether s[i] is in range and is a decimal digit.
pub fn is_digit<bytes: GoByteSequence + Clone + Send + Sync + 'static>(s: Arc<Mutex<Option<bytes>>>, i: Arc<Mutex<Option<i32>>>) -> bool {
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x <= __tmp_y } {
        return false;
    }
    let mut c = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_byte(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))));
    return { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y };
}

/// getnum parses s[0:1] or s[0:2] (fixed forces s[0:2])
/// as a decimal integer and returns the integer and the
/// remainder of the string.
pub fn getnum(s: Arc<Mutex<Option<String>>>, fixed: Arc<Mutex<Option<bool>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if !is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
        return (0, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, errBad.clone());
    }
    if !is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1)))) {
        if { let __v = (*fixed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (0, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, errBad.clone());
    }
        return ((*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
    }
    return ({
        let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 10; __tmp_x * __tmp_y };
        let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
        __tmp_x + __tmp_y
    }, Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
}

/// getnum3 parses s[0:1], s[0:2], or s[0:3] (fixed forces s[0:3])
/// as a decimal integer and returns the integer and the remainder
/// of the string.
pub fn getnum3(s: Arc<Mutex<Option<String>>>, fixed: Arc<Mutex<Option<bool>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } && is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *n.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __v = (*fixed.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x != __tmp_y } {
        return (0, { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, errBad.clone());
    }
    return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
}

pub fn cutspace(mut s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    while { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// skip removes the given prefix from value,
/// treating runs of space characters as equivalent.
pub fn skip(mut value: Arc<Mutex<Option<String>>>, mut prefix: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    while { let __tmp_x = ((*prefix.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __s = &((*prefix.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x != __tmp_y } {
        return ({ let __owned = value.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, errBad.clone());
    }
        { let new_val = cutspace(Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *prefix.lock().unwrap() = __moved_val; };
        { let new_val = cutspace(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        continue
    }
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = { let __s = &((*prefix.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; __tmp_x != __tmp_y } {
        return ({ let __owned = value.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, errBad.clone());
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*prefix.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *prefix.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
    }
    return ({ let __owned = value.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
}

/// Parse parses a formatted string and returns the time value it represents.
/// See the documentation for the constant called [Layout] to see how to
/// represent the format. The second argument must be parseable using
/// the format string (layout) provided as the first argument.
///
/// The example for [Time.Format] demonstrates the working of the layout string
/// in detail and is a good reference.
///
/// When parsing (only), the input may contain a fractional second
/// field immediately after the seconds field, even if the layout does not
/// signify its presence. In that case either a comma or a decimal point
/// followed by a maximal series of digits is parsed as a fractional second.
/// Fractional seconds are truncated to nanosecond precision.
///
/// Elements omitted from the layout are assumed to be zero or, when
/// zero is impossible, one, so parsing "3:04pm" returns the time
/// corresponding to Jan 1, year 0, 15:04:00 UTC (note that because the year is
/// 0, this time is before the zero Time).
/// Years must be in the range 0000..9999. The day of the week is checked
/// for syntax but it is otherwise ignored.
///
/// For layouts specifying the two-digit year 06, a value NN >= 69 will be treated
/// as 19NN and a value NN < 69 will be treated as 20NN.
///
/// The remainder of this comment describes the handling of time zones.
///
/// In the absence of a time zone indicator, Parse returns a time in UTC.
///
/// When parsing a time with a zone offset like -0700, if the offset corresponds
/// to a time zone used by the current location ([Local]), then Parse uses that
/// location and zone in the returned time. Otherwise it records the time as
/// being in a fabricated location with time fixed at the given zone offset.
///
/// When parsing a time with a zone abbreviation like MST, if the zone abbreviation
/// has a defined offset in the current location, then that offset is used.
/// The zone abbreviation "UTC" is recognized as UTC regardless of location.
/// If the zone abbreviation is unknown, Parse records the time as being
/// in a fabricated location with the given zone abbreviation and a zero offset.
/// This choice means that such a time can be parsed and reformatted with the
/// same layout losslessly, but the exact instant used in the representation will
/// differ by the actual zone offset. To avoid such problems, prefer time layouts
/// that use a numeric zone offset, or use [ParseInLocation].
pub fn parse(layout: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::r#mod::Time>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Optimize for RFC3339 as it accounts for over half of all representations.
    if { let __tmp_x = (*layout.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "2006-01-02T15:04:05Z07:00".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*layout.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "2006-01-02T15:04:05.999999999Z07:00".to_string(); __tmp_x == __tmp_y } {
        {
        let (mut t, mut ok) = parse_r_f_c3339::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __arg_holder = Local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });;
        if ok {
            return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));;
        }
    }
    }
    parse_1(Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __arg_holder = UTC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = Local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })
}

pub fn parse_1(mut layout: Arc<Mutex<Option<String>>>, mut value: Arc<Mutex<Option<String>>>, defaultLocation: Arc<Mutex<Option<Location>>>, local: Arc<Mutex<Option<Location>>>) -> (Arc<Mutex<Option<crate::r#mod::Time>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut alayout, mut avalue) = ({ let __owned = layout.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __owned = value.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    let mut rangeErrString = Arc::new(Mutex::new(Some("".to_string())));
    let mut amSet = Arc::new(Mutex::new(Some(false)));
    let mut pmSet = Arc::new(Mutex::new(Some(false)));

        // Time being constructed.
    let mut year: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut month: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut day: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut yday: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut hour: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut min: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut sec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut nsec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut z: Arc<Mutex<Option<Location>>> = Arc::new(Mutex::new(None));let mut zoneOffset: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(-1)));let mut zoneName: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

        // Each iteration processes one std value.
    loop {
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let (mut prefix, mut std, mut suffix) = next_std_chunk(Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut stdstr = Arc::new(Mutex::new(Some({ let __s = &((*layout.lock().unwrap().as_ref().unwrap()).clone()); let __low = ((*prefix.lock().unwrap().as_ref().unwrap()).len()) as usize; let __high = ({ let __tmp_x = ((*layout.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*suffix.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() })));
        { let (__tmp_0, __tmp_1) = skip(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string())))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        if { let __tmp_x = std; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(
            Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some("".to_string()))),
            Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(format!("{}{}", ": extra text: ".to_string(), (*quote(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())))))
        ).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        break
    }
        { let new_val = suffix.lock().unwrap().as_ref().unwrap().clone(); *layout.lock().unwrap() = Some(new_val); };
        let mut p: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut hold = { let __owned = value.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        '__go_switch_2: loop {
        {
        let _switch_val = { let __tmp_x = std; let __tmp_y = 65535; __tmp_x & __tmp_y };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 276) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
            {
                let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (2) as usize; __s[__low..__high].to_string() })));
                let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() })));
                *p.lock().unwrap() = __tmp_0.lock().unwrap().take();
                *value.lock().unwrap() = __tmp_1.lock().unwrap().take();
            };
            { let (__tmp_0, __tmp_1) = atoi::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *year.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        break '__go_switch_2
    }
            if { let __tmp_x = { let __v = (*year.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 69; __tmp_x >= __tmp_y } {
        { let __rhs = 1900; let mut guard = year.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        { let __rhs = 2000; let mut guard = year.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        }
        if !_matched && (_switch_val == 275) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x < __tmp_y } || !is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
            {
                let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (4) as usize; __s[__low..__high].to_string() })));
                let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (4) as usize; __s[__low..].to_string() })));
                *p.lock().unwrap() = __tmp_0.lock().unwrap().take();
                *value.lock().unwrap() = __tmp_1.lock().unwrap().take();
            };
            { let (__tmp_0, __tmp_1) = atoi::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *year.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        }
        if !_matched && (_switch_val == 258) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup(shortMonthNames.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *month.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            { let mut guard = month.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        }
        if !_matched && (_switch_val == 257) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup(longMonthNames.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *month.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            { let mut guard = month.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        }
        if !_matched && (_switch_val == 259 || _switch_val == 260) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = std; let __tmp_y = 260; __tmp_x == __tmp_y })))); *month.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && ({ let __tmp_x = { let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } || { let __tmp_x = 12; let __tmp_y = { let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) {
        { let new_val = "month".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
        }
        if !_matched && (_switch_val == 262) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Ignore weekday except for error checking.
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup(shortDayNames.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        }
        if !_matched && (_switch_val == 261) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup(longDayNames.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        }
        if !_matched && (_switch_val == 263 || _switch_val == 264 || _switch_val == 265) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = std; let __tmp_y = 264; __tmp_x == __tmp_y } && { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
    }
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = std; let __tmp_y = 265; __tmp_x == __tmp_y })))); *day.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        }
        if !_matched && (_switch_val == 522 || _switch_val == 523) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        if { let __tmp_x = std; let __tmp_y = 522; __tmp_x == __tmp_y } && { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum3(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = std; let __tmp_y = 523; __tmp_x == __tmp_y })))); *yday.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        }
        if !_matched && (_switch_val == 1036) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))); *hour.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            if { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = 24; let __tmp_y = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = "hour".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
        }
        if !_matched && (_switch_val == 1037 || _switch_val == 1038) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = std; let __tmp_y = 1038; __tmp_x == __tmp_y })))); *hour.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            if { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = 12; let __tmp_y = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = "hour".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
        }
        if !_matched && (_switch_val == 1039 || _switch_val == 1040) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = std; let __tmp_y = 1040; __tmp_x == __tmp_y })))); *min.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            if { let __tmp_x = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = 60; let __tmp_y = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = "minute".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
        }
        if !_matched && (_switch_val == 1041 || _switch_val == 1042) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = std; let __tmp_y = 1042; __tmp_x == __tmp_y })))); *sec.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        break '__go_switch_2
    }
            if { let __tmp_x = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = 60; let __tmp_y = { let __v = (*sec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = "second".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
        break '__go_switch_2
    }
                        // Special case: do we have a fractional second but no
                        // fractional second in the format?
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x >= __tmp_y } && comma_or_period(Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })))) && is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1)))) {
        { let (__tmp_0, __tmp_1, __tmp_2) = next_std_chunk(Arc::new(Mutex::new(Some({ let __arg_holder = layout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); std = __tmp_1; };
        { let __rhs = 65535; std = std & __rhs; };
        if { let __tmp_x = std; let __tmp_y = 34; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 35; __tmp_x == __tmp_y } {
                // Fractional second in the layout; proceed normally
        break '__go_switch_2
    }
                // Fractional second in the layout; proceed normally
                // No fractional second in the layout but we have one in the input.
        let mut n = Arc::new(Mutex::new(Some(2)));
        while { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && is_digit::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let (__tmp_0, __tmp_1, __tmp_2) = parse_nanoseconds::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *nsec.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *rangeErrString.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
    }
        }
        if !_matched && (_switch_val == 1045) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
            {
                let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (2) as usize; __s[__low..__high].to_string() })));
                let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() })));
                *p.lock().unwrap() = __tmp_0.lock().unwrap().take();
                *value.lock().unwrap() = __tmp_1.lock().unwrap().take();
            };
            { let _switch_val = (*p.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("PM".to_string()) {
            { let new_val = true; *pmSet.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("AM".to_string()) {
            { let new_val = true; *amSet.lock().unwrap() = Some(new_val); };
        } else {
            { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        }
    }
        }
        if !_matched && (_switch_val == 1046) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
            {
                let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (2) as usize; __s[__low..__high].to_string() })));
                let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() })));
                *p.lock().unwrap() = __tmp_0.lock().unwrap().take();
                *value.lock().unwrap() = __tmp_1.lock().unwrap().take();
            };
            { let _switch_val = (*p.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("pm".to_string()) {
            { let new_val = true; *pmSet.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("am".to_string()) {
            { let new_val = true; *amSet.lock().unwrap() = Some(new_val); };
        } else {
            { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        }
    }
        }
        if !_matched && (_switch_val == 24 || _switch_val == 26 || _switch_val == 27 || _switch_val == 25 || _switch_val == 28) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('Z' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        { let new_val = (*UTC.lock().unwrap().as_ref().unwrap()).clone(); z = new_val; };
        break '__go_switch_2
    }
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 29 || _switch_val == 31 || _switch_val == 32 || _switch_val == 30 || _switch_val == 33) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            let mut sign: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut hour: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut min: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut seconds: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
            if { let __tmp_x = std; let __tmp_y = 27; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 32; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 6; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        if { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] }; let __tmp_y = (':' as i32) as u8; __tmp_x != __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        {
            let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (1) as usize; __s[__low..__high].to_string() })));
            let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = (3) as usize; __s[__low..__high].to_string() })));
            let __tmp_2 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (4) as usize; let __high = (6) as usize; __s[__low..__high].to_string() })));
            let __tmp_3 = "00".to_string();
            let __tmp_4 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (6) as usize; __s[__low..].to_string() })));
            *sign.lock().unwrap() = __tmp_0.lock().unwrap().take();
            *hour.lock().unwrap() = __tmp_1.lock().unwrap().take();
            *min.lock().unwrap() = __tmp_2.lock().unwrap().take();
            *seconds.lock().unwrap() = Some(__tmp_3);
            *value.lock().unwrap() = __tmp_4.lock().unwrap().take();
        };
    } else if { let __tmp_x = std; let __tmp_y = 31; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 26; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        {
            let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (1) as usize; __s[__low..__high].to_string() })));
            let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = (3) as usize; __s[__low..__high].to_string() })));
            let __tmp_2 = "00".to_string();
            let __tmp_3 = "00".to_string();
            let __tmp_4 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (3) as usize; __s[__low..].to_string() })));
            *sign.lock().unwrap() = __tmp_0.lock().unwrap().take();
            *hour.lock().unwrap() = __tmp_1.lock().unwrap().take();
            *min.lock().unwrap() = Some(__tmp_2);
            *seconds.lock().unwrap() = Some(__tmp_3);
            *value.lock().unwrap() = __tmp_4.lock().unwrap().take();
        };
    } else if { let __tmp_x = std; let __tmp_y = 28; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 33; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 9; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        if { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] }; let __tmp_y = (':' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(6) as usize] }; let __tmp_y = (':' as i32) as u8; __tmp_x != __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        {
            let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (1) as usize; __s[__low..__high].to_string() })));
            let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = (3) as usize; __s[__low..__high].to_string() })));
            let __tmp_2 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (4) as usize; let __high = (6) as usize; __s[__low..__high].to_string() })));
            let __tmp_3 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (7) as usize; let __high = (9) as usize; __s[__low..__high].to_string() })));
            let __tmp_4 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (9) as usize; __s[__low..].to_string() })));
            *sign.lock().unwrap() = __tmp_0.lock().unwrap().take();
            *hour.lock().unwrap() = __tmp_1.lock().unwrap().take();
            *min.lock().unwrap() = __tmp_2.lock().unwrap().take();
            *seconds.lock().unwrap() = __tmp_3.lock().unwrap().take();
            *value.lock().unwrap() = __tmp_4.lock().unwrap().take();
        };
    } else if { let __tmp_x = std; let __tmp_y = 25; __tmp_x == __tmp_y } || { let __tmp_x = std; let __tmp_y = 30; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 7; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        {
            let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (1) as usize; __s[__low..__high].to_string() })));
            let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = (3) as usize; __s[__low..__high].to_string() })));
            let __tmp_2 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (3) as usize; let __high = (5) as usize; __s[__low..__high].to_string() })));
            let __tmp_3 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (5) as usize; let __high = (7) as usize; __s[__low..__high].to_string() })));
            let __tmp_4 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (7) as usize; __s[__low..].to_string() })));
            *sign.lock().unwrap() = __tmp_0.lock().unwrap().take();
            *hour.lock().unwrap() = __tmp_1.lock().unwrap().take();
            *min.lock().unwrap() = __tmp_2.lock().unwrap().take();
            *seconds.lock().unwrap() = __tmp_3.lock().unwrap().take();
            *value.lock().unwrap() = __tmp_4.lock().unwrap().take();
        };
    } else {
        if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 5; __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
        {
            let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (1) as usize; __s[__low..__high].to_string() })));
            let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = (3) as usize; __s[__low..__high].to_string() })));
            let __tmp_2 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (3) as usize; let __high = (5) as usize; __s[__low..__high].to_string() })));
            let __tmp_3 = "00".to_string();
            let __tmp_4 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (5) as usize; __s[__low..].to_string() })));
            *sign.lock().unwrap() = __tmp_0.lock().unwrap().take();
            *hour.lock().unwrap() = __tmp_1.lock().unwrap().take();
            *min.lock().unwrap() = __tmp_2.lock().unwrap().take();
            *seconds.lock().unwrap() = Some(__tmp_3);
            *value.lock().unwrap() = __tmp_4.lock().unwrap().take();
        };
    }
            let mut hr: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut mm: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut ss: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))); *hr.lock().unwrap() = Some(__tmp_0); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))); *mm.lock().unwrap() = Some(__tmp_0); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let (__tmp_0, __tmp_1, __tmp_2) = getnum(Arc::new(Mutex::new(Some({ let __arg_holder = seconds.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))); *ss.lock().unwrap() = Some(__tmp_0); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
    }
    }
                        // The range test use > rather than >=,
                        // as some people do write offsets of 24 hours
                        // or 60 minutes or 60 seconds.
            if { let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 24; __tmp_x > __tmp_y } {
        { let new_val = "time zone offset hour".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
            if { let __tmp_x = { let __v = (*mm.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x > __tmp_y } {
        { let new_val = "time zone offset minute".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
            if { let __tmp_x = { let __v = (*ss.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x > __tmp_y } {
        { let new_val = "time zone offset second".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
    }
            { let new_val = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*hr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*mm.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = 60; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*ss.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *zoneOffset.lock().unwrap() = Some(new_val); };
            { let _switch_val = { let __s = &((*sign.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] };
    if _switch_val == (('+' as i32) as u8) {
        } else if _switch_val == (('-' as i32) as u8) {
            { let new_val = -((*zoneOffset.lock().unwrap().as_ref().unwrap())); *zoneOffset.lock().unwrap() = Some(new_val); };
        } else {
            { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        }
    }
        }
        if !_matched && (_switch_val == 23) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Does it look like a time zone?
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (3) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "UTC".to_string(); __tmp_x == __tmp_y } {
        { let new_val = (*UTC.lock().unwrap().as_ref().unwrap()).clone(); z = new_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (3) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        break '__go_switch_2
    }
            let (mut n, mut ok) = parse_time_zone(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            if !ok {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
            {
                let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __high = (n) as usize; __s[..__high].to_string() })));
                let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (n) as usize; __s[__low..].to_string() })));
                *zoneName.lock().unwrap() = __tmp_0.lock().unwrap().take();
                *value.lock().unwrap() = __tmp_1.lock().unwrap().take();
            };
        }
        if !_matched && (_switch_val == 34) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // stdFracSecond0 requires the exact number of digits as specified in
                        // the layout.
            let mut ndigit = Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = digits_len(Arc::new(Mutex::new(Some(std)))); __tmp_x + __tmp_y })));
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __v = (*ndigit.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break '__go_switch_2
    }
            { let (__tmp_0, __tmp_1, __tmp_2) = parse_nanoseconds::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ndigit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *nsec.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *rangeErrString.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*ndigit.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        }
        if !_matched && (_switch_val == 35) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } || !comma_or_period(Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })))) || { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = ('9' as i32) as u8; let __tmp_y = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; __tmp_x < __tmp_y } {
                // Fractional second omitted.
        break '__go_switch_2
    }
                        // Fractional second omitted.
                        // Take any number of digits, even more than asked for,
                        // because it is what the stdSecond case would do.
            let mut i = Arc::new(Mutex::new(Some(0)));
            while { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let (__tmp_0, __tmp_1, __tmp_2) = parse_nanoseconds::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); *nsec.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *rangeErrString.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = 1; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        }
    };
        break;
    }
                // Unix time starts Dec 31 1969 in some time zones
                // Ignore weekday except for error checking.
                // Note that we allow any one- or two-digit day here.
                // The month, day, year combination is validated after we've completed parsing.
                // Note that we allow any one-, two-, or three-digit year-day here.
                // The year-day, year combination is validated after we've completed parsing.
                // Special case: do we have a fractional second but no
                // fractional second in the format?
                // Fractional second in the layout; proceed normally
                // No fractional second in the layout but we have one in the input.
                // The range test use > rather than >=,
                // as some people do write offsets of 24 hours
                // or 60 minutes or 60 seconds.
                // offset is in seconds
                // Does it look like a time zone?
                // stdFracSecond0 requires the exact number of digits as specified in
                // the layout.
                // Fractional second omitted.
                // Take any number of digits, even more than asked for,
                // because it is what the stdSecond case would do.
        if { let __tmp_x = (*rangeErrString.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(
            Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = stdstr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({
                let mut __s = String::new();
                __s.push_str(&format!("{}", ": ".to_string()));
                __s.push_str(&format!("{}", { let __v = (*rangeErrString.lock().unwrap().as_ref().unwrap()).clone(); __v }));
                __s.push_str(&format!("{}", " out of range".to_string()));
                __s
            })))
        ).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = stdstr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = hold.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string())))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
    }
        // Unix time starts Dec 31 1969 in some time zones
        // Ignore weekday except for error checking.
        // Note that we allow any one- or two-digit day here.
        // The month, day, year combination is validated after we've completed parsing.
        // Note that we allow any one-, two-, or three-digit year-day here.
        // The year-day, year combination is validated after we've completed parsing.
        // Special case: do we have a fractional second but no
        // fractional second in the format?
        // Fractional second in the layout; proceed normally
        // No fractional second in the layout but we have one in the input.
        // The range test use > rather than >=,
        // as some people do write offsets of 24 hours
        // or 60 minutes or 60 seconds.
        // offset is in seconds
        // Does it look like a time zone?
        // stdFracSecond0 requires the exact number of digits as specified in
        // the layout.
        // Fractional second omitted.
        // Take any number of digits, even more than asked for,
        // because it is what the stdSecond case would do.
    if { let __v = (*pmSet.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x < __tmp_y } {
        { (*hour.lock().unwrap().as_mut().unwrap()).push_str(&12); };
    } else if { let __v = (*amSet.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __v = (*hour.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x == __tmp_y } {
        { let new_val = 0; *hour.lock().unwrap() = Some(new_val); };
    }

        // Convert yday to day, month.
    if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let mut d: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let mut m: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        if is_leap(Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(FEBRUARY as i32 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *m.lock().unwrap() = __moved_val; };
        { let new_val = 29; *d.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x > __tmp_y } {
        { let mut guard = yday.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 365; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(": day-of-year out of range".to_string())))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 31; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; *m.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = days_before(Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32)))))))); let __tmp_y = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let mut guard = m.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = { let __tmp_x = { let __v = (*yday.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = days_before(Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some((*m.lock().unwrap().as_ref().unwrap()) as i32)))))))); __tmp_x - __tmp_y }; *d.lock().unwrap() = Some(new_val); };
    }
                // If month, day already seen, yday's m, d must match.
                // Otherwise, set them from m, d.
        if { let __tmp_x = { let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(": day-of-year does not match month".to_string())))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *month.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*day.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*day.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(": day-of-year does not match day".to_string())))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = d.lock().unwrap().as_ref().unwrap().clone(); *day.lock().unwrap() = Some(new_val); };
    } else {
        if { let __tmp_x = { let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(JANUARY as i32 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *month.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = { let __v = (*day.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = 1; *day.lock().unwrap() = Some(new_val); };
    }
    }

        // If month, day already seen, yday's m, d must match.
        // Otherwise, set them from m, d.
        // Validate the day of the month.
    if { let __tmp_x = { let __v = (*day.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*day.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = days_in(Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some((*month.lock().unwrap().as_ref().unwrap()) as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new((*new_parse_error(Arc::new(Mutex::new(Some({ let __arg_holder = alayout.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = avalue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(": day out of range".to_string())))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }

    if { let __nil_result = (*z.lock().unwrap()).is_some(); __nil_result } {
        return (date(
            Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some((*month.lock().unwrap().as_ref().unwrap()) as i32))))))),
            Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = nsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            z.clone()
        ), Arc::new(Mutex::new(None)));
    }

    if { let __tmp_x = { let __v = (*zoneOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1; __tmp_x != __tmp_y } {
        let mut t = date(
            Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some((*month.lock().unwrap().as_ref().unwrap()) as i32))))))),
            Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = nsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            { let __arg_holder = UTC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }
        );
        (*t.lock().unwrap().as_mut().unwrap()).add_sec(Arc::new(Mutex::new(Some(-((*zoneOffset.lock().unwrap().as_ref().unwrap()) as i64)))));
                // Look for local zone with the given offset.
                // If that zone was in effect at the given time, use it.
        let (mut name, mut offset, _, _, _) = { let __recv = local.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).unix_sec())))); __result };
        if { let __tmp_x = offset; let __tmp_y = { let __v = (*zoneOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && ({ let __tmp_x = (*zoneName.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*zoneName.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y }) {
        (*t.lock().unwrap().as_mut().unwrap()).set_loc(local.clone());
        return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
                // Otherwise create fake zone to record offset.
        let mut zoneNameCopy = internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = zoneName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*t.lock().unwrap().as_mut().unwrap()).set_loc(fixed_zone(Arc::new(Mutex::new(Some({ let __arg_holder = zoneNameCopy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = zoneOffset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }

        // Look for local zone with the given offset.
        // If that zone was in effect at the given time, use it.
        // Otherwise create fake zone to record offset.
        // avoid leaking the input value
    if { let __tmp_x = (*zoneName.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let mut t = date(
            Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some((*month.lock().unwrap().as_ref().unwrap()) as i32))))))),
            Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = nsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            { let __arg_holder = UTC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }
        );
                // Look for local zone with the given offset.
                // If that zone was in effect at the given time, use it.
        let (mut offset, mut ok) = { let __recv = local.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup_name(Arc::new(Mutex::new(Some({ let __arg_holder = zoneName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).unix_sec())))); __result };
        if ok {
        (*t.lock().unwrap().as_mut().unwrap()).add_sec(Arc::new(Mutex::new(Some(-(offset as i64)))));
        (*t.lock().unwrap().as_mut().unwrap()).set_loc(local.clone());
        return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
                // Otherwise, create fake zone with unknown offset.
        if { let __tmp_x = ((*zoneName.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x > __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*zoneName.lock().unwrap().as_ref().unwrap()).clone()); let __high = (3) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "GMT".to_string(); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = atoi::<String>(Arc::new(Mutex::new(Some({ let __s = &((*zoneName.lock().unwrap().as_ref().unwrap()).clone()); let __low = (3) as usize; __s[__low..].to_string() })))); offset = __tmp_0; };
        { let __rhs = 3600; offset = offset * __rhs; };
    }
                // Guaranteed OK by parseGMT.
        let mut zoneNameCopy = internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = zoneName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*t.lock().unwrap().as_mut().unwrap()).set_loc(fixed_zone(Arc::new(Mutex::new(Some({ let __arg_holder = zoneNameCopy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(offset)))));
        return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }

        // Look for local zone with the given offset.
        // If that zone was in effect at the given time, use it.
        // Otherwise, create fake zone with unknown offset.
        // Guaranteed OK by parseGMT.
        // avoid leaking the input value
        // Otherwise, fall back to default.
    return (date(
        Arc::new(Mutex::new(Some({ let __arg_holder = year.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some((*month.lock().unwrap().as_ref().unwrap()) as i32))))))),
        Arc::new(Mutex::new(Some({ let __arg_holder = day.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = hour.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = sec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = nsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        defaultLocation.clone()
    ), Arc::new(Mutex::new(None)));
}

/// parseTimeZone parses a time zone string and returns its length. Time zones
/// are human-generated and unpredictable. We can't do precise error checking.
/// On the other hand, for a correct parse there must be a time zone at the
/// beginning of the string, so it's almost always true that there's one
/// there. We look at the beginning of the string for a run of upper-case letters.
/// If there are more than 5, it's an error.
/// If there are 4 or 5 and the last is a T, it's a time zone.
/// If there are 3, it's a time zone.
/// Otherwise, other than special cases, it's not a time zone.
/// GMT is special because it can have an hour offset.
pub fn parse_time_zone(value: Arc<Mutex<Option<String>>>) -> (i32, bool) {
    let mut length: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x < __tmp_y } {
        return (0, false);
    }

        // Special case 1: ChST and MeST are the only zones with a lower-case letter.
    if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x >= __tmp_y } && ({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __high = (4) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "ChST".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __high = (4) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "MeST".to_string(); __tmp_x == __tmp_y }) {
        return (4, true);
    }

        // Special case 2: GMT may have an hour offset; treat it specially.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __high = (3) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "GMT".to_string(); __tmp_x == __tmp_y } {
        { let new_val = parse_g_m_t(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *length.lock().unwrap() = Some(new_val); };
        return ({ let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }

        // Special Case 3: Some time zones are not named, but have +/-00 format
    if { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = parse_signed_offset(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *length.lock().unwrap() = Some(new_val); };
        let mut ok = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y })));
        return ({ let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

        // parseSignedOffset returns 0 in case of bad input
        // How many upper-case letters are there? Need at least three, at most five.
    let mut nUpper: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let new_val = 0; *nUpper.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = { let __v = (*nUpper.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x < __tmp_y } {
        if { let __tmp_x = ({ let __v = (*nUpper.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } {
        break
    }
        {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*nUpper.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));;
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('A' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = ('Z' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            break;
        }
    }
        { let mut guard = nUpper.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let _switch_val = { let __v = (*nUpper.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) || _switch_val == (1) || _switch_val == (2) || _switch_val == (6) {
            return (0, false);
        } else if _switch_val == (5) {
            if { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(4) as usize] }; let __tmp_y = ('T' as i32) as u8; __tmp_x == __tmp_y } {
        return (5, true);
    }
        } else if _switch_val == (4) {
                        // Must end in T, except one special case.
            if { let __tmp_x = { let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] }; let __tmp_y = ('T' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __high = (4) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "WITA".to_string(); __tmp_x == __tmp_y } {
        return (4, true);
    }
        } else if _switch_val == (3) {
            return (3, true);
        }
    }
        // Must end in T to match.
        // Must end in T, except one special case.
    (0, false)
}

/// parseGMT parses a GMT time zone. The input string is known to start "GMT".
/// The function checks whether that is followed by a sign and a number in the
/// range -23 through +23 excluding zero.
pub fn parse_g_m_t(mut value: Arc<Mutex<Option<String>>>) -> i32 {
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (3) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
    if { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 3;
    }

    return { let __tmp_x = 3; let __tmp_y = parse_signed_offset(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x + __tmp_y };
}

/// parseSignedOffset parses a signed timezone offset (e.g. "+03" or "-04").
/// The function checks for a signed number in the range -23 through +23 excluding zero.
/// Returns length of the found offset string or 0 otherwise.
pub fn parse_signed_offset(value: Arc<Mutex<Option<String>>>) -> i32 {
    let mut sign = Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
    if { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('-' as i32) as u8; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('+' as i32) as u8; __tmp_x != __tmp_y } {
        return 0;
    }
    let (mut x, mut rem, mut err) = leading_int::<String>(Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))));

        // fail if nothing consumed by leadingInt
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*value.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*rem.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return 0;
    }
    if { let __tmp_x = x; let __tmp_y = 23 as u64; __tmp_x > __tmp_y } {
        return 0;
    }
    return { let __tmp_x = ((*value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*rem.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x - __tmp_y };
}

pub fn comma_or_period(b: Arc<Mutex<Option<u8>>>) -> bool {
    return { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (',' as i32) as u8; __tmp_x == __tmp_y };
}

pub fn parse_nanoseconds<bytes: GoByteSequence + Clone + Send + Sync + 'static>(mut value: Arc<Mutex<Option<bytes>>>, mut nbytes: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut ns: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut rangeErrString: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    if !comma_or_period(Arc::new(Mutex::new(Some((*value.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize))))) {
        { let __rhs_holder = errBad.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*ns.lock().unwrap().as_ref().unwrap()), rangeErrString.clone(), err.clone());
    }
    if { let __tmp_x = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*value.lock().unwrap().as_ref().unwrap()).go_slice_to_string(0, Some((10) as usize))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        { let new_val = 10; *nbytes.lock().unwrap() = Some(new_val); };
    }
    {
        { let (__tmp_0, __tmp_1) = atoi::<bytes>(Arc::new(Mutex::new(Some((*value.lock().unwrap().as_ref().unwrap()).go_slice_to_string((1) as usize, Some(({ let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)))))); *ns.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return ((*ns.lock().unwrap().as_ref().unwrap()), rangeErrString.clone(), err.clone());;
        }
    }
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = "fractional second".to_string(); *rangeErrString.lock().unwrap() = Some(new_val); };
        return ((*ns.lock().unwrap().as_ref().unwrap()), rangeErrString.clone(), err.clone());
    }

        // We need nanoseconds, which means scaling by the number
        // of missing digits in the format, maximum length 10.
    let mut scaleDigits = Arc::new(Mutex::new(Some({ let __tmp_x = 10; let __tmp_y = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*scaleDigits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __rhs = 10; let mut guard = ns.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return ((*ns.lock().unwrap().as_ref().unwrap()), rangeErrString.clone(), err.clone());
}

/// leadingInt consumes the leading [0-9]* from s.
pub fn leading_int<bytes: GoByteSequence + Clone + Send + Sync + 'static>(s: Arc<Mutex<Option<bytes>>>) -> (u64, Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut x: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut rem: Arc<Mutex<Option<bytes>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_byte(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x > __tmp_y } {
        break
    }
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as u64) << (63 as u64)) / (10 as u64)) as u64; __tmp_x > __tmp_y } {
                // overflow
        return (0, rem.clone(), errLeadingInt.clone());
    }
                // overflow
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = ('0' as u64); __tmp_x - __tmp_y }; *x.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u64) << (63 as u64)) as u64; __tmp_x > __tmp_y } {
                // overflow
        return (0, rem.clone(), errLeadingInt.clone());
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // overflow
        // overflow
    return ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize, None)))), Arc::new(Mutex::new(None)));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for ParseError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
