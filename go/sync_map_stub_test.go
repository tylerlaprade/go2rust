package main

import (
	"strings"
	"testing"
)

func TestSyncMapStubStoresGoValues(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"sync"
)

func main() {
	var m sync.Map
	i := 3
	m.Store(i, "three")
	m.Store("nil", nil)
	v, ok := m.Load(3)
	fmt.Println(v, ok)
}
`)
	for _, want := range []string{
		"pub struct sync_Map {",
		"fn go_sync_map_key<K: Any + std::fmt::Debug>(key: &K) -> GoSyncMapKey {",
		".load((3 as i64))",
		"None::<Box<dyn Any>>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sync.Map should be a real map whose methods get Go values; missing %q:\n%s", want, rust)
		}
	}
	for _, bad := range []string{
		"sync_Map.load bridge",
		".store(i.clone()",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("sync.Map should not panic or get the variable's cell; found %q:\n%s", bad, rust)
		}
	}
}
