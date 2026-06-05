use go2rust_stdlib_stubs::*;

use crate::{go_lookup_embedded_owner, go_register_embedded_owner};

use crate::exp::*;
use crate::zbootstrap::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct GoarmFeatures {
    pub version: Arc<Mutex<Option<i32>>>,
    pub soft_float: Arc<Mutex<Option<bool>>>,
}

impl GoarmFeatures {
    pub fn __go_value_clone(&self) -> Self {
        Self { version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, soft_float: { let __guard = self.soft_float.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for GoarmFeatures {
    fn default() -> Self {
        Self { version: Arc::new(Mutex::new(Some(0))), soft_float: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for GoarmFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for GoarmFeatures {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SoftFloat") {
            out.soft_float = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct Goarm64Features {
    pub version: Arc<Mutex<Option<String>>>,
    pub l_s_e: Arc<Mutex<Option<bool>>>,
    pub crypto: Arc<Mutex<Option<bool>>>,
}

impl Goarm64Features {
    pub fn __go_value_clone(&self) -> Self {
        Self { version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, l_s_e: { let __guard = self.l_s_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, crypto: { let __guard = self.crypto.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Goarm64Features {
    fn default() -> Self {
        Self { version: Arc::new(Mutex::new(Some(String::new()))), l_s_e: Arc::new(Mutex::new(Some(false))), crypto: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Goarm64Features {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Goarm64Features {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("LSE") {
            out.l_s_e = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Crypto") {
            out.crypto = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct gowasmFeatures {
    pub sat_conv: Arc<Mutex<Option<bool>>>,
    pub sign_ext: Arc<Mutex<Option<bool>>>,
}

impl gowasmFeatures {
    pub fn __go_value_clone(&self) -> Self {
        Self { sat_conv: { let __guard = self.sat_conv.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sign_ext: { let __guard = self.sign_ext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gowasmFeatures {
    fn default() -> Self {
        Self { sat_conv: Arc::new(Mutex::new(Some(false))), sign_ext: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for gowasmFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gowasmFeatures {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("SatConv") {
            out.sat_conv = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SignExt") {
            out.sign_ext = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static GOROOT: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOARCH: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOOS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GO386: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOAMD64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOARM: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoarmFeatures>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOARM64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Goarm64Features>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOMIPS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOMIPS64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOPPC64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GORISCV64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOWASM: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<gowasmFeatures>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ToolTags: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GO_LDSO: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static GOFIPS140: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Version: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Error: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *GOROOT.lock().unwrap() = Some(String::new());
    *GOARCH.lock().unwrap() = Some(String::new());
    *GOOS.lock().unwrap() = Some(String::new());
    *GO386.lock().unwrap() = Some(String::new());
    *GOAMD64.lock().unwrap() = Some(0);
    *GOARM.lock().unwrap() = Some(Default::default());
    *GOARM64.lock().unwrap() = Some(Default::default());
    *GOMIPS.lock().unwrap() = Some(String::new());
    *GOMIPS64.lock().unwrap() = Some(String::new());
    *GOPPC64.lock().unwrap() = Some(0);
    *GORISCV64.lock().unwrap() = Some(0);
    *GOWASM.lock().unwrap() = Some(Default::default());
    *ToolTags.lock().unwrap() = Some(vec![]);
    *GO_LDSO.lock().unwrap() = Some(String::new());
    *GOFIPS140.lock().unwrap() = Some(String::new());
    *Version.lock().unwrap() = Some(String::new());
    *Error.lock().unwrap() = None;
    *GOROOT.lock().unwrap() = Some((*os::getenv("GOROOT".to_string()).lock().unwrap().as_ref().unwrap()).clone());
    *GOARCH.lock().unwrap() = Some((*env_or(Arc::new(Mutex::new(Some("GOARCH".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_A_R_C_H.to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *GOOS.lock().unwrap() = Some((*env_or(Arc::new(Mutex::new(Some("GOOS".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_O_S.to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *GO386.lock().unwrap() = Some((*env_or(Arc::new(Mutex::new(Some("GO386".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O386.to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *GO_LDSO.lock().unwrap() = Some("".to_string());
    *Version.lock().unwrap() = Some("go1.24.5".to_string());
    *GOAMD64.lock().unwrap() = Some(goamd64());
    *GOARM.lock().unwrap() = Some((*goarm().lock().unwrap().as_ref().unwrap()).clone());
    *GOARM64.lock().unwrap() = Some((*goarm64().lock().unwrap().as_ref().unwrap()).clone());
    *GOMIPS.lock().unwrap() = Some((*gomips().lock().unwrap().as_ref().unwrap()).clone());
    *GOMIPS64.lock().unwrap() = Some((*gomips64().lock().unwrap().as_ref().unwrap()).clone());
    *GOPPC64.lock().unwrap() = Some(goppc64());
    *GORISCV64.lock().unwrap() = Some(goriscv64());
    *GOWASM.lock().unwrap() = Some((*gowasm().lock().unwrap().as_ref().unwrap()).clone());
    *GOFIPS140.lock().unwrap() = Some((*gofips140().lock().unwrap().as_ref().unwrap()).clone());
    *ToolTags.lock().unwrap() = Some((*tool_tags().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *GOROOT.lock().unwrap() = Some(String::new());
    *GOARCH.lock().unwrap() = Some(String::new());
    *GOOS.lock().unwrap() = Some(String::new());
    *GO386.lock().unwrap() = Some(String::new());
    *GOAMD64.lock().unwrap() = Some(0);
    *GOARM.lock().unwrap() = Some(Default::default());
    *GOARM64.lock().unwrap() = Some(Default::default());
    *GOMIPS.lock().unwrap() = Some(String::new());
    *GOMIPS64.lock().unwrap() = Some(String::new());
    *GOPPC64.lock().unwrap() = Some(0);
    *GORISCV64.lock().unwrap() = Some(0);
    *GOWASM.lock().unwrap() = Some(Default::default());
    *ToolTags.lock().unwrap() = Some(vec![]);
    *GO_LDSO.lock().unwrap() = Some(String::new());
    *GOFIPS140.lock().unwrap() = Some(String::new());
    *Version.lock().unwrap() = Some(String::new());
    *Error.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    *GOROOT.lock().unwrap() = Some((*os::getenv("GOROOT".to_string()).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_1() {
    *GOARCH.lock().unwrap() = Some((*env_or(Arc::new(Mutex::new(Some("GOARCH".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_A_R_C_H.to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_2() {
    *GOOS.lock().unwrap() = Some((*env_or(Arc::new(Mutex::new(Some("GOOS".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_O_S.to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_3() {
    *GO386.lock().unwrap() = Some((*env_or(Arc::new(Mutex::new(Some("GO386".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O386.to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_4() {
    *GO_LDSO.lock().unwrap() = Some("".to_string());
}


pub(crate) fn __go_init_order_5() {
    *Version.lock().unwrap() = Some("go1.24.5".to_string());
}


pub(crate) fn __go_init_order_6() {
    *GOAMD64.lock().unwrap() = Some(goamd64());
}


pub(crate) fn __go_init_order_7() {
    *GOARM.lock().unwrap() = Some((*goarm().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_8() {
    *GOARM64.lock().unwrap() = Some((*goarm64().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_9() {
    *GOMIPS.lock().unwrap() = Some((*gomips().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_10() {
    *GOMIPS64.lock().unwrap() = Some((*gomips64().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_11() {
    *GOPPC64.lock().unwrap() = Some(goppc64());
}


pub(crate) fn __go_init_order_12() {
    *GORISCV64.lock().unwrap() = Some(goriscv64());
}


pub(crate) fn __go_init_order_13() {
    *GOWASM.lock().unwrap() = Some((*gowasm().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_14() {
    *GOFIPS140.lock().unwrap() = Some((*gofips140().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_16() {
    *ToolTags.lock().unwrap() = Some((*tool_tags().lock().unwrap().as_ref().unwrap()).clone());
}


impl GoarmFeatures {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut armStr = Arc::new(Mutex::new(Some(({ let __selector_holder = self.version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }).to_string())));
        if (*self.soft_float.clone().lock().unwrap().as_ref().unwrap()) {
        { (*armStr.lock().unwrap().as_mut().unwrap()).push_str(&",softfloat".to_string()); };
    } else {
        { (*armStr.lock().unwrap().as_mut().unwrap()).push_str(&",hardfloat".to_string()); };
    }
        return { let __owned = armStr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
}

impl Goarm64Features {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut arm64Str = Arc::new(Mutex::new(Some({ let __selector_holder = self.version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*self.l_s_e.clone().lock().unwrap().as_ref().unwrap()) {
        { (*arm64Str.lock().unwrap().as_mut().unwrap()).push_str(&",lse".to_string()); };
    }
        if (*self.crypto.clone().lock().unwrap().as_ref().unwrap()) {
        { (*arm64Str.lock().unwrap().as_mut().unwrap()).push_str(&",crypto".to_string()); };
    }
        return { let __owned = arm64Str.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// Returns true if g supports giving ARM64 ISA
    /// Note that this function doesn't accept / test suffixes (like ",lse" or ",crypto")
    pub fn supports(&self, s: Arc<Mutex<Option<String>>>) -> bool {
                // We only accept "v{8-9}.{0-9}. Everything else is malformed.
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x != __tmp_y } {
        return false;
    }
        let mut major = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
        let mut minor = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] })));
                // We only accept "v{8-9}.{0-9}. Everything else is malformed.
        if { let __tmp_x = { let __v = (*major.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('8' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*major.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x > __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('v' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(2) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } {
        return false;
    }
        let mut g_major = Arc::new(Mutex::new(Some({ let __s = &((*self.version.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
        let mut g_minor = Arc::new(Mutex::new(Some({ let __s = &((*self.version.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] })));
        if { let __tmp_x = { let __v = (*major.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*g_major.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __tmp_x = { let __v = (*minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*g_minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
    } else if { let __tmp_x = { let __v = (*g_major.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x == __tmp_y } {
        return { let __tmp_x = { let __v = (*minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*g_minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5 as u8; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y };
    } else {
        return false;
    }
    }
}

impl gowasmFeatures {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut flags: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        if (*self.sat_conv.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __append_target = flags.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push("satconv".to_string()); __append_target.clone() }; flags = new_val; };
    }
        if (*self.sign_ext.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __append_target = flags.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push("signext".to_string()); __append_target.clone() }; flags = new_val; };
    }
        return Arc::new(Mutex::new(Some({ let __parts = (*flags.lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) })));
    }
}

pub fn env_or(key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    {
        let mut x = os::getenv({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });;
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            return { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
    return { let __owned = value.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn goamd64() -> i32 {
    let mut v = env_or(Arc::new(Mutex::new(Some("GOAMD64".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_A_M_D64.to_string()))));
    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("v1".to_string()) {
            return 1;
        } else if _switch_val == ("v2".to_string()) {
            return 2;
        } else if _switch_val == ("v3".to_string()) {
            return 3;
        } else if _switch_val == ("v4".to_string()) {
            return 4;
        }
    }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOAMD64: must be v1, v2, v3, v4"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &(DEFAULT_G_O_A_M_D64); __s.as_bytes()[("v".len()) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
}

pub fn gofips140() -> Arc<Mutex<Option<String>>> {
    let mut v = env_or(Arc::new(Mutex::new(Some("GOFIPS140".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_F_I_P_S140.to_string()))));
    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("off".to_string()) || _switch_val == ("latest".to_string()) || _switch_val == ("inprocess".to_string()) || _switch_val == ("certified".to_string()) {
            return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        }
    }
    if is_f_i_p_s_version(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOFIPS140: must be off, latest, inprocess, certified, or vX.Y.Z"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
    Arc::new(Mutex::new(Some(DEFAULT_G_O_F_I_P_S140.to_string())))
}

/// isFIPSVersion reports whether v is a valid FIPS version,
/// of the form vX.Y.Z.
pub fn is_f_i_p_s_version(mut v: Arc<Mutex<Option<String>>>) -> bool {
    if !(*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "v".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        return false;
    }
    let (__tmp_0, mut ok) = skip_num(Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __low = ("v".len()) as usize; __s[__low..].to_string() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0;;
    if !ok || !(*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = ".".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        return false;
    }
    { let (__tmp_0, __tmp_1) = skip_num(Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __low = (".".len()) as usize; __s[__low..].to_string() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; ok = __tmp_1; };
    if !ok || !(*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = ".".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        return false;
    }
    { let (__tmp_0, __tmp_1) = skip_num(Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __low = (".".len()) as usize; __s[__low..].to_string() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; ok = __tmp_1; };
    ok && { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y }
}

/// skipNum skips the leading text matching [0-9]+
/// in s, returning the rest and whether such text was found.
pub fn skip_num(s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    let mut rest: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y });
}

pub fn goarm() -> Arc<Mutex<Option<GoarmFeatures>>> {
    let mut g: Arc<Mutex<Option<GoarmFeatures>>> = Arc::new(Mutex::new(Some(Default::default())));

    const softFloatOpt: &'static str = ",softfloat";
const hardFloatOpt: &'static str = ",hardfloat";

    let mut def = Arc::new(Mutex::new(Some("7".to_string())));
    if { let __tmp_x = (*GOOS.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "android".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "arm".to_string(); __tmp_x == __tmp_y } {
                // Android arm devices always support GOARM=7.
        { let new_val = "7".to_string(); *def.lock().unwrap() = Some(new_val); };
    }
        // Android arm devices always support GOARM=7.
    let mut v = env_or(Arc::new(Mutex::new(Some("GOARM".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = def.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    let mut floatSpecified = Arc::new(Mutex::new(Some(false)));
    if (*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = softFloatOpt; __s.ends_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *(*g.lock().unwrap().as_ref().unwrap()).soft_float.lock().unwrap() = Some(new_val); };
        { let new_val = true; *floatSpecified.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*v.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 10; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
    }
    if (*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = hardFloatOpt; __s.ends_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *floatSpecified.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*v.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 10; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
    }

    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("5".to_string()) {
            { let new_val = 5; *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("6".to_string()) {
            { let new_val = 6; *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("7".to_string()) {
            { let new_val = 7; *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
        } else {
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOARM: must start with 5, 6, or 7, and may optionally end in either {:?} or {:?}", hardFloatOpt, softFloatOpt))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
            { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*def.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = __moved_val; };
        }
    }

        // 5 defaults to softfloat. 6 and 7 default to hardfloat.
    if !{ let __v = (*floatSpecified.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __field = (*g.lock().unwrap().as_ref().unwrap()).version.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 5; __tmp_x == __tmp_y } {
        { let new_val = true; *(*g.lock().unwrap().as_ref().unwrap()).soft_float.lock().unwrap() = Some(new_val); };
    }
    g
}

pub fn parse_goarm64(mut v: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Goarm64Features>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut g: Arc<Mutex<Option<Goarm64Features>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut e: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    const lseOpt: &'static str = ",lse";
const cryptoOpt: &'static str = ",crypto";


    { let new_val = false; *(*g.lock().unwrap().as_ref().unwrap()).l_s_e.lock().unwrap() = Some(new_val); };
    { let new_val = false; *(*g.lock().unwrap().as_ref().unwrap()).crypto.lock().unwrap() = Some(new_val); };

        // We allow any combination of suffixes, in any order
    loop {
        if (*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = lseOpt; __s.ends_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *(*g.lock().unwrap().as_ref().unwrap()).l_s_e.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*v.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
        continue
    }

        if (*Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __arg = cryptoOpt; __s.ends_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *(*g.lock().unwrap().as_ref().unwrap()).crypto.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*v.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 7; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
        continue
    }

        break
    }

    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("v8.0".to_string()) {
            { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("v8.1".to_string()) || _switch_val == ("v8.2".to_string()) || _switch_val == ("v8.3".to_string()) || _switch_val == ("v8.4".to_string()) || _switch_val == ("v8.5".to_string()) || _switch_val == ("v8.6".to_string()) || _switch_val == ("v8.7".to_string()) || _switch_val == ("v8.8".to_string()) || _switch_val == ("v8.9".to_string()) || _switch_val == ("v9.0".to_string()) || _switch_val == ("v9.1".to_string()) || _switch_val == ("v9.2".to_string()) || _switch_val == ("v9.3".to_string()) || _switch_val == ("v9.4".to_string()) || _switch_val == ("v9.5".to_string()) {
            { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
                        // LSE extension is mandatory starting from 8.1
            { let new_val = true; *(*g.lock().unwrap().as_ref().unwrap()).l_s_e.lock().unwrap() = Some(new_val); };
        } else {
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOARM64: must start with v8.{{0-9}} or v9.{{0-5}} and may optionally end in {:?} and/or {:?}", lseOpt, cryptoOpt))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *e.lock().unwrap() = new_val; };
            { let new_val = "v8.0".to_string(); *(*g.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
        }
    }

        // LSE extension is mandatory starting from 8.1
    (g, e)
}

pub fn goarm64() -> Arc<Mutex<Option<Goarm64Features>>> {
    let mut g: Arc<Mutex<Option<Goarm64Features>>> = Arc::new(Mutex::new(Some(Default::default())));

    { let (__tmp_0, __tmp_1) = parse_goarm64(env_or(Arc::new(Mutex::new(Some("GOARM64".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_A_R_M64.to_string()))))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *g.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *Error.lock().unwrap() = __moved_tmp_1; };
    g
}

pub fn gomips() -> Arc<Mutex<Option<String>>> {
    let mut v = env_or(Arc::new(Mutex::new(Some("GOMIPS".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_M_I_P_S.to_string()))));
    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("hardfloat".to_string()) || _switch_val == ("softfloat".to_string()) {
            return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        }
    }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOMIPS: must be hardfloat, softfloat"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
    Arc::new(Mutex::new(Some(DEFAULT_G_O_M_I_P_S.to_string())))
}

pub fn gomips64() -> Arc<Mutex<Option<String>>> {
    let mut v = env_or(Arc::new(Mutex::new(Some("GOMIPS64".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_M_I_P_S64.to_string()))));
    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("hardfloat".to_string()) || _switch_val == ("softfloat".to_string()) {
            return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        }
    }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOMIPS64: must be hardfloat, softfloat"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
    Arc::new(Mutex::new(Some(DEFAULT_G_O_M_I_P_S64.to_string())))
}

pub fn goppc64() -> i32 {
    let mut v = env_or(Arc::new(Mutex::new(Some("GOPPC64".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_P_P_C64.to_string()))));
    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("power8".to_string()) {
            return 8;
        } else if _switch_val == ("power9".to_string()) {
            return 9;
        } else if _switch_val == ("power10".to_string()) {
            return 10;
        }
    }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOPPC64: must be power8, power9, power10"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &(DEFAULT_G_O_P_P_C64); __s.as_bytes()[("power".len()) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
}

pub fn goriscv64() -> i32 {
    let mut v = env_or(Arc::new(Mutex::new(Some("GORISCV64".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_R_I_S_C_V64.to_string()))));
    { let _switch_val = (*v.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("rva20u64".to_string()) {
            return 20;
        } else if _switch_val == ("rva22u64".to_string()) {
            return 22;
        }
    }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GORISCV64: must be rva20u64, rva22u64"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
    let mut v = Arc::new(Mutex::new(Some({ let __s = &(DEFAULT_G_O_R_I_S_C_V64); let __low = ("rva".len()) as usize; __s[__low..].to_string() })));
    let mut i = strings::index_func({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some(Box::new(move |r: Arc<Mutex<Option<i32>>>| -> bool {
        return { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32); __tmp_x > __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
    let (mut year, _) = { let __atoi_input = (*Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); match __atoi_input.parse::<i32>() { Ok(n) => (n, Arc::new(Mutex::new(None))), Err(_) => (0 as i32, Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    year
}

pub fn gowasm() -> Arc<Mutex<Option<gowasmFeatures>>> {
    let mut f: Arc<Mutex<Option<gowasmFeatures>>> = Arc::new(Mutex::new(Some(Default::default())));

    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = (*env_or(Arc::new(Mutex::new(Some("GOWASM".to_string()))), Arc::new(Mutex::new(Some("".to_string())))).lock().unwrap().as_ref().unwrap()).clone(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for opt in __range_values.iter() {
        { let _switch_val = (*opt).clone();
    if _switch_val == ("satconv".to_string()) {
            { let new_val = true; *(*f.lock().unwrap().as_ref().unwrap()).sat_conv.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("signext".to_string()) {
            { let new_val = true; *(*f.lock().unwrap().as_ref().unwrap()).sign_ext.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("".to_string()) {
        } else {
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid GOWASM: no such feature {:?}", opt))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
        }
    }
    } }
        // ignore
    f
}

pub fn tool_tags() -> Arc<Mutex<Option<Vec<String>>>> {
    let mut tags = experiment_tags();
    { let new_val = { let __append_target = tags.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = gogoarch_tags().clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; tags = new_val; };
    return tags.clone();
}

pub fn experiment_tags() -> Arc<Mutex<Option<Vec<String>>>> {
    let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));

        // For each experiment that has been enabled in the toolchain, define a
        // build tag with the same name but prefixed by "goexperiment." which can be
        // used for compiling alternative files for the experiment. This allows
        // changes for the experiment, like extra struct fields in the runtime,
        // without affecting the base non-experiment code at all.
    { let __range_holder = (*Experiment.lock().unwrap().as_mut().unwrap()).enabled().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for exp in __range_values.iter() {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(format!("{}{}", "goexperiment.".to_string(), exp)); __append_target.clone() }; list = new_val; };
    } }
    return list.clone();
}

pub fn gogoarch_tags() -> Arc<Mutex<Option<Vec<String>>>> {
    { let _switch_val = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("386".to_string()) {
            return Arc::new(Mutex::new(Some(vec![{ let mut __s = String::new(); __s.push_str(&format!("{}", (*GOARCH.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", (*GO386.lock().unwrap().as_ref().unwrap()))); __s }])));
        } else if _switch_val == ("amd64".to_string()) {
            let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
            let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*GOAMD64.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}.v{}", { let __v = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; list = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            return list.clone();
        } else if _switch_val == ("arm".to_string()) {
            let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
            let mut i = Arc::new(Mutex::new(Some(5)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*GOARM.lock().unwrap().as_ref().unwrap()).version.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}.{}", { let __v = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; list = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            return list.clone();
        } else if _switch_val == ("arm64".to_string()) {
            let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
            let mut major = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*(*GOARM64.lock().unwrap().as_ref().unwrap()).version.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32)));
            let mut minor = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __s = &((*(*GOARM64.lock().unwrap().as_ref().unwrap()).version.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }) as i32)));
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}.v{}.{}", { let __v = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*major.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; list = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                        // ARM64 v9.x also includes support of v8.x+5 (i.e. v9.1 includes v8.(1+5) = v8.6).
            if { let __tmp_x = { let __v = (*major.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x == __tmp_y } {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*minor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}.v{}.{}", { let __v = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); __v }, 8, { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; list = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
            return list.clone();
        } else if _switch_val == ("mips".to_string()) || _switch_val == ("mipsle".to_string()) {
            return Arc::new(Mutex::new(Some(vec![{ let mut __s = String::new(); __s.push_str(&format!("{}", (*GOARCH.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", (*GOMIPS.lock().unwrap().as_ref().unwrap()))); __s }])));
        } else if _switch_val == ("mips64".to_string()) || _switch_val == ("mips64le".to_string()) {
            return Arc::new(Mutex::new(Some(vec![{ let mut __s = String::new(); __s.push_str(&format!("{}", (*GOARCH.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", (*GOMIPS64.lock().unwrap().as_ref().unwrap()))); __s }])));
        } else if _switch_val == ("ppc64".to_string()) || _switch_val == ("ppc64le".to_string()) {
            let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
            let mut i = Arc::new(Mutex::new(Some(8)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*GOPPC64.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}.power{}", { let __v = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; list = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            return list.clone();
        } else if _switch_val == ("riscv64".to_string()) {
            let mut list = Arc::new(Mutex::new(Some(vec![{ let mut __s = String::new(); __s.push_str(&format!("{}", (*GOARCH.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", "rva20u64".to_string())); __s }])));
            if { let __tmp_x = (*GORISCV64.lock().unwrap().as_ref().unwrap()); let __tmp_y = 22; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let mut __s = String::new(); __s.push_str(&format!("{}", (*GOARCH.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", "rva22u64".to_string())); __s }); __append_target.clone() }; list = new_val; };
    }
            return list.clone();
        } else if _switch_val == ("wasm".to_string()) {
            let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
            if (*{ let __field = (*GOWASM.lock().unwrap().as_ref().unwrap()).sat_conv.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(format!("{}{}", (*GOARCH.lock().unwrap().as_ref().unwrap()), ".satconv".to_string())); __append_target.clone() }; list = new_val; };
    }
            if (*{ let __field = (*GOWASM.lock().unwrap().as_ref().unwrap()).sign_ext.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(format!("{}{}", (*GOARCH.lock().unwrap().as_ref().unwrap()), ".signext".to_string())); __append_target.clone() }; list = new_val; };
    }
            return list.clone();
        }
    }
        // ARM64 v9.x also includes support of v8.x+5 (i.e. v9.1 includes v8.(1+5) = v8.6).
    return Arc::new(Mutex::new(None));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for GoarmFeatures {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Goarm64Features {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gowasmFeatures {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
