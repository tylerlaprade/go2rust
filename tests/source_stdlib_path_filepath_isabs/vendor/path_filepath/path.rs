use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path_unix::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub const SEPARATOR: i32 = os::PATH_SEPARATOR;
pub const LIST_SEPARATOR: i32 = os::PATH_LIST_SEPARATOR;


/// WalkFunc is the type of the function called by [Walk] to visit each
/// file or directory.
///
/// The path argument contains the argument to Walk as a prefix.
/// That is, if Walk is called with root argument "dir" and finds a file
/// named "a" in that directory, the walk function will be called with
/// argument "dir/a".
///
/// The directory and file are joined with Join, which may clean the
/// directory name: if Walk is called with the root argument "x/../dir"
/// and finds a file named "a" in that directory, the walk function will
/// be called with argument "dir/a", not "x/../dir/a".
///
/// The info argument is the fs.FileInfo for the named path.
///
/// The error result returned by the function controls how Walk continues.
/// If the function returns the special value [SkipDir], Walk skips the
/// current directory (path if info.IsDir() is true, otherwise path's
/// parent directory). If the function returns the special value [SkipAll],
/// Walk skips all remaining files and directories. Otherwise, if the function
/// returns a non-nil error, Walk stops entirely and returns that error.
///
/// The err argument reports an error related to path, signaling that Walk
/// will not walk into that directory. The function can decide how to
/// handle that error; as described earlier, returning the error will
/// cause Walk to stop walking the entire tree.
///
/// Walk calls the function with a non-nil err argument in two cases.
///
/// First, if an [os.Lstat] on the root directory or any directory or file
/// in the tree fails, Walk calls the function with path set to that
/// directory or file's path, info set to nil, and err set to the error
/// from os.Lstat.
///
/// Second, if a directory's Readdirnames method fails, Walk calls the
/// function with path set to the directory's path, info, set to an
/// [fs.FileInfo] describing the directory, and err set to the error from
/// Readdirnames.
pub type WalkFunc = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>;


pub static SkipDir: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static SkipAll: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lstat: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
    *SkipDir.lock().unwrap() = Some(fs::SkipDir());
    *SkipAll.lock().unwrap() = Some(fs::SkipAll());
    *lstat.lock().unwrap() = Some(os::lstat);
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
    filepathlite::clean(path.clone())
}

/// IsLocal reports whether path, using lexical analysis only, has all of these properties:
///
///   - is within the subtree rooted at the directory in which path is evaluated
///   - is not an absolute path
///   - is not empty
///   - on Windows, is not a reserved name such as "NUL"
///
/// If IsLocal(path) returns true, then
/// Join(base, path) will always produce a path contained within base and
/// Clean(path) will always produce an unrooted path with no ".." path elements.
///
/// IsLocal is a purely lexical operation.
/// In particular, it does not account for the effect of any symbolic links
/// that may exist in the filesystem.
pub fn is_local(path: Arc<Mutex<Option<String>>>) -> bool {
    (*filepathlite::is_local(path.clone()).lock().unwrap().as_ref().unwrap())
}

/// Localize converts a slash-separated path into an operating system path.
/// The input path must be a valid path as reported by [io/fs.ValidPath].
///
/// Localize returns an error if the path cannot be represented by the operating system.
/// For example, the path a\b is rejected on Windows, on which \ is a separator
/// character and cannot be part of a filename.
///
/// The path returned by Localize will always be local, as reported by IsLocal.
pub fn localize(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    filepathlite::localize(path.clone())
}

/// ToSlash returns the result of replacing each separator character
/// in path with a slash ('/') character. Multiple separators are
/// replaced by multiple slashes.
pub fn to_slash(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    filepathlite::to_slash(path.clone())
}

/// FromSlash returns the result of replacing each slash ('/') character
/// in path with a separator character. Multiple slashes are replaced
/// by multiple separators.
///
/// See also the Localize function, which converts a slash-separated path
/// as used by the io/fs package to an operating system path.
pub fn from_slash(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    filepathlite::from_slash(path.clone())
}

