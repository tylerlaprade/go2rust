package main

import (
	"fmt"
	"time"
)

func sink(args ...any) int {
	return len(args)
}

func main() {
	var value any = time.Unix(1, 2)
	fmt.Println(sink(value))
}
