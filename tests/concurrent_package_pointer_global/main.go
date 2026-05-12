package main

import "fmt"

type counter struct {
	value int
}

type valueReader interface {
	Value() int
}

func (c *counter) Value() int {
	return c.value
}

var current *counter
var fallback = &counter{value: 5}

func newCounter(value int) *counter {
	return &counter{value: value}
}

func setCounter(c *counter) {
	current = c
}

func getCounter() *counter {
	return current
}

func getFallback() valueReader {
	return fallback
}

func clearCounter() {
	current = nil
}

func markConcurrent(done chan bool) {
	go func() {
		done <- true
	}()
}

func main() {
	done := make(chan bool, 1)
	markConcurrent(done)
	<-done

	setCounter(newCounter(7))
	fmt.Println(getCounter().value)
	setCounter(newCounter(11))
	fmt.Println(getCounter().value)
	setCounter(nil)
	fmt.Println(getCounter() == nil)
	setCounter(newCounter(13))
	clearCounter()
	fmt.Println(getCounter() == nil)
	fmt.Println(getFallback().Value())
}
