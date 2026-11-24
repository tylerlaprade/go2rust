package main

import (
	"errors"
	"fmt"
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
}
