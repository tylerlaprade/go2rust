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

	// HashMap
	if it.needs["HashMap"] {
		imports = append(imports, "use std::collections::HashMap;")
	}

	// fmt imports
	var fmtImports []string
	if it.needs["fmt::self"] {
		fmtImports = append(fmtImports, "self")
	}
	if it.needs["Display"] {
		fmtImports = append(fmtImports, "Display")
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

	return result.String()
}

func generateAnyFormatter(out *strings.Builder) {
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
