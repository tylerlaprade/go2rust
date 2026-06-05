use go2rust_stdlib_stubs::*;

use crate::exp_aliastypeparams_on::*;
use crate::exp_arenas_off::*;
use crate::exp_boringcrypto_off::*;
use crate::exp_cacheprog_off::*;
use crate::exp_cgocheck2_off::*;
use crate::exp_coverageredesign_on::*;
use crate::exp_fieldtrack_off::*;
use crate::exp_heapminimum512kib_off::*;
use crate::exp_loopvar_off::*;
use crate::exp_newinliner_off::*;
use crate::exp_preemptibleloops_off::*;
use crate::exp_rangefunc_off::*;
use crate::exp_regabiargs_on::*;
use crate::exp_regabiwrappers_on::*;
use crate::exp_spinbitmutex_on::*;
use crate::exp_staticlockranking_off::*;
use crate::exp_swissmap_on::*;
use crate::exp_synchashtriemap_on::*;
use crate::exp_synctest_off::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// Flags is the set of experiments that can be enabled or disabled in
/// the current toolchain.
///
/// When specified in the GOEXPERIMENT environment variable or as build
/// tags, experiments use the strings.ToLower of their field name.
///
/// For the baseline experimental configuration, see
/// [internal/buildcfg.ParseGOEXPERIMENT].
///
/// If you change this struct definition, run "go generate".
#[derive(Debug, Clone)]
pub struct Flags {
    pub field_track: Arc<Mutex<Option<bool>>>,
    pub preemptible_loops: Arc<Mutex<Option<bool>>>,
    pub static_lock_ranking: Arc<Mutex<Option<bool>>>,
    pub boring_crypto: Arc<Mutex<Option<bool>>>,
    pub regabi_wrappers: Arc<Mutex<Option<bool>>>,
    pub regabi_args: Arc<Mutex<Option<bool>>>,
    pub heap_minimum512_ki_b: Arc<Mutex<Option<bool>>>,
    pub coverage_redesign: Arc<Mutex<Option<bool>>>,
    pub arenas: Arc<Mutex<Option<bool>>>,
    pub cgo_check2: Arc<Mutex<Option<bool>>>,
    pub loop_var: Arc<Mutex<Option<bool>>>,
    pub cache_prog: Arc<Mutex<Option<bool>>>,
    pub new_inliner: Arc<Mutex<Option<bool>>>,
    pub range_func: Arc<Mutex<Option<bool>>>,
    pub alias_type_params: Arc<Mutex<Option<bool>>>,
    pub swiss_map: Arc<Mutex<Option<bool>>>,
    pub spinbit_mutex: Arc<Mutex<Option<bool>>>,
    pub sync_hash_trie_map: Arc<Mutex<Option<bool>>>,
    pub synctest: Arc<Mutex<Option<bool>>>,
}

impl Flags {
    pub fn __go_value_clone(&self) -> Self {
        Self { field_track: { let __guard = self.field_track.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preemptible_loops: { let __guard = self.preemptible_loops.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, static_lock_ranking: { let __guard = self.static_lock_ranking.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, boring_crypto: { let __guard = self.boring_crypto.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, regabi_wrappers: { let __guard = self.regabi_wrappers.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, regabi_args: { let __guard = self.regabi_args.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, heap_minimum512_ki_b: { let __guard = self.heap_minimum512_ki_b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, coverage_redesign: { let __guard = self.coverage_redesign.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arenas: { let __guard = self.arenas.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cgo_check2: { let __guard = self.cgo_check2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, loop_var: { let __guard = self.loop_var.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cache_prog: { let __guard = self.cache_prog.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, new_inliner: { let __guard = self.new_inliner.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, range_func: { let __guard = self.range_func.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alias_type_params: { let __guard = self.alias_type_params.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, swiss_map: { let __guard = self.swiss_map.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spinbit_mutex: { let __guard = self.spinbit_mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sync_hash_trie_map: { let __guard = self.sync_hash_trie_map.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, synctest: { let __guard = self.synctest.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Flags {
    fn default() -> Self {
        Self { field_track: Arc::new(Mutex::new(Some(false))), preemptible_loops: Arc::new(Mutex::new(Some(false))), static_lock_ranking: Arc::new(Mutex::new(Some(false))), boring_crypto: Arc::new(Mutex::new(Some(false))), regabi_wrappers: Arc::new(Mutex::new(Some(false))), regabi_args: Arc::new(Mutex::new(Some(false))), heap_minimum512_ki_b: Arc::new(Mutex::new(Some(false))), coverage_redesign: Arc::new(Mutex::new(Some(false))), arenas: Arc::new(Mutex::new(Some(false))), cgo_check2: Arc::new(Mutex::new(Some(false))), loop_var: Arc::new(Mutex::new(Some(false))), cache_prog: Arc::new(Mutex::new(Some(false))), new_inliner: Arc::new(Mutex::new(Some(false))), range_func: Arc::new(Mutex::new(Some(false))), alias_type_params: Arc::new(Mutex::new(Some(false))), swiss_map: Arc::new(Mutex::new(Some(false))), spinbit_mutex: Arc::new(Mutex::new(Some(false))), sync_hash_trie_map: Arc::new(Mutex::new(Some(false))), synctest: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.field_track.lock().unwrap().as_ref().unwrap()), (*self.preemptible_loops.lock().unwrap().as_ref().unwrap()), (*self.static_lock_ranking.lock().unwrap().as_ref().unwrap()), (*self.boring_crypto.lock().unwrap().as_ref().unwrap()), (*self.regabi_wrappers.lock().unwrap().as_ref().unwrap()), (*self.regabi_args.lock().unwrap().as_ref().unwrap()), (*self.heap_minimum512_ki_b.lock().unwrap().as_ref().unwrap()), (*self.coverage_redesign.lock().unwrap().as_ref().unwrap()), (*self.arenas.lock().unwrap().as_ref().unwrap()), (*self.cgo_check2.lock().unwrap().as_ref().unwrap()), (*self.loop_var.lock().unwrap().as_ref().unwrap()), (*self.cache_prog.lock().unwrap().as_ref().unwrap()), (*self.new_inliner.lock().unwrap().as_ref().unwrap()), (*self.range_func.lock().unwrap().as_ref().unwrap()), (*self.alias_type_params.lock().unwrap().as_ref().unwrap()), (*self.swiss_map.lock().unwrap().as_ref().unwrap()), (*self.spinbit_mutex.lock().unwrap().as_ref().unwrap()), (*self.sync_hash_trie_map.lock().unwrap().as_ref().unwrap()), (*self.synctest.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Flags {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("FieldTrack") {
            out.field_track = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PreemptibleLoops") {
            out.preemptible_loops = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("StaticLockRanking") {
            out.static_lock_ranking = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("BoringCrypto") {
            out.boring_crypto = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("RegabiWrappers") {
            out.regabi_wrappers = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("RegabiArgs") {
            out.regabi_args = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HeapMinimum512KiB") {
            out.heap_minimum512_ki_b = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CoverageRedesign") {
            out.coverage_redesign = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Arenas") {
            out.arenas = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoCheck2") {
            out.cgo_check2 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("LoopVar") {
            out.loop_var = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CacheProg") {
            out.cache_prog = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("NewInliner") {
            out.new_inliner = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("RangeFunc") {
            out.range_func = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("AliasTypeParams") {
            out.alias_type_params = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SwissMap") {
            out.swiss_map = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SpinbitMutex") {
            out.spinbit_mutex = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SyncHashTrieMap") {
            out.sync_hash_trie_map = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Synctest") {
            out.synctest = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl GoValueClone for Flags {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
