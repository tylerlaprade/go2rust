package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// os.File's Read reads the file's bytes or the process's standard input,
// and its end of input is io.EOF, the value bufio, io.ReadAll and a direct
// caller compare with (DEV-148).
func TestOsFileReadServesFileAndStdin(t *testing.T) {
	dir := generateSourceIoProject(t, "example.com/os-file-read", `package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

func main() {
	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		fmt.Println(sc.Text())
	}
	f, err := os.Open("main.go")
	if err != nil {
		return
	}
	data, err := io.ReadAll(f)
	fmt.Println(len(data), err)
}
`)
	stubsRS := mustReadFile(t, filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	for _, want := range []string{
		"pub struct GoStubEOF;",
		"pub __go_read_pos: std::sync::Arc<std::sync::atomic::AtomicUsize>,",
		"pub __go_stdin: bool,",
		"std::io::Read::read(&mut std::io::stdin(), __dst)",
		// n, err := f.Read(buf) binds n as an int, as for any stub call.
		"pub fn read<T0: 'static>(&self, arg0: T0) -> (i64, ",
		"pub fn __go_register_io_eof(",
	} {
		if !strings.Contains(stubsRS, want) {
			t.Errorf("stub crate missing %q", want)
		}
	}
	if !strings.Contains(stubsRS, "os_File { __go_stdin: true, ..Default::default() }") {
		t.Errorf("os.Stdin should be the stdin file:\n%s", stubsRS)
	}
	ioLib := mustReadFile(t, filepath.Join(dir, "vendor", "io", "lib.rs"))
	if !strings.Contains(ioLib, "go2rust_stdlib_stubs::__go_register_io_eof((*EOF.lock().unwrap()).clone());") {
		t.Errorf("io's init should hand io.EOF to the stub crate:\n%s", ioLib)
	}
	ioRS := readCrateSources(t, filepath.Join(dir, "vendor", "io"))
	for _, want := range []string{
		"downcast_ref::<go2rust_stdlib_stubs::GoStubEOF>().is_some()",
		// io.ReadAll reads into b[len(b):cap(b)], which reaches capacity
		// an append reserved and never stored.
		"reslice3_zeroed(",
	} {
		if !strings.Contains(ioRS, want) {
			t.Errorf("io crate missing %q", want)
		}
	}
}

func readCrateSources(t *testing.T, dir string) string {
	t.Helper()
	entries, err := os.ReadDir(dir)
	if err != nil {
		t.Fatalf("ReadDir(%s) error = %v", dir, err)
	}
	var all strings.Builder
	for _, entry := range entries {
		if strings.HasSuffix(entry.Name(), ".rs") {
			all.WriteString(mustReadFile(t, filepath.Join(dir, entry.Name())))
		}
	}
	return all.String()
}

// Slicing a []byte up to its capacity fills the reserved, never-stored
// elements with zeros first, as Go's are; a slice of structs keeps
// reslice3.
func TestSliceToCapacityZeroFillsBasicElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type pair struct{ a, b int }

func main() {
	b := make([]byte, 0, 8)
	b = append(b, 'x')
	tail := b[len(b):cap(b)]
	ps := make([]pair, 1, 4)
	more := ps[1:3:4]
	fmt.Println(len(tail), tail[0], len(more))
}
`)
	if !strings.Contains(rust, ".reslice3_zeroed(__low, __high, __max)") {
		t.Errorf("[]byte slice to capacity should zero-fill:\n%s", rust)
	}
	if !strings.Contains(rust, ".reslice3(__low, __high, __max)") {
		t.Errorf("[]pair slice should keep reslice3:\n%s", rust)
	}
}
