package main

import (
	"fmt"
	"time"
)

func sink(args ...any) int {
	return len(args)
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var value any = time.Unix(1, 2)
	fmt.Println(sink(value))
}
