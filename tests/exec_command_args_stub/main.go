package main

import (
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	cmd := exec.Command("go", "list", "-export", "-f", "{{.Export}}", "pkg")
	fmt.Println(strings.Join(cmd.Args, " "))
}
