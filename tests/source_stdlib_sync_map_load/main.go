package main

import (
	"fmt"
	"sync"
)

// Source-transpiling sync and internal/race keep sync.Map on the real Go
// stdlib source path instead of the shared bridge stubs.
func main() {
	var m sync.Map
	m.Store("key", "value")
	value, ok := m.Load("key")
	fmt.Println(value, ok)
}
