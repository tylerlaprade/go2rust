package main

import (
	"errors"
	"fmt"
)

func receive(ch chan error) (err error) {
	defer func() {}()
	return <-ch
}

func selectReceive(ch chan error) error {
	select {
	case err := <-ch:
		return err
	default:
		return nil
	}
}

func localReceive(ch chan error) error {
	err := <-ch
	return err
}

func commaReceive(ch chan error) (bool, bool) {
	err, ok := <-ch
	return err != nil, ok
}

func commaAssign(ch chan error) (bool, bool) {
	var err error
	var ok bool
	err, ok = <-ch
	return err != nil, ok
}

func main() {
	ch := make(chan error, 2)
	var err error
	ch <- err
	fmt.Println(receive(ch) == nil)
	ch <- errors.New("boom")
	fmt.Println(selectReceive(ch) != nil)
	ch <- errors.New("local")
	fmt.Println(localReceive(ch) != nil)
	ch <- errors.New("comma")
	hasErr, ok := commaReceive(ch)
	fmt.Println(hasErr, ok)
	ch <- errors.New("assign")
	hasErr, ok = commaAssign(ch)
	fmt.Println(hasErr, ok)
}
