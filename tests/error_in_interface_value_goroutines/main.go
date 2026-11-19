package main

import (
	"errors"
	"fmt"
)

// kind tells values apart with a type switch, the way a handler does.
func kind(v any) string {
	switch x := v.(type) {
	case int:
		return fmt.Sprintf("int %d", x)
	case string:
		return "string " + x
	case error:
		return "error " + x.Error()
	default:
		return "other"
	}
}

func main() {
	// A goroutine makes every interface value one that can cross threads.
	// An error in an interface, and a slice that mixes errors with other
	// values, have to work there as they do without one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	plain := errors.New("bad")
	wrapped := fmt.Errorf("wrapped: %w", plain)

	var boxed any = plain
	back, ok := boxed.(error)
	fmt.Println(ok, back == plain, back)

	items := []interface{}{plain, "text", 3, wrapped}
	for _, item := range items {
		fmt.Println(item)
	}
	fmt.Println(len(items))

	others := []interface{}{"a", 1, 2.5, true}
	for _, other := range others {
		fmt.Println(other)
	}

	fmt.Println(kind(plain), "|", kind(wrapped), "|", kind(3), "|", kind("s"), "|", kind(2.5))
}
