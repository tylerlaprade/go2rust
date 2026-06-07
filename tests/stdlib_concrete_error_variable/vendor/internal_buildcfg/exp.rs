use go2rust_stdlib_stubs::*;

use crate::{GoReflectBoolGetter, GoReflectBoolSetter, GoReflectField, GoReflectStructTag, GoReflectType, GoReflectValue, go_reflect_tag_get};

use crate::cfg::*;
use crate::zbootstrap::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const DEFAULT_G_O_E_X_P_E_R_I_M_E_N_T: &'static str = "";


/// ExperimentFlags represents a set of GOEXPERIMENT flags relative to a baseline
/// (platform-default) experiment configuration.
#[derive(Clone)]
pub struct ExperimentFlags {
    pub flags: Arc<Mutex<Option<internal_goexperiment::flags::Flags>>>,
    pub baseline: Arc<Mutex<Option<internal_goexperiment::flags::Flags>>>,
}

impl ExperimentFlags {
    pub fn __go_value_clone(&self) -> Self {
        Self { flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, baseline: { let __guard = self.baseline.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ExperimentFlags {
    fn default() -> Self {
        Self { flags: Arc::new(Mutex::new(Some(Default::default()))), baseline: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for ExperimentFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ExperimentFlags {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub static Experiment: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<ExperimentFlags>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static FramePointerEnabled: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Experiment.lock().unwrap() = Some(Default::default());
    *FramePointerEnabled.lock().unwrap() = Some(false);
    *Experiment.lock().unwrap() = Some((*{ let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<ExperimentFlags>>> {
        let (mut flags, mut err) = parse_g_o_e_x_p_e_r_i_m_e_n_t(Arc::new(Mutex::new(Some({ let __arg_holder = GOOS.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = GOARCH.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), env_or(Arc::new(Mutex::new(Some("GOEXPERIMENT".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_E_X_P_E_R_I_M_E_N_T_1.to_string())))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
        return Arc::new(Mutex::new(Some(ExperimentFlags { flags: Arc::new(Mutex::new(Some(Default::default()))), baseline: Arc::new(Mutex::new(Some(Default::default()))) })));
    }
        return Arc::new(Mutex::new(Some({ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<ExperimentFlags>>> + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<ExperimentFlags>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<ExperimentFlags>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.lock().unwrap().as_ref().unwrap()).clone());
    *FramePointerEnabled.lock().unwrap() = Some({ let __tmp_x = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "amd64".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y });
}


pub(crate) fn __go_zero_globals() {
    *Experiment.lock().unwrap() = Some(Default::default());
    *FramePointerEnabled.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_15() {
    *Experiment.lock().unwrap() = Some((*{ let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<ExperimentFlags>>> {
        let (mut flags, mut err) = parse_g_o_e_x_p_e_r_i_m_e_n_t(Arc::new(Mutex::new(Some({ let __arg_holder = GOOS.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = GOARCH.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), env_or(Arc::new(Mutex::new(Some("GOEXPERIMENT".to_string()))), Arc::new(Mutex::new(Some(DEFAULT_G_O_E_X_P_E_R_I_M_E_N_T_1.to_string())))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *Error.lock().unwrap() = new_val; };
        return Arc::new(Mutex::new(Some(ExperimentFlags { flags: Arc::new(Mutex::new(Some(Default::default()))), baseline: Arc::new(Mutex::new(Some(Default::default()))) })));
    }
        return Arc::new(Mutex::new(Some({ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<ExperimentFlags>>> + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<ExperimentFlags>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<ExperimentFlags>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_17() {
    *FramePointerEnabled.lock().unwrap() = Some({ let __tmp_x = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "amd64".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*GOARCH.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y });
}


impl ExperimentFlags {
    /// String returns the canonical GOEXPERIMENT string to enable this experiment
    /// configuration. (Experiments in the same state as in the baseline are elided.)
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some({ let __parts = (*exp_list(self.flags.clone(), self.baseline.clone(), Arc::new(Mutex::new(Some(false)))).lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) })))
    }

    /// Enabled returns a list of enabled experiments, as
    /// lower-cased experiment names.
    pub fn enabled(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        exp_list(self.flags.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))))
    }

    /// All returns a list of all experiment settings.
    /// Disabled experiments appear in the list prefixed by "no".
    pub fn all(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        exp_list(self.flags.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))))
    }
}

/// ParseGOEXPERIMENT parses a (GOOS, GOARCH, GOEXPERIMENT)
/// configuration tuple and returns the enabled and baseline experiment
/// flag sets.
///
/// TODO(mdempsky): Move to internal/goexperiment.
pub fn parse_g_o_e_x_p_e_r_i_m_e_n_t(goos: Arc<Mutex<Option<String>>>, goarch: Arc<Mutex<Option<String>>>, goexp: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<ExperimentFlags>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // regabiSupported is set to true on platforms where register ABI is
        // supported and enabled by default.
        // regabiAlwaysOn is set to true on platforms where register ABI is
        // always on.
    let mut regabiSupported: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));let mut regabiAlwaysOn: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let _switch_val = (*goarch.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("amd64".to_string()) || _switch_val == ("arm64".to_string()) || _switch_val == ("loong64".to_string()) || _switch_val == ("ppc64le".to_string()) || _switch_val == ("ppc64".to_string()) || _switch_val == ("riscv64".to_string()) {
            { let new_val = true; *regabiAlwaysOn.lock().unwrap() = Some(new_val); };
            { let new_val = true; *regabiSupported.lock().unwrap() = Some(new_val); };
        }
    }

    let mut haveXchg8: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let _switch_val = (*goarch.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("386".to_string()) || _switch_val == ("amd64".to_string()) || _switch_val == ("arm".to_string()) || _switch_val == ("arm64".to_string()) || _switch_val == ("ppc64le".to_string()) || _switch_val == ("ppc64".to_string()) {
            { let new_val = true; *haveXchg8.lock().unwrap() = Some(new_val); };
        }
    }

    let mut baseline = internal_goexperiment::flags::Flags { regabi_wrappers: Arc::new(Mutex::new(Some({ let __arg_holder = regabiSupported.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), regabi_args: Arc::new(Mutex::new(Some({ let __arg_holder = regabiSupported.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), coverage_redesign: Arc::new(Mutex::new(Some(true))), alias_type_params: Arc::new(Mutex::new(Some(true))), swiss_map: Arc::new(Mutex::new(Some(true))), spinbit_mutex: Arc::new(Mutex::new(Some({ let __arg_holder = haveXchg8.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sync_hash_trie_map: Arc::new(Mutex::new(Some(true))), ..Default::default() };

        // Start with the statically enabled set of experiments.
    let mut flags = { let __owner = Arc::new(Mutex::new(Some(ExperimentFlags { flags: Arc::new(Mutex::new(Some(baseline.clone()))), baseline: Arc::new(Mutex::new(Some(baseline.clone()))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().flags.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner };

        // Pick up any changes to the baseline configuration from the
        // GOEXPERIMENT environment. This can be set at make.bash time
        // and overridden at build time.
    if { let __tmp_x = (*goexp.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // Create a map of known experiment names.
        let mut names = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>>>>>::new())));
        let mut rv = { let __recv = { let __reflect_target = (*flags.lock().unwrap().as_ref().unwrap()).flags.clone(); Arc::new(Mutex::new(Some(GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("goexperiment.Flags".to_string()))), fields: Arc::new(Mutex::new(Some(vec![GoReflectField { name: Arc::new(Mutex::new(Some("FieldTrack".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("PreemptibleLoops".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("StaticLockRanking".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("BoringCrypto".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("RegabiWrappers".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("RegabiArgs".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("HeapMinimum512KiB".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("CoverageRedesign".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("Arenas".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("CgoCheck2".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("LoopVar".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("CacheProg".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("NewInliner".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("RangeFunc".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("AliasTypeParams".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("SwissMap".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("SpinbitMutex".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("SyncHashTrieMap".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }, GoReflectField { name: Arc::new(Mutex::new(Some("Synctest".to_string()))), tag: Arc::new(Mutex::new(Some(GoReflectStructTag { raw: Arc::new(Mutex::new(Some("".to_string()))) }))) }]))) }))), fields: Arc::new(Mutex::new(Some(vec![GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.field_track.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.field_track.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.preemptible_loops.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.preemptible_loops.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.static_lock_ranking.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.static_lock_ranking.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.boring_crypto.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.boring_crypto.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.regabi_wrappers.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.regabi_wrappers.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.regabi_args.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.regabi_args.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.heap_minimum512_ki_b.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.heap_minimum512_ki_b.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.coverage_redesign.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.coverage_redesign.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.arenas.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.arenas.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.cgo_check2.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.cgo_check2.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.loop_var.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.loop_var.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.cache_prog.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.cache_prog.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.new_inliner.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.new_inliner.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.range_func.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.range_func.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.alias_type_params.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.alias_type_params.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.swiss_map.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.swiss_map.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.spinbit_mutex.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.spinbit_mutex.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.sync_hash_trie_map.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.sync_hash_trie_map.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }, GoReflectValue { typ: Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("bool".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) }))), fields: Arc::new(Mutex::new(Some(vec![]))), bool_getter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move || -> bool { let __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_ref().expect("reflect.Value.Bool requires a struct value"); let __field_value = { let __field_guard = __target_value.synctest.lock().unwrap(); (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }))), bool_setter: Arc::new(Mutex::new(Some({ let __field_target = __reflect_target.clone(); Box::new(move |__value: Arc<Mutex<Option<bool>>>| { let __new_value = (*__value.lock().unwrap().as_ref().unwrap()).clone(); let mut __target_guard = __field_target.lock().unwrap(); let __target_value = __target_guard.as_mut().expect("reflect.Value.SetBool requires a settable struct value"); *__target_value.synctest.lock().unwrap() = Some(__new_value); }) as GoReflectBoolSetter }))) }]))), bool_getter: Arc::new(Mutex::new(None)), bool_setter: Arc::new(Mutex::new(None)) }))) }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).elem(); __result };
        let mut rt = (*rv.lock().unwrap().as_ref().unwrap()).r#type();
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*rt.lock().unwrap().as_ref().unwrap()).num_field(); __tmp_x < __tmp_y } {
        let mut field = (*rv.lock().unwrap().as_ref().unwrap()).field({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        { let __map_key = { let __map_key_holder = Arc::new(Mutex::new(Some({ let __s = (*(*(*rt.lock().unwrap().as_ref().unwrap()).field(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); __s.to_lowercase() }))).clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some({ let mut __recv = (*field.lock().unwrap().as_ref().unwrap()).clone(); Box::new(move |__arg0: Arc<Mutex<Option<bool>>>| { __recv.set_bool(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }))); (*names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // "regabi" is an alias for all working regabi
                // subexperiments, and not an experiment itself. Doing
                // this as an alias make both "regabi" and "noregabi"
                // do the right thing.
        let flags_closure_clone = flags.clone(); { let __map_key = "regabi".to_string(); let __map_value = Arc::new(Mutex::new(Some(Box::new(move |v: Arc<Mutex<Option<bool>>>| {
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *(*(*flags_closure_clone.lock().unwrap().as_mut().unwrap()).flags.lock().unwrap().as_mut().unwrap()).regabi_wrappers.lock().unwrap() = Some(new_val); };
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *(*(*flags_closure_clone.lock().unwrap().as_mut().unwrap()).flags.lock().unwrap().as_mut().unwrap()).regabi_args.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>))); (*names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
                // Parse names.
        { let __range_holder = Arc::new(Mutex::new(Some({ let __s = (*goexp.lock().unwrap().as_ref().unwrap()).clone(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut f in __range_values.iter().cloned() {
        if { let __tmp_x = f.clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        continue
    }
        if { let __tmp_x = f.clone(); let __tmp_y = "none".to_string(); __tmp_x == __tmp_y } {
                // GOEXPERIMENT=none disables all experiment flags.
                // This is used by cmd/dist, which doesn't know how
                // to build with any experiment flags.
        { let new_val = internal_goexperiment::flags::Flags { ..Default::default() }; *(*flags.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap() = Some(new_val); };
        continue
    }
                // GOEXPERIMENT=none disables all experiment flags.
                // This is used by cmd/dist, which doesn't know how
                // to build with any experiment flags.
        let mut val = Arc::new(Mutex::new(Some(true)));
        if (*Arc::new(Mutex::new(Some({ let __s = f.clone(); let __arg = "no".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        { let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &(f); let __low = (2) as usize; __s[__low..].to_string() }))); let __tmp_1 = false; f = (*__tmp_0.lock().unwrap().as_ref().unwrap()).clone(); *val.lock().unwrap() = Some(__tmp_1); };
    }
        let (mut set, mut ok) = { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&f)) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("unknown GOEXPERIMENT {}", f))))));
    }
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = { let mut __f_guard = set.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(val.clone()) };
    } }
    }

        // Create a map of known experiment names.
        // "regabi" is an alias for all working regabi
        // subexperiments, and not an experiment itself. Doing
        // this as an alias make both "regabi" and "noregabi"
        // do the right thing.
        // Parse names.
        // GOEXPERIMENT=none disables all experiment flags.
        // This is used by cmd/dist, which doesn't know how
        // to build with any experiment flags.
    if { let __v = (*regabiAlwaysOn.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = true; *(*(*flags.lock().unwrap().as_mut().unwrap()).flags.lock().unwrap().as_mut().unwrap()).regabi_wrappers.lock().unwrap() = Some(new_val); };
        { let new_val = true; *(*(*flags.lock().unwrap().as_mut().unwrap()).flags.lock().unwrap().as_mut().unwrap()).regabi_args.lock().unwrap() = Some(new_val); };
    }

        // regabi is only supported on amd64, arm64, loong64, riscv64, ppc64 and ppc64le.
    if !{ let __v = (*regabiSupported.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = false; *(*(*flags.lock().unwrap().as_mut().unwrap()).flags.lock().unwrap().as_mut().unwrap()).regabi_wrappers.lock().unwrap() = Some(new_val); };
        { let new_val = false; *(*(*flags.lock().unwrap().as_mut().unwrap()).flags.lock().unwrap().as_mut().unwrap()).regabi_args.lock().unwrap() = Some(new_val); };
    }

        // Check regabi dependencies.
    if (*(*(*flags.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap().as_ref().unwrap()).regabi_args.lock().unwrap().as_ref().unwrap()) && !(*(*(*flags.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap().as_ref().unwrap()).regabi_wrappers.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("GOEXPERIMENT regabiargs requires regabiwrappers"))))));
    }
    return (flags.clone(), Arc::new(Mutex::new(None)));
}

/// expList returns the list of lower-cased experiment names for
/// experiments that differ from base. base may be nil to indicate no
/// experiments. If all is true, then include all experiment flags,
/// regardless of base.
pub fn exp_list(exp: Arc<Mutex<Option<internal_goexperiment::flags::Flags>>>, base: Arc<Mutex<Option<internal_goexperiment::flags::Flags>>>, all: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut list: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    {
        let __exp_guard = exp.lock().unwrap();
        let __exp_value = __exp_guard.as_ref().expect("internal/buildcfg.expList requires exp flags");
        let __base_guard = base.lock().unwrap();
        let __base_value = __base_guard.as_ref();
        let __all = (*all.lock().unwrap().as_ref().unwrap()).clone();
        let __val = (*__exp_value.field_track.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.field_track.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("fieldtrack".to_string()); } else { __list_guard.as_mut().unwrap().push("nofieldtrack".to_string()); }
        }
        let __val = (*__exp_value.preemptible_loops.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.preemptible_loops.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("preemptibleloops".to_string()); } else { __list_guard.as_mut().unwrap().push("nopreemptibleloops".to_string()); }
        }
        let __val = (*__exp_value.static_lock_ranking.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.static_lock_ranking.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("staticlockranking".to_string()); } else { __list_guard.as_mut().unwrap().push("nostaticlockranking".to_string()); }
        }
        let __val = (*__exp_value.boring_crypto.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.boring_crypto.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("boringcrypto".to_string()); } else { __list_guard.as_mut().unwrap().push("noboringcrypto".to_string()); }
        }
        let __val = (*__exp_value.regabi_wrappers.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.regabi_wrappers.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("regabiwrappers".to_string()); } else { __list_guard.as_mut().unwrap().push("noregabiwrappers".to_string()); }
        }
        let __val = (*__exp_value.regabi_args.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.regabi_args.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("regabiargs".to_string()); } else { __list_guard.as_mut().unwrap().push("noregabiargs".to_string()); }
        }
        let __val = (*__exp_value.heap_minimum512_ki_b.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.heap_minimum512_ki_b.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("heapminimum512kib".to_string()); } else { __list_guard.as_mut().unwrap().push("noheapminimum512kib".to_string()); }
        }
        let __val = (*__exp_value.coverage_redesign.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.coverage_redesign.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("coverageredesign".to_string()); } else { __list_guard.as_mut().unwrap().push("nocoverageredesign".to_string()); }
        }
        let __val = (*__exp_value.arenas.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.arenas.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("arenas".to_string()); } else { __list_guard.as_mut().unwrap().push("noarenas".to_string()); }
        }
        let __val = (*__exp_value.cgo_check2.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.cgo_check2.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("cgocheck2".to_string()); } else { __list_guard.as_mut().unwrap().push("nocgocheck2".to_string()); }
        }
        let __val = (*__exp_value.loop_var.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.loop_var.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("loopvar".to_string()); } else { __list_guard.as_mut().unwrap().push("noloopvar".to_string()); }
        }
        let __val = (*__exp_value.cache_prog.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.cache_prog.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("cacheprog".to_string()); } else { __list_guard.as_mut().unwrap().push("nocacheprog".to_string()); }
        }
        let __val = (*__exp_value.new_inliner.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.new_inliner.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("newinliner".to_string()); } else { __list_guard.as_mut().unwrap().push("nonewinliner".to_string()); }
        }
        let __val = (*__exp_value.range_func.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.range_func.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("rangefunc".to_string()); } else { __list_guard.as_mut().unwrap().push("norangefunc".to_string()); }
        }
        let __val = (*__exp_value.alias_type_params.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.alias_type_params.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("aliastypeparams".to_string()); } else { __list_guard.as_mut().unwrap().push("noaliastypeparams".to_string()); }
        }
        let __val = (*__exp_value.swiss_map.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.swiss_map.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("swissmap".to_string()); } else { __list_guard.as_mut().unwrap().push("noswissmap".to_string()); }
        }
        let __val = (*__exp_value.spinbit_mutex.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.spinbit_mutex.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("spinbitmutex".to_string()); } else { __list_guard.as_mut().unwrap().push("nospinbitmutex".to_string()); }
        }
        let __val = (*__exp_value.sync_hash_trie_map.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.sync_hash_trie_map.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("synchashtriemap".to_string()); } else { __list_guard.as_mut().unwrap().push("nosynchashtriemap".to_string()); }
        }
        let __val = (*__exp_value.synctest.lock().unwrap().as_ref().unwrap()).clone();
        let __base_val = __base_value.map(|__base| (*__base.synctest.lock().unwrap().as_ref().unwrap()).clone()).unwrap_or(false);
        if __all || __val != __base_val {
            let mut __list_guard = list.lock().unwrap();
            if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }
            if __val { __list_guard.as_mut().unwrap().push("synctest".to_string()); } else { __list_guard.as_mut().unwrap().push("nosynctest".to_string()); }
        }
    }
    list.clone()
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for ExperimentFlags {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
