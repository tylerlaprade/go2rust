package main

import (
	"errors"
	"fmt"
)

func collect(names []string) []error {
	var (
		n    = len(names)
		errs = make([]error, n)
	)
	for i, name := range names {
		func() {
			errs[i] = errors.New(name)
		}()
	}
	return errs
}

func main() {
	errs := collect([]string{"alpha", "beta"})
	fmt.Println(len(errs), errs[1].Error())
}
