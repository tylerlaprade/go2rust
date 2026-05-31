package main

import (
	"strings"
	"testing"
)

func TestFormatSliceImportNamesIncludeWrappedFormatter(t *testing.T) {
	ht := &HelperTracker{needsFormatSlice: true}
	names := ht.ImportNames()
	seen := make(map[string]bool)
	for _, name := range names {
		seen[name] = true
	}

	for _, name := range []string{"format_slice", "format_slice_values", "format_slice_wrapped"} {
		if !seen[name] {
			t.Fatalf("ImportNames() missing %q in %v", name, names)
		}
	}
}

func TestGoRWMutexHelperExportsLockMethods(t *testing.T) {
	ht := &HelperTracker{needsGoRWMutex: true}
	helper := ht.GenerateHelperModule()

	for _, want := range []string{"pub struct GoRWMutex", "pub fn r_lock(&self)", "pub fn r_unlock(&self)"} {
		if !strings.Contains(helper, want) {
			t.Fatalf("GoRWMutex helper should export %q for cross-crate ForkLock users:\n%s", want, helper)
		}
	}
}

func TestGoPtrKeyHelperModuleImportsWrapperTypes(t *testing.T) {
	prevConcurrencyDetector := GetConcurrencyDetector()
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
	})

	SetConcurrencyDetector(nil)
	helper := (&HelperTracker{needsGoPtrKey: true}).GenerateHelperModule()
	for _, want := range []string{"use std::rc::{Rc};", "use std::cell::{RefCell};"} {
		if !strings.Contains(helper, want) {
			t.Fatalf("single-threaded GoLocalPtrKey helper should import %q:\n%s", want, helper)
		}
	}

	cd := NewConcurrencyDetector()
	cd.hasChannels = true
	SetConcurrencyDetector(cd)
	helper = (&HelperTracker{needsGoPtrKey: true}).GenerateHelperModule()
	if !strings.Contains(helper, "use std::sync::{Arc, Mutex};") {
		t.Fatalf("concurrent GoLocalPtrKey helper should import Arc and Mutex:\n%s", helper)
	}
}
