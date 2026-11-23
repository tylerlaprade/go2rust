package main

import (
	"fmt"
	"os"
)

func exerciseRemoveAll() {
	const tree = "go2rust-remove-all-tree-goroutine"
	const file = "go2rust-remove-all-file-goroutine"

	_ = os.RemoveAll(tree)
	_ = os.RemoveAll(file)

	mkdirErr := os.MkdirAll(tree+"/nested", 0o700)
	writeTreeErr := os.WriteFile(tree+"/nested/payload", []byte("tree"), 0o600)
	dotErr := os.RemoveAll(tree + "/.")
	_, dotStatErr := os.Stat(tree + "/nested/payload")
	fmt.Println(dotErr != nil, dotStatErr == nil)
	removeTreeErr := os.RemoveAll(tree)
	_, treeStatErr := os.Stat(tree)
	fmt.Println(mkdirErr == nil, writeTreeErr == nil, removeTreeErr == nil, treeStatErr != nil)

	writeFileErr := os.WriteFile(file, []byte("file"), 0o600)
	removeFileErr := os.RemoveAll(file)
	_, fileStatErr := os.Stat(file)
	fmt.Println(writeFileErr == nil, removeFileErr == nil, fileStatErr != nil)

	missingErr := os.RemoveAll("go2rust-remove-all-missing-goroutine")
	emptyErr := os.RemoveAll("")
	fmt.Println(missingErr == nil, emptyErr == nil)
}

func main() {
	done := make(chan bool)
	go func() {
		exerciseRemoveAll()
		done <- true
	}()
	<-done
}
