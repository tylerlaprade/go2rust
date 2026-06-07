use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_any_slice, format_any_variadic, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::r#mod::*;
use crate::resolver::*;

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const PACKAGE_CLAUSE_ONLY: u64 = 1 << 0;
pub const IMPORTS_ONLY: u64 = 1 << 1;
pub const PARSE_COMMENTS: u64 = 1 << 2;
pub const TRACE: u64 = 1 << 3;
pub const DECLARATION_ERRORS: u64 = 1 << 4;
pub const SPURIOUS_ERRORS: u64 = 1 << 5;
pub const SKIP_OBJECT_RESOLUTION: u64 = 1 << 6;
pub const ALL_ERRORS: u64 = SPURIOUS_ERRORS;


/// A Mode value is a set of flags (or 0).
/// They control the amount of source code parsed and other optional
/// parser functionality.
#[derive(Debug, Clone, Default)]
pub struct Mode(pub Arc<Mutex<Option<u64>>>);

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for Mode {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for Mode {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Mode> for u64 {
    fn eq(&self, other: &Mode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Mode> for u64 {
    fn partial_cmp(&self, other: &Mode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Mode {
    type Output = Mode;
    fn add(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for Mode {
    type Output = Mode;
    fn add(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Mode> for u64 {
    type Output = Mode;
    fn add(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Mode {
    type Output = Mode;
    fn sub(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for Mode {
    type Output = Mode;
    fn sub(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Mode> for u64 {
    type Output = Mode;
    fn sub(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Mode {
    type Output = Mode;
    fn mul(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for Mode {
    type Output = Mode;
    fn mul(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Mode> for u64 {
    type Output = Mode;
    fn mul(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Mode {
    type Output = Mode;
    fn div(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for Mode {
    type Output = Mode;
    fn div(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Mode> for u64 {
    type Output = Mode;
    fn div(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Mode {
    type Output = Mode;
    fn rem(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for Mode {
    type Output = Mode;
    fn rem(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Mode> for u64 {
    type Output = Mode;
    fn rem(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Mode {
    type Output = Mode;
    fn bitand(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for Mode {
    type Output = Mode;
    fn bitand(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Mode> for u64 {
    type Output = Mode;
    fn bitand(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Mode {
    type Output = Mode;
    fn bitor(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for Mode {
    type Output = Mode;
    fn bitor(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Mode> for u64 {
    type Output = Mode;
    fn bitor(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Mode {
    type Output = Mode;
    fn bitxor(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for Mode {
    type Output = Mode;
    fn bitxor(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Mode> for u64 {
    type Output = Mode;
    fn bitxor(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Mode {
    type Output = Mode;
    fn not(self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Mode {
    type Output = Mode;
    fn shl(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Mode {
    type Output = Mode;
    fn shl(self, other: i32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Mode {
    type Output = Mode;
    fn shl(self, other: i8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Mode {
    type Output = Mode;
    fn shl(self, other: i16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Mode {
    type Output = Mode;
    fn shl(self, other: i64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Mode {
    type Output = Mode;
    fn shl(self, other: u32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Mode {
    type Output = Mode;
    fn shl(self, other: u8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Mode {
    type Output = Mode;
    fn shl(self, other: u16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Mode {
    type Output = Mode;
    fn shl(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Mode {
    type Output = Mode;
    fn shl(self, other: usize) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Mode {
    type Output = Mode;
    fn shr(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Mode {
    type Output = Mode;
    fn shr(self, other: i32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Mode {
    type Output = Mode;
    fn shr(self, other: i8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Mode {
    type Output = Mode;
    fn shr(self, other: i16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Mode {
    type Output = Mode;
    fn shr(self, other: i64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Mode {
    type Output = Mode;
    fn shr(self, other: u32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Mode {
    type Output = Mode;
    fn shr(self, other: u8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Mode {
    type Output = Mode;
    fn shr(self, other: u16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Mode {
    type Output = Mode;
    fn shr(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Mode {
    type Output = Mode;
    fn shr(self, other: usize) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Mode {}

impl Ord for Mode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// If src != nil, readSource converts src to a []byte if possible;
/// otherwise it returns an error. If src == nil, readSource returns
/// the result of reading the file specified by filename.
pub fn read_source(filename: Arc<Mutex<Option<String>>>, src: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __nil_result = (*src.lock().unwrap()).is_some(); __nil_result } {
        {
    let _ts_subject = src.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        drop(_ts_guard);
        return (Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec()))), Arc::new(Mutex::new(None)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Vec<u8>>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Vec<u8>>()).unwrap().clone())));
        drop(_ts_guard);
        return (s.clone(), Arc::new(Mutex::new(None)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<bytes_Buffer>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<bytes_Buffer>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        return ({ let __recv = s.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.bytes(); __result }, Arc::new(Mutex::new(None)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<io_Reader>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<io_Reader>()).unwrap().clone())));
        drop(_ts_guard);
        return io::read_all(s.clone());;
    }
    }
                // is io.Reader, but src is already available in []byte form
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid source".to_string())))));
    }
        // is io.Reader, but src is already available in []byte form
    os::read_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })
}

/// ParseFile parses the source code of a single Go source file and returns
/// the corresponding [ast.File] node. The source code may be provided via
/// the filename of the source file, or via the src parameter.
///
/// If src != nil, ParseFile parses the source from src and the filename is
/// only used when recording position information. The type of the argument
/// for the src parameter must be string, []byte, or [io.Reader].
/// If src == nil, ParseFile parses the file specified by filename.
///
/// The mode parameter controls the amount of source text parsed and
/// other optional parser functionality. If the [SkipObjectResolution]
/// mode bit is set (recommended), the object resolution phase of
/// parsing will be skipped, causing File.Scope, File.Unresolved, and
/// all Ident.Obj fields to be nil. Those fields are deprecated; see
/// [ast.Object] for details.
///
/// Position information is recorded in the file set fset, which must not be
/// nil.
///
/// If the source couldn't be read, the returned AST is nil and the error
/// indicates the specific failure. If the source was read but syntax
/// errors were found, the result is a partial AST (with [ast.Bad]* nodes
/// representing the fragments of erroneous source code). Multiple errors
/// are returned via a scanner.ErrorList which is sorted by source position.
pub fn parse_file(fset: Arc<Mutex<Option<go_token::position::FileSet>>>, filename: Arc<Mutex<Option<String>>>, src: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, mode: Arc<Mutex<Option<Mode>>>) -> (Arc<Mutex<Option<go_ast::r#mod::File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut f: Arc<Mutex<Option<go_ast::r#mod::File>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if { let __nil_result = (*fset.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("parser.ParseFile: no token.FileSet provided (fset == nil)".to_string()) as Box<dyn Any + Send + Sync>);
    }

                // get source
        let (mut text, __tmp_1) = read_source(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), src.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        *f.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (f.clone(), err.clone());
    }
    }

        let mut file = { let __recv = fset.clone(); let __recv_ptr: *mut go_token::position::FileSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut go_token::position::FileSet }; let __result = unsafe { &mut *__recv_ptr }.add_file(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some((*text.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))); __result };

        let mut p: Arc<Mutex<Option<parser>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut err_defer_captured = err.clone(); let mut f_defer_captured = f.clone(); let file_defer_captured = file.clone(); let p_defer_captured = p.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        {
        let mut e = go_recover();;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            let (mut bail, mut ok) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<bailout>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), false)
        }
    });;
            if !ok {
        std::panic::panic_any({ let __any_holder = e.clone(); let __any_guard = __any_holder.lock().unwrap(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) });
    } else if { let __tmp_x = { let __selector_holder = (*bail.lock().unwrap().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        (*(*p_defer_captured.lock().unwrap().as_ref().unwrap()).errors.lock().unwrap().as_mut().unwrap()).add((*(*p_defer_captured.lock().unwrap().as_ref().unwrap()).file.lock().unwrap().as_ref().unwrap()).position(Arc::new(Mutex::new(Some({ let __selector_holder = (*bail.lock().unwrap().as_ref().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))), Arc::new(Mutex::new(Some({ let __selector_holder = (*bail.lock().unwrap().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    };
        }
    }
        if { let __nil_result = (*f_defer_captured.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::File { name: Arc::new(Mutex::new(Some(go_ast::r#mod::Ident::default()))).clone(), scope: go_ast::new_scope(Arc::new(Mutex::new(None))).clone(), ..Default::default() }))).clone(); f_defer_captured = new_val; };
    }
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some({ let __recv = file_defer_captured.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.base(); __result } as i32)))); *(*f_defer_captured.lock().unwrap().as_ref().unwrap()).file_start.lock().unwrap() = Some(new_val); };
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv = file_defer_captured.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.base(); __result }; let __tmp_y = { let __recv = file_defer_captured.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.size(); __result }; __tmp_x + __tmp_y } as i32)))); *(*f_defer_captured.lock().unwrap().as_ref().unwrap()).file_end.lock().unwrap() = Some(new_val); };
        (*(*p_defer_captured.lock().unwrap().as_ref().unwrap()).errors.lock().unwrap().as_ref().unwrap()).sort();
        { let __rhs_holder = (*(*p_defer_captured.lock().unwrap().as_ref().unwrap()).errors.lock().unwrap().as_ref().unwrap()).err().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err_defer_captured.lock().unwrap() = new_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

                // resume same panic if it's not a bailout
                // set result values
                // source is not a valid Go source file - satisfy
                // ParseFile API and return a valid (but) empty
                // *ast.File
                // Ensure the start/end are consistent,
                // whether parsing succeeded or not.
                // parse source
        (*p.lock().unwrap().as_mut().unwrap()).init(file.clone(), text.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*p.lock().unwrap().as_mut().unwrap()).parse_file().clone(); f = new_val; };

        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (f.clone(), err.clone());
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
            (f.clone(), err.clone())
        }
    }
}