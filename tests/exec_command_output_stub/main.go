package main

import (
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	out, err := exec.Command("go", "list", "-e", "-f", "{{context.ReleaseTags}}", "--", "unsafe").Output()
	if err != nil {
		fmt.Println("error")
		return
	}
	fmt.Println(strings.HasPrefix(string(out), "[go1."))
}
