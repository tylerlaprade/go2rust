package main

import "fmt"

type brokenError struct {
	message string
}

var entered = make(chan bool, 2)
var release = make(chan bool, 2)

func (err *brokenError) Error() string {
	entered <- true
	<-release
	return err.message
}

func main() {
	var pointer *brokenError
	var err error = pointer

	start := make(chan bool)
	done := make(chan string, 2)
	for i := 0; i < 2; i++ {
		go func() {
			<-start
			done <- fmt.Sprint(err)
		}()
	}
	start <- true
	start <- true
	<-entered
	<-entered
	release <- true
	release <- true

	fmt.Println(<-done)
	fmt.Println(<-done)
	fmt.Println(err == err)
	release <- true
	fmt.Println(err)
}
