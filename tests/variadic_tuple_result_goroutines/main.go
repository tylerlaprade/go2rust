package main

import "fmt"

func fields() (string, string, string) {
	return "prefix:", "go", "rust"
}

func join(prefix string, values ...string) string {
	return prefix + values[0] + values[1]
}

func collect[T any](values ...T) T {
	return values[1]
}

func genericFields() (int, int) {
	return 13, 14
}

func channelFields() (chan int, int) {
	ch := make(chan int, 1)
	ch <- 5
	return ch, 7
}

func report(ch <-chan int, values ...int) {
	fmt.Println(<-ch, values[0])
}

func reportTag(tag any, values ...int) {
	switch tag := tag.(type) {
	case int:
		fmt.Println(tag, values[0])
	case string:
		fmt.Println(tag, values[0])
	}
}

func taggedNumber() (int, int)    { return 8, 9 }
func taggedString() (string, int) { return "go", 10 }

type tupleError struct{ message string }

func (e tupleError) Error() string { return e.message }

func errorFields() (tupleError, int, int) { return tupleError{"bad"}, 11, 12 }

func reportError(err error, values ...int) {
	fmt.Println(err.Error(), values[0], values[1])
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done
	fmt.Println(join(fields()))
	fmt.Println(collect(genericFields()))
	report(channelFields())
	reportTag(taggedNumber())
	reportTag(taggedString())
	reportError(errorFields())
}
