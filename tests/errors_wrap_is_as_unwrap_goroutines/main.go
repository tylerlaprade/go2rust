// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"errors"
	"fmt"
)

var ErrNotFound = errors.New("not found")
var ErrDenied = errors.New("denied")

type NotFound struct{ Key string }

func (e *NotFound) Error() string { return "missing " + e.Key }

type Code int

func (c Code) Error() string { return fmt.Sprintf("code %d", int(c)) }

func find(key string) error {
	if key == "" {
		return &NotFound{Key: "empty"}
	}
	return fmt.Errorf("find %q: %w", key, ErrNotFound)
}

func load(key string) error {
	if err := find(key); err != nil {
		return fmt.Errorf("load: %w", err)
	}
	return nil
}

var ErrTimeout = errors.New("timeout")

type QueryError struct {
	Query string
	Err   error
}

func (e *QueryError) Error() string { return e.Query + ": " + e.Err.Error() }
func (e *QueryError) Unwrap() error { return e.Err }

func run() error {
	return &QueryError{Query: "select", Err: ErrTimeout}
}

func userUnwrap() {
	err := run()
	fmt.Println(err, errors.Is(err, ErrTimeout), errors.Unwrap(err) == ErrTimeout)
	wrapped := fmt.Errorf("db: %w", err)
	var qe *QueryError
	fmt.Println(errors.Is(wrapped, ErrTimeout), errors.As(wrapped, &qe), qe.Query)
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	base := errors.New("base")
	wrapped := fmt.Errorf("ctx: %w", base)
	other := errors.New("base")
	fmt.Println(errors.Is(wrapped, base), errors.Is(wrapped, other))
	fmt.Println(errors.Unwrap(wrapped) == base, errors.Unwrap(base) == nil)
	fmt.Println(wrapped)

	err := load("k")
	fmt.Println(err, errors.Is(err, ErrNotFound), errors.Is(err, ErrDenied))
	inner := errors.Unwrap(errors.Unwrap(err))
	fmt.Println(inner == ErrNotFound, inner)

	err = load("")
	var nf *NotFound
	if errors.As(err, &nf) {
		fmt.Println("as:", nf.Key, nf)
	}
	var code Code
	fmt.Println(errors.As(err, &code))
	err = fmt.Errorf("outer: %w", Code(7))
	if errors.As(err, &code) {
		fmt.Println("code:", code, int(code))
	}

	both := fmt.Errorf("%w and %w", ErrNotFound, ErrDenied)
	fmt.Println(both, errors.Is(both, ErrNotFound), errors.Is(both, ErrDenied), errors.Unwrap(both) == nil)
	joined := errors.Join(ErrDenied, nil, base)
	fmt.Println(errors.Is(joined, base), errors.Is(joined, ErrNotFound))
	fmt.Println(joined)
	fmt.Println(errors.Join(nil, nil) == nil, errors.Is(nil, nil), errors.Is(nil, base))
	var nilErr error
	fmt.Println(fmt.Errorf("wrap nil: %w", nilErr), errors.Unwrap(fmt.Errorf("x: %w", nilErr)) == nil)
	userUnwrap()
}
