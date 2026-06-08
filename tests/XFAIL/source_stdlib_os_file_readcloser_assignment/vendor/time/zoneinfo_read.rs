use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoByteSequence, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{r#mod::{runtime_now}, sys_unix::{closefd, open, preadn, read}, zoneinfo::{ALPHA, Location, OMEGA, tzset, zone, zoneTrans}, zoneinfo_goroot::{goroot_zone_source}};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_FILE_SIZE: i32 = 10 << 20;


pub(crate) const SEEK_START: i32 = 0;
pub(crate) const SEEK_CURRENT: i32 = 1;
pub(crate) const SEEK_END: i32 = 2;


#[derive(Debug, Clone, Default)]
pub struct fileSizeError(pub Arc<Mutex<Option<String>>>);

impl Display for fileSizeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for fileSizeError {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


/// Simple I/O interface to binary blob of data.
#[derive(Debug, Clone)]
pub struct dataIO {
    pub p: Arc<Mutex<Option<Vec<u8>>>>,
    pub error: Arc<Mutex<Option<bool>>>,
}

impl dataIO {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: self.p.clone(), error: { let __guard = self.error.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for dataIO {
    fn default() -> Self {
        Self { p: Arc::new(Mutex::new(None)), error: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for dataIO {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.p), (*self.error.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for dataIO {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static loadFromEmbeddedTZData: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errBadData: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static loadTzinfoFromTzdata: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errBadData.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("malformed time zone information".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errBadData.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *errBadData.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_15() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("malformed time zone information".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errBadData.lock().unwrap() = new_val; }
}


impl fileSizeError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "time: file ".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()).clone()))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " is too large".to_string())); __s })));
    }
}

impl StdError for fileSizeError {}


