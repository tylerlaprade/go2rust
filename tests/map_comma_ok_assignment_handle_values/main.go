package main

import (
	"errors"
	"fmt"
)

// A two-result map read copies the function, interface, error, or channel
// handle into an existing variable. It must not take that handle out of the
// map's own value cell.
func main() {
	functions := map[string]func() string{"k": func() string { return "function" }}
	var function func() string
	var ok bool
	function, ok = functions["k"]
	fmt.Println("function", ok, function != nil, functions["k"] != nil)

	interfaces := map[string]any{"k": "interface"}
	var iface any
	iface, ok = interfaces["k"]
	fmt.Println("interface", ok, iface != nil, interfaces["k"] != nil)

	errorsByName := map[string]error{"k": errors.New("error")}
	var err error
	err, ok = errorsByName["k"]
	fmt.Println("error", ok, err != nil, errorsByName["k"] != nil)

	channels := map[string]chan int{"k": make(chan int, 1)}
	channel := make(chan int)
	channel, ok = channels["k"]
	fmt.Println("channel", ok, channel != nil, channels["k"] != nil)
}
