package main

import "fmt"

type Params struct {
	n int
}

func (p *Params) Len() int {
	return p.n
}

func (p *Params) At(i int) int {
	return i + 1
}

func total(p *Params) int {
	sum := 0
	for i := range p.Len() {
		if i == 0 {
			for i := 0; i < p.Len(); i++ {
				sum += p.At(i)
			}
			sum += i
		}
	}
	return sum
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	p := &Params{n: 3}
	fmt.Println(total(p))
}
