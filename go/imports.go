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
		imports = append(imports, "use std::error::Error;")
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
	needsFormatMap      bool
	needsFormatSlice    bool
	needsFormatAny      bool
	needsFormatAnySlice bool
	needsGoChannel      bool
	needsWaitGroup      bool
	needsGoMutex        bool
}

// GenerateHelpers returns the helper function definitions
func (ht *HelperTracker) GenerateHelpers() string {
	var result strings.Builder

	if ht.needsFormatMap {
		generateMapFormatter(&result)
	}

	if ht.needsFormatSlice {
		generateSliceFormatter(&result)
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

	return result.String()
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
	out.WriteString(`
struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn send(&self, val: T) {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            let _ = tx.send(val);
        }
    }

    fn try_send(&self, val: T) -> bool {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            tx.try_send(val).is_ok()
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        self.rx.lock().unwrap().recv().ok()
    }

    fn try_recv(&self) -> Option<T> {
        self.rx.lock().unwrap().try_recv().ok()
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}
`)
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
`)
}

func generateGoMutexHelper(out *strings.Builder) {
	out.WriteString(`
struct GoMutex {
    inner: std::sync::Mutex<()>,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Mutex::new(()),
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
        Self::new()
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}
`)
}
