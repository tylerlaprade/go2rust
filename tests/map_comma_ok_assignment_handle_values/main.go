package main

import (
	"errors"
	"fmt"
)

var recursiveFunctions map[string]func(int) int

func recursiveCall(n int) int {
	if n == 0 {
		return 0
	}
	next, found := recursiveFunctions["k"]
	if !found || next == nil {
		return -100
	}
	return 1 + next(n-1)
}

// A two-result map read copies the function, interface, error, or channel
// handle into an existing variable. It must not take that handle out of the
// map's own value cell.
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
	nilFunctions := map[string]func(){"k": nil}
	var nilFunction func()
	nilFunction, ok = nilFunctions["k"]
	fmt.Println("function nil", ok, nilFunction == nil, nilFunctions["k"] == nil)
	recursiveFunctions = map[string]func(int) int{"k": recursiveCall}
	recur, ok := recursiveFunctions["k"]
	fmt.Println("function recursive", ok, recur(4), recursiveFunctions["k"](3))

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
	channelValues := map[string]chan int{"copy": channels["k"]}
	shortChannel, shortOK := channelValues["copy"]
	shortChannel, shortOK = channelValues["copy"]
	fmt.Println("channel short", shortOK, shortChannel != nil)
	channelInterfaces := map[string]any{"k": channels["k"]}
	var channelInterface any
	channelInterface, ok = channelInterfaces["k"]
	channelInterface.(chan int) <- 17
	fmt.Println("channel interface", ok, <-channelInterfaces["k"].(chan int))
}
