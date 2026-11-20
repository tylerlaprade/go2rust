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

func dependencyDeferCannotRecoverOuter() (outer, callback string) {
	defer func() {
		outer = fmt.Sprint(recover())
	}()
	defer func() {
		worker.RunNormalDefer(func() {
			if recover() == nil {
				callback = "nil"
			} else {
				callback = "stolen"
			}
		})
	}()
	panic("outer")
}

func main() {
	fmt.Println(call(7))
	fmt.Println(call("text"))
	outer, callback := dependencyDeferCannotRecoverOuter()
	fmt.Println("mask", outer, callback)
}
