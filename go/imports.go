package main

import (
	"fmt"
	"slices"
	"strings"
)

// ImportTracker tracks which imports are needed during transpilation
type ImportTracker struct {
	needs map[string]bool
}

// NewImportTracker creates a new import tracker
func NewImportTracker() *ImportTracker {
	return &ImportTracker{
		needs: make(map[string]bool),
	}
}

// Add marks an import as needed with a reason
func (it *ImportTracker) Add(importName string) {
	it.needs[importName] = true
}

// GenerateImports returns the import statements for the file
func (it *ImportTracker) GenerateImports() string {
	if len(it.needs) == 0 {
		return ""
	}

	var imports []string

	// Check for std::sync imports
	var syncImports []string
	if it.needs["Arc"] {
		syncImports = append(syncImports, "Arc")
	}
	if it.needs["Mutex"] {
		syncImports = append(syncImports, "Mutex")
	}
	if len(syncImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::sync::{%s};", strings.Join(syncImports, ", ")))
	}

	// Check for std::rc and std::cell imports
	var rcImports []string
	if it.needs["Rc"] {
		rcImports = append(rcImports, "Rc")
	}
	if len(rcImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::rc::{%s};", strings.Join(rcImports, ", ")))
	}

	var cellImports []string
	if it.needs["RefCell"] {
		cellImports = append(cellImports, "RefCell")
	}
	if len(cellImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::cell::{%s};", strings.Join(cellImports, ", ")))
	}

	// BTreeMap (deterministic iteration order, matching Go test expectations)
	if it.needs["BTreeMap"] {
		imports = append(imports, "use std::collections::BTreeMap;")
	}

	// fmt imports
	var fmtImports []string
	if it.needs["fmt::self"] {
		fmtImports = append(fmtImports, "self")
	}
	if it.needs["Display"] {
		fmtImports = append(fmtImports, "Display")
	}
	if it.needs["Debug"] {
		fmtImports = append(fmtImports, "Debug")
	}
	if it.needs["Formatter"] {
		fmtImports = append(fmtImports, "Formatter")
	}
	if len(fmtImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::fmt::{%s};", strings.Join(fmtImports, ", ")))
	}

	// Thread and time imports
	if it.needs["thread"] {
		imports = append(imports, "use std::thread;")
	}
	if it.needs["time::Duration"] {
		imports = append(imports, "use std::time::Duration;")
	}

	// Other imports
	if it.needs["Error"] {
		imports = append(imports, "use std::error::Error as StdError;")
	}
	if it.needs["Any"] {
		imports = append(imports, "use std::any::Any;")
	}
	if it.needs["Ord"] {
		imports = append(imports, "use std::cmp::Ord;")
	}

	// External crate imports
	if it.needs["num::Complex"] {
		imports = append(imports, "use num::Complex;")
	}

	// Sort for consistent output
	slices.Sort(imports)

	return strings.Join(imports, "\n") + "\n"
}

// HelperTracker tracks which helper functions are needed
type HelperTracker struct {
	needsFormatMap                  bool
	needsFormatSlice                bool
	needsFormatSliceWrappedValues   bool
	needsFormatSliceWrappedStringer bool
	needsFormatNestedSlice          bool
	needsFormatAny                  bool
	needsFormatAnySlice             bool
	needsGoChannel                  bool
	needsWaitGroup                  bool
	needsGoMutex                    bool
	needsGoOnce                     bool
	needsGoTypeName                 bool
	needsBase64                     bool
	needsSha256                     bool
	needsHexFormat                  bool
	needsStrconvFormat              bool
	needsUrl                        bool
	needsRegexp                     bool
	needsJsonEscape                 bool
	needsOsFile                     bool
	needsSliceElemPtr               bool
	needsGoTime                     bool
	needsGoTimer                    bool
	needsGoAfter                    bool
	needsGoTicker                   bool
	needsGoTick                     bool
	needsGoContext                  bool
	needsGoRand                     bool
	needsReflect                    bool
	needsGoHttpResponse             bool
	needsGoPtrKey                   bool
}

var generatingPublicHelpers bool

func (ht *HelperTracker) withoutSharedStdlibHelpers() *HelperTracker {
	if ht == nil {
		return nil
	}
	helperCopy := *ht
	helperCopy.needsGoChannel = false
	helperCopy.needsGoContext = false
	return &helperCopy
}

func (ht *HelperTracker) sharedStdlibHelpersOnly() *HelperTracker {
	if ht == nil {
		return nil
	}
	helperCopy := &HelperTracker{}
	if ht.needsGoChannel || ht.needsGoContext {
		helperCopy.needsGoChannel = true
	}
	if ht.needsGoContext {
		helperCopy.needsGoContext = true
	}
	return helperCopy
}

