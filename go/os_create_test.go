package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"syscall"
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
		"match std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path) {",
		"os_File { __go_disk: std::sync::Arc::new(std::sync::Mutex::new(Some(disk))), __go_name: path.clone(), ..Default::default() }",
		"format!(\"{} {}: file already closed\", op, self.__go_name)",
		"if self.__go_handle_closed.swap(true, std::sync::atomic::Ordering::SeqCst) {",
		`path_error("open", path, err)`,
		// Read takes the disk lock before it checks the closed flag.
		"let mut __disk = self.__go_disk.lock().unwrap();\n            if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {",
		"} else if let Some(__disk) = __disk.as_mut() {",
		"if let Some(__file) = __disk.take() {",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// os.OpenFile opens the file with OpenOptions set from Go's flags, and
// os.IsNotExist looks at the I/O error's kind through an *fs.PathError.
// Both were generic stubs that panicked (DEV-150).
func TestOsOpenFileAndIsNotExistUseTheDiskFile(t *testing.T) {
	dir := generateSourceIoProject(t, "example.com/os-open-file", `package main

import (
	"fmt"
	"os"
)

func main() {
	f, err := os.OpenFile("log.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return
	}
	f.WriteString("line\n")
	f.Close()
	_, err = os.OpenFile("missing.txt", os.O_RDONLY|os.O_SYNC, 0)
	fmt.Println(os.IsNotExist(err))
}
`)
	rust := mustReadFile(t, filepath.Join(dir, "main.rs"))
	if stubs, err := os.ReadFile(filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs")); err == nil {
		rust += string(stubs)
	}
	for _, want := range []string{
		"pub fn open_file<T0: GoStringArg, T1: 'static, T2: 'static>(_arg0: T0, _arg1: T1, _arg2: T2)",
		"let mut options = std::fs::OpenOptions::new();",
		"options.append(true);",
		"pub fn is_not_exist<T0: 'static>(_arg0: T0) -> bool {",
		"return io.kind() == std::io::ErrorKind::NotFound;",
		// OpenFile's error is a *PathError; IsNotExist classifies its cause.
		"if let Some(path_error) = err.downcast_ref::<fs_PathError>() {",
		"return inner.downcast_ref::<std::io::Error>().map_or(false, |io| io.kind() == std::io::ErrorKind::NotFound);",
		// O_SYNC has no OpenOptions setter and goes in with custom_flags.
		fmt.Sprintf("let mut native = flag & %d;", syscall.O_SYNC),
		"std::os::unix::fs::OpenOptionsExt::custom_flags(&mut options, native as i32);",
		// Close reports a failed close(2) rather than dropping the File.
		"if unsafe { close(__fd) } != 0 {",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "open_file bridge: generic stub") || strings.Contains(rust, "is_not_exist bridge: generic stub") {
		t.Errorf("os.OpenFile and os.IsNotExist should not be generic stubs:\n%s", rust)
	}
}
