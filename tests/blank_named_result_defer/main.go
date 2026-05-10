package main

import (
	"errors"
	"fmt"
)

func load() (_ []int, err error) {
	defer func() {}()
	return nil, errors.New("missing")
}

func main() {
	_, err := load()
	fmt.Println(err != nil)
}
