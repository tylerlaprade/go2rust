use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::casetables::*;
use crate::digit::*;
use crate::graphic::*;
use crate::tables::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const MAX_RUNE: i32 = ('\u{10ffff}' as i32);
pub const REPLACEMENT_CHAR: i32 = ('\u{fffd}' as i32);
pub const MAX_A_S_C_I_I: i32 = ('\u{7f}' as i32);
pub const MAX_LATIN1: i32 = ('\u{ff}' as i32);


pub const UPPER_CASE: i32 = 0;
pub const LOWER_CASE: i32 = 1;
pub const TITLE_CASE: i32 = 2;
pub const MAX_CASE: i32 = 3;


pub const UPPER_LOWER: i32 = MAX_RUNE + 1;


pub(crate) const LINEAR_MAX: i32 = 18;


/// RangeTable defines a set of Unicode code points by listing the ranges of
/// code points within the set. The ranges are listed in two slices
/// to save space: a slice of 16-bit ranges and a slice of 32-bit ranges.
/// The two slices must be in sorted order and non-overlapping.
/// Also, R32 should contain only values >= 0x10000 (1<<16).
#[derive(Debug, Clone)]
pub struct RangeTable {
    pub r16: Arc<Mutex<Option<Vec<Range16>>>>,
    pub r32: Arc<Mutex<Option<Vec<Range32>>>>,
    pub latin_offset: Arc<Mutex<Option<i32>>>,
}

