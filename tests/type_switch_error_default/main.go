package main

import "fmt"

type plainErr struct{}

func (plainErr) Error() string {
	return "plain"
}

func classify(err error) string {
	switch err := err.(type) {
	default:
		return err.Error()
	}
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	fmt.Println(classify(plainErr{}))
}
