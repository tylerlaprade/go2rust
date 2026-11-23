package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {
	wd, wdErr := os.Getwd()
	cmd := exec.Command("sh", "-c", "pwd")
	cmd.Dir = wd
	out, err := cmd.Output()
	if len(out) > 0 && out[len(out)-1] == '\n' {
		out = out[:len(out)-1]
	}
	fmt.Println(wdErr == nil, err == nil)
	fmt.Println(out[len(out)-2:])
}
