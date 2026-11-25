package main

import (
	"path/filepath"
	"strings"
	"testing"
)

// cmp translates from source without opting in, so cmp.Compare runs Go's
// code instead of a stub with no body (DEV-180). The main package's calls
// are copies of that code, instantiated (stdlib_generic_inline.go).
func TestCmpTranslatesFromSourceByDefault(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "")
	if !shouldTranspileStdlibPackage("cmp") {
		t.Fatalf("cmp should translate from source by default")
	}
	dir := generateSourceIoProject(t, "example.com/cmp-default", `package main

import (
	"cmp"
	"fmt"
)

func main() {
	fmt.Println(cmp.Compare("a", "b"), cmp.Less(2, 1))
}
`)
	mainRS := mustReadFile(t, filepath.Join(dir, "main.rs"))
	for _, want := range []string{"fn __cmp__compare__string(", "fn __cmp__less__int(", "fn __cmp_is_na_n__int("} {
		if !strings.Contains(mainRS, want) {
			t.Errorf("the main crate should hold Go's Compare and Less, instantiated: missing %q\n%s", want, mainRS)
		}
	}
	stubsRS := mustReadFile(t, filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if strings.Contains(stubsRS, "compare bridge: generic stub function body has no implementation") {
		t.Errorf("cmp.Compare should not be a stub")
	}
}