impl dataIO {
    pub fn read(&mut self, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        if { let __tmp_x = (({ let __len_target = { let __field = self.p.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        *self.p.lock().unwrap() = None;
        { let new_val = true; *self.error.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(None));
    }
        let mut p = Arc::new(Mutex::new(Some({ let __seq_holder = self.p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (0) as usize; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.p = new_val; };
        return p.clone();
    }

    pub fn big4(&mut self) -> (u32, bool) {
    let mut n: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut p = self.read(Arc::new(Mutex::new(Some(4))));
        if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 4; __tmp_x < __tmp_y } {
        { let new_val = true; *self.error.lock().unwrap() = Some(new_val); };
        return (0, false);
    }
        return ({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() } as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 16; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 24; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }, true);
    }

    pub fn big8(&mut self) -> (u64, bool) {
    let mut n: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let (mut n1, mut ok1) = self.big4();
        let (mut n2, mut ok2) = self.big4();
        if !ok1 || !ok2 {
        { let new_val = true; *self.error.lock().unwrap() = Some(new_val); };
        return (0, false);
    }
        return ({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some(n1 as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some(n2 as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, true);
    }

    pub fn byte(&mut self) -> (u8, bool) {
    let mut n: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut p = self.read(Arc::new(Mutex::new(Some(1))));
        if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x < __tmp_y } {
        { let new_val = true; *self.error.lock().unwrap() = Some(new_val); };
        return (0, false);
    }
        return ({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }, true);
    }

    /// rest returns the rest of the data in the buffer.
    pub fn rest(&mut self) -> Arc<Mutex<Option<Vec<u8>>>> {
        let mut r = self.p.clone();
        *self.p.lock().unwrap() = None;
        return r.clone();
    }
}

/// Make a string by stopping at the first NUL
pub fn byte_string(mut p: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<String>>> {
    {
        let mut i = internal_bytealg::index_byte(p.clone(), Arc::new(Mutex::new(Some(0 as u8))));;
        if { let __tmp_x = i; let __tmp_y = -1; __tmp_x != __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };;
        }
    }
    Arc::new(Mutex::new(Some(String::from_utf8((*p.lock().unwrap().as_ref().unwrap()).clone()).unwrap())))
}

/// LoadLocationFromTZData returns a Location with the given name
/// initialized from the IANA Time Zone database-formatted data.
/// The data should be in the format of a standard IANA time zone file
/// (for example, the content of /etc/localtime on Unix systems).
pub fn load_location_from_t_z_data(mut name: Arc<Mutex<Option<String>>>, data: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<crate::zoneinfo::Location>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut d = Arc::new(Mutex::new(Some(dataIO { p: data.clone(), error: Arc::new(Mutex::new(Some(false))), ..Default::default() })));

        // 4-byte magic "TZif"
    {
        let mut magic = (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some(4))));;
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*magic.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "TZif".to_string(); __tmp_x != __tmp_y } {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        }
    }

        // 1-byte version, then 15 bytes of padding
    let mut version: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut p: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    {
        { let new_val = (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some(16)))); p = new_val; };;
        if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 16; __tmp_x != __tmp_y } {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        } else {
            { let _switch_val = { let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
    if _switch_val == (0 as u8) {
            { let new_val = 1; *version.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('2' as i32) as u8) {
            { let new_val = 2; *version.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('3' as i32) as u8) {
            { let new_val = 3; *version.lock().unwrap() = Some(new_val); };
        } else {
            return (Arc::new(Mutex::new(None)), errBadData.clone());
        }
    };
        }
    }

        // six big-endian 32-bit integers:
        //	number of UTC/local indicators
        //	number of standard/wall indicators
        //	number of leap seconds
        //	number of transition times
        //	number of local time zones
        //	number of characters of time zone abbrev strings
    const NUTCLocal: i32 = 0;
const NStdWall: i32 = 1;
const NLeap: i32 = 2;
const NTime: i32 = 3;
const NZone: i32 = 4;
const NChar: i32 = 5;

    let mut n: Arc<Mutex<Option<[i32; 6]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x < __tmp_y } {
        let (mut nn, mut ok) = (*d.lock().unwrap().as_mut().unwrap()).big4();
        if !ok {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(nn as i32 as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = nn; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        (*n.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(nn as i32))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // If we have version 2 or 3, then the data is first written out
        // in a 32-bit format, then written out again in a 64-bit format.
        // Skip the 32-bit format and read the 64-bit one, as it can
        // describe a broader range of dates.
    let mut is64 = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = { let __v = (*version.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
                // Skip the 32-bit data.
        let mut skip = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NTime) as usize].clone() }; let __tmp_y = 4; __tmp_x * __tmp_y }; let __tmp_y = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NTime) as usize].clone() }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NZone) as usize].clone() }; let __tmp_y = 6; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NChar) as usize].clone() }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NLeap) as usize].clone() }; let __tmp_y = 8; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NStdWall) as usize].clone() }; __tmp_x + __tmp_y }; let __tmp_y = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NUTCLocal) as usize].clone() }; __tmp_x + __tmp_y })));
                // Skip the version 2 header that we just read.
        { let __rhs = 20; let mut guard = skip.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = true; *is64.lock().unwrap() = Some(new_val); };
                // Read the counts again, they can differ.
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x < __tmp_y } {
        let (mut nn, mut ok) = (*d.lock().unwrap().as_mut().unwrap()).big4();
        if !ok {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(nn as i32 as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = nn; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        (*n.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(nn as i32))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

        // Skip the 32-bit data.
        // Skip the version 2 header that we just read.
        // Read the counts again, they can differ.
    let mut size = Arc::new(Mutex::new(Some(4)));
    if { let __v = (*is64.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = 8; *size.lock().unwrap() = Some(new_val); };
    }

        // Transition times.
    let mut txtimes = Arc::new(Mutex::new(Some(dataIO { p: (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NTime) as usize].clone() }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))), error: Arc::new(Mutex::new(Some(false))), ..Default::default() })));

        // Time zone indices for transition times.
    let mut txzones = (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NTime) as usize].clone() }))));

        // Zone info structures
    let mut zonedata = Arc::new(Mutex::new(Some(dataIO { p: (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NZone) as usize].clone() }; let __tmp_y = 6; __tmp_x * __tmp_y })))), error: Arc::new(Mutex::new(Some(false))), ..Default::default() })));

        // Time zone abbreviations.
    let mut abbrev = (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NChar) as usize].clone() }))));

        // Leap-second time pairs
    (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NLeap) as usize].clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }))));

        // Whether tx times associated with local time types
        // are specified as standard time or wall time.
    let mut isstd = (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NStdWall) as usize].clone() }))));

        // Whether tx times associated with local time types
        // are specified as UTC or local time.
    let mut isutc = (*d.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NUTCLocal) as usize].clone() }))));

    if (*{ let __field = (*d.lock().unwrap().as_ref().unwrap()).error.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }

    let mut extend: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut rest = (*d.lock().unwrap().as_mut().unwrap()).rest();
    if { let __tmp_x = ((*rest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = rest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = rest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*rest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = rest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = ({ let __tmp_x = ((*rest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *extend.lock().unwrap() = __moved_val; };
    }

        // Now we can build up a useful data structure.
        // First the zone information.
        //	utcoff[4] isdst[1] nameindex[1]
    let mut nzone = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NZone) as usize].clone() })));
    if { let __tmp_x = { let __v = (*nzone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Reject tzdata files with no zones. There's nothing useful in them.
                // This also avoids a panic later when we add and then use a fake transition (golang.org/issue/29437).
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        // Reject tzdata files with no zones. There's nothing useful in them.
        // This also avoids a panic later when we add and then use a fake transition (golang.org/issue/29437).
    let mut zones: Arc<Mutex<Option<Vec<crate::zoneinfo::zone>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __v = (*nzone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    for i in 0..(({ let __range_holder = zones.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let mut n: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*zonedata.lock().unwrap().as_mut().unwrap()).big4(); *n.lock().unwrap() = Some(__tmp_0); *ok.lock().unwrap() = Some(__tmp_1); };;
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        }
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32 as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        { let new_val = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __seq = { let __seq_holder = zones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.offset.lock().unwrap() = __moved_val; };
        let mut b: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*zonedata.lock().unwrap().as_mut().unwrap()).byte(); *b.lock().unwrap() = Some(__tmp_0); *ok.lock().unwrap() = Some(__tmp_1); };;
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        }
    }
        { let new_val = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *{ let __seq = { let __seq_holder = zones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.is_d_s_t.lock().unwrap() = Some(new_val); };
        {
        { let (__tmp_0, __tmp_1) = (*zonedata.lock().unwrap().as_mut().unwrap()).byte(); *b.lock().unwrap() = Some(__tmp_0); *ok.lock().unwrap() = Some(__tmp_1); };;
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = ((*Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*abbrev.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        }
    }
        { let new_val = byte_string(Arc::new(Mutex::new(Some({ let __seq_holder = abbrev.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __seq = { let __seq_holder = zones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.name.lock().unwrap() = __moved_val; };
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "aix".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 8; __tmp_x > __tmp_y } && ({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __high = (8) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Etc/GMT+".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __high = (8) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Etc/GMT-".to_string(); __tmp_x == __tmp_y }) {
                // There is a bug with AIX 7.2 TL 0 with files in Etc,
                // GMT+1 will return GMT-1 instead of GMT+1 or -01.
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Etc/GMT+0".to_string(); __tmp_x != __tmp_y } {
                // GMT+0 is OK
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = (4) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __seq = { let __seq_holder = zones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.name.lock().unwrap() = __moved_val; };
    }
    }
    }

        // There is a bug with AIX 7.2 TL 0 with files in Etc,
        // GMT+1 will return GMT-1 instead of GMT+1 or -01.
        // GMT+0 is OK
        // Now the transition time info.
    let mut tx: Arc<Mutex<Option<Vec<crate::zoneinfo::zoneTrans>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(NTime) as usize].clone() }) as usize])));
    for i in 0..(({ let __range_holder = tx.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if !{ let __v = (*is64.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let (mut n4, mut ok) = (*txtimes.lock().unwrap().as_mut().unwrap()).big4();;
        if !ok {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        } else {
            { let new_val = Arc::new(Mutex::new(Some(n4 as i32 as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };;
        }
    }
    } else {
        {
        let (mut n8, mut ok) = (*txtimes.lock().unwrap().as_mut().unwrap()).big8();;
        if !ok {
            return (Arc::new(Mutex::new(None)), errBadData.clone());;
        } else {
            { let new_val = Arc::new(Mutex::new(Some(n8 as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };;
        }
    }
    }
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.when.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = txzones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*zones.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } {
        return (Arc::new(Mutex::new(None)), errBadData.clone());
    }
        { let new_val = { let __seq = { let __seq_holder = txzones.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; *{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.index.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (i as i32); let __tmp_y = ((*isstd.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __seq = { let __seq_holder = isstd.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.isstd.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (i as i32); let __tmp_y = ((*isutc.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __seq = { let __seq_holder = isutc.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.isutc.lock().unwrap() = Some(new_val); };
    }
    }

    if { let __tmp_x = ((*tx.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Build fake transition to cover all time.
                // This happens in fixed locations like "Etc/GMT0".
        { let new_val = { let __append_target = tx.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(zoneTrans { when: Arc::new(Mutex::new(Some(ALPHA as i64))), index: Arc::new(Mutex::new(Some(0 as u8))), ..Default::default() }); __append_target.clone() }; tx = new_val; };
    }

        // Build fake transition to cover all time.
        // This happens in fixed locations like "Etc/GMT0".
        // Committed to succeed.
    let mut l = Arc::new(Mutex::new(Some(crate::zoneinfo::Location { zone: zones.clone(), tx: tx.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), extend: Arc::new(Mutex::new(Some({ let __arg_holder = extend.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));

        // Fill in the cache with information about right now,
        // since that will be the most common lookup.
    let (mut sec, _, _) = runtime_now();
    for i in 0..(({ let __range_holder = tx.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __tmp_x = (*{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = sec; __tmp_x <= __tmp_y } && ({ let __tmp_x = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*tx.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } || { let __tmp_x = sec; let __tmp_y = (*{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }.when.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y }) {
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*l.lock().unwrap().as_ref().unwrap()).cache_start.lock().unwrap() = Some(new_val); };
        { let new_val = OMEGA as i64; *(*l.lock().unwrap().as_ref().unwrap()).cache_end.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new((*l.lock().unwrap().as_ref().unwrap()).zone.clone(), ((*{ let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.index.lock().unwrap().as_ref().unwrap())) as usize)); (*l.lock().unwrap().as_mut().unwrap()).cache_zone = new_val; };
        if { let __tmp_x = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*tx.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = tx.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*l.lock().unwrap().as_ref().unwrap()).cache_end.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __selector_holder = (*l.lock().unwrap().as_ref().unwrap()).extend.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        {
        let (mut name, mut offset, mut estart, mut eend, mut isDST, mut ok) = tzset(Arc::new(Mutex::new(Some({ let __selector_holder = (*l.lock().unwrap().as_ref().unwrap()).extend.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*l.lock().unwrap().as_ref().unwrap()).cache_start.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(sec))));;
        if ok {
            { let new_val = estart; *(*l.lock().unwrap().as_ref().unwrap()).cache_start.lock().unwrap() = Some(new_val); };;
            { let new_val = eend; *(*l.lock().unwrap().as_ref().unwrap()).cache_end.lock().unwrap() = Some(new_val); };;
            {
        let mut zoneIdx = find_zone({ let __field = (*l.lock().unwrap().as_ref().unwrap()).zone.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(offset))), Arc::new(Mutex::new(Some(isDST))));;
        if { let __tmp_x = zoneIdx; let __tmp_y = -1; __tmp_x != __tmp_y } {
            { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new((*l.lock().unwrap().as_ref().unwrap()).zone.clone(), (zoneIdx) as usize)); (*l.lock().unwrap().as_mut().unwrap()).cache_zone = new_val; };;
        } else {
            { let new_val = GoPtr::local(Arc::new(Mutex::new(Some(zone { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), offset: Arc::new(Mutex::new(Some(offset))), is_d_s_t: Arc::new(Mutex::new(Some(isDST))), ..Default::default() }))).clone()); (*l.lock().unwrap().as_mut().unwrap()).cache_zone = new_val; };;
        }
    };
        }
    }
    }
                // If we're at the end of the known zone transitions,
                // try the extend string.
                // Find the zone that is returned by tzset to avoid allocation if possible.
        break
    }
    }

        // If we're at the end of the known zone transitions,
        // try the extend string.
        // Find the zone that is returned by tzset to avoid allocation if possible.
    return (l.clone(), Arc::new(Mutex::new(None)));
}

pub fn find_zone(zones: Arc<Mutex<Option<Vec<zone>>>>, name: Arc<Mutex<Option<String>>>, offset: Arc<Mutex<Option<i32>>>, isDST: Arc<Mutex<Option<bool>>>) -> i32 {
    { let __range_holder = zones.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, z) in __range_values.iter().enumerate() {
        if { let __tmp_x = { let __selector_holder = z.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __tmp_x = (*z.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = (*z.is_d_s_t.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*isDST.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return i as i32;
    }
    } }
    -(1)
}

/// loadTzinfoFromDirOrZip returns the contents of the file with the given name
/// in dir. dir can either be an uncompressed zip file, or a directory.
pub fn load_tzinfo_from_dir_or_zip(dir: Arc<Mutex<Option<String>>>, mut name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __tmp_x = ((*dir.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x > __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*dir.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = ((*dir.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x - __tmp_y }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".zip".to_string(); __tmp_x == __tmp_y } {
        return load_tzinfo_from_zip(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if { let __tmp_x = (*dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s }; *name.lock().unwrap() = Some(new_val); };
    }
    read_file(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// get4 returns the little-endian 32-bit value in b.
pub fn get4(b: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 4; __tmp_x < __tmp_y } {
        return 0;
    }
    return { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 16; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 24; __tmp_x << __tmp_y }; __tmp_x | __tmp_y };
}

/// get2 returns the little-endian 16-bit value in b.
pub fn get2(b: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        return 0;
    }
    return { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y };
}

/// loadTzinfoFromZip returns the contents of the file with the given name
/// in the given uncompressed zip file.
pub fn load_tzinfo_from_zip(zipfile: Arc<Mutex<Option<String>>>, name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (mut fd, mut err) = open(Arc::new(Mutex::new(Some({ let __arg_holder = zipfile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    }
        let fd_defer_captured = fd.clone(); __defer_stack.push(Box::new(move || {
        closefd(Arc::new(Mutex::new(Some(fd_defer_captured))));
    }));

        const zecheader: i32 = 0x06054b50;
const zcheader: i32 = 0x02014b50;
const ztailsize: i32 = 22;
const zheadersize: i32 = 30;
const zheader: i32 = 0x04034b50;


        let mut buf = Arc::new(Mutex::new(Some(vec![0; (ztailsize) as usize])));
        {
        let mut err = preadn(Arc::new(Mutex::new(Some(fd))), buf.clone(), Arc::new(Mutex::new(Some(-22))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = get4(buf.clone()); let __tmp_y = 101010256; __tmp_x != __tmp_y } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", "corrupt zip file ".to_string(), { let __v = (*zipfile.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    };
        }
    }
        let mut n = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (10) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut size = get4(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (12) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut off = get4(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (16) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));

        { let new_val = Arc::new(Mutex::new(Some(vec![0; (size) as usize]))); buf = new_val; };
        {
        let mut err = preadn(Arc::new(Mutex::new(Some(fd))), buf.clone(), Arc::new(Mutex::new(Some(off))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", "corrupt zip file ".to_string(), { let __v = (*zipfile.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    };
        }
    }

        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = n; __tmp_x < __tmp_y } {
                // zip entry layout:
                //	0	magic[4]
                //	4	madevers[1]
                //	5	madeos[1]
                //	6	extvers[1]
                //	7	extos[1]
                //	8	flags[2]
                //	10	meth[2]
                //	12	modtime[2]
                //	14	moddate[2]
                //	16	crc[4]
                //	20	csize[4]
                //	24	uncsize[4]
                //	28	namelen[2]
                //	30	xlen[2]
                //	32	fclen[2]
                //	34	disknum[2]
                //	36	iattr[2]
                //	38	eattr[4]
                //	42	off[4]
                //	46	name[namelen]
                //	46+namelen+xlen+fclen - next header
                //
        if { let __tmp_x = get4(buf.clone()); let __tmp_y = 33639248; __tmp_x != __tmp_y } {
        break
    }
        let mut meth = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (10) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut size = get4(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (24) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut namelen = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (28) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut xlen = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (30) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut fclen = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (32) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut off = get4(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (42) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut zname = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (46) as usize; let __high = ({ let __tmp_x = 46; let __tmp_y = namelen; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __tmp_x = { let __tmp_x = 46; let __tmp_y = namelen; __tmp_x + __tmp_y }; let __tmp_y = xlen; __tmp_x + __tmp_y }; let __tmp_y = fclen; __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); buf = new_val; };
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*zname.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        if { let __tmp_x = meth; let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "unsupported compression for ".to_string())); __s.push_str(&format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", " in ".to_string())); __s.push_str(&format!("{}", { let __v = (*zipfile.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s })))));
    }
    }

                // zip per-file header layout:
                //	0	magic[4]
                //	4	extvers[1]
                //	5	extos[1]
                //	6	flags[2]
                //	8	meth[2]
                //	10	modtime[2]
                //	12	moddate[2]
                //	14	crc[4]
                //	18	csize[4]
                //	22	uncsize[4]
                //	26	namelen[2]
                //	28	xlen[2]
                //	30	name[namelen]
                //	30+namelen+xlen - file data
                //
        { let new_val = Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = 30; let __tmp_y = namelen; __tmp_x + __tmp_y }) as usize]))); buf = new_val; };
        {
        let mut err = preadn(Arc::new(Mutex::new(Some(fd))), buf.clone(), Arc::new(Mutex::new(Some(off))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = get4(buf.clone()); let __tmp_y = 67324752; __tmp_x != __tmp_y } || { let __tmp_x = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (8) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); let __tmp_y = meth; __tmp_x != __tmp_y } || { let __tmp_x = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (26) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); let __tmp_y = namelen; __tmp_x != __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (30) as usize; let __high = ({ let __tmp_x = 30; let __tmp_y = namelen; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", "corrupt zip file ".to_string(), { let __v = (*zipfile.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    };
        }
    }
        { let new_val = get2(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (28) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); xlen = new_val; };

        { let new_val = Arc::new(Mutex::new(Some(vec![0; (size) as usize]))); buf = new_val; };
        {
        let mut err = preadn(Arc::new(Mutex::new(Some(fd))), buf.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = off; let __tmp_y = 30; __tmp_x + __tmp_y }; let __tmp_y = namelen; __tmp_x + __tmp_y }; let __tmp_y = xlen; __tmp_x + __tmp_y }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", "corrupt zip file ".to_string(), { let __v = (*zipfile.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    };
        }
    }

        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (buf.clone(), Arc::new(Mutex::new(None)));
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // zip entry layout:
                //	0	magic[4]
                //	4	madevers[1]
                //	5	madeos[1]
                //	6	extvers[1]
                //	7	extos[1]
                //	8	flags[2]
                //	10	meth[2]
                //	12	modtime[2]
                //	14	moddate[2]
                //	16	crc[4]
                //	20	csize[4]
                //	24	uncsize[4]
                //	28	namelen[2]
                //	30	xlen[2]
                //	32	fclen[2]
                //	34	disknum[2]
                //	36	iattr[2]
                //	38	eattr[4]
                //	42	off[4]
                //	46	name[namelen]
                //	46+namelen+xlen+fclen - next header
                //
                // zip per-file header layout:
                //	0	magic[4]
                //	4	extvers[1]
                //	5	extos[1]
                //	6	flags[2]
                //	8	meth[2]
                //	10	modtime[2]
                //	12	moddate[2]
                //	14	crc[4]
                //	18	csize[4]
                //	22	uncsize[4]
                //	26	namelen[2]
                //	28	xlen[2]
                //	30	name[namelen]
                //	30+namelen+xlen - file data
                //
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_N_O_E_N_T as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))
        }
    }
}

/// loadTzinfo returns the time zone information of the time zone
/// with the given name, from a given source. A source may be a
/// timezone database directory, tzdata database file or an uncompressed
/// zip file, containing the contents of such a directory.
pub fn load_tzinfo(name: Arc<Mutex<Option<String>>>, source: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __tmp_x = ((*source.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 6; __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*source.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = ((*source.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 6; __tmp_x - __tmp_y }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "tzdata".to_string(); __tmp_x == __tmp_y } {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = loadTzinfoFromTzdata.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = source.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
    }
    load_tzinfo_from_dir_or_zip(Arc::new(Mutex::new(Some({ let __arg_holder = source.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// loadLocation returns the Location with the given name from one of
/// the specified sources. See loadTzinfo for a list of supported sources.
/// The first timezone data matching the given name that is successfully loaded
/// and parsed is returned as a Location.
pub fn load_location_1(name: Arc<Mutex<Option<String>>>, sources: Arc<Mutex<Option<Vec<String>>>>) -> (Arc<Mutex<Option<crate::zoneinfo::Location>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut z: Arc<Mutex<Option<Location>>> = Arc::new(Mutex::new(None));
    let mut firstErr: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    { let __range_holder = sources.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for source in __range_values.iter() {
        let (mut zoneData, mut err) = load_tzinfo(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*source).clone()))));
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        {
        { let (__tmp_0, __tmp_1) = load_location_from_t_z_data(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), zoneData.clone()); z = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return (z.clone(), Arc::new(Mutex::new(None)));;
        }
    }
    }
        if { let __nil_result = (*firstErr.lock().unwrap()).is_none(); __nil_result } && { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_N_O_E_N_T as usize)).unwrap_or(false); !__matched } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *firstErr.lock().unwrap() = new_val; };
    }
    } }
    if { let __nil_result = (*loadFromEmbeddedTZData.lock().unwrap()).is_some(); __nil_result } {
        let (mut zoneData, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = loadFromEmbeddedTZData.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        {
        { let (__tmp_0, __tmp_1) = load_location_from_t_z_data(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(({ let __v = (*zoneData.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec())))); z = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return (z.clone(), Arc::new(Mutex::new(None)));;
        }
    }
    }
        if { let __nil_result = (*firstErr.lock().unwrap()).is_none(); __nil_result } && { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_N_O_E_N_T as usize)).unwrap_or(false); !__matched } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *firstErr.lock().unwrap() = new_val; };
    }
    }
    {
        let (mut source, mut ok) = goroot_zone_source(runtime::g_o_r_o_o_t());;
        if ok {
            let (mut zoneData, mut err) = load_tzinfo(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = source.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        {
        { let (__tmp_0, __tmp_1) = load_location_from_t_z_data(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), zoneData.clone()); z = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return (z.clone(), Arc::new(Mutex::new(None)));;
        }
    }
    };
            if { let __nil_result = (*firstErr.lock().unwrap()).is_none(); __nil_result } && { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_N_O_E_N_T as usize)).unwrap_or(false); !__matched } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *firstErr.lock().unwrap() = new_val; };
    };
        }
    }
    if { let __nil_result = (*firstErr.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), firstErr.clone());
    }
    return (Arc::new(Mutex::new(None)), errors::new(Arc::new(Mutex::new(Some(format!("{}{}", "unknown time zone ".to_string(), { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
}

/// readFile reads and returns the content of the named file.
/// It is a trivial implementation of os.ReadFile, reimplemented
/// here to avoid depending on io/ioutil or os.
/// It returns an error if name exceeds maxFileSize bytes.
pub fn read_file(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (mut f, mut err) = open(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    }
        let f_defer_captured = f.clone(); __defer_stack.push(Box::new(move || {
        closefd(Arc::new(Mutex::new(Some(f_defer_captured))));
    }));
        let mut buf: Arc<Mutex<Option<[u8; 4096]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));let mut ret: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        loop {
        { let (__tmp_0, __tmp_1) = read(Arc::new(Mutex::new(Some(f))), Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = ret.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; ret = new_val; };
    }
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        break
    }
        if { let __tmp_x = ((*ret.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 10485760; __tmp_x > __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(fileSizeError(Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn StdError + Send + Sync>))));
    }
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (ret.clone(), err.clone());
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for dataIO {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
