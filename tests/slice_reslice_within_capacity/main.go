package main

import "fmt"

func main() {
	s := make([]byte, 2, 4)
	s[0] = 'g'
	s[1] = 'o'
	s = s[:3]
	s[2] = '2'
	t := s[1:3]
	fmt.Println(len(s), cap(s), string(s))
	fmt.Println(len(t), cap(t), string(t))
}
