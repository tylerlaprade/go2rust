use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::{path::{lazybuf}};

use std::sync::{Arc, Mutex};

pub fn post_clean(out: Arc<Mutex<Option<lazybuf>>>) {
}