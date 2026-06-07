use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::sync::{Arc, Mutex};

/// FormatBool returns "true" or "false" according to the value of b.
pub fn format_bool(b: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
    if { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some("true".to_string())));
    }
    Arc::new(Mutex::new(Some("false".to_string())))
}