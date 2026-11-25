package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// os.ErrNotExist, ErrExist and ErrPermission are fs's errors: one shared
// instance each, with Go's text, which errors.Is matches (DEV-184).
func TestOsSentinelErrorsAreFsSentinels(t *testing.T) {
	dir := generateSourceIoProject(t, "example.com/os-sentinels", `package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

func main() {
	err := fmt.Errorf("x: %w", os.ErrNotExist)
	fmt.Println(errors.Is(err, fs.ErrNotExist), os.ErrExist, os.ErrPermission)
}
`)
	// Without a translated package the stubs are written into main.rs.
	mainRS := mustReadFile(t, filepath.Join(dir, "main.rs"))
	stubsRS := mainRS
	if data, err := os.ReadFile(filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs")); err == nil {
		stubsRS += string(data)
	}
	for _, want := range []string{
		`std::io::Error::new(std::io::ErrorKind::NotFound, "file does not exist")`,
		`std::io::Error::new(std::io::ErrorKind::AlreadyExists, "file already exists")`,
		`std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied")`,
		"pub fn __go_fs_sentinel(index: usize)",
		"crate::__go_fs_sentinel(0)",
		"crate::__go_fs_sentinel(1)",
		"crate::__go_fs_sentinel(2)",
	} {
		if !strings.Contains(stubsRS, want) {
			t.Errorf("stub crate missing %q", want)
		}
	}
	if !strings.Contains(mainRS, "fn go_native_error_is_sentinel(e: &std::io::Error) -> bool {") {
		t.Errorf("errors.Is should recognize the sentinels:\n%s", mainRS)
	}
}
