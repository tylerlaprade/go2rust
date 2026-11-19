package main

import (
	"errors"
	"fmt"
)

// This is the concurrent-wrapper twin of the ordinary fixture. The map's
// entry must keep its handle when comma-ok assignment writes an existing
// variable.
func main() {
	functions := map[string]func(string) string{"k": func(prefix string) string { return prefix + "function" }}
	function := func(prefix string) string { return prefix + "old function" }
	functionPointer := &function
	var ok bool
	function, ok = functions["k"]
	fmt.Println("function", ok, function("new ") == "new function", functions["k"]("map ") == "map function", &function == functionPointer)
	for i := 0; i < 100000; i++ {
		function, ok = functions["k"]
		if !ok {
			panic("function disappeared during repeated map reads")
		}
	}
	fmt.Println("function repeated", function("new ") == "new function", functions["k"]("map ") == "map function", &function == functionPointer)
	function, ok = functions["missing"]
	fmt.Println("function missing", ok, function == nil, functions["k"]("map ") == "map function", &function == functionPointer)

	interfaces := map[string]any{"k": "interface"}
	var iface any = "old interface"
	interfacePointer := &iface
	iface, ok = interfaces["k"]
	fmt.Println("interface", ok, iface.(string), interfaces["k"].(string), &iface == interfacePointer)
	iface, ok = interfaces["missing"]
	fmt.Println("interface missing", ok, iface == nil, interfaces["k"].(string), &iface == interfacePointer)

	errorsByName := map[string]error{"k": errors.New("error")}
	err := errors.New("old error")
	errorPointer := &err
	err, ok = errorsByName["k"]
	fmt.Println("error", ok, err.Error(), errorsByName["k"].Error(), &err == errorPointer)
	err, ok = errorsByName["missing"]
	fmt.Println("error missing", ok, err == nil, errorsByName["k"].Error(), &err == errorPointer)

	channels := map[string]chan int{"k": make(chan int, 1)}
	channel := make(chan int)
	channel, ok = channels["k"]
	fmt.Println("channel", ok, channel != nil, channels["k"] != nil)
}
