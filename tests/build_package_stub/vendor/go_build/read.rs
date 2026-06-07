use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;
use crate::gc::*;
use crate::zcgo::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct importReader {
    pub b: Arc<Mutex<Option<bufio_Reader>>>,
    pub buf: Arc<Mutex<Option<Vec<u8>>>>,
    pub peek: Arc<Mutex<Option<u8>>>,
    pub err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
    pub eof: Arc<Mutex<Option<bool>>>,
    pub nerr: Arc<Mutex<Option<i32>>>,
    pub pos: Arc<Mutex<Option<token_Position>>>,
}

impl importReader {
    pub fn __go_value_clone(&self) -> Self {
        Self { b: self.b.clone(), buf: self.buf.clone(), peek: { let __guard = self.peek.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, err: self.err.clone(), eof: { let __guard = self.eof.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nerr: { let __guard = self.nerr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for importReader {
    fn default() -> Self {
        Self { b: Arc::new(Mutex::new(None)), buf: Arc::new(Mutex::new(None)), peek: Arc::new(Mutex::new(Some(0))), err: Arc::new(Mutex::new(None)), eof: Arc::new(Mutex::new(Some(false))), nerr: Arc::new(Mutex::new(Some(0))), pos: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for importReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", { let __guard = self.b.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.buf), (*self.peek.lock().unwrap().as_ref().unwrap()), (*self.err.lock().unwrap().as_ref().unwrap()), (*self.eof.lock().unwrap().as_ref().unwrap()), (*self.nerr.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for importReader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub vendor: Arc<Mutex<Option<Vec<String>>>>,
    pub goroot: Arc<Mutex<Option<String>>>,
    pub gopath: Arc<Mutex<Option<Vec<String>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { vendor: self.vendor.clone(), goroot: { let __guard = self.goroot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gopath: self.gopath.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { vendor: Arc::new(Mutex::new(None)), goroot: Arc::new(Mutex::new(Some(String::new()))), gopath: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.vendor), (*self.goroot.lock().unwrap().as_ref().unwrap()), format_slice(&self.gopath))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static bom: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errSyntax: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errNUL: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goEmbed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *bom.lock().unwrap() = Some(vec![]);
    *errSyntax.lock().unwrap() = None;
    *errNUL.lock().unwrap() = None;
    *goEmbed.lock().unwrap() = Some(vec![]);
    *bom.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![0xef as u8, 0xbb as u8, 0xbf as u8]))).lock().unwrap().as_ref().unwrap()).clone());
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("syntax error".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errSyntax.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("unexpected NUL in input".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNUL.lock().unwrap() = new_val; }
    *goEmbed.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("go:embed".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *bom.lock().unwrap() = Some(vec![]);
    *errSyntax.lock().unwrap() = None;
    *errNUL.lock().unwrap() = None;
    *goEmbed.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_12() {
    *bom.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![0xef as u8, 0xbb as u8, 0xbf as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_13() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("syntax error".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errSyntax.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_14() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("unexpected NUL in input".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNUL.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_15() {
    *goEmbed.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("go:embed".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


impl importReader {
    /// syntaxError records a syntax error, but only if an I/O error has not already been recorded.
    pub fn syntax_error(&mut self) {
        if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = errSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *self.err.lock().unwrap() = new_val; };
    }
    }

    /// readByte reads the next byte from the input, saves it in buf, and returns it.
    /// If an error occurs, readByte records the error in r.err and returns 0.
    pub fn read_byte(&mut self) -> u8 {
        let (mut c, mut err) = (*self.b.lock().unwrap().as_mut().unwrap()).read_byte();
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = { let __append_target = self.buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(c); __append_target.clone() }; self.buf = new_val; };
        if { let __tmp_x = c; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let __rhs_holder = errNUL.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        { let new_val = true; *self.eof.lock().unwrap() = Some(new_val); };
    } else if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *self.err.lock().unwrap() = new_val; };
    }
        { let new_val = 0 as u8; c = new_val; };
    }
        c
    }

    /// readByteNoBuf is like readByte but doesn't buffer the byte.
    /// It exhausts r.buf before reading from r.b.
    pub fn read_byte_no_buf(&mut self) -> u8 {
        let mut c: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *c.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
    } else {
        { let (__tmp_0, __tmp_1) = (*self.b.lock().unwrap().as_mut().unwrap()).read_byte(); *c.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let __rhs_holder = errNUL.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        { let new_val = true; *self.eof.lock().unwrap() = Some(new_val); };
    } else if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *self.err.lock().unwrap() = new_val; };
    }
        return 0;
    }
        { let __target = (*self.pos.lock().unwrap().as_ref().unwrap()).offset.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } {
        { let __target = (*self.pos.lock().unwrap().as_ref().unwrap()).line.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = 1; *(*self.pos.lock().unwrap().as_ref().unwrap()).column.lock().unwrap() = Some(new_val); };
    } else {
        { let __target = (*self.pos.lock().unwrap().as_ref().unwrap()).column.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// peekByte returns the next byte from the input reader but does not advance beyond it.
    /// If skipSpace is set, peekByte skips leading spaces and comments.
    pub fn peek_byte(&mut self, skipSpace: Arc<Mutex<Option<bool>>>) -> u8 {
        if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
        { let __target = self.nerr.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        if { let __tmp_x = (*self.nerr.lock().unwrap().as_ref().unwrap()); let __tmp_y = 10000; __tmp_x > __tmp_y } {
            std::panic::panic_any(Box::new("go/build: import reader looping".to_string()) as Box<dyn Any + Send + Sync>);;
        }
    }
        return 0;
    }
                // Use r.peek as first input byte.
                // Don't just return r.peek here: it might have been left by peekByte(false)
                // and this might be peekByte(true).
        let mut c = Arc::new(Mutex::new(Some({ let __selector_holder = self.peek.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let new_val = self.read_byte(); *c.lock().unwrap() = Some(new_val); };
    }
        while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        if { let __v = (*skipSpace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // For the purposes of this reader, semicolons are never necessary to
                // understand the input and are treated as spaces.
        { let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == ((' ' as i32) as u8) || _switch_val == (('\u{c}' as i32) as u8) || _switch_val == (('\t' as i32) as u8) || _switch_val == (('\r' as i32) as u8) || _switch_val == (('\n' as i32) as u8) || _switch_val == ((';' as i32) as u8) {
            { let new_val = self.read_byte(); *c.lock().unwrap() = Some(new_val); };
            continue
        } else if _switch_val == (('/' as i32) as u8) {
            { let new_val = self.read_byte(); *c.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('/' as i32) as u8; __tmp_x == __tmp_y } {
        while { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32) as u8; __tmp_x != __tmp_y } && { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = self.read_byte(); *c.lock().unwrap() = Some(new_val); };
    }
    } else if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y } {
        let mut c1: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
        while ({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('*' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('/' as i32) as u8; __tmp_x != __tmp_y }) && { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        self.syntax_error();
    }
        { let __tmp_0 = (*c1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = self.read_byte(); *c.lock().unwrap() = Some(__tmp_0); *c1.lock().unwrap() = Some(__tmp_1); };
    }
    } else {
        self.syntax_error();
    }
            { let new_val = self.read_byte(); *c.lock().unwrap() = Some(new_val); };
            continue
        }
    }
    }
                // For the purposes of this reader, semicolons are never necessary to
                // understand the input and are treated as spaces.
        break
    }
                // For the purposes of this reader, semicolons are never necessary to
                // understand the input and are treated as spaces.
        { let new_val = c.lock().unwrap().as_ref().unwrap().clone(); *self.peek.lock().unwrap() = Some(new_val); };
        return (*self.peek.lock().unwrap().as_ref().unwrap());
    }

    /// nextByte is like peekByte but advances beyond the returned byte.
    pub fn next_byte(&mut self, skipSpace: Arc<Mutex<Option<bool>>>) -> u8 {
        let mut c = self.peek_byte(Arc::new(Mutex::new(Some({ let __arg_holder = skipSpace.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = 0 as u8; *self.peek.lock().unwrap() = Some(new_val); };
        c
    }

    /// findEmbed advances the input reader to the next //go:embed comment.
    /// It reports whether it found a comment.
    /// (Otherwise it found an error or EOF.)
    pub fn find_embed(&mut self, first: Arc<Mutex<Option<bool>>>) -> bool {
                // The import block scan stopped after a non-space character,
                // so the reader is not at the start of a line on the first call.
                // After that, each //go:embed extraction leaves the reader
                // at the end of a line.
        let mut startLine = Arc::new(Mutex::new(Some(!{ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        let mut c: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
        while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        'reswitch: {
            { let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('\n' as i32) as u8) {
            { let new_val = true; *startLine.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ((' ' as i32) as u8) || _switch_val == (('\t' as i32) as u8) {
        } else if _switch_val == (('"' as i32) as u8) {
            { let new_val = false; *startLine.lock().unwrap() = Some(new_val); };
            while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        self.syntax_error();
    }
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\\' as i32) as u8; __tmp_x == __tmp_y } {
        self.read_byte_no_buf();
        if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.syntax_error();
        return false;
    }
        continue
    }
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('"' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        // TODO: unsupported goto reswitch
    }
    }
            // TODO: unsupported goto reswitch
        } else if _switch_val == (('`' as i32) as u8) {
            { let new_val = false; *startLine.lock().unwrap() = Some(new_val); };
            while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        self.syntax_error();
    }
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('`' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        // TODO: unsupported goto reswitch
    }
    }
        } else if _switch_val == (('\'' as i32) as u8) {
            { let new_val = false; *startLine.lock().unwrap() = Some(new_val); };
            while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        self.syntax_error();
    }
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\\' as i32) as u8; __tmp_x == __tmp_y } {
        self.read_byte_no_buf();
        if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.syntax_error();
        return false;
    }
        continue
    }
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\'' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        // TODO: unsupported goto reswitch
    }
    }
        } else if _switch_val == (('/' as i32) as u8) {
            { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
            { let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('*' as i32) as u8) {
            let mut c1: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
            while ({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('*' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('/' as i32) as u8; __tmp_x != __tmp_y }) && { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        self.syntax_error();
    }
        { let __tmp_0 = (*c1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = self.read_byte_no_buf(); *c.lock().unwrap() = Some(__tmp_0); *c1.lock().unwrap() = Some(__tmp_1); };
    }
            { let new_val = false; *startLine.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('/' as i32) as u8) {
            if { let __v = (*startLine.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Try to read this as a //go:embed comment.
        for i in 0..(({ let __range_holder = goEmbed.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __seq = { let __seq_holder = goEmbed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; __tmp_x != __tmp_y } {
        // TODO: unsupported goto skip_slash_slash
    }
    }
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\t' as i32) as u8; __tmp_x == __tmp_y } {
                // Found one!
        return true;
    }
    }
                        // Try to read this as a //go:embed comment.
                        // Found one!
            'skip_slash_slash: while { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32) as u8; __tmp_x != __tmp_y } && { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = self.read_byte_no_buf(); *c.lock().unwrap() = Some(new_val); };
    }
            { let new_val = true; *startLine.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = false; *startLine.lock().unwrap() = Some(new_val); };
            // TODO: unsupported goto reswitch
        }
    }
        } else {
            { let new_val = false; *startLine.lock().unwrap() = Some(new_val); };
        }
    }
        }
    }
                // leave startLine alone
                // Try to read this as a //go:embed comment.
                // Found one!
        return false;
        unreachable!()
    }

    /// readKeyword reads the given keyword from the input.
    /// If the keyword is not present, readKeyword records a syntax error.
    pub fn read_keyword(&mut self, kw: Arc<Mutex<Option<String>>>) {
        self.peek_byte(Arc::new(Mutex::new(Some(true))));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*kw.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = self.next_byte(Arc::new(Mutex::new(Some(false)))); let __tmp_y = { let __s = &((*kw.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x != __tmp_y } {
        self.syntax_error();
        return;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if is_ident(Arc::new(Mutex::new(Some(self.peek_byte(Arc::new(Mutex::new(Some(false)))))))) {
        self.syntax_error();
    }
    }

    /// readIdent reads an identifier from the input.
    /// If an identifier is not present, readIdent records a syntax error.
    pub fn read_ident(&mut self) {
        let mut c = self.peek_byte(Arc::new(Mutex::new(Some(true))));
        if !is_ident(Arc::new(Mutex::new(Some(c)))) {
        self.syntax_error();
        return;
    }
        while is_ident(Arc::new(Mutex::new(Some(self.peek_byte(Arc::new(Mutex::new(Some(false)))))))) {
        { let new_val = 0 as u8; *self.peek.lock().unwrap() = Some(new_val); };
    }
    }

    /// readString reads a quoted string literal from the input.
    /// If an identifier is not present, readString records a syntax error.
    pub fn read_string(&mut self) {
        { let _switch_val = self.next_byte(Arc::new(Mutex::new(Some(true))));
    if _switch_val == (('`' as i32) as u8) {
            while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if { let __tmp_x = self.next_byte(Arc::new(Mutex::new(Some(false)))); let __tmp_y = ('`' as i32) as u8; __tmp_x == __tmp_y } {
        break
    }
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) {
        self.syntax_error();
    }
    }
        } else if _switch_val == (('"' as i32) as u8) {
            while { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        let mut c = self.next_byte(Arc::new(Mutex::new(Some(false))));
        if { let __tmp_x = c; let __tmp_y = ('"' as i32) as u8; __tmp_x == __tmp_y } {
        break
    }
        if (*self.eof.clone().lock().unwrap().as_ref().unwrap()) || { let __tmp_x = c; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } {
        self.syntax_error();
    }
        if { let __tmp_x = c; let __tmp_y = ('\\' as i32) as u8; __tmp_x == __tmp_y } {
        self.next_byte(Arc::new(Mutex::new(Some(false))));
    }
    }
        } else {
            self.syntax_error();
        }
    }
    }

    /// readImport reads an import clause - optional identifier followed by quoted string -
    /// from the input.
    pub fn read_import(&mut self) {
        let mut c = self.peek_byte(Arc::new(Mutex::new(Some(true))));
        if { let __tmp_x = c; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = 0 as u8; *self.peek.lock().unwrap() = Some(new_val); };
    } else if is_ident(Arc::new(Mutex::new(Some(c)))) {
        self.read_ident();
    }
        self.read_string();
    }
}

pub fn new_import_reader(name: Arc<Mutex<Option<String>>>, r: Arc<Mutex<Option<io_Reader>>>) -> Arc<Mutex<Option<importReader>>> {
    let mut b = bufio::new_reader(r.clone());

        // Remove leading UTF-8 BOM.
        // Per https://golang.org/ref/spec#Source_code_representation:
        // a compiler may ignore a UTF-8-encoded byte order mark (U+FEFF)
        // if it is the first Unicode code point in the source text.
    {
        let (mut leadingBytes, mut err) = { let __recv = b.clone(); let __recv_ptr: *mut bufio_Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Reader }; let __result = unsafe { &mut *__recv_ptr }.peek(3); __result };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && bytes::equal(leadingBytes.clone(), bom.clone()) {
            { let __recv = b.clone(); let __recv_ptr: *mut bufio_Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio_Reader }; let __result = unsafe { &mut *__recv_ptr }.discard(3); __result };;
        }
    }
    return Arc::new(Mutex::new(Some(importReader { b: b.clone(), pos: Arc::new(Mutex::new(Some(token_Position { filename: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), line: Arc::new(Mutex::new(Some(1))), column: Arc::new(Mutex::new(Some(1))), ..Default::default() }))), ..Default::default() })));
}

pub fn is_ident(c: Arc<Mutex<Option<u8>>>) -> bool {
    return { let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y } || { let __tmp_x = ('a' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('z' as i32) as u8; __tmp_x <= __tmp_y } || { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } || { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y };
}

/// readComments is like io.ReadAll, except that it only reads the leading
/// block of comments in the file.
///
/// readComments should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bazelbuild/bazel-gazelle
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname readComments
pub fn read_comments(f: Arc<Mutex<Option<io_Reader>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut r = new_import_reader(Arc::new(Mutex::new(Some("".to_string()))), f.clone());
    { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.peek_byte(Arc::new(Mutex::new(Some(true)))); __result };
    if { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).eof.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Didn't reach EOF, so must have found a non-space byte. Remove it.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); (*r.lock().unwrap().as_mut().unwrap()).buf = new_val; };
    }
        // Didn't reach EOF, so must have found a non-space byte. Remove it.
    return ({ let __return_value_0 = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); __return_value_0 }, { let __return_value_1 = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); __return_value_1 });
}

