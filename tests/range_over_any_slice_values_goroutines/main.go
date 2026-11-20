// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

func show(v interface{}) { fmt.Println(v) }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	values := []interface{}{"a", 1, 2.5, true}
	for _, val := range values {
		show(val)
	}
	for i, val := range values {
		var copy any = val
		fmt.Println(i, copy)
	}
	for _, val := range values {
		switch v := val.(type) {
		case string:
			fmt.Println("s", v)
		default:
			fmt.Printf("%T %v\n", v, v)
			show(v)
		}
	}
}
