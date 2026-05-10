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

func main() {
	ch := make(chan error, 2)
	var err error
	ch <- err
	fmt.Println(receive(ch) == nil)
	ch <- errors.New("boom")
	fmt.Println(selectReceive(ch) != nil)
}
