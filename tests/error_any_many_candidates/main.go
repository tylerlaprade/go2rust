package main

import "fmt"

type alphaErr struct{}

func (alphaErr) Error() string { return "alpha" }

type betaErr struct{}

func (betaErr) Error() string { return "beta" }

type gammaErr struct{}

func (gammaErr) Error() string { return "gamma" }

type deltaErr struct{}

func (deltaErr) Error() string { return "delta" }

func asAny(err error) any {
	return err
}

func main() {
	go func() {}()

	_ = asAny(deltaErr{})
	fmt.Println("boxed")
}
