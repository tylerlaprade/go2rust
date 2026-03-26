package main

import "fmt"

func main() {
	var s []string
	s = append(s, "a")
	s = append(s, "b", "c")
	fmt.Println("slice:", s)

	c := make([]string, len(s))
	copy(c, s)
	fmt.Println("copy:", c)

	l := s[1:3]
	fmt.Println("slice[1:3]:", l)
}
