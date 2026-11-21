package main

import "fmt"

type valueError struct {
	code string
}

func (err valueError) Error() string {
	return err.code
}

type pointerError struct {
	n int
}

type pairError struct {
	a int
	b int
}

type nestedError struct {
	key nestedCode
}

type nanError struct {
	value float64
}

type namedValue interface {
	Value() float64
}

type namedNaN struct {
	value float64
}

type namedHolder struct {
	value namedValue
}

func (err pairError) Error() string {
	return fmt.Sprintf("%d:%d", err.a, err.b)
}

func (err nestedError) Error() string {
	return err.key.code
}

func (err nanError) Error() string {
	return "nan"
}

func (value namedNaN) Value() float64 {
	return value.value
}

func (err *pointerError) Error() string {
	return fmt.Sprintf("pointer:%d", err.n)
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
	done := make(chan bool)
	go func() { done <- true }()
	<-done

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

	var pairLeft error = pairError{a: 1, b: 2}
	var pairRight error = pairError{a: 1, b: 2}
	compared := make(chan bool)
	go func() {
		for i := 0; i < 1000; i++ {
			_ = pairLeft == pairRight
		}
		compared <- true
	}()
	go func() {
		for i := 0; i < 1000; i++ {
			_ = pairRight == pairLeft
		}
		compared <- true
	}()
	fmt.Println(<-compared && <-compared)

	var nestedLeft error = nestedError{key: nestedCode{code: "nested"}}
	var nestedRight error = nestedError{key: nestedCode{code: "nested"}}
	fmt.Println(nestedLeft == nestedRight)

	zero := 0.0
	var nan error = nanError{value: zero / zero}
	nanCopy := nan
	fmt.Println(nan == nan, nan == nanCopy)

	holder := namedHolder{value: namedNaN{value: zero / zero}}
	holderCopy := holder
	fmt.Println(holder == holder, holder == holderCopy)
}
