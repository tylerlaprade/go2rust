package main

import (
	"fmt"
	"time"
)

func main() {
	c1 := make(chan string, 1)
	go func() {
		time.Sleep(1 * time.Second)
		c1 <- "result 1"
	}()

	timeout1 := time.After(500 * time.Millisecond)
	select {
	case res := <-c1:
		fmt.Println(res)
	case <-timeout1:
		fmt.Println("timeout 1")
	}

	c2 := make(chan string, 1)
	go func() {
		time.Sleep(1 * time.Second)
		c2 <- "result 2"
	}()
	timeout2 := time.After(1500 * time.Millisecond)
	select {
	case res := <-c2:
		fmt.Println(res)
	case <-timeout2:
		fmt.Println("timeout 2")
	}
}
