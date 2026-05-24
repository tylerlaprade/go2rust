package main

import "fmt"

type Stack []int

func (s *Stack) push(n int) {
	*s = append((*s), n)
}

func (s *Stack) pop() int {
	i := len(*s)
	if i == 0 {
		return -1
	}
	top := (*s)[i-1]
	*s = (*s)[0 : i-1]
	return top
}

func main() {
	var s Stack
	s.push(1)
	s.push(2)
	s.push(3)
	fmt.Println(s.pop())
	fmt.Println(s.pop())
	fmt.Println(s.pop())
}