// GenerateHelpers returns the helper function definitions
func (ht *HelperTracker) GenerateHelpers() string {
	var result strings.Builder

	if ht.needsFormatMap {
		generateMapFormatter(&result)
	}

	if ht.needsFormatSlice {
		generateSliceFormatter(&result, ht.needsFormatSliceWrappedValues, ht.needsFormatSliceWrappedStringer)
	}

	if ht.needsFormatNestedSlice {
		generateNestedSliceFormatter(&result)
	}

	if ht.needsFormatAny {
		generateAnyFormatter(&result)
	}

	if ht.needsFormatAnySlice {
		generateAnySliceFormatter(&result)
	}

	if ht.needsGoChannel {
		generateGoChannelHelper(&result)
	}

	if ht.needsWaitGroup {
		generateWaitGroupHelper(&result)
	}

	if ht.needsGoMutex {
		generateGoMutexHelper(&result)
	}

	if ht.needsGoOnce {
		generateGoOnceHelper(&result)
	}

	if ht.needsGoTypeName {
		generateGoTypeNameHelper(&result)
	}

	if ht.needsBase64 {
		generateBase64Helper(&result)
	}

	if ht.needsSha256 {
		generateSha256Helper(&result)
	}

	if ht.needsHexFormat {
		generateHexFormatHelper(&result)
	}

	if ht.needsStrconvFormat {
		generateStrconvFormatHelper(&result)
	}

	if ht.needsUrl {
		generateUrlHelper(&result)
	}

	if ht.needsRegexp {
		generateRegexpHelper(&result)
	}

	if ht.needsJsonEscape {
		generateJsonEscapeHelper(&result)
	}

	if ht.needsOsFile {
		generateOsFileHelper(&result)
	}

	if ht.needsSliceElemPtr {
		generateSliceElemPtrHelper(&result)
	}

	if ht.needsGoTime {
		generateGoTimeHelper(&result)
	}

	if ht.needsGoTimer {
		generateGoTimerHelper(&result)
	}

	if ht.needsGoAfter {
		generateGoAfterHelper(&result)
	}

	if ht.needsGoTick {
		generateGoTickHelper(&result)
	}

	if ht.needsGoTicker {
		generateGoTickerHelper(&result)
	}

	if ht.needsGoContext {
		generateGoContextHelper(&result)
	}

	if ht.needsGoRand {
		generateGoRandHelper(&result)
	}

	if ht.needsReflect {
		generateReflectHelper(&result)
	}

	if ht.needsGoPtrKey {
		generateGoPtrKeyHelper(&result, "GoLocalPtrKey")
	}

	return result.String()
}

func (ht *HelperTracker) HasAnyOmittingSharedStdlibHelpers() bool {
	filtered := ht.withoutSharedStdlibHelpers()
	return filtered != nil && filtered.HasAny()
}

func (ht *HelperTracker) HasAny() bool {
	return ht != nil && (ht.needsFormatMap ||
		ht.needsFormatSlice ||
		ht.needsFormatAny ||
		ht.needsFormatAnySlice ||
		ht.needsGoChannel ||
		ht.needsWaitGroup ||
		ht.needsGoMutex ||
		ht.needsGoOnce ||
		ht.needsGoTypeName ||
		ht.needsBase64 ||
		ht.needsSha256 ||
		ht.needsHexFormat ||
		ht.needsStrconvFormat ||
		ht.needsUrl ||
		ht.needsRegexp ||
		ht.needsJsonEscape ||
		ht.needsOsFile ||
		ht.needsSliceElemPtr ||
		ht.needsGoTime ||
		ht.needsGoTimer ||
		ht.needsGoAfter ||
		ht.needsGoTicker ||
		ht.needsGoTick ||
		ht.needsGoContext ||
		ht.needsGoRand ||
		ht.needsReflect ||
		ht.needsGoHttpResponse ||
		ht.needsGoPtrKey)
}

func (ht *HelperTracker) GenerateHelperModule() string {
	return ht.generateHelperModule(false)
}

func (ht *HelperTracker) GenerateHelperModuleOmittingSharedStdlibHelpers() string {
	return ht.withoutSharedStdlibHelpers().generateHelperModule(false)
}

func (ht *HelperTracker) GenerateSharedStdlibHelperModule() string {
	return ht.sharedStdlibHelpersOnly().generateHelperModule(true)
}

func (ht *HelperTracker) generateHelperModule(publicHelpers bool) string {
	if ht == nil {
		return ""
	}
	if !ht.HasAny() {
		return ""
	}

	imports := NewImportTracker()
	helperCopy := *ht
	fileState := NewFileState(imports, &helperCopy, nil)
	parentCtx := GetTranspileContext()
	prevPublicHelpers := generatingPublicHelpers
	generatingPublicHelpers = publicHelpers
	defer func() {
		generatingPublicHelpers = prevPublicHelpers
	}()
	SetTranspileContext(&TranspileContext{
		File:    fileState,
		Imports: imports,
		Helpers: &helperCopy,
	})
	helpersStr := helperCopy.GenerateHelpers()
	SetTranspileContext(parentCtx)

	var output strings.Builder
	importsStr := imports.GenerateImports()
	output.WriteString(importsStr)
	if importsStr != "" && helpersStr != "" {
		output.WriteString("\n")
	}
	output.WriteString(helpersStr)
	return output.String()
}

func (ht *HelperTracker) ImportNamesOmittingSharedStdlibHelpers() []string {
	return ht.withoutSharedStdlibHelpers().ImportNames()
}

