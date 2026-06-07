use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::badlinkname_unix::*;
use crate::bpf_bsd::*;
use crate::dirent::*;
use crate::env_unix::*;
use crate::exec_libc2::*;
use crate::exec_unix::*;
use crate::flock_bsd::*;
use crate::forkpipe::*;
use crate::linkname_bsd::*;
use crate::linkname_darwin::*;
use crate::linkname_libc::*;
use crate::linkname_unix::*;
use crate::net::*;
use crate::rlimit::*;
use crate::rlimit_darwin::*;
use crate::route_bsd::*;
use crate::route_darwin::*;
use crate::sockcmsg_unix::*;
use crate::sockcmsg_unix_other::*;
use crate::r#mod::*;
use crate::syscall_bsd::*;
use crate::syscall_darwin::*;
use crate::syscall_darwin_arm64::*;
use crate::syscall_unix::*;
use crate::time_nofake::*;
use crate::timestruct::*;
use crate::zerrors_darwin_arm64::*;
use crate::zsyscall_darwin_arm64::*;
use crate::zsysnum_darwin_arm64::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SIZEOF_PTR: i32 = 0x8;
pub(crate) const SIZEOF_SHORT: i32 = 0x2;
pub(crate) const SIZEOF_INT: i32 = 0x4;
pub(crate) const SIZEOF_LONG: i32 = 0x8;
pub(crate) const SIZEOF_LONG_LONG: i32 = 0x8;


pub(crate) const PATH_MAX: i32 = 0x400;


pub const SIZEOF_SOCKADDR_INET4: i32 = 0x10;
pub const SIZEOF_SOCKADDR_INET6: i32 = 0x1c;
pub const SIZEOF_SOCKADDR_ANY: i32 = 0x6c;
pub const SIZEOF_SOCKADDR_UNIX: i32 = 0x6a;
pub const SIZEOF_SOCKADDR_DATALINK: i32 = 0x14;
pub const SIZEOF_LINGER: i32 = 0x8;
pub const SIZEOF_I_P_MREQ: i32 = 0x8;
pub const SIZEOF_I_PV6_MREQ: i32 = 0x14;
pub const SIZEOF_MSGHDR: i32 = 0x30;
pub const SIZEOF_CMSGHDR: i32 = 0xc;
pub const SIZEOF_INET4_PKTINFO: i32 = 0xc;
pub const SIZEOF_INET6_PKTINFO: i32 = 0x14;
pub const SIZEOF_I_PV6_M_T_U_INFO: i32 = 0x20;
pub const SIZEOF_I_C_M_PV6_FILTER: i32 = 0x20;


pub const P_T_R_A_C_E__T_R_A_C_E_M_E: i32 = 0x0;
pub const P_T_R_A_C_E__C_O_N_T: i32 = 0x7;
pub const P_T_R_A_C_E__K_I_L_L: i32 = 0x8;


pub const SIZEOF_IF_MSGHDR: i32 = 0x70;
pub const SIZEOF_IF_DATA: i32 = 0x60;
pub const SIZEOF_IFA_MSGHDR: i32 = 0x14;
pub const SIZEOF_IFMA_MSGHDR: i32 = 0x10;
pub const SIZEOF_IFMA_MSGHDR2: i32 = 0x14;
pub const SIZEOF_RT_MSGHDR: i32 = 0x5c;
pub const SIZEOF_RT_METRICS: i32 = 0x38;


pub const SIZEOF_BPF_VERSION: i32 = 0x4;
pub const SIZEOF_BPF_STAT: i32 = 0x8;
pub const SIZEOF_BPF_PROGRAM: i32 = 0x10;
pub const SIZEOF_BPF_INSN: i32 = 0x8;
pub const SIZEOF_BPF_HDR: i32 = 0x14;


pub(crate) const __A_T__F_D_C_W_D: i32 = -0x2;


#[derive(Debug, Clone, Default)]
pub struct _C_int(pub Arc<Mutex<Option<i32>>>);

