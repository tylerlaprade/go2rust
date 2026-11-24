package main

import (
	"errors"
	"fmt"
	"sync"
)

func logf(format string, args ...any) {
	fmt.Printf("[log] "+format+"\n", args...)
}

func count(args ...any) int {
	n := 0
	for _, a := range args {
		if a == nil {
			n++
		}
	}
	return n
}

func kind(v any) string {
	switch v.(type) {
	case nil:
		return "nil"
	case int:
		return "int"
	default:
		return "other"
	}
}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		var gerr error
		logf("goroutine err=%v", gerr)
	}()
	wg.Wait()
	var err error
	logf("err=%v", err)
	err = errors.New("boom")
	logf("err=%v", err)
	var e any
	xs := []any{nil, 1, e}
	xs = append(xs, nil, err)
	fmt.Println(len(xs), xs[0] == nil, xs[1] == nil, count(xs...), count(nil, 2))
	fmt.Println(xs...)
	for _, x := range xs {
		fmt.Print(kind(x), " ")
	}
	fmt.Println()
	first := xs[0]
	fmt.Println(first == nil, kind(xs[2]))
	m := map[string]any{"a": nil, "b": 1}
	fmt.Println(m["a"] == nil, m["b"] == nil, m["zz"] == nil)
	var ys []any
	ys = append(ys, xs...)
	fmt.Println(len(ys), ys[0] == nil)
	fmt.Printf("%v %v\n", nil, xs[0])
}
