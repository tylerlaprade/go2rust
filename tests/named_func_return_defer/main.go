package main

import "fmt"

func buildCleanup(empty bool) (cleanup func(), err error) {
	if empty {
		return func() {
			fmt.Println("empty")
		}, nil
	}
	defer func() {
		cleanup = func() {
			fmt.Println("cleanup")
		}
		if err != nil {
			cleanup()
			cleanup = nil
		}
	}()
	return nil, nil
}

func main() {
	emptyCleanup, emptyErr := buildCleanup(true)
	fmt.Println(emptyErr == nil)
	emptyCleanup()

	cleanup, err := buildCleanup(false)
	fmt.Println(err == nil)
	cleanup()
}
