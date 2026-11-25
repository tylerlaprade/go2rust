// A go statement whose call reads an interface{} value: the goroutine gets
// a copy of it, as time.AfterFunc's goFunc does with "go arg.(func())()".
package main

import (
	"fmt"
	"sync"
)

var wg sync.WaitGroup

// launch starts the function held in an interface{} value, as time's goFunc
// does for AfterFunc.
func launch(arg any) {
	go arg.(func())()
}

func describe(v any, label string) {
	go func() {
		defer wg.Done()
		fmt.Println(label, v)
	}()
}

func main() {
	var mu sync.Mutex
	var got []string
	for _, name := range []string{"a", "b"} {
		wg.Add(1)
		n := name
		launch(func() {
			defer wg.Done()
			mu.Lock()
			got = append(got, n)
			mu.Unlock()
		})
		wg.Wait()
	}
	fmt.Println(got)
	var v any = 42
	wg.Add(1)
	go func(x any) {
		defer wg.Done()
		fmt.Println("arg", x)
	}(v)
	wg.Wait()
	wg.Add(1)
	describe("text", "captured")
	wg.Wait()
}