/// SplitList splits a list of paths joined by the OS-specific [ListSeparator],
/// usually found in PATH or GOPATH environment variables.
/// Unlike strings.Split, SplitList returns an empty slice when passed an empty
/// string.
pub fn split_list(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    split_list_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Split splits path immediately following the final [Separator],
/// separating it into a directory and file name component.
/// If there is no Separator in path, Split returns an empty dir
/// and file set to path.
/// The returned values have the property that path = dir+file.
pub fn split(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
    let mut dir: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    filepathlite::split(path.clone())
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

/// Ext returns the file name extension used by path.
/// The extension is the suffix beginning at the final dot
/// in the final element of path; it is empty if there is
/// no dot.
pub fn ext(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    filepathlite::ext(path.clone())
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
    (*filepathlite::is_abs(path.clone()).lock().unwrap().as_ref().unwrap())
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
    if (*is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()) {
        return (clean(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(None)));
    }
    let (mut wd, mut err) = os::getwd();
    if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
    (join(Arc::new(Mutex::new(Some(vec![{ let __v = (*wd.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }])))), Arc::new(Mutex::new(None)))
}

/// Rel returns a relative path that is lexically equivalent to targpath when
/// joined to basepath with an intervening separator. That is,
/// [Join](basepath, Rel(basepath, targpath)) is equivalent to targpath itself.
/// On success, the returned path will always be relative to basepath,
/// even if basepath and targpath share no elements.
/// An error is returned if targpath can't be made relative to basepath or if
/// knowing the current working directory would be necessary to compute it.
/// Rel calls [Clean] on the result.
pub fn rel(basepath: Arc<Mutex<Option<String>>>, targpath: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut baseVol = volume_name(Arc::new(Mutex::new(Some({ let __arg_holder = basepath.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut targVol = volume_name(Arc::new(Mutex::new(Some({ let __arg_holder = targpath.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut base = clean(Arc::new(Mutex::new(Some({ let __arg_holder = basepath.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut targ = clean(Arc::new(Mutex::new(Some({ let __arg_holder = targpath.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if (*same_word(Arc::new(Mutex::new(Some({ let __arg_holder = targ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(Some(".".to_string()))), Arc::new(Mutex::new(None)));
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*base.lock().unwrap().as_ref().unwrap()).clone()); __s[((*baseVol.lock().unwrap().as_ref().unwrap()).len()) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *base.lock().unwrap() = __moved_val; };
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*targ.lock().unwrap().as_ref().unwrap()).clone()); __s[((*targVol.lock().unwrap().as_ref().unwrap()).len()) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *targ.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*base.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "."; __tmp_x == __tmp_y } {
        { let new_val = "".to_string(); *base.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*base.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ""; __tmp_x == __tmp_y } && { let __tmp_x = (*filepathlite::volume_name_len(baseVol.clone()).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(char::from_u32(((*Separator.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *base.lock().unwrap() = __moved_val; };
    }

        /* isUNC */
        // Treat any targetpath matching `\\host\share` basepath as absolute path.
        // Can't use IsAbs - `\a` and `a` are both relative in Windows.
    let mut baseSlashed = Arc::new(Mutex::new(Some({ let __tmp_x = ((*base.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*base.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = SEPARATOR as u8; __tmp_x == __tmp_y })));
    let mut targSlashed = Arc::new(Mutex::new(Some({ let __tmp_x = ((*targ.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*targ.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = SEPARATOR as u8; __tmp_x == __tmp_y })));
    if { let __tmp_x = { let __v = (*baseSlashed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*targSlashed.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } || !(*same_word(Arc::new(Mutex::new(Some({ let __arg_holder = baseVol.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = targVol.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(format!("{}{}", format!("{}{}", format!("{}{}", "Rel: can't make ".to_string(), { let __v = (*targpath.lock().unwrap().as_ref().unwrap()).clone(); __v }), " relative to ".to_string()), { let __v = (*basepath.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }

        // Position base[b0:bi] and targ[t0:ti] at the first differing elements.
    let mut bl = Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()).len() as i32)));
    let mut tl = Arc::new(Mutex::new(Some((*targ.lock().unwrap().as_ref().unwrap()).len() as i32)));
    let mut b0: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut bi: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut t0: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut ti: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    loop {
        while { let __tmp_x = { let __v = (*bi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __s = &((*base.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*bi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = SEPARATOR as u8; __tmp_x != __tmp_y } {
        { let mut guard = bi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while { let __tmp_x = { let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __s = &((*targ.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = SEPARATOR as u8; __tmp_x != __tmp_y } {
        { let mut guard = ti.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !(*same_word(Arc::new(Mutex::new(Some({ let __s = &((*targ.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*base.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*b0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*bi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() })))).lock().unwrap().as_ref().unwrap()) {
        break
    }
        if { let __tmp_x = { let __v = (*bi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let mut guard = bi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let mut guard = ti.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = bi.lock().unwrap().as_ref().unwrap().clone(); *b0.lock().unwrap() = Some(new_val); };
        { let new_val = ti.lock().unwrap().as_ref().unwrap().clone(); *t0.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*base.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*b0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*bi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".."; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(format!("{}{}", format!("{}{}", format!("{}{}", "Rel: can't make ".to_string(), { let __v = (*targpath.lock().unwrap().as_ref().unwrap()).clone(); __v }), " relative to ".to_string()), { let __v = (*basepath.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }
    if { let __tmp_x = { let __v = (*b0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // Base elements left. Must go up before going down.
        let mut seps = bytealg::count_string(Arc::new(Mutex::new(Some({ let __s = &((*base.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*b0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*bl.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))), SEPARATOR);
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = { let __tmp_x = { let __v = (*seps.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*tl.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let __rhs = { let __tmp_x = { let __tmp_x = 1; let __tmp_y = { let __v = (*tl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        let mut buf = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
        let mut n = { let _src = "..".to_string().as_bytes().to_vec(); let _n = std::cmp::min(({ let __v = (*buf.lock().unwrap().as_ref().unwrap()).clone(); __v }).len(), _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*seps.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = SEPARATOR as u8;
        { let _dst_start = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let _dst_len = (*buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = "..".to_string().as_bytes().to_vec(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let __rhs = 3; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = SEPARATOR as u8;
        { let _dst_start = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let _dst_len = (*buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __s = &((*targ.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
        return (Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), Arc::new(Mutex::new(None)));
    }
        // Base elements left. Must go up before going down.
    return (Arc::new(Mutex::new(Some({ let __s = &((*targ.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_string() }))), Arc::new(Mutex::new(None)));
}

/// walkDir recursively descends path, calling walkDirFn.
pub fn walk_dir_1(path: Arc<Mutex<Option<String>>>, d: Arc<Mutex<Option<fs_DirEntry>>>, walkDirFn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    {
        let mut err = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = walkDirFn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), d.clone(), Arc::new(Mutex::new(None))) };;
        if (*err.lock().unwrap()).is_some() || !(*(*d.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()) {
            if (*err.lock().unwrap()).is_none() == (*SkipDir.lock().unwrap()).is_none() && (*(*d.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()) {
        *err.lock().unwrap() = None;
    };
            return err.clone();;
        }
    }

        // Successfully skipped directory.
    let (mut dirs, mut err) = os::read_dir(path.clone());
    if (*err.lock().unwrap()).is_some() {
                // Second call, to report ReadDir error.
        { let __rhs_holder = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = walkDirFn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), d.clone(), err.clone()) }.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        if (*err.lock().unwrap()).is_some() {
        if (*err.lock().unwrap()).is_none() == (*SkipDir.lock().unwrap()).is_none() && (*(*d.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()) {
        *err.lock().unwrap() = None;
    }
        return err.clone();
    }
    }

        // Second call, to report ReadDir error.
    { let __range_holder = dirs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for d1 in __range_values.iter() {
        let mut path1 = join(Arc::new(Mutex::new(Some(vec![{ let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, d1.name()]))));
        {
        let mut err = walk_dir_1(Arc::new(Mutex::new(Some({ let __arg_holder = path1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*d1).clone()))), walkDirFn.clone());;
        if (*err.lock().unwrap()).is_some() {
            if (*err.lock().unwrap()).is_none() == (*SkipDir.lock().unwrap()).is_none() {
        break
    };
            return err.clone();;
        }
    }
    } }
    Arc::new(Mutex::new(None))
}

/// walk recursively descends path, calling walkFn.
pub fn walk_1(path: Arc<Mutex<Option<String>>>, info: Arc<Mutex<Option<fs_FileInfo>>>, walkFn: WalkFunc) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    if !(*(*info.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()) {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = walkFn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), info.clone(), Arc::new(Mutex::new(None))) };
    }

    let (mut names, mut err) = read_dir_names(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut err1 = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = walkFn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), info.clone(), err.clone()) };

        // If err != nil, walk can't walk into this directory.
        // err1 != nil means walkFn want walk to skip this directory or stop walking.
        // Therefore, if one of err and err1 isn't nil, walk will return.
    if (*err.lock().unwrap()).is_some() || (*err1.lock().unwrap()).is_some() {
                // The caller's behavior is controlled by the return value, which is decided
                // by walkFn. walkFn may ignore err and return nil.
                // If walkFn returns SkipDir or SkipAll, it will be handled by the caller.
                // So walk should return whatever walkFn returns.
        return err1.clone();
    }

        // The caller's behavior is controlled by the return value, which is decided
        // by walkFn. walkFn may ignore err and return nil.
        // If walkFn returns SkipDir or SkipAll, it will be handled by the caller.
        // So walk should return whatever walkFn returns.
    { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        let mut filename = join(Arc::new(Mutex::new(Some(vec![{ let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, name]))));
        let (mut fileInfo, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = lstat.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
        if (*err.lock().unwrap()).is_some() {
        {
        let mut err = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = walkFn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), fileInfo.clone(), err.clone()) };;
        if (*err.lock().unwrap()).is_some() && (*err.lock().unwrap()).is_none() != (*SkipDir.lock().unwrap()).is_none() {
            return err.clone();;
        }
    }
    } else {
        { let __rhs_holder = walk_1(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), fileInfo.clone(), walkFn.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        if (*err.lock().unwrap()).is_some() {
        if !(*(*fileInfo.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()) || (*err.lock().unwrap()).is_none() != (*SkipDir.lock().unwrap()).is_none() {
        return err.clone();
    }
    }
    }
    } }
    Arc::new(Mutex::new(None))
}

/// WalkDir walks the file tree rooted at root, calling fn for each file or
/// directory in the tree, including root.
///
/// All errors that arise visiting files and directories are filtered by fn:
/// see the [fs.WalkDirFunc] documentation for details.
///
/// The files are walked in lexical order, which makes the output deterministic
/// but requires WalkDir to read an entire directory into memory before proceeding
/// to walk that directory.
///
/// WalkDir does not follow symbolic links.
///
/// WalkDir calls fn with paths that use the separator character appropriate
/// for the operating system. This is unlike [io/fs.WalkDir], which always
/// uses slash separated paths.
pub fn walk_dir_1(root: Arc<Mutex<Option<String>>>, r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let (mut info, mut err) = os::lstat(root.clone());
    if (*err.lock().unwrap()).is_some() {
        { let __rhs_holder = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_DirEntry>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), err.clone()) }.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    } else {
        { let __rhs_holder = walk_dir_1(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), fs::file_info_to_dir_entry(info.clone()), r#fn.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    if (*err.lock().unwrap()).is_none() == (*SkipDir.lock().unwrap()).is_none() || (*err.lock().unwrap()).is_none() == (*SkipAll.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(None));
    }
    err.clone()
}

/// Walk walks the file tree rooted at root, calling fn for each file or
/// directory in the tree, including root.
///
/// All errors that arise visiting files and directories are filtered by fn:
/// see the [WalkFunc] documentation for details.
///
/// The files are walked in lexical order, which makes the output deterministic
/// but requires Walk to read an entire directory into memory before proceeding
/// to walk that directory.
///
/// Walk does not follow symbolic links.
///
/// Walk is less efficient than [WalkDir], introduced in Go 1.16,
/// which avoids calling os.Lstat on every visited file or directory.
pub fn walk_1(root: Arc<Mutex<Option<String>>>, r#fn: WalkFunc) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let (mut info, mut err) = os::lstat(root.clone());
    if (*err.lock().unwrap()).is_some() {
        { let __rhs_holder = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), err.clone()) }.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    } else {
        { let __rhs_holder = walk_1(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), info.clone(), r#fn.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    if (*err.lock().unwrap()).is_none() == (*SkipDir.lock().unwrap()).is_none() || (*err.lock().unwrap()).is_none() == (*SkipAll.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(None));
    }
    err.clone()
}

/// readDirNames reads the directory named by dirname and returns
/// a sorted list of directory entry names.
pub fn read_dir_names(dirname: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut f, mut err) = os::open(dirname.clone());
    if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    let (mut names, mut err) = { let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.readdirnames(Arc::new(Mutex::new(Some(-1)))); __result };
    { let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    { let mut __sort_guard = names.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    (names.clone(), Arc::new(Mutex::new(None)))
}

/// Base returns the last element of path.
/// Trailing path separators are removed before extracting the last element.
/// If the path is empty, Base returns ".".
/// If the path consists entirely of separators, Base returns a single separator.
pub fn base(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    filepathlite::base(path.clone())
}

/// Dir returns all but the last element of path, typically the path's directory.
/// After dropping the final element, Dir calls [Clean] on the path and trailing
/// slashes are removed.
/// If the path is empty, Dir returns ".".
/// If the path consists entirely of separators, Dir returns a single separator.
/// The returned path does not end in a separator unless it is the root directory.
pub fn dir(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    filepathlite::dir(path.clone())
}

/// VolumeName returns leading volume name.
/// Given "C:\foo\bar" it returns "C:" on Windows.
/// Given "\\host\share\foo" it returns "\\host\share".
/// On other platforms it returns "".
pub fn volume_name(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    filepathlite::volume_name(path.clone())
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
