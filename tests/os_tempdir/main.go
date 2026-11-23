package main

import (
	"fmt"
	"os"
)

func main() {
	first := os.TempDir()
	stableDefault := first != "" && first == os.TempDir()
	raw := string([]byte{'/', 't', 'm', 'p', '/', 0xfa, 'T'})
	setErr := os.Setenv("TMPDIR", raw)
	fmt.Println(stableDefault, setErr == nil, os.TempDir() == raw)
}
