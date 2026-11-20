package main

import (
	"fmt"
	"path/filepath"
)

func main() {
	parts := []string{filepath.Base("/tmp/go2rust"), "local"}
	fmt.Println(parts)
}
