package main

import (
	"fmt"
	"os/exec"
)

func main() {
	path, err := exec.LookPath("go")
	fmt.Println(err == nil, path != "")
}