/// readGoInfo expects a Go file as input and reads the file up to and including the import section.
/// It records what it learned in *info.
/// If info.fset is non-nil, readGoInfo parses the file and sets info.parsed, info.parseErr,
/// info.imports and info.embeds.
///
/// It only returns an error if there are problems reading the file,
/// not for syntax errors in the file itself.
pub fn read_go_info(f: Arc<Mutex<Option<io_Reader>>>, info: Arc<Mutex<Option<fileInfo>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut r = new_import_reader(Arc::new(Mutex::new(Some({ let __selector_holder = (*info.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), f.clone());

    { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_keyword(Arc::new(Mutex::new(Some("package".to_string())))); __result };
    { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_ident(); __result };
    while { let __tmp_x = { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.peek_byte(Arc::new(Mutex::new(Some(true)))); __result }; let __tmp_y = ('i' as i32) as u8; __tmp_x == __tmp_y } {
        { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_keyword(Arc::new(Mutex::new(Some("import".to_string())))); __result };
        if { let __tmp_x = { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.peek_byte(Arc::new(Mutex::new(Some(true)))); __result }; let __tmp_y = ('(' as i32) as u8; __tmp_x == __tmp_y } {
        { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.next_byte(Arc::new(Mutex::new(Some(false)))); __result };
        while { let __tmp_x = { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.peek_byte(Arc::new(Mutex::new(Some(true)))); __result }; let __tmp_y = (')' as i32) as u8; __tmp_x != __tmp_y } && { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_import(); __result };
    }
        { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.next_byte(Arc::new(Mutex::new(Some(false)))); __result };
    } else {
        { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_import(); __result };
    }
    }

    { let new_val = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); (*info.lock().unwrap().as_mut().unwrap()).header = new_val; };

        // If we stopped successfully before EOF, we read a byte that told us we were done.
        // Return all but that last byte, which would cause a syntax error if we let it through.
    if { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).eof.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); (*info.lock().unwrap().as_mut().unwrap()).header = new_val; };
    }

        // If we stopped for a syntax error, consume the whole file so that
        // we are sure we don't change the errors that go/parser returns.
    if { let __left = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __right = errSyntax.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        *(*r.lock().unwrap().as_ref().unwrap()).err.lock().unwrap() = None;
        while { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && !(*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).eof.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_byte(); __result };
    }
        { let new_val = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); (*info.lock().unwrap().as_mut().unwrap()).header = new_val; };
    }
    if { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return (*r.lock().unwrap().as_ref().unwrap()).err.clone();
    }

    if { let __nil_target = (*info.lock().unwrap().as_ref().unwrap()).fset.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }

        // Parse file header & record imports.
    { let (__tmp_0, __tmp_1) = parser::parse_file({ let __go_arg = (*info.lock().unwrap().as_ref().unwrap()).fset.clone(); __go_arg }, { let __selector_holder = (*info.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __go_arg = { let __selector_holder = (*info.lock().unwrap().as_ref().unwrap()).header.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }; __go_arg }, parser_Mode((((parser::IMPORTS_ONLY).0 as u64) | ((parser::PARSE_COMMENTS).0 as u64)) as u64)); (*info.lock().unwrap().as_mut().unwrap()).parsed = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*info.lock().unwrap().as_ref().unwrap()).parse_err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_target = (*info.lock().unwrap().as_ref().unwrap()).parse_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }

    let mut hasEmbed = Arc::new(Mutex::new(Some(false)));
    { let __range_holder = (*(*info.lock().unwrap().as_ref().unwrap()).parsed.lock().unwrap().as_ref().unwrap()).decls.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for decl in __range_values.iter() {
        let (mut d, mut ok) = ({
        let val = decl.clone();
        if let Some(typed_val) = val.downcast_ref::<Arc<Mutex<Option<ast_GenDecl>>>>() {
            (typed_val.clone(), true)
        } else {
            (Arc::new(Mutex::new(None::<ast_GenDecl>)), false)
        }
    });
        if !ok {
        continue
    }
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).specs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for dspec in __range_values.iter() {
        let (mut spec, mut ok) = ({
        let val = dspec.clone();
        if let Some(typed_val) = val.downcast_ref::<Arc<Mutex<Option<ast_ImportSpec>>>>() {
            (typed_val.clone(), true)
        } else {
            (Arc::new(Mutex::new(None::<ast_ImportSpec>)), false)
        }
    });
        if !ok {
        continue
    }
        let mut quoted = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*spec.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let (mut path, mut err) = strconv::unquote({ let __arg_holder = quoted.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("parser returned invalid quoted string: <{}>", { let __v = (*quoted.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    }
        if !is_valid_import(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // The parser used to return a parse error for invalid import paths, but
                // no longer does, so check for and create the error here instead.
        { let new_val = Box::new(scanner_Error { pos: (*(*info.lock().unwrap().as_ref().unwrap()).fset.lock().unwrap().as_mut().unwrap()).position({ let __recv = spec.clone(); let __recv_ptr: *mut ast_ImportSpec = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut ast_ImportSpec }; let __result = unsafe { &mut *__recv_ptr }.pos(); __result }), msg: Arc::new(Mutex::new(Some(format!("{}{}", "invalid import path: ".to_string(), { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v })))), ..Default::default() }) as Box<dyn StdError + Send + Sync>; *(*info.lock().unwrap().as_ref().unwrap()).parse_err.lock().unwrap() = Some(new_val); };
        *(*info.lock().unwrap().as_ref().unwrap()).imports.lock().unwrap() = None;
        return Arc::new(Mutex::new(None));
    }
                // The parser used to return a parse error for invalid import paths, but
                // no longer does, so check for and create the error here instead.
        if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "embed".to_string(); __tmp_x == __tmp_y } {
        { let new_val = true; *hasEmbed.lock().unwrap() = Some(new_val); };
    }
        let mut doc = (*spec.lock().unwrap().as_ref().unwrap()).doc.clone();
        if { let __nil_result = (*doc.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = (({ let __len_target = { let __field = (*d.lock().unwrap().as_ref().unwrap()).specs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let new_val = (*d.lock().unwrap().as_ref().unwrap()).doc.clone(); doc = new_val; };
    }
        { let new_val = { let __append_target = (*info.lock().unwrap().as_ref().unwrap()).imports.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(fileImport { path: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: { let __recv = spec.clone(); let __recv_ptr: *mut ast_ImportSpec = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut ast_ImportSpec }; let __result = unsafe { &mut *__recv_ptr }.pos(); __result }, doc: doc.clone(), ..Default::default() }); __append_target.clone() }; (*info.lock().unwrap().as_mut().unwrap()).imports = new_val; };
    } }
    } }

        // The parser used to return a parse error for invalid import paths, but
        // no longer does, so check for and create the error here instead.
        // Extract directives.
    { let __range_holder = (*(*info.lock().unwrap().as_ref().unwrap()).parsed.lock().unwrap().as_ref().unwrap()).comments.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for group in __range_values.iter() {
        if { let __tmp_x = (*{ let __recv = group.clone(); let __recv_ptr: *mut ast_CommentGroup = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut ast_CommentGroup }; let __result = unsafe { &mut *__recv_ptr }.pos(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*(*info.lock().unwrap().as_ref().unwrap()).parsed.lock().unwrap().as_ref().unwrap()).package.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x >= __tmp_y } {
        break
    }
        { let __range_holder = (*group.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter() {
        if strings::has_prefix(Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).text.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some("//go:".to_string())))) {
        { let new_val = { let __append_target = (*info.lock().unwrap().as_ref().unwrap()).directives.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Directive { text: Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).text.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), pos: (*(*info.lock().unwrap().as_ref().unwrap()).fset.lock().unwrap().as_mut().unwrap()).position({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).slash.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }), ..Default::default() }); __append_target.clone() }; (*info.lock().unwrap().as_mut().unwrap()).directives = new_val; };
    }
    } }
    } }

        // If the file imports "embed",
        // we have to look for //go:embed comments
        // in the remainder of the file.
        // The compiler will enforce the mapping of comments to
        // declared variables. We just need to know the patterns.
        // If there were //go:embed comments earlier in the file
        // (near the package statement or imports), the compiler
        // will reject them. They can be (and have already been) ignored.
    if { let __v = (*hasEmbed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut line: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
        let mut first = Arc::new(Mutex::new(Some(true)));
    while { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.find_embed(Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = line.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); line = new_val; };
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        loop {
        let mut c = { let __recv = r.clone(); let __recv_ptr: *mut importReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut importReader }; let __result = unsafe { &mut *__recv_ptr }.read_byte_no_buf(); __result };
        if { let __tmp_x = c; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } || { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).eof.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        break
    }
        { let new_val = { let __append_target = line.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(c); __append_target.clone() }; line = new_val; };
    }

                // Add args if line is well-formed.
                // Ignore badly-formed lines - the compiler will report them when it finds them,
                // and we can pretend they are not there to help go list succeed with what it knows.
        let (mut embs, mut err) = parse_go_embed(Arc::new(Mutex::new(Some(String::from_utf8((*line.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = { let __append_target = (*info.lock().unwrap().as_ref().unwrap()).embeds.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = embs.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*info.lock().unwrap().as_mut().unwrap()).embeds = new_val; };
    }
        { let new_val = false; *first.lock().unwrap() = Some(new_val); };
    }
    }

        // Add args if line is well-formed.
        // Ignore badly-formed lines - the compiler will report them when it finds them,
        // and we can pretend they are not there to help go list succeed with what it knows.
    return Arc::new(Mutex::new(None));
}

/// isValidImport checks if the import is a valid import using the more strict
/// checks allowed by the implementation restriction in https://go.dev/ref/spec#Import_declarations.
/// It was ported from the function of the same name that was removed from the
/// parser in CL 424855, when the parser stopped doing these checks.
pub fn is_valid_import(s: Arc<Mutex<Option<String>>>) -> bool {
    const illegalChars: &'static str = "!\"#$%&'()*,:;<=>?[\\]^{|}`\u{fffd}";

    for (_, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if !unicode::is_graphic(Arc::new(Mutex::new(Some(r as i32)))) || unicode::is_space(Arc::new(Mutex::new(Some(r as i32)))) || strings::contains_rune(Arc::new(Mutex::new(Some("!\"#$%&'()*,:;<=>?[\\]^{|}`\u{fffd}".to_string()))), Arc::new(Mutex::new(Some(r as i32)))) {
        return false;
    }
    }
    return { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y };
}

