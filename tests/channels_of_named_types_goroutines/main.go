// In a program with goroutines: channels of named types, a constant sent
// on a chan Level and a named slice or map received and printed.
package main

import "fmt"

type Level int

type Celsius float64

type Name string

type IDs []int

type Tags map[string]int

func main() {
	ch := make(chan Level, 2)
	ch <- 5
	ch <- Level(7)
	fmt.Println(<-ch, <-ch)
	fc := make(chan Celsius, 1)
	fc <- 21.5
	fmt.Println(<-fc)
	nc := make(chan Name, 1)
	nc <- "bo"
	n := <-nc
	fmt.Println(n, len(n))
	done := make(chan IDs)
	go func() { done <- IDs{1, 2} }()
	fmt.Println(<-done)
	tc := make(chan Tags, 1)
	tc <- Tags{"a": 1}
	fmt.Println(<-tc)
}
