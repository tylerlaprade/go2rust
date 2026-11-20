package main

import (
	"fmt"

	"example.com/panicpayload/worker"
)

func classify(result *string) {
	switch recovered := recover().(type) {
	case int:
		*result = fmt.Sprintf("int %d", recovered)
	case string:
		*result = "string " + recovered
	default:
		*result = "unknown"
	}
}

func call(value any) (result string) {
	defer classify(&result)
	worker.Fail(value)
	return
}

func main() {
	fmt.Println(call(7))
	fmt.Println(call("text"))
}
