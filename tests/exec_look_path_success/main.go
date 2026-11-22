package main

import (
	"fmt"
	runner "os/exec"
)

func main() {
	goPath, goErr := runner.LookPath("go")
	fmt.Println(goErr == nil, goPath != "")

	shPath, shErr := runner.LookPath("/bin/sh")
	fmt.Println(shErr == nil, shPath == "/bin/sh")

	nonExecutable, nonExecutableErr := runner.LookPath("./main.go")
	fmt.Println(nonExecutable == "", nonExecutableErr != nil)

	missing, missingErr := runner.LookPath("__go2rust_missing_executable__")
	fmt.Println(missing == "", missingErr != nil)
}
