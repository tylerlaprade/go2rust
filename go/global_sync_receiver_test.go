package main

import (
	"strings"
	"testing"
)

// A method of a package-level sync.WaitGroup runs on a clone of its handle,
// so wg.Wait() does not hold the variable's lock while goroutines call
// wg.Done().
func TestPackageLevelWaitGroupMethodsRunOnSharedClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var wg sync.WaitGroup

func main() {
	wg.Add(1)
	go func() { defer wg.Done() }()
	wg.Wait()
}
`)
	if !strings.Contains(rust, ".as_ref().unwrap()).clone(); __sync_value }.wait();") {
		t.Errorf("wg.Wait() should run on a clone of the handle:\n%s", rust)
	}
	if strings.Contains(rust, ".as_mut().unwrap()).wait()") {
		t.Errorf("wg.Wait() must not hold the variable's lock:\n%s", rust)
	}
}

// A go statement copies an interface{} value it reads through the value
// registry: Box<dyn Any> has no Clone.
func TestGoStatementCopiesCapturedEmptyInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var wg sync.WaitGroup

func launch(arg any) {
	go arg.(func())()
}

func main() {
	wg.Add(1)
	launch(func() { wg.Done() })
	wg.Wait()
}
`)
	if !strings.Contains(rust, "let arg_thread = go_any_handle_copy(&arg);") {
		t.Errorf("the interface{} capture should go through go_any_handle_copy:\n%s", rust)
	}
}
