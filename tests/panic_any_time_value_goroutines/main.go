package main

import (
	"fmt"
	"time"
)

func sink(args ...any) int {
	if len(args) != 1 {
		panic("unexpected argument count")
	}
	value, ok := args[0].(time.Time)
	if !ok || value.UnixNano() != 1_000_000_002 {
		panic("unexpected time value")
	}
	return len(args)
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var value any = time.Unix(1, 2)
	fmt.Println(sink(value))
}
