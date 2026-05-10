package main

import (
	"fmt"
)

type loadError struct {
	importStack []string
}

func main() {
	err := &loadError{importStack: []string{"root", "dep"}}
	msg := "import cycle not allowed"
	if len(err.importStack) != 0 {
		msg += fmt.Sprintf(": import stack: %v", err.importStack)
	}
	fmt.Println(msg)
}
