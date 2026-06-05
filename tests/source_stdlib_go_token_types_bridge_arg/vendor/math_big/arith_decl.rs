use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
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
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::sync::{Arc, Mutex};

/// addVV should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname addVV
///go:noescape
pub fn add_v_v(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Vec<Word>>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


/// subVV should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname subVV
///go:noescape
pub fn sub_v_v(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Vec<Word>>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


/// addVW should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname addVW
///go:noescape
pub fn add_v_w(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


/// subVW should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname subVW
///go:noescape
pub fn sub_v_w(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


/// shlVU should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname shlVU
///go:noescape
pub fn shl_v_u(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, s: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn shr_v_u(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, s: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


/// mulAddVWW should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname mulAddVWW
///go:noescape
pub fn mul_add_v_w_w(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Word>>>, r: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}


/// addMulVVW should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/remyoudompheng/bigfft
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname addMulVVW
///go:noescape
pub fn add_mul_v_v_w(z: Arc<Mutex<Option<Vec<Word>>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    unimplemented!("Go function declaration has no body");
}
