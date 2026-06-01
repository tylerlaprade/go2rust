package main

import (
	"fmt"
	"sync"
)

// Source-transpiling sync is required before sync.Map bridge methods can
// retire. Today the generated internal/sync crate fails to compile on the
// hashtriemap generic implementation and a Mutex name collision.
func main() {
	var m sync.Map
	m.Store("key", "value")
	value, ok := m.Load("key")
	fmt.Println(value, ok)
}
