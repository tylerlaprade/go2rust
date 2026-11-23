package main

import (
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	raw := string([]byte{'A', 0xff, 'B'})
	out, err := exec.Command("sh", "-c", "printf '%s' \"$1\" | od -An -tu1", "go2rust", raw).Output()
	fmt.Println(strings.Join(strings.Fields(string(out)), ","), err == nil)
}
