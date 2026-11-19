package main

import "fmt"

func printIndices(n int) {
	var a [4]int
	fmt.Printf("%d:", n)
	for i := range a[:n] {
		fmt.Printf(" %d", i)
	}
	fmt.Println()
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
	printIndices(0)
	printIndices(3)
	printIndices(4)
	s := scanner{offset: 2, data: []byte(" aBC d")}
	s.advance()
	fmt.Println(s.offset)
}
