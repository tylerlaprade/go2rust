package main

import (
	"path/filepath"
	"strings"
	"testing"
)

// os.Stdout and os.Stderr handed on as values, here to bufio.NewWriter, write
// to the process's standard output and error, and every read of either is
// the same File (DEV-177).
func TestOsStdoutValueWritesToProcessOutput(t *testing.T) {
	dir := generateSourceIoProject(t, "example.com/os-stdout-writer", `package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	w := bufio.NewWriter(os.Stdout)
	defer w.Flush()
	fmt.Fprintln(w, "hello")
	e := bufio.NewWriter(os.Stderr)
	fmt.Fprintln(e, "oops")
	e.Flush()
}
`)
	stubsRS := mustReadFile(t, filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	for _, want := range []string{
		"pub __go_std_stream: u8,",
		"if self.__go_std_stream != 0 {",
		"let mut __out = std::io::stdout().lock();",
		"let mut __out = std::io::stderr().lock();",
		// One shared *File handle (DEV-181).
		"static __GO_FILE: ",
		`os_File { __go_std_stream: 1, __go_name: "/dev/stdout".to_string(), ..Default::default() }`,
		`os_File { __go_std_stream: 2, __go_name: "/dev/stderr".to_string(), ..Default::default() }`,
	} {
		if !strings.Contains(stubsRS, want) {
			t.Errorf("stub crate missing %q", want)
		}
	}
}