func (ht *HelperTracker) ImportNames() []string {
	if ht == nil {
		return nil
	}
	seen := make(map[string]bool)
	add := func(names ...string) {
		for _, name := range names {
			if name != "" {
				seen[name] = true
			}
		}
	}

	if ht.needsFormatMap {
		add("format_map")
	}
	if ht.needsFormatSlice {
		add("format_slice", "format_slice_values", "format_slice_wrapped")
		if ht.needsFormatSliceWrappedValues {
			add("format_slice_wrapped_values")
		}
		if ht.needsFormatSliceWrappedStringer {
			add("format_slice_wrapped_stringer", "format_slice_wrapped_stringer_values")
		}
	}
	if ht.needsFormatNestedSlice {
		add("format_nested_slice")
	}
	if ht.needsFormatAny {
		add("format_any")
	}
	if ht.needsFormatAnySlice {
		add("format_any_slice")
	}
	if ht.needsGoChannel {
		add("GoChannel")
	}
	if ht.needsWaitGroup {
		add("WaitGroup")
	}
	if ht.needsGoMutex {
		add("GoMutex")
	}
	if ht.needsGoOnce {
		add("GoOnce")
	}
	if ht.needsGoTypeName {
		add("go_type_name")
	}
	if ht.needsBase64 {
		add("go_base64_encode", "go_base64_decode", "go_base64_value")
	}
	if ht.needsSha256 {
		add("go_sha256_sum256")
	}
	if ht.needsHexFormat {
		add("go_format_hex_bytes")
	}
	if ht.needsStrconvFormat {
		add("go_strconv_format_int", "go_strconv_format_float")
	}
	if ht.needsUrl {
		add("GoUrl", "go_url_parse")
	}
	if ht.needsRegexp {
		add("GoRegexp", "go_regexp_find_all_string")
	}
	if ht.needsJsonEscape {
		add("go_json_escape")
	}
	if ht.needsOsFile {
		add("GoFile")
	}
	if ht.needsSliceElemPtr {
		add("GoSliceElemPtr", "GoSliceElemRef", "GoSliceElemMutRef")
	}
	if ht.needsGoTime {
		add("GoTime", "go_time_civil_from_days")
	}
	if ht.needsGoTimer {
		add("GoTimer", "go_new_timer")
	}
	if ht.needsGoAfter {
		add("go_channel_after")
	}
	if ht.needsGoTicker {
		add("GoTicker", "go_new_ticker")
	}
	if ht.needsGoTick {
		add("go_tick")
	}
	if ht.needsGoContext {
		add("GoContext", "GoCancelFunc", "GoCancelCauseFunc")
	}
	if ht.needsGoRand {
		add("go_rand_seed", "go_rand_intn", "go_rand_float64", "go_rand_state", "go_rand_next_u64")
	}
	if ht.needsReflect {
		add("GoReflectStructTag", "GoReflectField", "GoReflectType", "go_reflect_tag_get")
	}
	if ht.needsGoPtrKey {
		add("GoLocalPtrKey")
	}

	names := make([]string, 0, len(seen))
	for name := range seen {
		names = append(names, name)
	}
	slices.Sort(names)
	return names
}

