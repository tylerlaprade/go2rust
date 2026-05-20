package main

import (
	"errors"
	"fmt"
)

func load() (_ []int, err error) {
	defer func() {
		if err == nil {
			err = errors.New("deferred")
		}
	}()
	values := []int{1, 2}
	return values, nil
}

func pair() ([]int, error) {
	return []int{3, 4, 5}, nil
}

func loadTuple() (_ []int, err error) {
	defer func() {
		if err == nil {
			err = errors.New("tuple deferred")
		}
	}()
	return pair()
}

func main() {
	values, err := load()
	fmt.Println(len(values), err != nil, err)
	values, err = loadTuple()
	fmt.Println(len(values), err != nil, err)
}
