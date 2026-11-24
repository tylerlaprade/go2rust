package main

import (
	"path/filepath"
	"strings"
	"testing"
)

func generateSourceIoProject(t *testing.T, module string, source string) string {
	t.Helper()
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), "module "+module+"\n\ngo 1.24\n")
	writeTestFile(t, filepath.Join(tempDir, "main.go"), source)
	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	return tempDir
}

// exec.Cmd's Stdout, Stderr and Stdin are io interfaces. With io translated
// from source the stub crate cannot name them, so the fields hold stub-side
// closures that call the io trait methods, and Run feeds Stdin to the
// process and writes its output back through them.
func TestExecCmdIoFieldsBridgeSourceIoInterfaces(t *testing.T) {
	dir := generateSourceIoProject(t, "example.com/exec-io", `package main

import (
	"bytes"
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	var out bytes.Buffer
	cmd := exec.Command("sort")
	cmd.Stdin = strings.NewReader("b\na\n")
	cmd.Stdout = &out
	cmd.Stderr = &out
	fmt.Println(cmd.Run(), out.String())
}
`)
	stubsRS := mustReadFile(t, filepath.Join(dir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	for _, want := range []string{
		"pub struct GoStubWriter",
		"pub struct GoStubReader",
		"pub stdout: Arc<Mutex<Option<GoStubWriter>>>",
		"pub stderr: Arc<Mutex<Option<GoStubWriter>>>",
		"pub stdin: Arc<Mutex<Option<GoStubReader>>>",
		"let data = stdin.__go_read_all();",
		"stdout.__go_write_bytes(&output.stdout);",
		"Self::__go_error(Self::__go_exit_message(&output.status))",
	} {
		if !strings.Contains(stubsRS, want) {
			t.Errorf("stub crate missing %q", want)
		}
	}
	mainRS := mustReadFile(t, filepath.Join(dir, "main.rs"))
	for _, want := range []string{
		".stdout.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubWriter::__go_from_fn(",
		"move |__data: &[u8]| { let _ = __io.lock().unwrap().write(Arc::new(Mutex::new(Some(Vec::from(__data))))); }",
		".stdin.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubReader::__go_from_fn(",
		"let (__n, __err) = __io.lock().unwrap().read(__buf.clone());",
	} {
		if !strings.Contains(mainRS, want) {
			t.Errorf("main.rs missing %q:\n%s", want, mainRS)
		}
	}
}

// flag, translated from source, writes its usage to os.Stderr through an
// io.Writer. The program never imports os itself, so the io crate's
// impls for the bridged os.File must come from flag importing os.
func TestSourceStdlibImporterGivesStubFileIoImpls(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "flag")
	dir := generateSourceIoProject(t, "example.com/flag-io", `package main

import (
	"flag"
	"fmt"
)

func main() {
	name := flag.String("name", "World", "a name")
	flag.Parse()
	fmt.Println(*name)
}
`)
	ioRS := mustReadFile(t, filepath.Join(dir, "vendor", "io", "mod.rs"))
	if !strings.Contains(ioRS, "impl Writer for os_File") {
		t.Fatalf("io crate should implement Writer for the bridged os.File:\n%s", ioRS)
	}
}

// A range variable over a [][]byte is held bare, as the elements are; a
// []byte parameter takes it in a cell of its own, as bufio's
// collectFragments passes each buffer to strings.Builder.Write.
func TestBareRangeSliceArgumentGetsItsOwnCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type sink struct{ n int }

func (s *sink) Write(p []byte) { s.n += len(p) }

func main() {
	var s sink
	chunks := [][]byte{[]byte("ab"), []byte("c")}
	for _, c := range chunks {
		s.Write(c)
	}
	fmt.Println(s.n)
}
`)
	if !strings.Contains(rust, ".write(Rc::new(RefCell::new(Some(c.clone()))))") {
		t.Fatalf("range variable should be passed in a cell of its own:\n%s", rust)
	}
}
