use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path::*;
use crate::path_unix::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn walk_symlinks(mut path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {

    let mut volLen = filepathlite::volume_name_len(path.clone());
    let mut pathSeparator = Arc::new(Mutex::new(Some(char::from_u32((os::PATH_SEPARATOR) as u32).unwrap().to_string())));

    if { let __tmp_x = ({ let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && (*os::is_path_separator({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        { let mut guard = volLen.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut vol = Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[..({ let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() })));
    let mut dest = Arc::new(Mutex::new(Some(vol.lock().unwrap().as_ref().unwrap().clone())));
    let mut linksWalked = Arc::new(Mutex::new(Some(0)));
    let (mut start, mut end) = (Arc::new(Mutex::new(Some(volLen.lock().unwrap().as_ref().unwrap().clone()))), Arc::new(Mutex::new(Some(volLen.lock().unwrap().as_ref().unwrap().clone()))));
    while { let __tmp_x = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        while { let __tmp_x = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && (*os::is_path_separator({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        { let mut guard = start.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = start.lock().unwrap().as_ref().unwrap().clone(); *end.lock().unwrap() = Some(new_val); };
        while { let __tmp_x = ({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && !(*os::is_path_separator({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        { let mut guard = end.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // On Windows, "." can be a symlink.
                // We look it up, and use the value if it is absolute.
                // If not, we just return ".".
        let mut isWindowsDot = Arc::new(Mutex::new(Some({ let __tmp_x = runtime::G_O_O_S; let __tmp_y = "windows"; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[(*filepathlite::volume_name_len(path.clone()).lock().unwrap().as_ref().unwrap()) as usize..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "."; __tmp_x == __tmp_y })));

                // The next path component is in path[start:end].
        if { let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
                // No more path components.
        break
    } else if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "."; __tmp_x == __tmp_y } && !{ let __v = (*isWindowsDot.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = end.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };; continue
    } else if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".."; __tmp_x == __tmp_y } {
        let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let new_val = { let __tmp_x = ((*dest.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }; *r.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        if (*os::is_path_separator({ let __s = &((*dest.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        break
    }
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*dest.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".."; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*dest.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } {
        { (*dest.lock().unwrap().as_mut().unwrap()).push_str(&{ let __v = (*pathSeparator.lock().unwrap().as_ref().unwrap()).clone(); __v }); };
    }
        { (*dest.lock().unwrap().as_mut().unwrap()).push_str(&"..".to_string()); };
    } else {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*dest.lock().unwrap().as_ref().unwrap()).clone()); __s[..({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dest.lock().unwrap() = __moved_val; };
    }
        { let new_val = end.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };; continue
    }

                // No more path components.
                // Ignore path component ".".
                // Back up to previous component if possible.
                // Note that volLen includes any leading slash.
                // Set r to the index of the last slash in dest,
                // after the volume.
                // Either path has no slashes
                // (it's empty or just "C:")
                // or it ends in a ".." we had to keep.
                // Either way, keep this "..".
                // Discard everything since the last slash.
                // Ordinary path component. Add it to result.
        if { let __tmp_x = ((*dest.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*filepathlite::volume_name_len(dest.clone()).lock().unwrap().as_ref().unwrap()) as i32); __tmp_x > __tmp_y } && !(*os::is_path_separator({ let __s = &((*dest.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = ((*dest.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        { (*dest.lock().unwrap().as_mut().unwrap()).push_str(&{ let __v = (*pathSeparator.lock().unwrap().as_ref().unwrap()).clone(); __v }); };
    }

        { (*dest.lock().unwrap().as_mut().unwrap()).push_str(&Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() })))); };

                // Resolve symlink.
        let (mut fi, mut err) = os::lstat(dest.clone());
        if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }

        if { let __tmp_x = { let __tmp_x = (*(*fi.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = fs_FileMode(fs::MODE_SYMLINK.0 as u32); __tmp_x & __tmp_y }; let __tmp_y = fs_FileMode(0 as u32); __tmp_x == __tmp_y } {
        if !(*{ let __recv = (*fi.lock().unwrap().as_ref().unwrap()).mode(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_dir(); __result }.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = ({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::new(syscall::E_N_O_T_D_I_R) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = end.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };; continue
    }

                // Found symlink.
        { let mut guard = linksWalked.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*linksWalked.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 255; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("EvalSymlinks: too many links".to_string())))));
    }

        let (mut link, mut err) = os::readlink(dest.clone());
        if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }

        if { let __v = (*isWindowsDot.lock().unwrap().as_ref().unwrap()).clone(); __v } && !(*is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = link.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()) {
                // On Windows, if "." is a relative symlink,
                // just return ".".
        break
    }

                // On Windows, if "." is a relative symlink,
                // just return ".".
        { let new_val = format!("{}{}", { let __v = (*link.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_string() }))).lock().unwrap().as_ref().unwrap())); *path.lock().unwrap() = Some(new_val); };

        let mut v = filepathlite::volume_name_len(link.clone());
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
                // Symlink to drive name is an absolute path.
        if { let __tmp_x = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*link.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && (*os::is_path_separator({ let __s = &((*link.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        { let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*link.lock().unwrap().as_ref().unwrap()).clone()); __s[..({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *vol.lock().unwrap() = __moved_val; };
        { let new_val = vol.lock().unwrap().as_ref().unwrap().clone(); *dest.lock().unwrap() = Some(new_val); };
        { let new_val = (*vol.lock().unwrap().as_ref().unwrap()).len() as i32; *end.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = ((*link.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } && (*os::is_path_separator({ let __s = &((*link.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }).lock().unwrap().as_ref().unwrap()) {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*link.lock().unwrap().as_ref().unwrap()).clone()); __s[..(1) as usize].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dest.lock().unwrap() = __moved_val; };
        { let new_val = 1; *end.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*link.lock().unwrap().as_ref().unwrap()).clone()); __s[..(1) as usize].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *vol.lock().unwrap() = __moved_val; };
        { let new_val = 1; *volLen.lock().unwrap() = Some(new_val); };
    } else {
        let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let new_val = { let __tmp_x = ((*dest.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }; *r.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        if (*os::is_path_separator({ let __s = &((*dest.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }).lock().unwrap().as_ref().unwrap()) {
        break
    }
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*volLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = vol.lock().unwrap().as_ref().unwrap().clone(); *dest.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*dest.lock().unwrap().as_ref().unwrap()).clone()); __s[..({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dest.lock().unwrap() = __moved_val; };
    }
        { let new_val = 0; *end.lock().unwrap() = Some(new_val); };
    }
        { let new_val = end.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };
    }
        // On Windows, "." can be a symlink.
        // We look it up, and use the value if it is absolute.
        // If not, we just return ".".
        // The next path component is in path[start:end].
        // No more path components.
        // Ignore path component ".".
        // Back up to previous component if possible.
        // Note that volLen includes any leading slash.
        // Set r to the index of the last slash in dest,
        // after the volume.
        // Either path has no slashes
        // (it's empty or just "C:")
        // or it ends in a ".." we had to keep.
        // Either way, keep this "..".
        // Discard everything since the last slash.
        // Ordinary path component. Add it to result.
        // Resolve symlink.
        // Found symlink.
        // On Windows, if "." is a relative symlink,
        // just return ".".
        // Symlink to drive name is an absolute path.
        // Symlink to absolute path.
        // Symlink to relative path; replace last
        // path component in dest.
    return (clean(Arc::new(Mutex::new(Some({ let __arg_holder = dest.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(None)));
}