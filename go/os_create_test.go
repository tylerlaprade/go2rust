package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// os.Create returns the os stub's File, which writes through to the file
// it opened, so the File's methods and fmt.Fprintf compile and write to
// disk. It returned a separate GoFile, and a method call on it was written
// for os_File (DEV-150).
func TestOsCreateReturnsWriteThroughOsFile(t *testing.T) {
	dir := generateSourceIoProject(t, "example.com/os-create", `package main

import (
	"fmt"
	"os"
)

func main() {
	f, err := os.Create("out.txt")
	if err != nil {
		return
	}
	fmt.Fprintf(f, "n=%d\n", 1)
	f.WriteString("x")
	f.Close()
	data, _ := os.ReadFile("out.txt")
	fmt.Println(string(data))
}
`)
	rust := mustReadFile(t, filepath.Join(dir, "main.rs"))
	if stubs, err := os.ReadFile(filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs")); err == nil {
		rust += string(stubs)
	}
	if strings.Contains(rust, "GoFile") {
		t.Errorf("os.Create should not use the GoFile helper:\n%s", rust)
	}
	for _, want := range []string{
		"os::create(",
		"pub fn create<T0: GoStringArg>(_arg0: T0)",
		"match std::fs::File::create(&path) {",
		"os_File { __go_disk: std::sync::Arc::new(std::sync::Mutex::new(Some(disk))), ..Default::default() }",
		`path_error("open", path, err)`,
		"if let Some(__disk) = self.__go_disk.lock().unwrap().as_mut() {",
		"self.__go_disk.lock().unwrap().take();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
