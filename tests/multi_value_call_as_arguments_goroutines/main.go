// The same multi-value calls as arguments in a program with a goroutine, so
// every value is behind a lock.
package main

import (
	"errors"
	"fmt"
	"sync"
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
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println(divmod(100, 7))
	}()
	wg.Wait()
}
