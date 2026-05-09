package main

import (
	"fmt"
	"go/types"
)

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	fmt.Println(types.Unsafe == types.Unsafe)
}
