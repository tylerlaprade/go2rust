package main

import (
	"path/filepath"
	"strings"
	"testing"
)

// cmp translates from source without opting in, so cmp.Compare runs Go's
// code instead of a stub with no body (DEV-180).
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
	cmpRS := readCrateSources(t, filepath.Join(dir, "vendor", "cmp"))
	if !strings.Contains(cmpRS, "pub fn Compare<") && !strings.Contains(cmpRS, "pub fn compare<") {
		t.Errorf("the cmp crate should hold Go's Compare:\n%s", cmpRS)
	}
	stubsRS := mustReadFile(t, filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if strings.Contains(stubsRS, "compare bridge: generic stub function body has no implementation") {
		t.Errorf("cmp.Compare should not be a stub")
	}
}