impl RangeTable {
    pub fn __go_value_clone(&self) -> Self {
        Self { r16: self.r16.clone(), r32: self.r32.clone(), latin_offset: { let __guard = self.latin_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RangeTable {
    fn default() -> Self {
        Self { r16: Arc::new(Mutex::new(None)), r32: Arc::new(Mutex::new(None)), latin_offset: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for RangeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.r16), format_slice(&self.r32), (*self.latin_offset.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for RangeTable {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("LatinOffset") {
            out.latin_offset = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// Range16 represents of a range of 16-bit Unicode code points. The range runs from Lo to Hi
/// inclusive and has the specified stride.
#[derive(Debug, Clone)]
pub struct Range16 {
    pub lo: Arc<Mutex<Option<u16>>>,
    pub hi: Arc<Mutex<Option<u16>>>,
    pub stride: Arc<Mutex<Option<u16>>>,
}

impl Range16 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lo: { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hi: { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stride: { let __guard = self.stride.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Range16 {
    fn default() -> Self {
        Self { lo: Arc::new(Mutex::new(Some(0))), hi: Arc::new(Mutex::new(Some(0))), stride: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Range16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lo.lock().unwrap().as_ref().unwrap()), (*self.hi.lock().unwrap().as_ref().unwrap()), (*self.stride.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Range16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Lo") {
            out.lo = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hi") {
            out.hi = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Stride") {
            out.stride = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// Range32 represents of a range of Unicode code points and is used when one or
/// more of the values will not fit in 16 bits. The range runs from Lo to Hi
/// inclusive and has the specified stride. Lo and Hi must always be >= 1<<16.
#[derive(Debug, Clone)]
pub struct Range32 {
    pub lo: Arc<Mutex<Option<u32>>>,
    pub hi: Arc<Mutex<Option<u32>>>,
    pub stride: Arc<Mutex<Option<u32>>>,
}

impl Range32 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lo: { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hi: { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stride: { let __guard = self.stride.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Range32 {
    fn default() -> Self {
        Self { lo: Arc::new(Mutex::new(Some(0))), hi: Arc::new(Mutex::new(Some(0))), stride: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Range32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lo.lock().unwrap().as_ref().unwrap()), (*self.hi.lock().unwrap().as_ref().unwrap()), (*self.stride.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Range32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Lo") {
            out.lo = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hi") {
            out.hi = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Stride") {
            out.stride = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// CaseRange represents a range of Unicode code points for simple (one
/// code point to one code point) case conversion.
/// The range runs from Lo to Hi inclusive, with a fixed stride of 1. Deltas
/// are the number to add to the code point to reach the code point for a
/// different case for that character. They may be negative. If zero, it
/// means the character is in the corresponding case. There is a special
/// case representing sequences of alternating corresponding Upper and Lower
/// pairs. It appears with a fixed Delta of
///
///	{UpperLower, UpperLower, UpperLower}
///
/// The constant UpperLower has an otherwise impossible delta value.
#[derive(Debug, Clone)]
pub struct CaseRange {
    pub lo: Arc<Mutex<Option<u32>>>,
    pub hi: Arc<Mutex<Option<u32>>>,
    pub delta: Arc<Mutex<Option<d>>>,
}

impl CaseRange {
    pub fn __go_value_clone(&self) -> Self {
        Self { lo: { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hi: { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, delta: { let __guard = self.delta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for CaseRange {
    fn default() -> Self {
        Self { lo: Arc::new(Mutex::new(Some(0))), hi: Arc::new(Mutex::new(Some(0))), delta: Arc::new(Mutex::new(Some(d(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))) }
    }
}

impl std::fmt::Display for CaseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lo.lock().unwrap().as_ref().unwrap()), (*self.hi.lock().unwrap().as_ref().unwrap()), (*self.delta.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for CaseRange {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Lo") {
            out.lo = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hi") {
            out.hi = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// SpecialCase represents language-specific case mappings such as Turkish.
/// Methods of SpecialCase customize (by overriding) the standard mappings.
#[derive(Debug, Clone, Default)]
pub struct SpecialCase(pub Arc<Mutex<Option<Vec<CaseRange>>>>);


#[derive(Debug, Clone)]
pub struct d(pub Arc<Mutex<Option<[i32; 3]>>>);

impl Default for d {
    fn default() -> Self {
        d(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Display for d {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


/// caseOrbit is defined in tables.go as []foldPair. Right now all the
/// entries fit in uint16, so use uint16. If that changes, compilation
/// will fail (the constants in the composite literal will not fit in uint16)
/// and the types here can change to uint32.
#[derive(Debug, Clone)]
pub struct foldPair {
    pub from: Arc<Mutex<Option<u16>>>,
    pub to: Arc<Mutex<Option<u16>>>,
}

impl foldPair {
    pub fn __go_value_clone(&self) -> Self {
        Self { from: { let __guard = self.from.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, to: { let __guard = self.to.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for foldPair {
    fn default() -> Self {
        Self { from: Arc::new(Mutex::new(Some(0))), to: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for foldPair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.from.lock().unwrap().as_ref().unwrap()), (*self.to.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for foldPair {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("From") {
            out.from = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("To") {
            out.to = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl SpecialCase {
    /// ToUpper maps the rune to upper case giving priority to the special mapping.
    pub fn to_upper(&self, r: Arc<Mutex<Option<i32>>>) -> i32 {
        let (mut r1, mut hadMapping) = to_1(Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), self.0.clone());
        if { let __tmp_x = r1; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && !hadMapping {
        { let new_val = to_upper(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); r1 = new_val; };
    }
        r1
    }

    /// ToTitle maps the rune to title case giving priority to the special mapping.
    pub fn to_title(&self, r: Arc<Mutex<Option<i32>>>) -> i32 {
        let (mut r1, mut hadMapping) = to_1(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), self.0.clone());
        if { let __tmp_x = r1; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && !hadMapping {
        { let new_val = to_title(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); r1 = new_val; };
    }
        r1
    }

    /// ToLower maps the rune to lower case giving priority to the special mapping.
    pub fn to_lower(&self, r: Arc<Mutex<Option<i32>>>) -> i32 {
        let (mut r1, mut hadMapping) = to_1(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), self.0.clone());
        if { let __tmp_x = r1; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && !hadMapping {
        { let new_val = to_lower(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); r1 = new_val; };
    }
        r1
    }
}

/// is16 reports whether r is in the sorted slice of 16-bit ranges.
pub fn is16(ranges: Arc<Mutex<Option<Vec<Range16>>>>, r: Arc<Mutex<Option<u16>>>) -> bool {
    if { let __tmp_x = ((*ranges.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 18; __tmp_x <= __tmp_y } || { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_LATIN1 as u16; __tmp_x <= __tmp_y } {
        for i in 0..(({ let __range_holder = ranges.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut range_: Option<GoSliceElemPtr<Range16>> = Some(GoSliceElemPtr::new(ranges.clone(), (i) as usize));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return false;
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as u16; __tmp_x == __tmp_y };
    }
    }
        return false;
    }

        // binary search over ranges
    let mut lo = Arc::new(Mutex::new(Some(0)));
    let mut hi = Arc::new(Mutex::new(Some((*ranges.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut m = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));
        let mut range_: Option<GoSliceElemPtr<Range16>> = Some(GoSliceElemPtr::new(ranges.clone(), ({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        if { let __tmp_x = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as u16; __tmp_x == __tmp_y };
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *hi.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *lo.lock().unwrap() = Some(new_val); };
    }
    }
    false
}

/// is32 reports whether r is in the sorted slice of 32-bit ranges.
pub fn is32(ranges: Arc<Mutex<Option<Vec<Range32>>>>, r: Arc<Mutex<Option<u32>>>) -> bool {
    if { let __tmp_x = ((*ranges.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 18; __tmp_x <= __tmp_y } {
        for i in 0..(({ let __range_holder = ranges.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut range_: Option<GoSliceElemPtr<Range32>> = Some(GoSliceElemPtr::new(ranges.clone(), (i) as usize));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return false;
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*{ let __field = (*range_.as_ref().unwrap().borrow().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y };
    }
    }
        return false;
    }

        // binary search over ranges
    let mut lo = Arc::new(Mutex::new(Some(0)));
    let mut hi = Arc::new(Mutex::new(Some((*ranges.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut m = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));
        let mut range_ = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        if { let __tmp_x = (*{ let __field = (*range_.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*range_.lock().unwrap().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*{ let __field = (*range_.lock().unwrap().as_ref().unwrap()).stride.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y };
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*range_.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *hi.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *lo.lock().unwrap() = Some(new_val); };
    }
    }
    false
}

/// Is reports whether the rune is in the specified table of ranges.
pub fn is(rangeTab: Arc<Mutex<Option<RangeTable>>>, r: Arc<Mutex<Option<i32>>>) -> bool {
    let mut r16 = (*rangeTab.lock().unwrap().as_ref().unwrap()).r16.clone();

        // Compare as uint32 to correctly handle negative runes.
    if { let __tmp_x = ((*r16.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = r16.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*r16.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return is16(r16.clone(), Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u16))));
    }
    let mut r32 = (*rangeTab.lock().unwrap().as_ref().unwrap()).r32.clone();
    if { let __tmp_x = ((*r32.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = r32.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return is32(r32.clone(), Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))));
    }
    false
}

pub fn is_excluding_latin(rangeTab: Arc<Mutex<Option<RangeTable>>>, r: Arc<Mutex<Option<i32>>>) -> bool {
    let mut r16 = (*rangeTab.lock().unwrap().as_ref().unwrap()).r16.clone();

        // Compare as uint32 to correctly handle negative runes.
    {
        let mut off = Arc::new(Mutex::new(Some({ let __selector_holder = (*rangeTab.lock().unwrap().as_ref().unwrap()).latin_offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = ((*r16.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = r16.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*r16.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
            return is16(Arc::new(Mutex::new(Some({ let __seq_holder = r16.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u16))));;
        }
    }
    let mut r32 = (*rangeTab.lock().unwrap().as_ref().unwrap()).r32.clone();
    if { let __tmp_x = ((*r32.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = r32.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return is32(r32.clone(), Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))));
    }
    false
}

/// IsUpper reports whether the rune is an upper case letter.
pub fn is_upper(r: Arc<Mutex<Option<i32>>>) -> bool {
        // See comment in IsGraphic.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_LATIN1 as u32; __tmp_x <= __tmp_y } {
        return { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = properties.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()) as usize].clone() }; let __tmp_y = P_LMASK as u8; __tmp_x & __tmp_y }; let __tmp_y = P_LU as u8; __tmp_x == __tmp_y };
    }
    is_excluding_latin({ let __arg_holder = Upper.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// lookupCaseRange returns the CaseRange mapping for rune r or nil if no
/// mapping exists for r.
pub fn lookup_case_range(r: Arc<Mutex<Option<i32>>>, caseRange: Arc<Mutex<Option<Vec<CaseRange>>>>) -> Option<GoSliceElemPtr<CaseRange>> {
        // binary search over ranges
    let mut lo = Arc::new(Mutex::new(Some(0)));
    let mut hi = Arc::new(Mutex::new(Some((*caseRange.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut m = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));
        let mut cr: Option<GoSliceElemPtr<CaseRange>> = Some(GoSliceElemPtr::new(caseRange.clone(), ({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*cr.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*cr.as_ref().unwrap().borrow().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return cr.clone();
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*cr.as_ref().unwrap().borrow().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *hi.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *lo.lock().unwrap() = Some(new_val); };
    }
    }
    return None;
}

/// convertCase converts r to _case using CaseRange cr.
pub fn convert_case(_case: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<i32>>>, cr: GoPtr<CaseRange>) -> i32 {
    let mut delta = Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_array = (*{ let __ptr_value = cr.with_mut(|__ptr_value| __ptr_value.delta.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*_case.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
    if { let __tmp_x = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_RUNE as i32; __tmp_x > __tmp_y } {
                // In an Upper-Lower sequence, which always starts with
                // an UpperCase letter, the real deltas always look like:
                //	{0, 1, 0}    UpperCase (Lower is next)
                //	{-1, 0, -1}  LowerCase (Upper, Title are previous)
                // The characters at even offsets from the beginning of the
                // sequence are upper case; the ones at odd offsets are lower.
                // The correct mapping can be done by clearing or setting the low
                // bit in the sequence offset.
                // The constants UpperCase and TitleCase are even while LowerCase
                // is odd so we take the low bit from _case.
        return { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = cr.with_mut(|__ptr_value| __ptr_value.lo.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = cr.with_mut(|__ptr_value| __ptr_value.lo.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = 1 as i32; __tmp_x & ! __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*_case.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }); __tmp_x + __tmp_y };
    }
        // In an Upper-Lower sequence, which always starts with
        // an UpperCase letter, the real deltas always look like:
        //	{0, 1, 0}    UpperCase (Lower is next)
        //	{-1, 0, -1}  LowerCase (Upper, Title are previous)
        // The characters at even offsets from the beginning of the
        // sequence are upper case; the ones at odd offsets are lower.
        // The correct mapping can be done by clearing or setting the low
        // bit in the sequence offset.
        // The constants UpperCase and TitleCase are even while LowerCase
        // is odd so we take the low bit from _case.
    return { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
}

/// to maps the rune using the specified case mapping.
/// It additionally reports whether caseRange contained a mapping for r.
pub fn to_1(_case: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<i32>>>, caseRange: Arc<Mutex<Option<Vec<CaseRange>>>>) -> (i32, bool) {
    let mut mappedRune: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut foundMapping: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    if { let __tmp_x = { let __v = (*_case.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = 3; let __tmp_y = { let __v = (*_case.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        return (REPLACEMENT_CHAR as i32, false);
    }
        // as reasonable an error as any
    {
        let mut cr: Option<GoSliceElemPtr<CaseRange>> = lookup_case_range(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), caseRange.clone());;
        if cr.is_some() {
            return (convert_case(Arc::new(Mutex::new(Some({ let __arg_holder = _case.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::slice_elem_opt(cr.clone())), true);;
        }
    }
    return ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

/// To maps the rune to the specified case: [UpperCase], [LowerCase], or [TitleCase].
pub fn to(_case: Arc<Mutex<Option<i32>>>, mut r: Arc<Mutex<Option<i32>>>) -> i32 {
    { let (__tmp_0, __tmp_1) = to_1(Arc::new(Mutex::new(Some({ let __arg_holder = _case.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), CaseRanges.clone()); *r.lock().unwrap() = Some(__tmp_0); };
    return { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// ToUpper maps the rune to upper case.
pub fn to_upper(mut r: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_A_S_C_I_I as i32; __tmp_x <= __tmp_y } {
        if { let __tmp_x = ('a' as i32); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('z' as i32); __tmp_x <= __tmp_y } {
        { let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y } as i32; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        return { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    to(Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// ToLower maps the rune to lower case.
pub fn to_lower(mut r: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_A_S_C_I_I as i32; __tmp_x <= __tmp_y } {
        if { let __tmp_x = ('A' as i32); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('Z' as i32); __tmp_x <= __tmp_y } {
        { let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y } as i32; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    to(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// ToTitle maps the rune to title case.
pub fn to_title(mut r: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_A_S_C_I_I as i32; __tmp_x <= __tmp_y } {
        if { let __tmp_x = ('a' as i32); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('z' as i32); __tmp_x <= __tmp_y } {
        { let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y } as i32; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        return { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        // title case is upper case for ASCII
    to(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

impl GoValueClone for RangeTable {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Range16 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Range32 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for CaseRange {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for foldPair {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
