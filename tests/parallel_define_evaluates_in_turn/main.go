// The values of a parallel definition are evaluated one after another, so a
// method that mutates its receiver can appear in more than one of them.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Stack []int

func (s *Stack) Push(v int) { *s = append(*s, v) }
func (s *Stack) Pop() int {
	old := *s
	v := old[len(old)-1]
	*s = old[:len(old)-1]
	return v
}

type counter struct{ n int }

func (c *counter) next() int {
	c.n++
	return c.n
}

type Matrix [][]float64

func (m Matrix) Same(o Matrix) bool { return len(m) == len(o) }

func evalRPN(expr string) int {
	var st Stack
	for _, t := range strings.Fields(expr) {
		switch t {
		case "+", "*":
			b, a := st.Pop(), st.Pop()
			if t == "+" {
				st.Push(a + b)
			} else {
				st.Push(a * b)
			}
		default:
			n, _ := strconv.Atoi(t)
			st.Push(n)
		}
	}
	return st.Pop()
}

func main() {
	fmt.Println(evalRPN("3 4 + 2 *"))
	c := &counter{}
	x, y, z := c.next(), c.next(), c.next()
	fmt.Println(x, y, z)
	v := 7
	p, q := v, v
	fmt.Println(p, q)
	byLen := map[int][]string{}
	for _, w := range strings.Fields("go is fun to use") {
		byLen[len(w)] = append(byLen[len(w)], w)
	}
	fmt.Println(byLen[2], byLen[3])
	fmt.Println(Matrix{{1}}.Same(Matrix{{2}}))
}
