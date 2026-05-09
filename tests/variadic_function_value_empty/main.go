package main

import "fmt"

type reporter func(string, ...any)

func call(report reporter) {
	report("ready")
}

func main() {
	call(func(format string, args ...any) {
		fmt.Println(format, len(args))
	})
}
