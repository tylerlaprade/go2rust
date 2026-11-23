package main

import (
	"path/filepath"
	"strings"
	"testing"
)

func TestPointerMethodCallingPromotedMutatorTakesMutSelf(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Basic struct{ balance float64 }

func (b *Basic) Deposit(a float64) { b.balance += a }

type Savings struct {
	Basic
	rate float64
}

func (s *Savings) AddInterest() { s.Deposit(s.balance * s.rate) }
`)

	if !strings.Contains(rust, "pub fn add_interest(&mut self)") {
		t.Fatalf("a method calling a promoted method that changes the receiver should take &mut self:\n%s", rust)
	}
}

func TestSelectReceiveFiresOnClosedChannel(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func wait(done chan struct{}, results chan int) (int, bool) {
loop:
	for {
		select {
		case <-done:
			break loop
		case v, ok := <-results:
			return v, ok
		}
	}
	return 0, false
}
`)

	for _, want := range []string{
		"if let Some(_) = done.try_recv_select() {",
		"if let Some(__select_recv) = results.try_recv_select() {",
		"let v = match __select_recv { Some(__v) => __v, None => Default::default() };",
		"break 'r#loop;",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("a select receive should be ready on a closed channel and a labeled break should end its statement, missing %q:\n%s", want, rust)
		}
	}
}

func TestTimeSleepTakesConstantAndValueDurations(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "time"

func pause(d time.Duration, n int) {
	time.Sleep(time.Millisecond)
	time.Sleep(d)
	time.Sleep(time.Duration(n) * time.Millisecond)
}
`)

	for _, want := range []string{
		"std::thread::sleep(std::time::Duration::from_nanos(1000000))",
		"std::thread::sleep({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v })",
		"std::time::Duration::from_millis(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as u64)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("time.Sleep should take a Duration as it is, missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Duration::from_millis(1) as u64") {
		t.Fatalf("a Duration should not be cast to u64:\n%s", rust)
	}
}

func TestGeneratedCargoTomlLetsIntegersWrap(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), "module example.com/main\n\ngo 1.22\n")
	writeTestFile(t, filepath.Join(tempDir, "main.go"), "package main\n\nfunc main() {}\n")
	if err := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")}).Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	cargo := mustReadFile(t, filepath.Join(tempDir, "Cargo.toml"))
	if !strings.Contains(cargo, "[profile.dev]\noverflow-checks = false\n") {
		t.Fatalf("Go integer arithmetic wraps, so a debug build should not check for overflow:\n%s", cargo)
	}
}

func TestUntypedIntegerStubConstantTooBigForI32(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), "module example.com/main\n\ngo 1.22\n")
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"fmt"
	"math"
)

func main() {
	var a int32 = math.MaxInt32
	var b int64 = math.MaxInt64
	fmt.Println(a, b)
}
`)
	if err := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")}).Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	rust := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(rust, "pub const MAX_INT64: i64 = 9223372036854775807;") || !strings.Contains(rust, "pub const MAX_INT32: i64 = 2147483647;") {
		t.Fatalf("an untyped constant should get a Rust integer that holds it:\n%s", rust)
	}
}
