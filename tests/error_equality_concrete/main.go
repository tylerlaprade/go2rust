package main

import (
	"errors"
	"fmt"
)

type valueError struct {
	code string
}

func (err valueError) Error() string {
	return err.code
}

type pointerError struct {
	n int
}

// This used to collide with the generated pointerErrorPtr support type.
type pointerErrorPtr struct {
	n int
}

// A reserved-looking Go identifier is still a source name and must also not
// collide with the generated pointer-error support type.
type __go_error_ptr_pointerError struct{}

func (err *pointerError) Error() string {
	return fmt.Sprintf("pointer:%d", err.n)
}

type nestedError struct {
	key nestedCode
}

type nanError struct {
	value float64
}

func (err nestedError) Error() string {
	return err.key.code
}

func (err nanError) Error() string {
	return "nan"
}

type wrapError struct {
	cause error
}

func (err *wrapError) Error() string {
	return "wrapped: " + err.cause.Error()
}

func (err *wrapError) Unwrap() error {
	return err.cause
}

var evaluationOrder string

func pointerOperand(err *pointerError) *pointerError {
	evaluationOrder += "P"
	return err
}

func errorOperand(err error) error {
	evaluationOrder += "E"
	return err
}

var ErrBusy = &pointerError{n: 1}

func returnBusy() error {
	return ErrBusy
}

func sameBySwitch(err, target error) bool {
	switch err {
	case target:
		return true
	}
	return false
}

func main() {
	var left error = valueError{code: "same"}
	var equal error = valueError{code: "same"}
	var different error = valueError{code: "different"}
	fmt.Println(left == equal, left == different)

	pointer := &pointerError{n: 1}
	var first error = pointer
	var second error = pointer
	fmt.Println(first == second)
	pointer.n = 2
	fmt.Println(first.Error(), second.Error())

	fmt.Println(returnBusy() == ErrBusy)
	fmt.Println(sameBySwitch(left, equal), sameBySwitch(first, second))

	var literal error = &pointerError{n: 3}
	literalPointer, literalOK := literal.(*pointerError)
	fmt.Println(literalOK, literalPointer.n)
	asserted := first.(*pointerError)
	fmt.Println(asserted == pointer)

	var nilPointer1 *pointerError
	var nilPointer2 *pointerError
	var nilError1 error = nilPointer1
	var nilError2 error = nilPointer2
	fmt.Println(nilError1 == nilError2, nilError1 == nil)

	var nestedLeft error = nestedError{key: nestedCode{code: "nested"}}
	var nestedRight error = nestedError{key: nestedCode{code: "nested"}}
	fmt.Println(nestedLeft == nestedRight)

	zero := 0.0
	var nan error = nanError{value: zero / zero}
	nanCopy := nan
	fmt.Println(nan == nan, nan == nanCopy)

	leaf := errors.New("leaf")
	var wrapped error = &wrapError{cause: leaf}
	fmt.Println(errors.Is(wrapped, leaf), errors.Unwrap(wrapped) == leaf)

	evaluationOrder = ""
	fmt.Println(pointerOperand(pointer) == errorOperand(first), evaluationOrder)
}
