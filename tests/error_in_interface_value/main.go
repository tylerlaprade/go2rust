package main

import (
	"errors"
	"fmt"
)

// show and count take their arguments as interface values, the way a logging
// helper does. An error made by errors.New or fmt.Errorf has to survive that.
func show(args ...any) {
	for _, arg := range args {
		fmt.Println(arg)
	}
}

func count(args ...any) int {
	return len(args)
}

func main() {
	plain := errors.New("bad")
	wrapped := fmt.Errorf("wrapped: %w", plain)
	show("first", plain, wrapped, 3)
	fmt.Println(count(plain, wrapped))

	var boxed any = plain
	if err, ok := boxed.(error); ok {
		fmt.Println("asserted:", err)
	}
	fmt.Println(boxed)
}
