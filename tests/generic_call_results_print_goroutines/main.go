// The result of calling a generic function prints like the result of any
// other call, whether the function is in this package or another one.
package main

import (
	"fmt"

	"example.com/genericcallresultsprintgoroutines/dep"
)

func local[T any](v T) T { return v }

func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	fmt.Println(dep.Pick[string]("qualified"), dep.Pick("inferred"))
	fmt.Println(local[string]("local"), local(3))
	fmt.Println(dep.Join[string, int]("a", 12, "-"), dep.Join("b", "c", "+"))
	fmt.Printf("%s|%v\n", dep.Pick[string]("formatted"), dep.Join[int, int](1, 2, ":"))
}
