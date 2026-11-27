// A runtime error that nothing recovers ends the program with Go's text
// for it and exit status 2, in a function with no defer as well.
package main

import "fmt"

func at(s []int, i int) int { return s[i] }

func main() {
	s := []int{1, 2, 3}
	fmt.Println("before", at(s, 2))
	fmt.Print("partial line ")
	fmt.Println(at(s, len(s)+2))
}
