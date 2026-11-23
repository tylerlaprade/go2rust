package main

import (
	"fmt"
	"os"
	runner "os/exec"
)

func main() {
	goPath, goErr := runner.LookPath("go")
	fmt.Println(goErr == nil, goPath != "")
	firstDir := os.Getenv("GO2RUST_TEST_PATH_FIRST")
	secondDir := os.Getenv("GO2RUST_TEST_PATH_SECOND")
	firstPath, firstErr := runner.LookPath("go2rust-path-first")
	fmt.Println(firstErr == nil, firstPath == firstDir+"/go2rust-path-first")
	fallbackPath, fallbackErr := runner.LookPath("go2rust-path-fallback")
	fmt.Println(fallbackErr == nil, fallbackPath == secondDir+"/go2rust-path-fallback")

	shPath, shErr := runner.LookPath("/bin/sh")
	fmt.Println(shErr == nil, shPath == "/bin/sh")

	nonExecutable, nonExecutableErr := runner.LookPath("./main.go")
	fmt.Println(nonExecutable == "", nonExecutableErr != nil)

	missing, missingErr := runner.LookPath("__go2rust_missing_executable__")
	fmt.Println(missing == "", missingErr != nil)
}