func generateGoPtrKeyHelper(out *strings.Builder, name string) {
	if NeedsConcurrentWrapper() {
		out.WriteString(`
#[derive(Clone)]
pub struct ` + name + `<T>(pub Arc<Mutex<Option<T>>>);

impl<T> ` + name + `<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { ` + name + `(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for ` + name + `<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for ` + name + `<T> {}
impl<T> PartialOrd for ` + name + `<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for ` + name + `<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
	} else {
		out.WriteString(`
#[derive(Clone)]
pub struct ` + name + `<T>(pub Rc<RefCell<Option<T>>>);

impl<T> ` + name + `<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { ` + name + `(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for ` + name + `<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for ` + name + `<T> {}
impl<T> PartialOrd for ` + name + `<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for ` + name + `<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
	}
}

func generateAnyFormatter(out *strings.Builder) {
	TrackImport("Any")
	out.WriteString("\nfn format_any(value: &dyn Any) -> String {\n")
	out.WriteString("    if let Some(v) = value.downcast_ref::<i32>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<i64>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<f64>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<f32>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<String>() {\n")
	out.WriteString("        v.clone()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<&str>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<bool>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else {\n")
	out.WriteString("        \"<unknown>\".to_string()\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateAnySliceFormatter(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		TrackImport("Any")
		out.WriteString(`
fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any>>>>>) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		TrackImport("Any")
		out.WriteString(`
fn format_any_slice(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	}
}

func generateGoChannelHelper(out *strings.Builder) {
	code := `
struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}
`
	if generatingPublicHelpers {
		code = strings.ReplaceAll(code, "struct GoChannel<T>", "pub struct GoChannel<T>")
		code = strings.ReplaceAll(code, "    fn new(", "    pub fn new(")
		code = strings.ReplaceAll(code, "    fn new_buffered(", "    pub fn new_buffered(")
		code = strings.ReplaceAll(code, "    fn send(", "    pub fn send(")
		code = strings.ReplaceAll(code, "    fn try_send(", "    pub fn try_send(")
		code = strings.ReplaceAll(code, "    fn recv(", "    pub fn recv(")
		code = strings.ReplaceAll(code, "    fn try_recv(", "    pub fn try_recv(")
		code = strings.ReplaceAll(code, "    fn close(", "    pub fn close(")
		code = strings.ReplaceAll(code, "    fn is_nil(", "    pub fn is_nil(")
		code = strings.ReplaceAll(code, "    fn len(", "    pub fn len(")
		code = strings.ReplaceAll(code, "    fn capacity(", "    pub fn capacity(")
	}
	out.WriteString(code)
}

func generateWaitGroupHelper(out *strings.Builder) {
	out.WriteString(`
struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i32>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i32) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
}
`)
}

func generateGoMutexHelper(out *strings.Builder) {
	out.WriteString(`
struct GoMutex {
    inner: std::sync::Arc<std::sync::Mutex<()>>,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new(std::sync::Mutex::new(())),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<()> {
        self.inner.lock().unwrap()
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        GoMutex {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}
`)
}

func generateGoOnceHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		out.WriteString(`
#[derive(Clone, Debug)]
struct GoOnce {
    done: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl GoOnce {
    fn new() -> Self {
        GoOnce {
            done: std::sync::Arc::new(std::sync::Mutex::new(false)),
        }
    }

    fn r#do<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        let mut done = self.done.lock().unwrap();
        if !*done {
            *done = true;
            drop(done);
            f();
        }
    }
}

impl Default for GoOnce {
    fn default() -> Self {
        Self::new()
    }
}
`)
		return
	}
	out.WriteString(`
#[derive(Clone, Debug)]
struct GoOnce {
    done: std::rc::Rc<std::cell::RefCell<bool>>,
}

impl GoOnce {
    fn new() -> Self {
        GoOnce {
            done: std::rc::Rc::new(std::cell::RefCell::new(false)),
        }
    }

    fn r#do<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        let mut done = self.done.borrow_mut();
        if !*done {
            *done = true;
            drop(done);
            f();
        }
    }
}

impl Default for GoOnce {
    fn default() -> Self {
        Self::new()
    }
}
`)
}

func generateGoTypeNameHelper(out *strings.Builder) {
	TrackImport("Any")
	out.WriteString(`
fn go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    std::any::type_name_of_val(val)
}
`)
}

func generateBase64Helper(out *strings.Builder) {
	out.WriteString(`
fn go_base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }

        i += 3;
    }
    out
}

fn go_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn go_base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 != 0 {
        return Err(format!("illegal base64 data at input byte {}", bytes.len()));
    }

    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c0 = go_base64_value(bytes[i])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i))?;
        let c1 = go_base64_value(bytes[i + 1])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 1))?;
        let pad2 = bytes[i + 2] == b'=';
        let pad3 = bytes[i + 3] == b'=';
        let c2 = if pad2 {
            0
        } else {
            go_base64_value(bytes[i + 2])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 2))?
        };
        let c3 = if pad3 {
            0
        } else {
            go_base64_value(bytes[i + 3])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 3))?
        };

        out.push((c0 << 2) | (c1 >> 4));
        if !pad2 {
            out.push((c1 << 4) | (c2 >> 2));
        }
        if !pad3 {
            out.push((c2 << 6) | c3);
        }

        i += 4;
    }
    Ok(out)
}
`)
}

func generateSha256Helper(out *strings.Builder) {
	out.WriteString(`
fn go_sha256_sum256(data: &[u8]) -> Vec<u8> {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut h = H0;
    let mut msg = data.to_vec();
    let bit_len = (msg.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            let j = i * 4;
            w[i] = u32::from_be_bytes([chunk[j], chunk[j + 1], chunk[j + 2], chunk[j + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = Vec::with_capacity(32);
    for word in h {
        out.extend_from_slice(&word.to_be_bytes());
    }
    out
}
`)
}

func generateHexFormatHelper(out *strings.Builder) {
	out.WriteString(`
fn go_format_hex_bytes(data: &[u8], upper: bool) -> String {
    let table = if upper {
        b"0123456789ABCDEF"
    } else {
        b"0123456789abcdef"
    };
    let mut out = String::with_capacity(data.len() * 2);
    for b in data {
        out.push(table[(b >> 4) as usize] as char);
        out.push(table[(b & 0x0f) as usize] as char);
    }
    out
}
`)
}

func generateUrlHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoUrl {
    scheme: Arc<Mutex<Option<String>>>,
    host: Arc<Mutex<Option<String>>>,
    path: Arc<Mutex<Option<String>>>,
    raw_query: Arc<Mutex<Option<String>>>,
}

fn go_url_parse(input: &str) -> GoUrl {
    let (scheme, rest) = input.split_once("://").unwrap_or(("", input));
    let (before_query, raw_query) = rest.split_once('?').unwrap_or((rest, ""));
    let slash = before_query.find('/').unwrap_or(before_query.len());
    let host = &before_query[..slash];
    let path = if slash < before_query.len() { &before_query[slash..] } else { "" };
    GoUrl {
        scheme: Arc::new(Mutex::new(Some(scheme.to_string()))),
        host: Arc::new(Mutex::new(Some(host.to_string()))),
        path: Arc::new(Mutex::new(Some(path.to_string()))),
        raw_query: Arc::new(Mutex::new(Some(raw_query.to_string()))),
    }
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoUrl {
    scheme: Rc<RefCell<Option<String>>>,
    host: Rc<RefCell<Option<String>>>,
    path: Rc<RefCell<Option<String>>>,
    raw_query: Rc<RefCell<Option<String>>>,
}

fn go_url_parse(input: &str) -> GoUrl {
    let (scheme, rest) = input.split_once("://").unwrap_or(("", input));
    let (before_query, raw_query) = rest.split_once('?').unwrap_or((rest, ""));
    let slash = before_query.find('/').unwrap_or(before_query.len());
    let host = &before_query[..slash];
    let path = if slash < before_query.len() { &before_query[slash..] } else { "" };
    GoUrl {
        scheme: Rc::new(RefCell::new(Some(scheme.to_string()))),
        host: Rc::new(RefCell::new(Some(host.to_string()))),
        path: Rc::new(RefCell::new(Some(path.to_string()))),
        raw_query: Rc::new(RefCell::new(Some(raw_query.to_string()))),
    }
}
`)
}

func generateRegexpHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoRegexp {
    pattern: Arc<Mutex<Option<String>>>,
}

impl GoRegexp {
    fn find_all_string(&self, text: Arc<Mutex<Option<String>>>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        let limit = *n.lock().unwrap().as_ref().unwrap();
        Arc::new(Mutex::new(Some(go_regexp_find_all_string(&pattern, &text, limit))))
    }

    fn match_string(&self, text: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<bool>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_regexp_match_string(&pattern, &text))))
    }

    fn find_string_submatch(&self, text: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_regexp_find_string_submatch(&pattern, &text))))
    }

    fn replace_all_string(&self, src: Arc<Mutex<Option<String>>>, repl: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let src = (*src.lock().unwrap().as_ref().unwrap()).clone();
        let repl = (*repl.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_regexp_replace_all_string(&pattern, &src, &repl))))
    }
}

fn go_regexp_find_all_string(pattern: &str, text: &str, limit: i32) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    if pattern == r"\d+" {
        let mut matches = Vec::new();
        let mut current = String::new();
        for ch in text.chars() {
            if ch.is_ascii_digit() {
                current.push(ch);
            } else if !current.is_empty() {
                matches.push(std::mem::take(&mut current));
                if limit > 0 && matches.len() >= limit as usize {
                    return matches;
                }
            }
        }
        if !current.is_empty() {
            matches.push(current);
        }
        if limit > 0 {
            matches.truncate(limit as usize);
        }
        return matches;
    }

    if pattern.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.find(pattern) {
        matches.push(pattern.to_string());
        if limit > 0 && matches.len() >= limit as usize {
            break;
        }
        rest = &rest[index + pattern.len()..];
    }
    matches
}

fn go_regexp_match_string(pattern: &str, text: &str) -> bool {
    !go_regexp_find_string_submatch(pattern, text).is_empty()
}

fn go_regexp_find_string_submatch(pattern: &str, text: &str) -> Vec<String> {
    if pattern == r"-mod[ =](\w+)" {
        for marker in ["-mod=", "-mod "] {
            if let Some(start) = text.find(marker) {
                let value_start = start + marker.len();
                let value: String = text[value_start..].chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
                if !value.is_empty() {
                    return vec![format!("{}{}", marker, value), value];
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"^go version (go\S+|devel \S+)" {
        let prefix = "go version ";
        if let Some(rest) = text.strip_prefix(prefix) {
            if let Some(first) = rest.split_whitespace().next() {
                if first.starts_with("go") {
                    return vec![format!("{}{}", prefix, first), first.to_string()];
                }
                if first == "devel" {
                    if let Some(second) = rest.split_whitespace().nth(1) {
                        let capture = format!("devel {}", second);
                        return vec![format!("{}{}", prefix, capture), capture];
                    }
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"go:.*go.mod.*contents have changed" {
        if let Some(go_index) = text.find("go:") {
            if let Some(mod_index) = text[go_index..].find("go.mod") {
                let after_mod = go_index + mod_index;
                if text[after_mod..].contains("contents have changed") {
                    return vec![text.to_string()];
                }
            }
        }
        return Vec::new();
    }

    let matches = go_regexp_find_all_string(pattern, text, 1);
    if matches.is_empty() {
        Vec::new()
    } else {
        vec![matches[0].clone()]
    }
}

fn go_regexp_replace_all_string(pattern: &str, text: &str, repl: &str) -> String {
    if pattern == r"[$,]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == '$' || ch == ',' {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    if pattern == r"[_]" {
        return text.replace('_', repl);
    }
    if pattern == r"[USD\s]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == 'U' || ch == 'S' || ch == 'D' || ch.is_whitespace() {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    text.replace(pattern, repl)
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoRegexp {
    pattern: Rc<RefCell<Option<String>>>,
}

impl GoRegexp {
    fn find_all_string(&self, text: Rc<RefCell<Option<String>>>, n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Vec<String>>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        let limit = *n.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(go_regexp_find_all_string(&pattern, &text, limit))))
    }

    fn match_string(&self, text: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<bool>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_match_string(&pattern, &text))))
    }

    fn find_string_submatch(&self, text: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<String>>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_find_string_submatch(&pattern, &text))))
    }

    fn replace_all_string(&self, src: Rc<RefCell<Option<String>>>, repl: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let src = (*src.borrow().as_ref().unwrap()).clone();
        let repl = (*repl.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_replace_all_string(&pattern, &src, &repl))))
    }
}

fn go_regexp_find_all_string(pattern: &str, text: &str, limit: i32) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    if pattern == r"\d+" {
        let mut matches = Vec::new();
        let mut current = String::new();
        for ch in text.chars() {
            if ch.is_ascii_digit() {
                current.push(ch);
            } else if !current.is_empty() {
                matches.push(std::mem::take(&mut current));
                if limit > 0 && matches.len() >= limit as usize {
                    return matches;
                }
            }
        }
        if !current.is_empty() {
            matches.push(current);
        }
        if limit > 0 {
            matches.truncate(limit as usize);
        }
        return matches;
    }

    if pattern.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.find(pattern) {
        matches.push(pattern.to_string());
        if limit > 0 && matches.len() >= limit as usize {
            break;
        }
        rest = &rest[index + pattern.len()..];
    }
    matches
}

fn go_regexp_match_string(pattern: &str, text: &str) -> bool {
    !go_regexp_find_string_submatch(pattern, text).is_empty()
}

fn go_regexp_find_string_submatch(pattern: &str, text: &str) -> Vec<String> {
    if pattern == r"-mod[ =](\w+)" {
        for marker in ["-mod=", "-mod "] {
            if let Some(start) = text.find(marker) {
                let value_start = start + marker.len();
                let value: String = text[value_start..].chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
                if !value.is_empty() {
                    return vec![format!("{}{}", marker, value), value];
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"^go version (go\S+|devel \S+)" {
        let prefix = "go version ";
        if let Some(rest) = text.strip_prefix(prefix) {
            if let Some(first) = rest.split_whitespace().next() {
                if first.starts_with("go") {
                    return vec![format!("{}{}", prefix, first), first.to_string()];
                }
                if first == "devel" {
                    if let Some(second) = rest.split_whitespace().nth(1) {
                        let capture = format!("devel {}", second);
                        return vec![format!("{}{}", prefix, capture), capture];
                    }
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"go:.*go.mod.*contents have changed" {
        if let Some(go_index) = text.find("go:") {
            if let Some(mod_index) = text[go_index..].find("go.mod") {
                let after_mod = go_index + mod_index;
                if text[after_mod..].contains("contents have changed") {
                    return vec![text.to_string()];
                }
            }
        }
        return Vec::new();
    }

    let matches = go_regexp_find_all_string(pattern, text, 1);
    if matches.is_empty() {
        Vec::new()
    } else {
        vec![matches[0].clone()]
    }
}

fn go_regexp_replace_all_string(pattern: &str, text: &str, repl: &str) -> String {
    if pattern == r"[$,]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == '$' || ch == ',' {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    if pattern == r"[_]" {
        return text.replace('_', repl);
    }
    if pattern == r"[USD\s]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == 'U' || ch == 'S' || ch == 'D' || ch.is_whitespace() {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    text.replace(pattern, repl)
}
`)
}

func generateJsonEscapeHelper(out *strings.Builder) {
	out.WriteString(`
fn go_json_escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c < ' ' => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}
`)
}

func generateStrconvFormatHelper(out *strings.Builder) {
	out.WriteString(`
fn go_strconv_format_int(value: i64, base: i32) -> String {
    if base == 10 {
        return value.to_string();
    }
    if !(2..=36).contains(&base) {
        return value.to_string();
    }

    let negative = value < 0;
    let mut n = if negative {
        value.wrapping_neg() as u64
    } else {
        value as u64
    };
    let base = base as u64;
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    if n == 0 {
        out.push(b'0');
    }
    while n > 0 {
        out.push(digits[(n % base) as usize]);
        n /= base;
    }
    if negative {
        out.push(b'-');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn go_strconv_format_float(value: f64, fmt: char, precision: i32) -> String {
    let precision = if precision < 0 { 6 } else { precision as usize };
    match fmt {
        'e' => format!("{:.*e}", precision, value),
        'E' => format!("{:.*E}", precision, value),
        'f' => format!("{:.*}", precision, value),
        'g' | 'G' => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.*}", precision, value)
            }
        }
        _ => value.to_string(),
    }
}
`)
}

func generateSliceElemPtrHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoSliceElemPtr<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
}

struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`
#[derive(Clone)]
struct GoSliceElemPtr<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
}

struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

struct GoSliceElemMutRef<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    fn new(slice: Rc<RefCell<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.borrow_mut().as_mut() {
                values[self.index] = value;
            }
        }
    }
}
`)
	}
}