impl Display for _C_int {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for _C_int {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for _C_int {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for _C_int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for _C_int {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<_C_int> for i32 {
    fn eq(&self, other: &_C_int) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<_C_int> for i32 {
    fn partial_cmp(&self, other: &_C_int) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for _C_int {
    type Output = _C_int;
    fn add(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for _C_int {
    type Output = _C_int;
    fn add(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<_C_int> for i32 {
    type Output = _C_int;
    fn add(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for _C_int {
    type Output = _C_int;
    fn sub(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for _C_int {
    type Output = _C_int;
    fn sub(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<_C_int> for i32 {
    type Output = _C_int;
    fn sub(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for _C_int {
    type Output = _C_int;
    fn mul(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for _C_int {
    type Output = _C_int;
    fn mul(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<_C_int> for i32 {
    type Output = _C_int;
    fn mul(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for _C_int {
    type Output = _C_int;
    fn div(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for _C_int {
    type Output = _C_int;
    fn div(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<_C_int> for i32 {
    type Output = _C_int;
    fn div(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for _C_int {
    type Output = _C_int;
    fn neg(self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for _C_int {
    type Output = _C_int;
    fn rem(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for _C_int {
    type Output = _C_int;
    fn rem(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<_C_int> for i32 {
    type Output = _C_int;
    fn rem(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for _C_int {
    type Output = _C_int;
    fn bitand(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for _C_int {
    type Output = _C_int;
    fn bitand(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<_C_int> for i32 {
    type Output = _C_int;
    fn bitand(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for _C_int {
    type Output = _C_int;
    fn bitor(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for _C_int {
    type Output = _C_int;
    fn bitor(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<_C_int> for i32 {
    type Output = _C_int;
    fn bitor(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for _C_int {
    type Output = _C_int;
    fn bitxor(self, other: Self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for _C_int {
    type Output = _C_int;
    fn bitxor(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<_C_int> for i32 {
    type Output = _C_int;
    fn bitxor(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for _C_int {
    type Output = _C_int;
    fn not(self) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for _C_int {
    type Output = _C_int;
    fn shl(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for _C_int {
    type Output = _C_int;
    fn shl(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for _C_int {
    type Output = _C_int;
    fn shl(self, other: i8) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for _C_int {
    type Output = _C_int;
    fn shl(self, other: i16) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for _C_int {
    type Output = _C_int;
    fn shl(self, other: i64) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for _C_int {
    type Output = _C_int;
    fn shl(self, other: u32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for _C_int {
    type Output = _C_int;
    fn shl(self, other: u8) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for _C_int {
    type Output = _C_int;
    fn shl(self, other: u16) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for _C_int {
    type Output = _C_int;
    fn shl(self, other: u64) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for _C_int {
    type Output = _C_int;
    fn shl(self, other: usize) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for _C_int {
    type Output = _C_int;
    fn shr(self, other: _C_int) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for _C_int {
    type Output = _C_int;
    fn shr(self, other: i32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for _C_int {
    type Output = _C_int;
    fn shr(self, other: i8) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for _C_int {
    type Output = _C_int;
    fn shr(self, other: i16) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for _C_int {
    type Output = _C_int;
    fn shr(self, other: i64) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for _C_int {
    type Output = _C_int;
    fn shr(self, other: u32) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for _C_int {
    type Output = _C_int;
    fn shr(self, other: u8) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for _C_int {
    type Output = _C_int;
    fn shr(self, other: u16) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for _C_int {
    type Output = _C_int;
    fn shr(self, other: u64) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for _C_int {
    type Output = _C_int;
    fn shr(self, other: usize) -> _C_int {
        _C_int(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for _C_int {}

impl Ord for _C_int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Timespec {
    pub sec: Arc<Mutex<Option<i64>>>,
    pub nsec: Arc<Mutex<Option<i64>>>,
}

impl Timespec {
    pub fn __go_value_clone(&self) -> Self {
        Self { sec: { let __guard = self.sec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nsec: { let __guard = self.nsec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Timespec {
    fn default() -> Self {
        Self { sec: Arc::new(Mutex::new(Some(0))), nsec: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Timespec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.sec.lock().unwrap().as_ref().unwrap()), (*self.nsec.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Timespec {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Sec") {
            out.sec = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Nsec") {
            out.nsec = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Timeval32 {
    pub sec: Arc<Mutex<Option<i32>>>,
    pub usec: Arc<Mutex<Option<i32>>>,
}

impl Timeval32 {
    pub fn __go_value_clone(&self) -> Self {
        Self { sec: { let __guard = self.sec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, usec: { let __guard = self.usec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Timeval32 {
    fn default() -> Self {
        Self { sec: Arc::new(Mutex::new(Some(0))), usec: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Timeval32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.sec.lock().unwrap().as_ref().unwrap()), (*self.usec.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Timeval32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Sec") {
            out.sec = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Usec") {
            out.usec = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Rlimit {
    pub cur: Arc<Mutex<Option<u64>>>,
    pub max: Arc<Mutex<Option<u64>>>,
}

impl Rlimit {
    pub fn __go_value_clone(&self) -> Self {
        Self { cur: { let __guard = self.cur.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, max: { let __guard = self.max.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Rlimit {
    fn default() -> Self {
        Self { cur: Arc::new(Mutex::new(Some(0))), max: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Rlimit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.cur.lock().unwrap().as_ref().unwrap()), (*self.max.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Rlimit {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Cur") {
            out.cur = <Arc<Mutex<Option<u64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Max") {
            out.max = <Arc<Mutex<Option<u64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Stat_t {
    pub dev: Arc<Mutex<Option<i32>>>,
    pub mode: Arc<Mutex<Option<u16>>>,
    pub nlink: Arc<Mutex<Option<u16>>>,
    pub ino: Arc<Mutex<Option<u64>>>,
    pub uid: Arc<Mutex<Option<u32>>>,
    pub gid: Arc<Mutex<Option<u32>>>,
    pub rdev: Arc<Mutex<Option<i32>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 4]>>>,
    pub atimespec: Arc<Mutex<Option<Timespec>>>,
    pub mtimespec: Arc<Mutex<Option<Timespec>>>,
    pub ctimespec: Arc<Mutex<Option<Timespec>>>,
    pub birthtimespec: Arc<Mutex<Option<Timespec>>>,
    pub size: Arc<Mutex<Option<i64>>>,
    pub blocks: Arc<Mutex<Option<i64>>>,
    pub blksize: Arc<Mutex<Option<i32>>>,
    pub flags: Arc<Mutex<Option<u32>>>,
    pub gen: Arc<Mutex<Option<u32>>>,
    pub lspare: Arc<Mutex<Option<i32>>>,
    pub qspare: Arc<Mutex<Option<[i64; 2]>>>,
}

impl Stat_t {
    pub fn __go_value_clone(&self) -> Self {
        Self { dev: { let __guard = self.dev.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nlink: { let __guard = self.nlink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ino: { let __guard = self.ino.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, uid: { let __guard = self.uid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gid: { let __guard = self.gid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rdev: { let __guard = self.rdev.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, atimespec: { let __guard = self.atimespec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mtimespec: { let __guard = self.mtimespec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ctimespec: { let __guard = self.ctimespec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, birthtimespec: { let __guard = self.birthtimespec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, blocks: { let __guard = self.blocks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, blksize: { let __guard = self.blksize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gen: { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lspare: { let __guard = self.lspare.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, qspare: { let __guard = self.qspare.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Stat_t {
    fn default() -> Self {
        Self { dev: Arc::new(Mutex::new(Some(0))), mode: Arc::new(Mutex::new(Some(0))), nlink: Arc::new(Mutex::new(Some(0))), ino: Arc::new(Mutex::new(Some(0))), uid: Arc::new(Mutex::new(Some(0))), gid: Arc::new(Mutex::new(Some(0))), rdev: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), atimespec: Arc::new(Mutex::new(Some(Timespec::default()))), mtimespec: Arc::new(Mutex::new(Some(Timespec::default()))), ctimespec: Arc::new(Mutex::new(Some(Timespec::default()))), birthtimespec: Arc::new(Mutex::new(Some(Timespec::default()))), size: Arc::new(Mutex::new(Some(0))), blocks: Arc::new(Mutex::new(Some(0))), blksize: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))), gen: Arc::new(Mutex::new(Some(0))), lspare: Arc::new(Mutex::new(Some(0))), qspare: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for Stat_t {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.dev.lock().unwrap().as_ref().unwrap()), (*self.mode.lock().unwrap().as_ref().unwrap()), (*self.nlink.lock().unwrap().as_ref().unwrap()), (*self.ino.lock().unwrap().as_ref().unwrap()), (*self.uid.lock().unwrap().as_ref().unwrap()), (*self.gid.lock().unwrap().as_ref().unwrap()), (*self.rdev.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0), (*self.atimespec.lock().unwrap().as_ref().unwrap()), (*self.mtimespec.lock().unwrap().as_ref().unwrap()), (*self.ctimespec.lock().unwrap().as_ref().unwrap()), (*self.birthtimespec.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()), (*self.blocks.lock().unwrap().as_ref().unwrap()), (*self.blksize.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.gen.lock().unwrap().as_ref().unwrap()), (*self.lspare.lock().unwrap().as_ref().unwrap()), format_slice(&self.qspare))
    }
}

impl GoJsonDecode for Stat_t {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Dev") {
            out.dev = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Mode") {
            out.mode = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Nlink") {
            out.nlink = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ino") {
            out.ino = <Arc<Mutex<Option<u64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Uid") {
            out.uid = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Gid") {
            out.gid = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Rdev") {
            out.rdev = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Size") {
            out.size = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Blocks") {
            out.blocks = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Blksize") {
            out.blksize = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Gen") {
            out.gen = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Lspare") {
            out.lspare = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Qspare") {
            out.qspare = <Arc<Mutex<Option<[i64; 2]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Dirent {
    pub ino: Arc<Mutex<Option<u64>>>,
    pub seekoff: Arc<Mutex<Option<u64>>>,
    pub reclen: Arc<Mutex<Option<u16>>>,
    pub namlen: Arc<Mutex<Option<u16>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub name: Arc<Mutex<Option<[i8; 1024]>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 3]>>>,
}

impl Dirent {
    pub fn __go_value_clone(&self) -> Self {
        Self { ino: { let __guard = self.ino.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seekoff: { let __guard = self.seekoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reclen: { let __guard = self.reclen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, namlen: { let __guard = self.namlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Dirent {
    fn default() -> Self {
        Self { ino: Arc::new(Mutex::new(Some(0))), seekoff: Arc::new(Mutex::new(Some(0))), reclen: Arc::new(Mutex::new(Some(0))), namlen: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))), name: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for Dirent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.ino.lock().unwrap().as_ref().unwrap()), (*self.seekoff.lock().unwrap().as_ref().unwrap()), (*self.reclen.lock().unwrap().as_ref().unwrap()), (*self.namlen.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), format_slice(&self.name), format_slice(&self.pad_cgo_0))
    }
}

impl GoJsonDecode for Dirent {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Ino") {
            out.ino = <Arc<Mutex<Option<u64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Seekoff") {
            out.seekoff = <Arc<Mutex<Option<u64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Reclen") {
            out.reclen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Namlen") {
            out.namlen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<[i8; 1024]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 3]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RawSockaddrInet4 {
    pub len: Arc<Mutex<Option<u8>>>,
    pub family: Arc<Mutex<Option<u8>>>,
    pub port: Arc<Mutex<Option<u16>>>,
    pub addr: Arc<Mutex<Option<[u8; 4]>>>,
    pub zero: Arc<Mutex<Option<[i8; 8]>>>,
}

impl RawSockaddrInet4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, family: { let __guard = self.family.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, port: { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, zero: { let __guard = self.zero.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RawSockaddrInet4 {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), family: Arc::new(Mutex::new(Some(0))), port: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), zero: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for RawSockaddrInet4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), (*self.family.lock().unwrap().as_ref().unwrap()), (*self.port.lock().unwrap().as_ref().unwrap()), format_slice(&self.addr), format_slice(&self.zero))
    }
}

impl GoJsonDecode for RawSockaddrInet4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Family") {
            out.family = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Port") {
            out.port = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addr") {
            out.addr = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Zero") {
            out.zero = <Arc<Mutex<Option<[i8; 8]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RawSockaddrInet6 {
    pub len: Arc<Mutex<Option<u8>>>,
    pub family: Arc<Mutex<Option<u8>>>,
    pub port: Arc<Mutex<Option<u16>>>,
    pub flowinfo: Arc<Mutex<Option<u32>>>,
    pub addr: Arc<Mutex<Option<[u8; 16]>>>,
    pub scope_id: Arc<Mutex<Option<u32>>>,
}

impl RawSockaddrInet6 {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, family: { let __guard = self.family.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, port: { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flowinfo: { let __guard = self.flowinfo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scope_id: { let __guard = self.scope_id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RawSockaddrInet6 {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), family: Arc::new(Mutex::new(Some(0))), port: Arc::new(Mutex::new(Some(0))), flowinfo: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), scope_id: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for RawSockaddrInet6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), (*self.family.lock().unwrap().as_ref().unwrap()), (*self.port.lock().unwrap().as_ref().unwrap()), (*self.flowinfo.lock().unwrap().as_ref().unwrap()), format_slice(&self.addr), (*self.scope_id.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for RawSockaddrInet6 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Family") {
            out.family = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Port") {
            out.port = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flowinfo") {
            out.flowinfo = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addr") {
            out.addr = <Arc<Mutex<Option<[u8; 16]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Scope_id") {
            out.scope_id = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RawSockaddrUnix {
    pub len: Arc<Mutex<Option<u8>>>,
    pub family: Arc<Mutex<Option<u8>>>,
    pub path: Arc<Mutex<Option<[i8; 104]>>>,
}

impl RawSockaddrUnix {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, family: { let __guard = self.family.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, path: { let __guard = self.path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RawSockaddrUnix {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), family: Arc::new(Mutex::new(Some(0))), path: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for RawSockaddrUnix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), (*self.family.lock().unwrap().as_ref().unwrap()), format_slice(&self.path))
    }
}

impl GoJsonDecode for RawSockaddrUnix {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Family") {
            out.family = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Path") {
            out.path = <Arc<Mutex<Option<[i8; 104]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RawSockaddrDatalink {
    pub len: Arc<Mutex<Option<u8>>>,
    pub family: Arc<Mutex<Option<u8>>>,
    pub index: Arc<Mutex<Option<u16>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub nlen: Arc<Mutex<Option<u8>>>,
    pub alen: Arc<Mutex<Option<u8>>>,
    pub slen: Arc<Mutex<Option<u8>>>,
    pub data: Arc<Mutex<Option<[i8; 12]>>>,
}

impl RawSockaddrDatalink {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, family: { let __guard = self.family.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nlen: { let __guard = self.nlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alen: { let __guard = self.alen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, slen: { let __guard = self.slen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RawSockaddrDatalink {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), family: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))), nlen: Arc::new(Mutex::new(Some(0))), alen: Arc::new(Mutex::new(Some(0))), slen: Arc::new(Mutex::new(Some(0))), data: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for RawSockaddrDatalink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), (*self.family.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.nlen.lock().unwrap().as_ref().unwrap()), (*self.alen.lock().unwrap().as_ref().unwrap()), (*self.slen.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for RawSockaddrDatalink {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Family") {
            out.family = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Index") {
            out.index = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Nlen") {
            out.nlen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Alen") {
            out.alen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Slen") {
            out.slen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<[i8; 12]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RawSockaddr {
    pub len: Arc<Mutex<Option<u8>>>,
    pub family: Arc<Mutex<Option<u8>>>,
    pub data: Arc<Mutex<Option<[i8; 14]>>>,
}

impl RawSockaddr {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, family: { let __guard = self.family.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RawSockaddr {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), family: Arc::new(Mutex::new(Some(0))), data: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for RawSockaddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), (*self.family.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for RawSockaddr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Family") {
            out.family = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<[i8; 14]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RawSockaddrAny {
    pub addr: Arc<Mutex<Option<RawSockaddr>>>,
    pub pad: Arc<Mutex<Option<[i8; 92]>>>,
}

impl RawSockaddrAny {
    pub fn __go_value_clone(&self) -> Self {
        Self { addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RawSockaddrAny {
    fn default() -> Self {
        Self { addr: Arc::new(Mutex::new(Some(RawSockaddr::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for RawSockaddrAny {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.addr.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for RawSockaddrAny {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Pad") {
            out.pad = <Arc<Mutex<Option<[i8; 92]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct _Socklen(pub Arc<Mutex<Option<u32>>>);

impl Display for _Socklen {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for _Socklen {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for _Socklen {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for _Socklen {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for _Socklen {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<_Socklen> for u32 {
    fn eq(&self, other: &_Socklen) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<_Socklen> for u32 {
    fn partial_cmp(&self, other: &_Socklen) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for _Socklen {
    type Output = _Socklen;
    fn add(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for _Socklen {
    type Output = _Socklen;
    fn add(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<_Socklen> for u32 {
    type Output = _Socklen;
    fn add(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for _Socklen {
    type Output = _Socklen;
    fn sub(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for _Socklen {
    type Output = _Socklen;
    fn sub(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<_Socklen> for u32 {
    type Output = _Socklen;
    fn sub(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for _Socklen {
    type Output = _Socklen;
    fn mul(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for _Socklen {
    type Output = _Socklen;
    fn mul(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<_Socklen> for u32 {
    type Output = _Socklen;
    fn mul(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for _Socklen {
    type Output = _Socklen;
    fn div(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for _Socklen {
    type Output = _Socklen;
    fn div(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<_Socklen> for u32 {
    type Output = _Socklen;
    fn div(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for _Socklen {
    type Output = _Socklen;
    fn rem(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for _Socklen {
    type Output = _Socklen;
    fn rem(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<_Socklen> for u32 {
    type Output = _Socklen;
    fn rem(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for _Socklen {
    type Output = _Socklen;
    fn bitand(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for _Socklen {
    type Output = _Socklen;
    fn bitand(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<_Socklen> for u32 {
    type Output = _Socklen;
    fn bitand(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for _Socklen {
    type Output = _Socklen;
    fn bitor(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for _Socklen {
    type Output = _Socklen;
    fn bitor(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<_Socklen> for u32 {
    type Output = _Socklen;
    fn bitor(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for _Socklen {
    type Output = _Socklen;
    fn bitxor(self, other: Self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for _Socklen {
    type Output = _Socklen;
    fn bitxor(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<_Socklen> for u32 {
    type Output = _Socklen;
    fn bitxor(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for _Socklen {
    type Output = _Socklen;
    fn not(self) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: i32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: i8) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: i16) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: i64) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: u8) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: u16) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: u64) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for _Socklen {
    type Output = _Socklen;
    fn shl(self, other: usize) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: _Socklen) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: i32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: i8) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: i16) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: i64) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: u32) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: u8) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: u16) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: u64) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for _Socklen {
    type Output = _Socklen;
    fn shr(self, other: usize) -> _Socklen {
        _Socklen(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for _Socklen {}

impl Ord for _Socklen {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Linger {
    pub onoff: Arc<Mutex<Option<i32>>>,
    pub linger: Arc<Mutex<Option<i32>>>,
}

impl Linger {
    pub fn __go_value_clone(&self) -> Self {
        Self { onoff: { let __guard = self.onoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, linger: { let __guard = self.linger.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Linger {
    fn default() -> Self {
        Self { onoff: Arc::new(Mutex::new(Some(0))), linger: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Linger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.onoff.lock().unwrap().as_ref().unwrap()), (*self.linger.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Linger {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Onoff") {
            out.onoff = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Linger") {
            out.linger = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Iovec {
    pub base: GoPtr<u8>,
    pub len: Arc<Mutex<Option<u64>>>,
}

impl Iovec {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: self.base.clone(), len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Iovec {
    fn default() -> Self {
        Self { base: GoPtr::nil(), len: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Iovec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { if self.base.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.len.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Iovec {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Base") {
            out.base = GoPtr::local(<Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?);
        }
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct IPMreq {
    pub multiaddr: Arc<Mutex<Option<[u8; 4]>>>,
    pub interface: Arc<Mutex<Option<[u8; 4]>>>,
}

impl IPMreq {
    pub fn __go_value_clone(&self) -> Self {
        Self { multiaddr: { let __guard = self.multiaddr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, interface: { let __guard = self.interface.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IPMreq {
    fn default() -> Self {
        Self { multiaddr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), interface: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for IPMreq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.multiaddr), format_slice(&self.interface))
    }
}

impl GoJsonDecode for IPMreq {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Multiaddr") {
            out.multiaddr = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Interface") {
            out.interface = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct IPv6Mreq {
    pub multiaddr: Arc<Mutex<Option<[u8; 16]>>>,
    pub interface: Arc<Mutex<Option<u32>>>,
}

impl IPv6Mreq {
    pub fn __go_value_clone(&self) -> Self {
        Self { multiaddr: { let __guard = self.multiaddr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, interface: { let __guard = self.interface.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IPv6Mreq {
    fn default() -> Self {
        Self { multiaddr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), interface: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for IPv6Mreq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.multiaddr), (*self.interface.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IPv6Mreq {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Multiaddr") {
            out.multiaddr = <Arc<Mutex<Option<[u8; 16]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Interface") {
            out.interface = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Msghdr {
    pub name: Arc<Mutex<Option<u8>>>,
    pub namelen: Arc<Mutex<Option<u32>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 4]>>>,
    pub iov: Arc<Mutex<Option<Iovec>>>,
    pub iovlen: Arc<Mutex<Option<i32>>>,
    pub pad_cgo_1: Arc<Mutex<Option<[u8; 4]>>>,
    pub control: GoPtr<u8>,
    pub controllen: Arc<Mutex<Option<u32>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
}

impl Msghdr {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: self.name.clone(), namelen: { let __guard = self.namelen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, iov: self.iov.clone(), iovlen: { let __guard = self.iovlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_1: { let __guard = self.pad_cgo_1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, control: self.control.clone(), controllen: { let __guard = self.controllen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Msghdr {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(None)), namelen: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), iov: Arc::new(Mutex::new(None)), iovlen: Arc::new(Mutex::new(Some(0))), pad_cgo_1: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), control: GoPtr::nil(), controllen: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Msghdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", { let __guard = self.name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.namelen.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0), { let __guard = self.iov.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.iovlen.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_1), { if self.control.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.controllen.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Msghdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Namelen") {
            out.namelen = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Iovlen") {
            out.iovlen = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_1") {
            out.pad_cgo_1 = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Control") {
            out.control = GoPtr::local(<Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?);
        }
        if let Some(field_value) = object.get("Controllen") {
            out.controllen = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct IfMsghdr {
    pub msglen: Arc<Mutex<Option<u16>>>,
    pub version: Arc<Mutex<Option<u8>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub addrs: Arc<Mutex<Option<i32>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
    pub index: Arc<Mutex<Option<u16>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 2]>>>,
    pub data: Arc<Mutex<Option<IfData>>>,
}

impl IfMsghdr {
    pub fn __go_value_clone(&self) -> Self {
        Self { msglen: { let __guard = self.msglen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addrs: { let __guard = self.addrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IfMsghdr {
    fn default() -> Self {
        Self { msglen: Arc::new(Mutex::new(Some(0))), version: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))), addrs: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), data: Arc::new(Mutex::new(Some(IfData::default()))) }
    }
}

impl std::fmt::Display for IfMsghdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.msglen.lock().unwrap().as_ref().unwrap()), (*self.version.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.addrs.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0), (*self.data.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IfMsghdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msglen") {
            out.msglen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addrs") {
            out.addrs = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Index") {
            out.index = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 2]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct IfData {
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub typelen: Arc<Mutex<Option<u8>>>,
    pub physical: Arc<Mutex<Option<u8>>>,
    pub addrlen: Arc<Mutex<Option<u8>>>,
    pub hdrlen: Arc<Mutex<Option<u8>>>,
    pub recvquota: Arc<Mutex<Option<u8>>>,
    pub xmitquota: Arc<Mutex<Option<u8>>>,
    pub unused1: Arc<Mutex<Option<u8>>>,
    pub mtu: Arc<Mutex<Option<u32>>>,
    pub metric: Arc<Mutex<Option<u32>>>,
    pub baudrate: Arc<Mutex<Option<u32>>>,
    pub ipackets: Arc<Mutex<Option<u32>>>,
    pub ierrors: Arc<Mutex<Option<u32>>>,
    pub opackets: Arc<Mutex<Option<u32>>>,
    pub oerrors: Arc<Mutex<Option<u32>>>,
    pub collisions: Arc<Mutex<Option<u32>>>,
    pub ibytes: Arc<Mutex<Option<u32>>>,
    pub obytes: Arc<Mutex<Option<u32>>>,
    pub imcasts: Arc<Mutex<Option<u32>>>,
    pub omcasts: Arc<Mutex<Option<u32>>>,
    pub iqdrops: Arc<Mutex<Option<u32>>>,
    pub noproto: Arc<Mutex<Option<u32>>>,
    pub recvtiming: Arc<Mutex<Option<u32>>>,
    pub xmittiming: Arc<Mutex<Option<u32>>>,
    pub lastchange: Arc<Mutex<Option<Timeval32>>>,
    pub unused2: Arc<Mutex<Option<u32>>>,
    pub hwassist: Arc<Mutex<Option<u32>>>,
    pub reserved1: Arc<Mutex<Option<u32>>>,
    pub reserved2: Arc<Mutex<Option<u32>>>,
}

impl IfData {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typelen: { let __guard = self.typelen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, physical: { let __guard = self.physical.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addrlen: { let __guard = self.addrlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hdrlen: { let __guard = self.hdrlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recvquota: { let __guard = self.recvquota.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, xmitquota: { let __guard = self.xmitquota.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, unused1: { let __guard = self.unused1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mtu: { let __guard = self.mtu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, metric: { let __guard = self.metric.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, baudrate: { let __guard = self.baudrate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ipackets: { let __guard = self.ipackets.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ierrors: { let __guard = self.ierrors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, opackets: { let __guard = self.opackets.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, oerrors: { let __guard = self.oerrors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, collisions: { let __guard = self.collisions.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ibytes: { let __guard = self.ibytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, obytes: { let __guard = self.obytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, imcasts: { let __guard = self.imcasts.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, omcasts: { let __guard = self.omcasts.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, iqdrops: { let __guard = self.iqdrops.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, noproto: { let __guard = self.noproto.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recvtiming: { let __guard = self.recvtiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, xmittiming: { let __guard = self.xmittiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lastchange: { let __guard = self.lastchange.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, unused2: { let __guard = self.unused2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hwassist: { let __guard = self.hwassist.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reserved1: { let __guard = self.reserved1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reserved2: { let __guard = self.reserved2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IfData {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(Some(0))), typelen: Arc::new(Mutex::new(Some(0))), physical: Arc::new(Mutex::new(Some(0))), addrlen: Arc::new(Mutex::new(Some(0))), hdrlen: Arc::new(Mutex::new(Some(0))), recvquota: Arc::new(Mutex::new(Some(0))), xmitquota: Arc::new(Mutex::new(Some(0))), unused1: Arc::new(Mutex::new(Some(0))), mtu: Arc::new(Mutex::new(Some(0))), metric: Arc::new(Mutex::new(Some(0))), baudrate: Arc::new(Mutex::new(Some(0))), ipackets: Arc::new(Mutex::new(Some(0))), ierrors: Arc::new(Mutex::new(Some(0))), opackets: Arc::new(Mutex::new(Some(0))), oerrors: Arc::new(Mutex::new(Some(0))), collisions: Arc::new(Mutex::new(Some(0))), ibytes: Arc::new(Mutex::new(Some(0))), obytes: Arc::new(Mutex::new(Some(0))), imcasts: Arc::new(Mutex::new(Some(0))), omcasts: Arc::new(Mutex::new(Some(0))), iqdrops: Arc::new(Mutex::new(Some(0))), noproto: Arc::new(Mutex::new(Some(0))), recvtiming: Arc::new(Mutex::new(Some(0))), xmittiming: Arc::new(Mutex::new(Some(0))), lastchange: Arc::new(Mutex::new(Some(Timeval32::default()))), unused2: Arc::new(Mutex::new(Some(0))), hwassist: Arc::new(Mutex::new(Some(0))), reserved1: Arc::new(Mutex::new(Some(0))), reserved2: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for IfData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.typelen.lock().unwrap().as_ref().unwrap()), (*self.physical.lock().unwrap().as_ref().unwrap()), (*self.addrlen.lock().unwrap().as_ref().unwrap()), (*self.hdrlen.lock().unwrap().as_ref().unwrap()), (*self.recvquota.lock().unwrap().as_ref().unwrap()), (*self.xmitquota.lock().unwrap().as_ref().unwrap()), (*self.unused1.lock().unwrap().as_ref().unwrap()), (*self.mtu.lock().unwrap().as_ref().unwrap()), (*self.metric.lock().unwrap().as_ref().unwrap()), (*self.baudrate.lock().unwrap().as_ref().unwrap()), (*self.ipackets.lock().unwrap().as_ref().unwrap()), (*self.ierrors.lock().unwrap().as_ref().unwrap()), (*self.opackets.lock().unwrap().as_ref().unwrap()), (*self.oerrors.lock().unwrap().as_ref().unwrap()), (*self.collisions.lock().unwrap().as_ref().unwrap()), (*self.ibytes.lock().unwrap().as_ref().unwrap()), (*self.obytes.lock().unwrap().as_ref().unwrap()), (*self.imcasts.lock().unwrap().as_ref().unwrap()), (*self.omcasts.lock().unwrap().as_ref().unwrap()), (*self.iqdrops.lock().unwrap().as_ref().unwrap()), (*self.noproto.lock().unwrap().as_ref().unwrap()), (*self.recvtiming.lock().unwrap().as_ref().unwrap()), (*self.xmittiming.lock().unwrap().as_ref().unwrap()), (*self.lastchange.lock().unwrap().as_ref().unwrap()), (*self.unused2.lock().unwrap().as_ref().unwrap()), (*self.hwassist.lock().unwrap().as_ref().unwrap()), (*self.reserved1.lock().unwrap().as_ref().unwrap()), (*self.reserved2.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IfData {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Typelen") {
            out.typelen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Physical") {
            out.physical = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addrlen") {
            out.addrlen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hdrlen") {
            out.hdrlen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Recvquota") {
            out.recvquota = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Xmitquota") {
            out.xmitquota = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Unused1") {
            out.unused1 = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Mtu") {
            out.mtu = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Metric") {
            out.metric = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Baudrate") {
            out.baudrate = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ipackets") {
            out.ipackets = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ierrors") {
            out.ierrors = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Opackets") {
            out.opackets = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Oerrors") {
            out.oerrors = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Collisions") {
            out.collisions = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ibytes") {
            out.ibytes = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Obytes") {
            out.obytes = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Imcasts") {
            out.imcasts = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Omcasts") {
            out.omcasts = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Iqdrops") {
            out.iqdrops = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Noproto") {
            out.noproto = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Recvtiming") {
            out.recvtiming = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Xmittiming") {
            out.xmittiming = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Unused2") {
            out.unused2 = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hwassist") {
            out.hwassist = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Reserved1") {
            out.reserved1 = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Reserved2") {
            out.reserved2 = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct IfaMsghdr {
    pub msglen: Arc<Mutex<Option<u16>>>,
    pub version: Arc<Mutex<Option<u8>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub addrs: Arc<Mutex<Option<i32>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
    pub index: Arc<Mutex<Option<u16>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 2]>>>,
    pub metric: Arc<Mutex<Option<i32>>>,
}

impl IfaMsghdr {
    pub fn __go_value_clone(&self) -> Self {
        Self { msglen: { let __guard = self.msglen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addrs: { let __guard = self.addrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, metric: { let __guard = self.metric.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IfaMsghdr {
    fn default() -> Self {
        Self { msglen: Arc::new(Mutex::new(Some(0))), version: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))), addrs: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), metric: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for IfaMsghdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.msglen.lock().unwrap().as_ref().unwrap()), (*self.version.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.addrs.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0), (*self.metric.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IfaMsghdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msglen") {
            out.msglen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addrs") {
            out.addrs = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Index") {
            out.index = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 2]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Metric") {
            out.metric = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct IfmaMsghdr2 {
    pub msglen: Arc<Mutex<Option<u16>>>,
    pub version: Arc<Mutex<Option<u8>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub addrs: Arc<Mutex<Option<i32>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
    pub index: Arc<Mutex<Option<u16>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 2]>>>,
    pub refcount: Arc<Mutex<Option<i32>>>,
}

impl IfmaMsghdr2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { msglen: { let __guard = self.msglen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addrs: { let __guard = self.addrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, refcount: { let __guard = self.refcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for IfmaMsghdr2 {
    fn default() -> Self {
        Self { msglen: Arc::new(Mutex::new(Some(0))), version: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))), addrs: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), refcount: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for IfmaMsghdr2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.msglen.lock().unwrap().as_ref().unwrap()), (*self.version.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.addrs.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0), (*self.refcount.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for IfmaMsghdr2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msglen") {
            out.msglen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addrs") {
            out.addrs = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Index") {
            out.index = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 2]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Refcount") {
            out.refcount = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RtMsghdr {
    pub msglen: Arc<Mutex<Option<u16>>>,
    pub version: Arc<Mutex<Option<u8>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub index: Arc<Mutex<Option<u16>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 2]>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
    pub addrs: Arc<Mutex<Option<i32>>>,
    pub pid: Arc<Mutex<Option<i32>>>,
    pub seq: Arc<Mutex<Option<i32>>>,
    pub errno: Arc<Mutex<Option<i32>>>,
    pub r#use: Arc<Mutex<Option<i32>>>,
    pub inits: Arc<Mutex<Option<u32>>>,
    pub rmx: Arc<Mutex<Option<RtMetrics>>>,
}

impl RtMsghdr {
    pub fn __go_value_clone(&self) -> Self {
        Self { msglen: { let __guard = self.msglen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addrs: { let __guard = self.addrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pid: { let __guard = self.pid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seq: { let __guard = self.seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, errno: { let __guard = self.errno.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#use: { let __guard = self.r#use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inits: { let __guard = self.inits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rmx: { let __guard = self.rmx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RtMsghdr {
    fn default() -> Self {
        Self { msglen: Arc::new(Mutex::new(Some(0))), version: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), flags: Arc::new(Mutex::new(Some(0))), addrs: Arc::new(Mutex::new(Some(0))), pid: Arc::new(Mutex::new(Some(0))), seq: Arc::new(Mutex::new(Some(0))), errno: Arc::new(Mutex::new(Some(0))), r#use: Arc::new(Mutex::new(Some(0))), inits: Arc::new(Mutex::new(Some(0))), rmx: Arc::new(Mutex::new(Some(RtMetrics::default()))) }
    }
}

impl std::fmt::Display for RtMsghdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.msglen.lock().unwrap().as_ref().unwrap()), (*self.version.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.addrs.lock().unwrap().as_ref().unwrap()), (*self.pid.lock().unwrap().as_ref().unwrap()), (*self.seq.lock().unwrap().as_ref().unwrap()), (*self.errno.lock().unwrap().as_ref().unwrap()), (*self.r#use.lock().unwrap().as_ref().unwrap()), (*self.inits.lock().unwrap().as_ref().unwrap()), (*self.rmx.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for RtMsghdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msglen") {
            out.msglen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Index") {
            out.index = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pad_cgo_0") {
            out.pad_cgo_0 = <Arc<Mutex<Option<[u8; 2]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addrs") {
            out.addrs = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pid") {
            out.pid = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Seq") {
            out.seq = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Errno") {
            out.errno = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Use") {
            out.r#use = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Inits") {
            out.inits = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct RtMetrics {
    pub locks: Arc<Mutex<Option<u32>>>,
    pub mtu: Arc<Mutex<Option<u32>>>,
    pub hopcount: Arc<Mutex<Option<u32>>>,
    pub expire: Arc<Mutex<Option<i32>>>,
    pub recvpipe: Arc<Mutex<Option<u32>>>,
    pub sendpipe: Arc<Mutex<Option<u32>>>,
    pub ssthresh: Arc<Mutex<Option<u32>>>,
    pub rtt: Arc<Mutex<Option<u32>>>,
    pub rttvar: Arc<Mutex<Option<u32>>>,
    pub pksent: Arc<Mutex<Option<u32>>>,
    pub filler: Arc<Mutex<Option<[u32; 4]>>>,
}

impl RtMetrics {
    pub fn __go_value_clone(&self) -> Self {
        Self { locks: { let __guard = self.locks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mtu: { let __guard = self.mtu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hopcount: { let __guard = self.hopcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, expire: { let __guard = self.expire.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recvpipe: { let __guard = self.recvpipe.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sendpipe: { let __guard = self.sendpipe.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ssthresh: { let __guard = self.ssthresh.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rtt: { let __guard = self.rtt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rttvar: { let __guard = self.rttvar.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pksent: { let __guard = self.pksent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, filler: { let __guard = self.filler.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for RtMetrics {
    fn default() -> Self {
        Self { locks: Arc::new(Mutex::new(Some(0))), mtu: Arc::new(Mutex::new(Some(0))), hopcount: Arc::new(Mutex::new(Some(0))), expire: Arc::new(Mutex::new(Some(0))), recvpipe: Arc::new(Mutex::new(Some(0))), sendpipe: Arc::new(Mutex::new(Some(0))), ssthresh: Arc::new(Mutex::new(Some(0))), rtt: Arc::new(Mutex::new(Some(0))), rttvar: Arc::new(Mutex::new(Some(0))), pksent: Arc::new(Mutex::new(Some(0))), filler: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for RtMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", (*self.locks.lock().unwrap().as_ref().unwrap()), (*self.mtu.lock().unwrap().as_ref().unwrap()), (*self.hopcount.lock().unwrap().as_ref().unwrap()), (*self.expire.lock().unwrap().as_ref().unwrap()), (*self.recvpipe.lock().unwrap().as_ref().unwrap()), (*self.sendpipe.lock().unwrap().as_ref().unwrap()), (*self.ssthresh.lock().unwrap().as_ref().unwrap()), (*self.rtt.lock().unwrap().as_ref().unwrap()), (*self.rttvar.lock().unwrap().as_ref().unwrap()), (*self.pksent.lock().unwrap().as_ref().unwrap()), format_slice(&self.filler))
    }
}

impl GoJsonDecode for RtMetrics {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Locks") {
            out.locks = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Mtu") {
            out.mtu = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hopcount") {
            out.hopcount = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Expire") {
            out.expire = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Recvpipe") {
            out.recvpipe = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Sendpipe") {
            out.sendpipe = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ssthresh") {
            out.ssthresh = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Rtt") {
            out.rtt = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Rttvar") {
            out.rttvar = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pksent") {
            out.pksent = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Filler") {
            out.filler = <Arc<Mutex<Option<[u32; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub nlen: Arc<Mutex<Option<u8>>>,
    pub alen: Arc<Mutex<Option<u8>>>,
    pub slen: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nlen: { let __guard = self.nlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alen: { let __guard = self.alen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, slen: { let __guard = self.slen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(Some(0))), nlen: Arc::new(Mutex::new(Some(0))), alen: Arc::new(Mutex::new(Some(0))), slen: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.nlen.lock().unwrap().as_ref().unwrap()), (*self.alen.lock().unwrap().as_ref().unwrap()), (*self.slen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Nlen") {
            out.nlen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Alen") {
            out.alen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Slen") {
            out.slen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl GoValueClone for Timespec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Timeval32 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Rlimit {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Stat_t {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Dirent {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RawSockaddrInet4 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RawSockaddrInet6 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RawSockaddrUnix {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RawSockaddrDatalink {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RawSockaddr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RawSockaddrAny {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Linger {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Iovec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IPMreq {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IPv6Mreq {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Msghdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IfMsghdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IfData {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IfaMsghdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for IfmaMsghdr2 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RtMsghdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RtMetrics {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
