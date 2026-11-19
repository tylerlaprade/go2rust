package main

import (
	"errors"
	"fmt"
)

var ErrNotFound = errors.New("not found")
var ErrClosed = errors.New("closed")

type Missing struct{ Name string }

func (m Missing) Error() string { return "missing " + m.Name }

type Reader struct {
	err  error
	read int
}

func (r *Reader) Err() error { return r.err }

func (r *Reader) fail(err error) { r.err = err }

func run(fail bool) error {
	if fail {
		return errors.New("boom")
	}
	return nil
}

func find(key string) error {
	if key == "" {
		return ErrNotFound
	}
	if key == "closed" {
		return ErrClosed
	}
	return nil
}

func lookup(key string) (int, error) {
	if err := find(key); err != nil {
		return 0, err
	}
	return len(key), nil
}

func report(err error) {
	fmt.Println("report:", err)
	err = nil
}

func classify(err error) string {
	switch err {
	case nil:
		return "ok"
	case ErrNotFound:
		return "not found"
	case ErrClosed:
		return "closed"
	}
	return "other: " + err.Error()
}

// work replaces its own result in a deferred function.
func work(fail bool) (err error) {
	defer func() {
		if err != nil {
			err = fmt.Errorf("wrapped: %v", err)
		}
	}()
	if fail {
		return errors.New("inner")
	}
	return nil
}

func main() {
	// Assigning an error leaves the one it was assigned from.
	var last error
	err := run(true)
	if err != nil {
		last = err
	}
	fmt.Println(err != nil, last != nil)
	fmt.Println(err, last)

	// A copy is a variable of its own.
	e1 := errors.New("first")
	e2 := e1
	e1 = errors.New("second")
	fmt.Println(e1, e2)

	// A package-level error can be returned, compared and returned again.
	fmt.Println(find(""), find("closed"), find("x"))
	fmt.Println(find("") == ErrNotFound, find("") == ErrClosed, find("x") == nil)
	fmt.Println(classify(find("")), classify(find("closed")), classify(find("x")), classify(err))
	if n, err := lookup(""); err == ErrNotFound {
		fmt.Println("lookup:", n, err)
	}
	got := find("")
	got = nil
	fmt.Println(got == nil, ErrNotFound, find("") == ErrNotFound)

	// Two errors with the same text are two errors. A copy is the same error.
	a := errors.New("same")
	b := errors.New("same")
	c := a
	fmt.Println(a == b, a == c, a != b, c == a)

	// Passing an error does not give the variable away, and the function
	// cannot change it.
	report(err)
	report(err)
	fmt.Println(err)

	// An error in a struct field.
	r := &Reader{}
	r.fail(err)
	held := r.Err()
	held = nil
	fmt.Println(held == nil, r.Err(), r.err == err, err)
	r.err = ErrClosed
	fmt.Println(r.Err() == ErrClosed, classify(r.Err()))

	// Errors in a slice and in a map.
	list := []error{err, last, ErrNotFound, nil}
	for i, item := range list {
		fmt.Println(i, item, item == err)
	}
	list = append(list, e2)
	fmt.Println(len(list), list[4], err)
	byName := map[string]error{"a": err, "b": ErrClosed}
	fmt.Println(byName["a"], byName["b"] == ErrClosed, byName["c"] == nil)

	// A custom error type.
	var m error = Missing{"key"}
	n := m
	m = nil
	fmt.Println(m == nil, n)

	last = nil
	fmt.Println(last == nil, err != nil)

	// An interface value holds a copy as well: it does not follow the
	// variable it was made from.
	var boxed interface{} = err
	err = errors.New("changed")
	fmt.Println(boxed, err)

	// So does what a closure returns.
	check := func() error { return err }
	fromClosure := check()
	fromClosure = nil
	fmt.Println(fromClosure == nil, err, check() == err)

	// A deferred function can still replace a named result.
	fmt.Println(work(true), work(false))
}
