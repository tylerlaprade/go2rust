package main

import "fmt"

func indexSum(n int) int {
	var a [4]int
	sum := 0
	for i := range a[:n] {
		sum += i
	}
	return sum
}

type scanner struct {
	offset int
	data   []byte
}

func (s *scanner) advance() {
	for i, b := range s.data[s.offset:] {
		if b == ' ' {
			s.offset += i
			break
		}
	}
}

func main() {
	fmt.Println(indexSum(0), indexSum(3), indexSum(4))
	s := scanner{offset: 2, data: []byte(" aBC d")}
	s.advance()
	fmt.Println(s.offset)
}
