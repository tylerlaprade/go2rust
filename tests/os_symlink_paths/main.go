package main

import (
	"fmt"
	"os"
)

func symlinkAndFollow(target, link string) {
	writeErr := os.WriteFile(target, []byte("initial"), 0o600)
	linkErr := os.Symlink(target, link)
	mutateErr := os.WriteFile(target, []byte("updated"), 0o600)
	data, readErr := os.ReadFile(link)
	fmt.Println(writeErr == nil, linkErr == nil, mutateErr == nil, readErr == nil, string(data))
}

func main() {
	symlinkAndFollow("ordinary-target", "ordinary-link")
	symlinkAndFollow(string([]byte{0xfa, 'T'}), string([]byte{0xfb, 'L'}))
}