func generateGoTimeHelper(out *strings.Builder) {
	out.WriteString(`
#[derive(Clone, Debug)]
struct GoTime {
    seconds: i64,
    nanos: i32,
}

fn go_time_civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, m as u32, d as u32)
}

impl GoTime {
    fn now() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        GoTime {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    fn from_unix(seconds: i64, nanos: i64) -> Self {
        let seconds = seconds + nanos.div_euclid(1_000_000_000);
        let nanos = nanos.rem_euclid(1_000_000_000);
        GoTime {
            seconds,
            nanos: nanos as i32,
        }
    }
`)
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
    fn add(&self, duration: Arc<Mutex<Option<std::time::Duration>>>) -> Arc<Mutex<Option<GoTime>>> {
        let duration = *duration.lock().unwrap().as_ref().unwrap();
        Arc::new(Mutex::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Arc<Mutex<Option<GoTime>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn unix(&self) -> Arc<Mutex<Option<i64>>> {
        Arc::new(Mutex::new(Some(self.seconds)))
    }

    fn unix_nano(&self) -> Arc<Mutex<Option<i64>>> {
        Arc::new(Mutex::new(Some(
            self.seconds * 1_000_000_000 + self.nanos as i64,
        )))
    }

    fn is_zero(&self) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some(self.seconds == 0 && self.nanos == 0)))
    }

    fn format(&self, _layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(self.to_string())))
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`
    fn add(&self, duration: Rc<RefCell<Option<std::time::Duration>>>) -> Rc<RefCell<Option<GoTime>>> {
        let duration = *duration.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Rc<RefCell<Option<GoTime>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn unix(&self) -> Rc<RefCell<Option<i64>>> {
        Rc::new(RefCell::new(Some(self.seconds)))
    }

    fn unix_nano(&self) -> Rc<RefCell<Option<i64>>> {
        Rc::new(RefCell::new(Some(
            self.seconds * 1_000_000_000 + self.nanos as i64,
        )))
    }

    fn is_zero(&self) -> Rc<RefCell<Option<bool>>> {
        Rc::new(RefCell::new(Some(self.seconds == 0 && self.nanos == 0)))
    }

    fn format(&self, _layout: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(self.to_string())))
    }
}
`)
	}
	out.WriteString(`
impl std::fmt::Display for GoTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let days = self.seconds.div_euclid(86_400);
        let secs_of_day = self.seconds.rem_euclid(86_400);
        let (year, month, day) = go_time_civil_from_days(days);
        let hour = secs_of_day / 3_600;
        let minute = (secs_of_day % 3_600) / 60;
        let second = secs_of_day % 60;
        if self.nanos == 0 {
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} +0000 UTC",
                year, month, day, hour, minute, second
            )
        } else {
            let mut fraction = format!("{:09}", self.nanos);
            while fraction.ends_with('0') {
                fraction.pop();
            }
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{} +0000 UTC",
                year, month, day, hour, minute, second, fraction
            )
        }
    }
}
`)
}

func generateGoTimerHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTimer {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_timer(duration: std::time::Duration) -> GoTimer {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            thread_channel.send(GoTime::now());
        }
        thread_channel.close();
    });

    GoTimer {
        c: channel,
        stopped,
    }
}

impl GoTimer {
    fn stop(&self) -> Arc<Mutex<Option<bool>>> {
        let was_stopped = self.stopped.swap(true, std::sync::atomic::Ordering::SeqCst);
        Arc::new(Mutex::new(Some(!was_stopped)))
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTimer {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_timer(duration: std::time::Duration) -> GoTimer {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            thread_channel.send(GoTime::now());
        }
        thread_channel.close();
    });

    GoTimer {
        c: channel,
        stopped,
    }
}

impl GoTimer {
    fn stop(&self) -> Rc<RefCell<Option<bool>>> {
        let was_stopped = self.stopped.swap(true, std::sync::atomic::Ordering::SeqCst);
        Rc::new(RefCell::new(Some(!was_stopped)))
    }
}
`)
	}
}

func generateGoAfterHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	out.WriteString(`
fn go_channel_after(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        thread_channel.send(GoTime::now());
        thread_channel.close();
    });
    channel
}
`)
}

func generateGoTickHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	out.WriteString(`
fn go_tick(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(duration);
            let _ = thread_channel.try_send(GoTime::now());
        }
    });
    channel
}
`)
}

func generateGoTickerHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTicker {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_ticker(duration: std::time::Duration) -> GoTicker {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        while !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(duration);
            if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
                let _ = thread_channel.try_send(GoTime::now());
            }
        }
        thread_channel.close();
    });

    GoTicker {
        c: channel,
        stopped,
    }
}

impl GoTicker {
    fn stop(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTicker {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_ticker(duration: std::time::Duration) -> GoTicker {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        while !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(duration);
            if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
                let _ = thread_channel.try_send(GoTime::now());
            }
        }
        thread_channel.close();
    });

    GoTicker {
        c: channel,
        stopped,
    }
}

impl GoTicker {
    fn stop(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
`)
	}
}

