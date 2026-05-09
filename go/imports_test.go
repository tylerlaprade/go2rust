package main

import "testing"

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
