package main

import (
	"strings"
	"testing"
)

const packageGlobalMapSource = `package main

import "fmt"

var registry = map[string]int{"a": 1}

func add(m map[string]int, k string) { m[k] = len(m) }

func main() {
	go func() {}()
	r := registry
	add(registry, "c")
	registry = map[string]int{}
	registry = nil
	fmt.Println(len(r))
}
`

// A package-level map variable is the map's handle, as a local one is: a
// copy of it and an argument made from it share the map, and assigning a map
// to it takes that map's handle (#328).
func TestPackageGlobalMapIsTheMapHandle(t *testing.T) {
	rust := transpileTypedRegression(t, strings.Replace(packageGlobalMapSource, "\tgo func() {}()\n", "", 1))
	for _, want := range []string{
		"static registry_1: GoGlobalMap<BTreeMap<String, Rc<RefCell<Option<i64>>>>> = GoGlobalMap::new();",
		"fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> {\n        self.handle().clone()",
		"let mut r = registry_1.clone();",
		"add(registry_1.clone(), ",
		"{ let new_val = Rc::new(RefCell::new(None)); registry_1.set(new_val); }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// With goroutines, a package-level map variable is a GoGlobalMapSync, whose
// handle a copy shares and an assignment replaces.
func TestConcurrentPackageGlobalMapIsTheMapHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, packageGlobalMapSource)
	for _, want := range []string{
		"static registry_1: std::sync::LazyLock<GoGlobalMapSync<BTreeMap<String, Arc<Mutex<Option<i64>>>>>> = std::sync::LazyLock::new(GoGlobalMapSync::new);",
		"fn lock(&self) -> std::sync::LockResult<GoGlobalMapGuard<T>> {",
		"let mut r = registry_1.clone();",
		"{ let new_val = Arc::new(Mutex::new(None)); registry_1.set(new_val); }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}
