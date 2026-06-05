use go2rust_stdlib_stubs::*;

use crate::{format_any, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::r#mod::*;
use crate::commentmap::*;
use crate::import::*;
use crate::print::*;
use crate::resolve::*;
use crate::scope::*;
use crate::walk::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const FILTER_FUNC_DUPLICATES: u64 = 1 << 0;
pub const FILTER_UNASSOCIATED_COMMENTS: u64 = 1 << 1;
pub const FILTER_IMPORT_DUPLICATES: u64 = 1 << 2;


/// The MergeMode flags control the behavior of [MergePackageFiles].
#[derive(Debug, Clone, Default)]
pub struct MergeMode(pub Arc<Mutex<Option<u64>>>);

impl Display for MergeMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for MergeMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for MergeMode {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for MergeMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for MergeMode {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<MergeMode> for u64 {
    fn eq(&self, other: &MergeMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<MergeMode> for u64 {
    fn partial_cmp(&self, other: &MergeMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for MergeMode {
    type Output = MergeMode;
    fn add(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for MergeMode {
    type Output = MergeMode;
    fn add(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<MergeMode> for u64 {
    type Output = MergeMode;
    fn add(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for MergeMode {
    type Output = MergeMode;
    fn sub(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for MergeMode {
    type Output = MergeMode;
    fn sub(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<MergeMode> for u64 {
    type Output = MergeMode;
    fn sub(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for MergeMode {
    type Output = MergeMode;
    fn mul(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for MergeMode {
    type Output = MergeMode;
    fn mul(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<MergeMode> for u64 {
    type Output = MergeMode;
    fn mul(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for MergeMode {
    type Output = MergeMode;
    fn div(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for MergeMode {
    type Output = MergeMode;
    fn div(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<MergeMode> for u64 {
    type Output = MergeMode;
    fn div(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for MergeMode {
    type Output = MergeMode;
    fn rem(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for MergeMode {
    type Output = MergeMode;
    fn rem(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<MergeMode> for u64 {
    type Output = MergeMode;
    fn rem(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for MergeMode {
    type Output = MergeMode;
    fn bitand(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for MergeMode {
    type Output = MergeMode;
    fn bitand(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<MergeMode> for u64 {
    type Output = MergeMode;
    fn bitand(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for MergeMode {
    type Output = MergeMode;
    fn bitor(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for MergeMode {
    type Output = MergeMode;
    fn bitor(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<MergeMode> for u64 {
    type Output = MergeMode;
    fn bitor(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for MergeMode {
    type Output = MergeMode;
    fn bitxor(self, other: Self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for MergeMode {
    type Output = MergeMode;
    fn bitxor(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<MergeMode> for u64 {
    type Output = MergeMode;
    fn bitxor(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for MergeMode {
    type Output = MergeMode;
    fn not(self) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: i32) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: i8) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: i16) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: i64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: u32) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: u8) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: u16) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for MergeMode {
    type Output = MergeMode;
    fn shl(self, other: usize) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: MergeMode) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: i32) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: i8) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: i16) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: i64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: u32) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: u8) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: u16) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: u64) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for MergeMode {
    type Output = MergeMode;
    fn shr(self, other: usize) -> MergeMode {
        MergeMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for MergeMode {}

impl Ord for MergeMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static separator: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::r#mod::Comment>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *separator.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *separator.lock().unwrap() = Some(Arc::new(Mutex::new(Some(Comment { slash: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), text: Arc::new(Mutex::new(Some("//".to_string()))), ..Default::default() }))));
}


pub(crate) fn __go_zero_globals() {
    *separator.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_0() {
    *separator.lock().unwrap() = Some(Arc::new(Mutex::new(Some(Comment { slash: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), text: Arc::new(Mutex::new(Some("//".to_string()))), ..Default::default() }))));
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
