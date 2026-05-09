package main

import (
	"fmt"
	"os/exec"
)

func main() {
	if false {
		var err error
		if ee, ok := err.(*exec.ExitError); ok {
			fmt.Println(len(ee.Stderr))
		}
	}
}
