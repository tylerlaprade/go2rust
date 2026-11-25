package main

import (
	"bytes"
	"go/printer"
	"path/filepath"
	"strings"
	"testing"
)

// loadStdlibInlineProgram loads a one-file main package and returns the
// loader and the package's files as Go source after the load's rewrites.
func loadStdlibInlineProgram(t *testing.T, source string) (*PackageLoader, string) {
	t.Helper()
	dir := t.TempDir()
	writeTestFile(t, filepath.Join(dir, "go.mod"), "module example.com/inline\n\ngo 1.24\n")
	writeTestFile(t, filepath.Join(dir, "main.go"), source)
	loader := NewPackageLoader(dir)
	if err := loader.LoadWithDependencies([]string{"."}); err != nil {
		t.Fatalf("LoadWithDependencies() error = %v", err)
	}
	var buf bytes.Buffer
	for _, file := range loader.mainPkg.Syntax {
		if err := printer.Fprint(&buf, loader.mainPkg.Fset, file); err != nil {
			t.Fatalf("printing the main package: %v", err)
		}
		buf.WriteString("\n")
	}
	return loader, buf.String()
}

func TestStdlibGenericsAreCopiedAndInstantiated(t *testing.T) {
	loader, src := loadStdlibInlineProgram(t, `package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
)

type Task struct {
	Name string
	Pri  int
}

func main() {
	tasks := []*Task{{"b", 2}, {"a", 1}}
	slices.SortFunc(tasks, func(x, y *Task) int { return cmp.Compare(x.Pri, y.Pri) })
	keys := slices.Sorted(maps.Keys(map[string]int{"z": 1}))
	fmt.Println(tasks[0].Name, slices.Index(keys, "z"))
}
`)
	for _, want := range []string{
		"func __slices_SortFunc__slice_ptr_Task__ptr_Task(",
		"func __slices_Sorted__string(",
		"func __maps_Keys__map_string_int__string__int(",
		"func __slices_Index__slice_string__string(",
		"func __cmp_Compare__int(",
		"__slices_SortFunc__slice_ptr_Task__ptr_Task(tasks, func(",
	} {
		if !strings.Contains(src, want) {
			t.Errorf("the main package should contain %q:\n%s", want, src)
		}
	}
	for _, unwanted := range []string{"slices.", "maps.", `"slices"`, `"maps"`} {
		if strings.Contains(src, unwanted) {
			t.Errorf("the main package should not use %s after the copy:\n%s", unwanted, src)
		}
	}
	// pdqsortCmpFunc calls bits.Len, so math/bits is translated with the
	// program.
	if loader.allPackages["math/bits"] == nil {
		t.Errorf("math/bits, which the copies import, should be collected; loaded: %v", loader.orderedAllPackagePaths())
	}
	if !strings.Contains(src, `"math/bits"`) || !strings.Contains(src, "bits.Len(") {
		t.Errorf("the copies should import math/bits by its name:\n%s", src)
	}
}

func TestStdlibGenericOverridesReplaceUnsafeAndRuntimeBodies(t *testing.T) {
	_, src := loadStdlibInlineProgram(t, `package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	s := slices.Insert([]int{1, 4}, 1, 2, 3)
	m := maps.Clone(map[string]int{"a": 1})
	fmt.Println(s, m)
}
`)
	if !strings.Contains(src, "func __slices_overlaps__int(") || !strings.Contains(src, "&a[i] == &b[0]") {
		t.Errorf("slices.overlaps should be the override that compares element addresses:\n%s", src)
	}
	if strings.Contains(src, "unsafe") {
		t.Errorf("no copy should use unsafe:\n%s", src)
	}
	if !strings.Contains(src, "func __maps_Clone__map_string_int__string__int(") || strings.Contains(src, "__maps_clone") {
		t.Errorf("maps.Clone should be the override that ranges over the map, not a call of the runtime's clone:\n%s", src)
	}
}

func TestStdlibGenericCopiesImportUnderAFreeName(t *testing.T) {
	_, src := loadStdlibInlineProgram(t, `package main

import (
	"fmt"
	"slices"
)

var bits = 3

func main() {
	s := []int{3, 1, 2}
	slices.Sort(s)
	fmt.Println(s, bits)
}
`)
	if !strings.Contains(src, `__go2rust_bits "math/bits"`) || !strings.Contains(src, "__go2rust_bits.Len(") {
		t.Errorf("math/bits should be imported under a name the package does not declare:\n%s", src)
	}
}

func TestStdlibGenericsStayStubsWhenTheCopiesCannotMonomorphize(t *testing.T) {
	// A type declared inside a function cannot be named at package level,
	// so the instantiation with it cannot be written.
	_, src := loadStdlibInlineProgram(t, `package main

import (
	"fmt"
	"slices"
)

func main() {
	type pair struct{ k, v int }
	ps := []pair{{2, 1}, {1, 2}}
	fmt.Println(slices.Contains(ps, pair{1, 2}))
}
`)
	if !strings.Contains(src, "slices.Contains(ps,") || strings.Contains(src, "__slices_") {
		t.Errorf("the program should be left as it was:\n%s", src)
	}
}

func TestStdlibGenericsKeptGenericWithGenericRust(t *testing.T) {
	t.Setenv("GO2RUST_GENERIC_RUST", "1")
	_, src := loadStdlibInlineProgram(t, `package main

import (
	"fmt"
	"slices"
)

func main() {
	fmt.Println(slices.Index([]int{1, 2}, 2))
}
`)
	if !strings.Contains(src, "slices.Index(") || strings.Contains(src, "__slices_") {
		t.Errorf("GO2RUST_GENERIC_RUST=1 should leave the calls as they are:\n%s", src)
	}
}
