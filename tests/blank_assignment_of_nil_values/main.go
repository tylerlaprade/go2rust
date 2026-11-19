// "_ = x" uses a variable and reads nothing from it, so it must work when the
// variable is nil. It used to read the value and stop the program for a nil
// error, pointer, slice, map, function or interface.
package main

import (
	"errors"
	"fmt"
)

type Result struct {
	err  error
	next *Result
	tags map[string]int
}

var lastErr error
var cache map[string]int

func (r Result) check() {
	_ = r.err
	_ = r.next
}

func (r *Result) checkPointer() {
	_ = r.err
	_ = r.tags
}

type T struct{ x int }

func find() (*T, error) { return nil, nil }

func main() {
	var err error
	_ = err
	var ptr *T
	_ = ptr
	var m map[string]int
	_ = m
	var s []int
	_ = s
	var a any
	_ = a
	q, e2 := find()
	_ = q
	_ = e2
	var f func()
	_ = f

	_ = lastErr
	_ = cache
	var r Result
	_ = r.err
	_ = r.next
	_ = r.tags
	_ = (r.err)
	r.check()
	p := &Result{}
	_ = p.err
	_ = p.next
	p.checkPointer()
	inner := &Result{next: &Result{}}
	_ = inner.next.err

	lastErr = errors.New("kept")
	_ = lastErr
	fmt.Println(lastErr, r.err == nil, p.next == nil)

	count := 0
	bump := func() int { count++; return count }
	_ = bump()
	_ = []int{bump()}[0]
	fmt.Println(count)
}
