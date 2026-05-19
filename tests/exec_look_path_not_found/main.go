package main

import (
	"fmt"
	"os/exec"
)

func main() {
	_, err := exec.LookPath("__go2rust_missing_executable__")
	fmt.Println(err != nil)
}
