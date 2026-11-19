package main

import "fmt"

type Inst struct {
	Op  int
	Out int
}

type runner struct{ inst *Inst }

func (i Inst) matches(op int) bool { return i.Op == op }

func assign(r *runner, program []Inst) { r.inst = &program[0] }

func step(r *runner) int {
	i := r.inst
	if i.matches(2) {
		return i.Out
	}
	switch i.Op {
	case 1:
		return i.Out
	}
	return 0
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	program := []Inst{{Op: 2, Out: 5}}
	r := &runner{}
	assign(r, program)
	fmt.Println(step(r))
	program[0].Op = 1
	program[0].Out = 7
	fmt.Println(step(r))
	program[0].Op = 3
	fmt.Println(step(r))
}
