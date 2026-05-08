package main

import "fmt"

type numbers []int

func (ns numbers) String() string {
	return fmt.Sprintf("numbers(%d)", len(ns))
}

func (ns numbers) intersect(other numbers) numbers {
	out := numbers{}
	for _, left := range ns {
		for _, right := range other {
			if left == right {
				out = append(out, left)
			}
		}
	}
	return out
}

type holder struct {
	terms numbers
}

func combine(a, b *holder) {
	a.terms = a.terms.intersect(b.terms)
}

func main() {
	a := &holder{terms: numbers{1, 2, 3}}
	b := &holder{terms: numbers{2, 4}}
	combine(a, b)
	fmt.Println(len(a.terms), a.terms[0])
}
