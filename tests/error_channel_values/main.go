package main

import (
	"errors"
	"fmt"
)

// Errors sent on a channel, nil among them, print as Go prints them when the
// receive is the print's own operand.

func work(fail bool) error {
	if fail {
		return errors.New("boom")
	}
	return nil
}

func main() {
	errc := make(chan error, 4)
	errc <- work(true)
	errc <- nil
	errc <- work(false)
	fmt.Println(<-errc)
	fmt.Printf("%v|%s\n", <-errc, "x")
	err := <-errc
	fmt.Println(err, err == nil)
	errc <- fmt.Errorf("wrapped: %w", errors.New("inner"))
	fmt.Println("got:", <-errc)
	format := "dynamic %v\n"
	errc <- errors.New("late")
	errc <- nil
	fmt.Printf(format, <-errc)
	fmt.Print(fmt.Sprintf(format, <-errc))
	errc <- nil
	plus := "%+v|%s|\n"
	fmt.Printf(plus, <-errc, nil)
	close(errc)
	e, ok := <-errc
	fmt.Println(e, ok)
}
