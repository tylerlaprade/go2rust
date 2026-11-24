package main

import (
	"strings"
	"testing"
)

// A chan any carries the Option inside an interface{} handle, like a
// channel of a named interface: sends box the value, nil is None, and a
// receive fills a handle of its own (DEV-139).
func TestChanAnyCarriesInterfaceHandleOption(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	ch := make(chan any, 3)
	ch <- "old"
	ch <- nil
	ch <- 42
	fmt.Println((<-ch).(string))
	v, ok := <-ch
	m := map[string]any{}
	m["k"] = <-ch
	fmt.Println(v, ok, m)
}
`)
	for _, want := range []string{
		"GoChannel::<Option<Box<dyn Any",
		"None::<Box<dyn Any",
		".take(); __iface_value }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, `ch.send("old".to_string())`) {
		t.Errorf("a string sent on chan any should be boxed:\n%s", rust)
	}
	if strings.Contains(rust, "Some(Rc::new(RefCell::new(ch.recv()") || strings.Contains(rust, "Some(Arc::new(Mutex::new(ch.recv()") {
		t.Errorf("a received handle stored in a map should not be wrapped twice:\n%s", rust)
	}
}
