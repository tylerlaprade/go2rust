// A call that returns several values can supply all the arguments of
// another call: fmt.Println(divmod(7, 3)), add(pair()), return g(f()).
package main

import (
	"errors"
	"fmt"
)

func divmod(a, b int) (int, int) { return a / b, a % b }

func add(x, y int) int { return x + y }

func pair() (string, error) { return "v", errors.New("e") }

func describe(s string, err error) string { return s + ":" + err.Error() }

func flag() (uint64, bool) { return 6, true }

func sum3(a, b, c int) int { return a + b + c }

func three() (int, int, int) { return 1, 2, 3 }

func addDivmod(a, b int) int {
	return add(divmod(a, b))
}

type acc struct{ total int }

func (a *acc) addPair(x, y int) { a.total += x + y }

func main() {
	fmt.Println(divmod(7, 3))
	fmt.Println(flag())
	fmt.Println(add(divmod(9, 4)))
	s := describe(pair())
	fmt.Println(s)
	var total int
	total = sum3(three())
	fmt.Println(total, addDivmod(17, 5))
	a := &acc{}
	a.addPair(divmod(10, 3))
	a.addPair(divmod(20, 6))
	fmt.Println(a.total)
	msg := fmt.Sprint(divmod(5, 2))
	fmt.Println(msg)
	run := func() { fmt.Println(divmod(100, 7)) }
	run()
}
