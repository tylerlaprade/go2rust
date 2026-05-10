package main

import (
	"fmt"
	"time"
)

func main() {
	delay := 0 * time.Nanosecond
	delay = 30 * time.Second
	fmt.Println(delay == 30*time.Second)
}