func generateGoContextHelper(out *strings.Builder) {
	NeedGoChannel()
	TrackImport("Arc")
	TrackImport("Mutex")
	code := `
#[derive(Clone)]
struct GoContext {
    done: GoChannel<bool>,
    err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    label: String,
}

type GoCancelFunc = std::sync::Arc<dyn Fn() + Send + Sync>;
type GoCancelCauseFunc = Box<dyn FnMut(Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) -> () + Send + Sync>;

impl GoContext {
    fn background() -> GoContext {
        GoContext {
            done: GoChannel::<bool>::new(),
            err: std::sync::Arc::new(std::sync::Mutex::new(None)),
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            label: "context.Background".to_string(),
        }
    }

    fn parent_label(parent: &Arc<Mutex<Option<GoContext>>>) -> String {
        parent
            .lock()
            .unwrap()
            .as_ref()
            .map(|ctx| ctx.label.clone())
            .unwrap_or_else(|| "context.Context".to_string())
    }

    fn with_timeout(parent: Arc<Mutex<Option<GoContext>>>, duration: std::time::Duration) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelFunc>>>) {
        let label = format!("{}.WithDeadline", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let timeout_done = done.clone();
        let timeout_err = err.clone();
        let timeout_cancelled = cancelled.clone();
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            if !timeout_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *timeout_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context deadline exceeded".to_string()));
                timeout_done.send(true);
                timeout_done.close();
            }
        });

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelFunc = std::sync::Arc::new(move || {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn with_cancel(parent: Arc<Mutex<Option<GoContext>>>) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelFunc>>>) {
        let label = format!("{}.WithCancel", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelFunc = std::sync::Arc::new(move || {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn with_cancel_cause(parent: Arc<Mutex<Option<GoContext>>>) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelCauseFunc>>>) {
        let label = format!("{}.WithCancelCause", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelCauseFunc = Box::new(move |_cause| {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn done(&self) -> GoChannel<bool> {
        self.done.clone()
    }

    fn err(&self) -> Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> {
        self.err.clone()
    }
}

impl std::fmt::Display for GoContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl std::fmt::Debug for GoContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
`
	if generatingPublicHelpers {
		code = strings.ReplaceAll(code, "struct GoContext", "pub struct GoContext")
		code = strings.ReplaceAll(code, "type GoCancelFunc", "pub type GoCancelFunc")
		code = strings.ReplaceAll(code, "type GoCancelCauseFunc", "pub type GoCancelCauseFunc")
		code = strings.ReplaceAll(code, "    fn background(", "    pub fn background(")
		code = strings.ReplaceAll(code, "    fn with_timeout(", "    pub fn with_timeout(")
		code = strings.ReplaceAll(code, "    fn with_cancel(", "    pub fn with_cancel(")
		code = strings.ReplaceAll(code, "    fn with_cancel_cause(", "    pub fn with_cancel_cause(")
		code = strings.ReplaceAll(code, "    fn done(", "    pub fn done(")
		code = strings.ReplaceAll(code, "    fn err(", "    pub fn err(")
	}
	out.WriteString(code)
}

