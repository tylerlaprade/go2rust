package main

import (
	"path/filepath"
	"strings"
	"testing"
)

// A struct type declared in a function and used as a map key gets the Ord a
// BTreeMap key needs, and a goroutine that takes one as a parameter gets a
// copy of the bare value rather than its cell (#325).
func TestFunctionLocalStructMapKeyHasOrd(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"fmt"
	"sync"
)

func main() {
	type key struct{ a, b int }
	grid := map[key]string{{1, 2}: "x"}
	var wg sync.WaitGroup
	wg.Add(1)
	go func(k key) {
		defer wg.Done()
		fmt.Println(k.a, grid[k])
	}(key{1, 2})
	wg.Wait()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "impl Ord for AnonymousStruct1") {
		t.Fatalf("the local struct key should implement Ord:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, ".take().unwrap(); __go_arg_value }") {
		t.Fatalf("the goroutine should get the bare struct value:\n%s", mainRS)
	}
}
