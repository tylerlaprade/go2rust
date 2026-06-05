use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Version is a parsed Go version: major[.Minor[.Patch]][kind[pre]]
/// The numbers are the original decimal strings to avoid integer overflows
/// and since there is very little actual math. (Probably overflow doesn't matter in practice,
/// but at the time this code was written, there was an existing test that used
/// go1.99999999999, which does not fit in an int on 32-bit platforms.
/// The "big decimal" representation avoids the problem entirely.)
#[derive(Debug, Clone)]
pub struct Version {
    pub major: Arc<Mutex<Option<String>>>,
    pub minor: Arc<Mutex<Option<String>>>,
    pub patch: Arc<Mutex<Option<String>>>,
    pub kind: Arc<Mutex<Option<String>>>,
    pub pre: Arc<Mutex<Option<String>>>,
}

impl Version {
    pub fn __go_value_clone(&self) -> Self {
        Self { major: { let __guard = self.major.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, minor: { let __guard = self.minor.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, patch: { let __guard = self.patch.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pre: { let __guard = self.pre.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Version {
    fn default() -> Self {
        Self { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.major.lock().unwrap().as_ref().unwrap()), (*self.minor.lock().unwrap().as_ref().unwrap()), (*self.patch.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.pre.lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.major.lock().unwrap(); let __right = other.major.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.minor.lock().unwrap(); let __right = other.minor.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.patch.lock().unwrap(); let __right = other.patch.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.kind.lock().unwrap(); let __right = other.kind.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.pre.lock().unwrap(); let __right = other.pre.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl GoJsonDecode for Version {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Major") {
            out.major = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Minor") {
            out.minor = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Patch") {
            out.patch = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Kind") {
            out.kind = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pre") {
            out.pre = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// Compare returns -1, 0, or +1 depending on whether
/// x < y, x == y, or x > y, interpreted as toolchain versions.
/// The versions x and y must not begin with a "go" prefix: just "1.21" not "go1.21".
/// Malformed versions compare less than well-formed versions and equal to each other.
/// The language version "1.21" compares less than the release candidate and eventual releases "1.21rc1" and "1.21.0".
pub fn compare(x: Arc<Mutex<Option<String>>>, y: Arc<Mutex<Option<String>>>) -> i32 {
    let mut vx = parse(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut vy = parse(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    {
        let mut c = cmp_int({ let __field = (*vx.lock().unwrap().as_ref().unwrap()).major.clone(); __field }, { let __field = (*vy.lock().unwrap().as_ref().unwrap()).major.clone(); __field });;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    {
        let mut c = cmp_int({ let __field = (*vx.lock().unwrap().as_ref().unwrap()).minor.clone(); __field }, { let __field = (*vy.lock().unwrap().as_ref().unwrap()).minor.clone(); __field });;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    {
        let mut c = cmp_int({ let __field = (*vx.lock().unwrap().as_ref().unwrap()).patch.clone(); __field }, { let __field = (*vy.lock().unwrap().as_ref().unwrap()).patch.clone(); __field });;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    {
        let mut c = cmp::compare::<String>({ let __selector_holder = (*vx.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __selector_holder = (*vy.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    {
        let mut c = cmp_int({ let __field = (*vx.lock().unwrap().as_ref().unwrap()).pre.clone(); __field }, { let __field = (*vy.lock().unwrap().as_ref().unwrap()).pre.clone(); __field });;
        if { let __tmp_x = c; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return c;;
        }
    }
    0
}

/// Lang returns the Go language version. For example, Lang("1.2.3") == "1.2".
pub fn lang(x: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut v = parse(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = { let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).minor.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).major.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "1".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).minor.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "0".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).major.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
    return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).major.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).minor.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())); __s })));
}

/// Parse parses the Go version string x into a version.
/// It returns the zero version if x is malformed.
pub fn parse(mut x: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Version>>> {
    let mut v: Arc<Mutex<Option<Version>>> = Arc::new(Mutex::new(Some(Default::default())));

        // Parse major version.
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1, __tmp_2) = cut_int(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*v.lock().unwrap().as_ref().unwrap()).major.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }
    if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
                // Interpret "1" as "1.0.0".
        { let new_val = "0".to_string(); *(*v.lock().unwrap().as_ref().unwrap()).minor.lock().unwrap() = Some(new_val); };
        { let new_val = "0".to_string(); *(*v.lock().unwrap().as_ref().unwrap()).patch.lock().unwrap() = Some(new_val); };
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

        // Interpret "1" as "1.0.0".
        // Parse . before minor version.
    if { let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }

        // Parse minor version.
    { let (__tmp_0, __tmp_1, __tmp_2) = cut_int(Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*v.lock().unwrap().as_ref().unwrap()).minor.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }
    if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
                // Patch missing is same as "0" for older versions.
                // Starting in Go 1.21, patch missing is different from explicit .0.
        if { let __tmp_x = cmp_int({ let __field = (*v.lock().unwrap().as_ref().unwrap()).minor.clone(); __field }, Arc::new(Mutex::new(Some("21".to_string())))); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = "0".to_string(); *(*v.lock().unwrap().as_ref().unwrap()).patch.lock().unwrap() = Some(new_val); };
    }
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

        // Patch missing is same as "0" for older versions.
        // Starting in Go 1.21, patch missing is different from explicit .0.
        // Parse patch if present.
    if { let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = cut_int(Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*v.lock().unwrap().as_ref().unwrap()).patch.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // Note that we are disallowing prereleases (alpha, beta, rc) for patch releases here (x != "").
                // Allowing them would be a bit confusing because we already have:
                //	1.21 < 1.21rc1
                // But a prerelease of a patch would have the opposite effect:
                //	1.21.3rc1 < 1.21.3
                // We've never needed them before, so let's not start now.
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }
                // Note that we are disallowing prereleases (alpha, beta, rc) for patch releases here (x != "").
                // Allowing them would be a bit confusing because we already have:
                //	1.21 < 1.21rc1
                // But a prerelease of a patch would have the opposite effect:
                //	1.21.3rc1 < 1.21.3
                // We've never needed them before, so let's not start now.
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

        // Note that we are disallowing prereleases (alpha, beta, rc) for patch releases here (x != "").
        // Allowing them would be a bit confusing because we already have:
        //	1.21 < 1.21rc1
        // But a prerelease of a patch would have the opposite effect:
        //	1.21.3rc1 < 1.21.3
        // We've never needed them before, so let's not start now.
        // Parse prerelease.
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && ({ let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = ('9' as i32) as u8; let __tmp_y = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x < __tmp_y }) {
        if { let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('a' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = ('z' as i32) as u8; let __tmp_y = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }
    { let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))); let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); *(*v.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = __tmp_0.lock().unwrap().take(); *x.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    { let (__tmp_0, __tmp_1, __tmp_2) = cut_int(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*v.lock().unwrap().as_ref().unwrap()).pre.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1; *ok.lock().unwrap() = Some(__tmp_2); };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Version { major: Arc::new(Mutex::new(Some(String::new()))), minor: Arc::new(Mutex::new(Some(String::new()))), patch: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(String::new()))), pre: Arc::new(Mutex::new(Some(String::new()))) })));
    }

    return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// cutInt scans the leading decimal number at the start of x to an integer
/// and returns that value and the rest of the string.
pub fn cut_int(x: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>, bool) {
    let mut n: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut rest: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    return (Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), true);
}

/// CmpInt returns cmp.Compare(x, y) interpreting x and y as decimal numbers.
/// (Copied from golang.org/x/mod/semver's compareInt.)
pub fn cmp_int(x: Arc<Mutex<Option<String>>>, y: Arc<Mutex<Option<String>>>) -> i32 {
    if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return 0;
    }
    if { let __tmp_x = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*y.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*y.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x > __tmp_y } {
        return 1;
    }
    if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        return -(1);
    } else {
        return 1;
    }
}

impl GoValueClone for Version {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
