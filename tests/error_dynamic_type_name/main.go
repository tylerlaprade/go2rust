// %T of an error names the error it holds: a value or pointer implementor of
// the package, a wrap from fmt.Errorf with %w, errors.New's error, or nil.
package main

import (
	"errors"
	"fmt"
)

type NotFound struct{ Key string }

func (e NotFound) Error() string { return "not found: " + e.Key }

type Limit struct{ Max int }

func (e *Limit) Error() string { return fmt.Sprint("over ", e.Max) }

type Result struct {
	Err error
}

func check(n int) error {
	switch {
	case n < 0:
		return NotFound{fmt.Sprint(n)}
	case n > 10:
		return &Limit{10}
	case n == 5:
		return errors.New("five")
	case n == 7:
		return fmt.Errorf("seven: %w", NotFound{"7"})
	}
	return nil
}

func main() {
	for _, n := range []int{-1, 11, 5, 7, 3} {
		err := check(n)
		fmt.Printf("%d %T %v\n", n, err, err)
	}
	r := Result{Err: check(-2)}
	fmt.Printf("%T|%T\n", r.Err, errors.Join(check(5), check(11)))
}
