use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path_unix::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub const SEPARATOR: i32 = os::PATH_SEPARATOR as i32;
pub const LIST_SEPARATOR: i32 = os::PATH_LIST_SEPARATOR as i32;


pub static SkipDir: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static SkipAll: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lstat: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
    { let __rhs_holder = fs::SkipDir().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipDir.lock().unwrap() = new_val; }
    { let __rhs_holder = fs::SkipAll().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipAll.lock().unwrap() = new_val; }
    *lstat.lock().unwrap() = Some(Box::new(os::lstat));
}


pub(crate) fn __go_zero_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_1() {
    { let __rhs_holder = fs::SkipDir().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipDir.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = fs::SkipAll().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipAll.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_3() {
    *lstat.lock().unwrap() = Some(Box::new(os::lstat));
}


/// Clean returns the shortest path name equivalent to path
/// by purely lexical processing. It applies the following rules
/// iteratively until no further processing can be done:
///
///  1. Replace multiple [Separator] elements with a single one.
///  2. Eliminate each . path name element (the current directory).
///  3. Eliminate each inner .. path name element (the parent directory)
///     along with the non-.. element that precedes it.
///  4. Eliminate .. elements that begin a rooted path:
///     that is, replace "/.." by "/" at the beginning of a path,
///     assuming Separator is '/'.
///
/// The returned path ends in a slash only if it represents a root directory,
/// such as "/" on Unix or `C:\` on Windows.
///
/// Finally, any occurrences of slash are replaced by Separator.
///
/// If the result of this process is an empty string, Clean
/// returns the string ".".
///
/// On Windows, Clean does not modify the volume name other than to replace
/// occurrences of "/" with `\`.
/// For example, Clean("//host/share/../x") returns `\\host\share\x`.
///
/// See also Rob Pike, “Lexical File Names in Plan 9 or
/// Getting Dot-Dot Right,”
/// https://9p.io/sys/doc/lexnames.html
pub fn clean(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    internal_filepathlite::clean(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Join joins any number of path elements into a single path,
/// separating them with an OS specific [Separator]. Empty elements
/// are ignored. The result is Cleaned. However, if the argument
/// list is empty or all its elements are empty, Join returns
/// an empty string.
/// On Windows, the result will only be a UNC path if the first
/// non-empty element is a UNC path.
pub fn join(elem: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {
    join_1(elem.clone())
}

/// Base returns the last element of path.
/// Trailing path separators are removed before extracting the last element.
/// If the path is empty, Base returns ".".
/// If the path consists entirely of separators, Base returns a single separator.
pub fn base(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    internal_filepathlite::base(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
