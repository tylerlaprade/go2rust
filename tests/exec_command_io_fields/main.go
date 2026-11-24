package main

import (
	"bytes"
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	var out, errb bytes.Buffer
	cmd := exec.Command("echo", "hello", "world")
	cmd.Stdout = &out
	cmd.Stderr = &errb
	err := cmd.Run()
	fmt.Println(err, strings.TrimSpace(out.String()), errb.Len())

	var sb strings.Builder
	c2 := exec.Command("printf", "a\nb\n")
	c2.Stdout = &sb
	fmt.Println(c2.Run(), strings.Count(sb.String(), "\n"))

	var sorted bytes.Buffer
	c3 := exec.Command("sort")
	c3.Stdin = strings.NewReader("pear\napple\nfig\n")
	c3.Stdout = &sorted
	fmt.Println(c3.Run())
	fmt.Print(sorted.String())

	var failed bytes.Buffer
	c4 := exec.Command("sh", "-c", "echo oops >&2; exit 3")
	c4.Stderr = &failed
	fmt.Println(c4.Run(), strings.TrimSpace(failed.String()))
}
