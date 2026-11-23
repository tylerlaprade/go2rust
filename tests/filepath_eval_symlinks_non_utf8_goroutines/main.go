package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	link := string([]byte{0xfe, 'L'})
	target := string([]byte{0xfa, 'T'})

	info, lstatErr := os.Lstat(link)
	isLink := lstatErr == nil && info.Mode()&os.ModeSymlink != 0
	resolved, evalErr := filepath.EvalSymlinks(link)

	fmt.Println(lstatErr == nil, isLink, evalErr == nil)
	fmt.Println(resolved == target)
	fmt.Println([]byte(resolved))
}
