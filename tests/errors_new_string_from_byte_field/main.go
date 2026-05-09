package main

import (
	"errors"
	"fmt"
	"os/exec"
)

func main() {
	if false {
		var err error
		if ee, ok := err.(*exec.ExitError); ok && len(ee.Stderr) > 0 {
			err = errors.New(string(ee.Stderr))
			fmt.Println(err)
		}
	}
}
