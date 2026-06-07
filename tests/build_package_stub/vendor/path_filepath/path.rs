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

/// ToSlash returns the result of replacing each separator character
/// in path with a slash ('/') character. Multiple separators are
/// replaced by multiple slashes.
pub fn to_slash(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    internal_filepathlite::to_slash(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// SplitList splits a list of paths joined by the OS-specific [ListSeparator],
/// usually found in PATH or GOPATH environment variables.
/// Unlike strings.Split, SplitList returns an empty slice when passed an empty
/// string.
pub fn split_list(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    split_list_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
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

/// EvalSymlinks returns the path name after the evaluation of any symbolic
/// links.
/// If path is relative the result will be relative to the current directory,
/// unless one of the components is an absolute symbolic link.
/// EvalSymlinks calls [Clean] on the result.
pub fn eval_symlinks(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    eval_symlinks_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// IsAbs reports whether the path is absolute.
pub fn is_abs(path: Arc<Mutex<Option<String>>>) -> bool {
    internal_filepathlite::is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Abs returns an absolute representation of path.
/// If the path is not absolute it will be joined with the current
/// working directory to turn it into an absolute path. The absolute
/// path name for a given file is not guaranteed to be unique.
/// Abs calls [Clean] on the result.
pub fn abs(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    abs_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn unix_abs(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return (clean(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(None)));
    }
    let (mut wd, mut err) = os::getwd();
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
    return (join(Arc::new(Mutex::new(Some(vec![(*wd.lock().unwrap().as_ref().unwrap()).clone(), (*path.lock().unwrap().as_ref().unwrap()).clone()])))), Arc::new(Mutex::new(None)));
}

/// Dir returns all but the last element of path, typically the path's directory.
/// After dropping the final element, Dir calls [Clean] on the path and trailing
/// slashes are removed.
/// If the path is empty, Dir returns ".".
/// If the path consists entirely of separators, Dir returns a single separator.
/// The returned path does not end in a separator unless it is the root directory.
pub fn dir(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    internal_filepathlite::dir(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
