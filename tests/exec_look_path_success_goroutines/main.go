package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {
	done := make(chan bool)
	go func() {
		goPath, goErr := exec.LookPath("go")
		fmt.Println(goErr == nil, goPath != "")
		firstDir := os.Getenv("GO2RUST_TEST_PATH_FIRST")
		secondDir := os.Getenv("GO2RUST_TEST_PATH_SECOND")
		firstPath, firstErr := exec.LookPath("go2rust-path-first")
		fmt.Println(firstErr == nil, firstPath == firstDir+"/go2rust-path-first")
		fallbackPath, fallbackErr := exec.LookPath("go2rust-path-fallback")
		fmt.Println(fallbackErr == nil, fallbackPath == secondDir+"/go2rust-path-fallback")

		shPath, shErr := exec.LookPath("/bin/sh")
		fmt.Println(shErr == nil, shPath == "/bin/sh")

		nonExecutable, nonExecutableErr := exec.LookPath("./main.go")
		fmt.Println(nonExecutable == "", nonExecutableErr != nil)

		missing, missingErr := exec.LookPath("__go2rust_missing_executable__")
		fmt.Println(missing == "", missingErr != nil)
		done <- true
	}()
	<-done
}
