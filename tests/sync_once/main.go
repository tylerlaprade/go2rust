package main

import (
	"fmt"
	"sync"
)

func main() {
	var once sync.Once
	count := 0

	once.Do(func() {
		count++
	})
	once.Do(func() {
		count += 10
	})

	fmt.Println("count:", count)
}
