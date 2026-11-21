package main

import "fmt"

type brokenError struct {
	message string
}

func (err *brokenError) Error() string { return err.message }

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

	fmt.Println(<-done)
	fmt.Println(<-done)
	fmt.Println(err == err)
	fmt.Println(err)
}