/// parseGoEmbed parses the text following "//go:embed" to extract the glob patterns.
/// It accepts unquoted space-separated patterns as well as double-quoted and back-quoted Go strings.
/// This is based on a similar function in cmd/compile/internal/gc/noder.go;
/// this version calculates position information as well.
pub fn parse_go_embed(args: Arc<Mutex<Option<String>>>, pos: Arc<Mutex<Option<token_Position>>>) -> (Arc<Mutex<Option<Vec<crate::r#mod::fileEmbed>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut args_closure_clone = args.clone(); let pos_closure_clone = pos.clone(); let mut trimBytes = Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| {
        { let __target = (*pos_closure_clone.lock().unwrap().as_ref().unwrap()).offset.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = (*pos_closure_clone.lock().unwrap().as_ref().unwrap()).column.clone(); let __rhs = unicode_utf8::rune_count_in_string(Arc::new(Mutex::new(Some({ let __s = &((*args_closure_clone.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*args_closure_clone.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *args_closure_clone.lock().unwrap() = __moved_val; };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>)));
    let args_closure_clone = args.clone(); let trimBytes_closure_clone = trimBytes.clone(); let mut trimSpace = Arc::new(Mutex::new(Some(Box::new(move || {
        let mut trim = strings::trim_left_func(Arc::new(Mutex::new(Some({ let __arg_holder = args_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> bool { unicode::is_space(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = trimBytes_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = ((*args_closure_clone.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*trim.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x - __tmp_y })))) };
    }) as Box<dyn FnMut() -> () + Send + Sync>)));

    let mut list: Arc<Mutex<Option<Vec<fileEmbed>>>> = Arc::new(Mutex::new(None));
    { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = trimSpace.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    while { let __tmp_x = (*args.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let mut path: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut pathPos = { let __owned = pos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        'switch: {
            { let _switch_val = { let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] };
    if _switch_val == (('`' as i32) as u8) {
            let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
            { let (__tmp_0, __tmp_1, __tmp_2) = strings::cut(Arc::new(Mutex::new(Some({ let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some("`".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *path.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_2); };
            if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid quoted string in //go:embed: {}", { let __v = (*args.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = trimBytes.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = 1; let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y })))) };
        } else if _switch_val == (('"' as i32) as u8) {
            let mut i = Arc::new(Mutex::new(Some(1)));
            while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*args.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('\\' as i32) as u8; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        if { let __tmp_x = { let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('"' as i32) as u8; __tmp_x == __tmp_y } {
        let (mut q, mut err) = strconv::unquote(Arc::new(Mutex::new(Some({ let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[..__high].to_string() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid quoted string in //go:embed: {}", (*Arc::new(Mutex::new(Some({ let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap())))))));
    }
        { let new_val = q.lock().unwrap().as_ref().unwrap().clone(); *path.lock().unwrap() = Some(new_val); };
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = trimBytes.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))) };
        break 'switch
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*args.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid quoted string in //go:embed: {}", { let __v = (*args.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }
        } else {
            let mut i = Arc::new(Mutex::new(Some((*args.lock().unwrap().as_ref().unwrap()).len() as i32)));
            for (j, c) in (*args.lock().unwrap().as_ref().unwrap()).char_indices() {
        if unicode::is_space(Arc::new(Mutex::new(Some(c as i32)))) {
        { let new_val = j as i32; *i.lock().unwrap() = Some(new_val); };
        break
    }
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*args.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *path.lock().unwrap() = __moved_val; };
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = trimBytes.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(i.clone()) };
        }
    }
        }

        if { let __tmp_x = (*args.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let (mut r, _) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = args.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if !unicode::is_space(Arc::new(Mutex::new(Some(r)))) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid quoted string in //go:embed: {}", { let __v = (*args.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }
    }
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(fileEmbed { pattern: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pathPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }); __append_target.clone() }; list = new_val; };
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = trimSpace.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
    return (list.clone(), Arc::new(Mutex::new(None)));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for importReader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
