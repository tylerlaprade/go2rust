package main

import (
	"fmt"
	"os/exec"
)

func main() {
	goPath, goErr := exec.LookPath("go")
	fmt.Println(goErr == nil, goPath != "")

	shPath, shErr := exec.LookPath("/bin/sh")
	fmt.Println(shErr == nil, shPath == "/bin/sh")

	nonExecutable, nonExecutableErr := exec.LookPath("/etc/passwd")
	fmt.Println(nonExecutable == "", nonExecutableErr != nil)

	missing, missingErr := exec.LookPath("__go2rust_missing_executable__")
	fmt.Println(missing == "", missingErr != nil)
}
