package main

import (
	"errors"
	"fmt"
)

type parseValue struct {
	err error
}

func parse(v *parseValue) error {
	return v.err
}

func parsePair(v *parseValue) (int, error) {
	return 7, v.err
}

func main() {
	v := &parseValue{err: errors.New("bad")}
	err := parse(v)
	fmt.Println(err.Error())

	n, err := parsePair(v)
	fmt.Println(n)
	fmt.Println(err.Error())
}
