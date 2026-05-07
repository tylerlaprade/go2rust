package main

import (
	"errors"
	"fmt"
)

func label() string {
	return "value"
}

func fail() error {
	return errors.New("bad")
}

func count(args ...any) int {
	return len(args)
}

func main() {
	err := fail()
	fmt.Println(count(label(), err, "literal", 3))
}