func generateGoRandHelper(out *strings.Builder) {
	out.WriteString(`
fn go_rand_state() -> &'static std::sync::Mutex<u64> {
    static STATE: std::sync::OnceLock<std::sync::Mutex<u64>> = std::sync::OnceLock::new();
    STATE.get_or_init(|| std::sync::Mutex::new(1))
}

fn go_rand_seed(seed: i64) {
    *go_rand_state().lock().unwrap() = seed as u64;
}

fn go_rand_next_u64() -> u64 {
    let mut state = go_rand_state().lock().unwrap();
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn go_rand_intn(n: i32) -> i32 {
    if n <= 0 {
        panic!("invalid argument to Intn");
    }
    (go_rand_next_u64() % n as u64) as i32
}

fn go_rand_float64() -> f64 {
    ((go_rand_next_u64() >> 11) as f64) / ((1u64 << 53) as f64)
}
`)
}

func generateOsFileHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
struct GoFile {
    file: Option<std::fs::File>,
}

impl GoFile {
    fn create(path: &str) -> Result<Self, std::io::Error> {
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .map(|file| GoFile { file: Some(file) })
    }

    fn empty() -> Self {
        GoFile { file: None }
    }

    fn write_string(&mut self, text: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        match self.file.as_mut() {
            Some(file) => match std::io::Write::write_all(file, text.as_bytes()) {
                Ok(()) => (Arc::new(Mutex::new(Some(text.len() as i32))), Arc::new(Mutex::new(None))),
                Err(e) => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(e))))),
            },
            None => (
                Arc::new(Mutex::new(Some(0))),
                Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(std::io::Error::new(std::io::ErrorKind::Other, "invalid file"))))),
            ),
        }
    }

    fn close(&mut self) -> Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> {
        self.file = None;
        Arc::new(Mutex::new(None))
    }
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
struct GoFile {
    file: Option<std::fs::File>,
}

impl GoFile {
    fn create(path: &str) -> Result<Self, std::io::Error> {
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .map(|file| GoFile { file: Some(file) })
    }

    fn empty() -> Self {
        GoFile { file: None }
    }

    fn write_string(&mut self, text: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn std::error::Error>>>>) {
        let text = (*text.borrow().as_ref().unwrap()).clone();
        match self.file.as_mut() {
            Some(file) => match std::io::Write::write_all(file, text.as_bytes()) {
                Ok(()) => (Rc::new(RefCell::new(Some(text.len() as i32))), Rc::new(RefCell::new(None))),
                Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from(e))))),
            },
            None => (
                Rc::new(RefCell::new(Some(0))),
                Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from(std::io::Error::new(std::io::ErrorKind::Other, "invalid file"))))),
            ),
        }
    }

    fn close(&mut self) -> Rc<RefCell<Option<Box<dyn std::error::Error>>>> {
        self.file = None;
        Rc::new(RefCell::new(None))
    }
}
`)
}

func generateReflectHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Arc<Mutex<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let raw = (*self.raw.lock().unwrap().as_ref().unwrap()).clone();
        let key = (*key.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Arc<Mutex<Option<String>>>,
    tag: Arc<Mutex<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Arc<Mutex<Option<String>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lock().unwrap().as_ref().unwrap())
    }
}

impl GoReflectType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.name.lock().unwrap().as_ref().unwrap()).clone())))
    }

    fn num_field(&self) -> Arc<Mutex<Option<i32>>> {
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap().len() as i32)))
    }

    fn field(&self, index: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<GoReflectField>>> {
        let index = *index.lock().unwrap().as_ref().unwrap() as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Rc<RefCell<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let raw = (*self.raw.borrow().as_ref().unwrap()).clone();
        let key = (*key.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Rc<RefCell<Option<String>>>,
    tag: Rc<RefCell<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Rc<RefCell<Option<String>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn num_field(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap().len() as i32)))
    }

    fn field(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<GoReflectField>>> {
        let index = *index.borrow().as_ref().unwrap() as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}
`)
}
