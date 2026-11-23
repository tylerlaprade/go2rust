package main

import (
	"fmt"
	"os"
)

func check(results chan bool) {
	regular, err := os.Stat("regular.txt")
	if err != nil {
		panic(err)
	}
	directory, err := os.Stat(".")
	if err != nil {
		panic(err)
	}
	link, err := os.Lstat("regular-link")
	if err != nil {
		panic(err)
	}
	followed, err := os.Stat("regular-link")
	if err != nil {
		panic(err)
	}

	results <- regular.Mode()&os.ModePerm != 0
	results <- regular.Mode().IsRegular()
	results <- directory.Mode()&os.ModePerm != 0
	results <- directory.Mode().IsDir()
	results <- link.Mode().Type() == os.ModeSymlink
	results <- followed.Mode().IsRegular()
	results <- followed.Mode().Type() == 0
	results <- regular.Mode()&os.ModePerm == followed.Mode()&os.ModePerm
}

func main() {
	results := make(chan bool, 8)
	go check(results)
	for i := 0; i < 8; i++ {
		fmt.Println(<-results)
	}
}
