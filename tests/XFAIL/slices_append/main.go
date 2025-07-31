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

	l := s[2:5]
	fmt.Println("slice[2:5]:", l)
}
