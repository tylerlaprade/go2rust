use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// In an [ErrorList], an error is represented by an *Error.
/// The position Pos, if valid, points to the beginning of
/// the offending token, and the error condition is described
/// by Msg.
#[derive(Clone)]
pub struct Error {
    pub pos: Arc<Mutex<Option<go_token::position::Position>>>,
    pub msg: Arc<Mutex<Option<String>>>,
}

impl Error {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, msg: { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Error {
    fn default() -> Self {
        Self { pos: Arc::new(Mutex::new(Some(Default::default()))), msg: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for Error {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msg") {
            out.msg = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// ErrorList is a list of *Errors.
/// The zero value for an ErrorList is an empty ErrorList ready to use.
#[derive(Clone, Default)]
pub struct ErrorList(pub Arc<Mutex<Option<Vec<Arc<Mutex<Option<Error>>>>>>>);

impl Display for ErrorList {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for ErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


impl Error {
    /// Error implements the error interface.
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = { let __selector_holder = (*self.pos.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } || (*self.pos.lock().unwrap().as_ref().unwrap()).is_valid() {
                // don't print "<unknown position>"
                // TODO(gri) reconsider the semantics of Position.IsValid
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", (*(*self.pos.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ": ".to_string())); __s.push_str(&format!("{}", (*self.msg.clone().lock().unwrap().as_ref().unwrap()))); __s })));
    }
                // don't print "<unknown position>"
                // TODO(gri) reconsider the semantics of Position.IsValid
        return self.msg.clone();
    }
}

impl StdError for Error {}


impl ErrorList {
    /// Add adds an [Error] with given position and error message to an [ErrorList].
    pub fn add(&mut self, pos: Arc<Mutex<Option<go_token::position::Position>>>, msg: Arc<Mutex<Option<String>>>) {
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(Arc::new(Mutex::new(Some(Error { pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), msg: Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))); Arc::new(Mutex::new(Some(ErrorList(Arc::new(Mutex::new(Some(__values))))))) }; *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }

    /// Reset resets an [ErrorList] to no errors.
    pub fn reset(&mut self) {
        { let new_val = ErrorList(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[(0) as usize..(0) as usize].to_vec() })))); *self = new_val; };
    }

    /// [ErrorList] implements the sort Interface.
    pub fn len(&self) -> i32 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32;
    }

    pub fn swap(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) {
        { let __tmp_0 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_1 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_1; };
    }

    pub fn less(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> bool {
        let mut e = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pos.clone();
        let mut f = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pos.clone();
                // Note that it is not sufficient to simply compare file offsets because
                // the offsets do not reflect modified line information (through //line
                // comments).
        if { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } {
        return { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x < __tmp_y };
    }
        if { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
    }
        if { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
    }
        return { let __tmp_x = { let __selector_holder = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x < __tmp_y };
    }

    /// Sort sorts an [ErrorList]. *[Error] entries are sorted by position,
    /// other errors are sorted by error message, and before any *[Error]
    /// entry.
    pub fn sort(&self) {
        sort::sort(Arc::new(Mutex::new(Some(Box::new((*self).clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>))));
    }

    /// RemoveMultiples sorts an [ErrorList] and removes all but the first error per line.
    pub fn remove_multiples(&mut self) {
        sort::sort(Arc::new(Mutex::new(Some(Box::new(ErrorListPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn sort::r#mod::Interface + Send + Sync>))));
        let mut last: Arc<Mutex<Option<go_token::position::Position>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut i = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*(*e.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*last.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } || { let __tmp_x = (*(*(*e.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap()).line.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*last.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *last.lock().unwrap() = Some(new_val); };
        (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = e.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
        { let new_val = ErrorList(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[(0) as usize..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); *self = new_val; };
    }

    /// An [ErrorList] implements the error interface.
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        { let _switch_val = { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) };
    if _switch_val == (0) {
            return Arc::new(Mutex::new(Some("no errors".to_string())));
        } else if _switch_val == (1) {
            return { let __recv = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).error(); __result };
        }
    }
        Arc::new(Mutex::new(Some(format!("{} (and {} more errors)", format!("&{}", (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap())), { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }))))
    }

    /// Err returns an error equivalent to this error list.
    /// If the list is empty, Err returns nil.
    pub fn err(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn StdError + Send + Sync>)))
    }
}

impl StdError for ErrorList {}


impl sort::r#mod::Interface for ErrorList {
    /// [ErrorList] implements the sort Interface.
    fn len(&self) -> i32 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32;
    }
    fn less(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> bool {
        let mut e = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pos.clone();
        let mut f = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).pos.clone();
                // Note that it is not sufficient to simply compare file offsets because
                // the offsets do not reflect modified line information (through //line
                // comments).
        if { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } {
        return { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x < __tmp_y };
    }
        if { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
    }
        if { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return { let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
    }
        return { let __tmp_x = { let __selector_holder = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x < __tmp_y };
    }
    fn swap(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) {
        { let __tmp_0 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_1 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_1; };
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ErrorList>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ErrorListPtr(pub Arc<Mutex<Option<ErrorList>>>);

impl std::fmt::Display for ErrorListPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl sort::r#mod::Interface for ErrorListPtr {
    fn len(&self) -> i32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ErrorList::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ErrorList::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ErrorList::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ErrorListPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl GoValueClone for Error {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
