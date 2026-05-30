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

	for _, want := range []string{"pub fn r_lock(&self)", "pub fn r_unlock(&self)"} {
		if !strings.Contains(helper, want) {
			t.Fatalf("GoRWMutex helper should export %q for cross-crate ForkLock users:\n%s", want, helper)
		}
	}
}
