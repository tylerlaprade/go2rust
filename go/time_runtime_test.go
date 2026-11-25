package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func timeRuntimeHookBody(t *testing.T, decl string) string {
	t.Helper()
	file, err := parser.ParseFile(token.NewFileSet(), "time.go", "package time\n\ntype Timer struct{ C <-chan int; initTimer bool }\n\n"+decl+"\n", 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	for _, d := range file.Decls {
		if fn, ok := d.(*ast.FuncDecl); ok {
			var out strings.Builder
			if !writeTimeRuntimeHookBody(&out, fn, "    ") {
				t.Fatalf("%s should be one of time's runtime hooks", fn.Name.Name)
			}
			return out.String()
		}
	}
	t.Fatalf("no function in %q", decl)
	return ""
}

func TestTimeRuntimeHooksReadTheClock(t *testing.T) {
	if body := timeRuntimeHookBody(t, "func runtimeNano() int64"); !strings.Contains(body, "go_time_runtime_nano()") {
		t.Fatalf("runtimeNano should read the monotonic clock:\n%s", body)
	}
	body := timeRuntimeHookBody(t, "func runtimeNow() (sec int64, nsec int32, mono int64)")
	for _, want := range []string{"std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)", "go_time_runtime_nano()"} {
		if !strings.Contains(body, want) {
			t.Fatalf("runtimeNow should read the wall and monotonic clocks; missing %q:\n%s", want, body)
		}
	}
	if body := timeRuntimeHookBody(t, "func Sleep(d Duration)"); !strings.Contains(body, "std::thread::sleep(std::time::Duration::from_nanos(__nanos as u64))") {
		t.Fatalf("Sleep should sleep for the Duration's nanoseconds:\n%s", body)
	}
}

func TestTimeRuntimeTimerHooksUseTheTimerThread(t *testing.T) {
	prev := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.hasGoroutines = true
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prev)

	body := timeRuntimeHookBody(t, "func newTimer(when, period int64, f func(any, uintptr, int64), arg any, cp unsafe.Pointer) *Timer")
	for _, want := range []string{
		"init_timer: Arc::new(Mutex::new(Some(true)))",
		"go_time_timer_start(&__owner, __when, __period,",
		"downcast_ref::<GoChannel<Time>>()",
	} {
		if !strings.Contains(body, want) {
			t.Fatalf("newTimer should start a timer on the timer thread; missing %q:\n%s", want, body)
		}
	}
	if body := timeRuntimeHookBody(t, "func stopTimer(*Timer) bool"); !strings.Contains(body, "go_time_timer_stop(&__owner)") {
		t.Fatalf("stopTimer should stop the timer:\n%s", body)
	}
	if body := timeRuntimeHookBody(t, "func resetTimer(t *Timer, when, period int64) bool"); !strings.Contains(body, "go_time_timer_reset(&__owner, ") {
		t.Fatalf("resetTimer should reset the timer:\n%s", body)
	}
}
