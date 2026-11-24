package main

import (
	"fmt"
	"path/filepath"
)

func main() {
	done := make(chan string, 1)
	go func() {
		done <- filepath.Base("/tmp/go2rust")
	}()
	fmt.Println(<-done)
}
