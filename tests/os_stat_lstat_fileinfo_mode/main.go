package main

import (
	"fmt"
	"os"
)

func main() {
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

	fmt.Println(regular.Mode()&os.ModePerm != 0)
	fmt.Println(regular.Mode().IsRegular())
	fmt.Println(directory.Mode()&os.ModePerm != 0)
	fmt.Println(directory.Mode().IsDir())
	fmt.Println(link.Mode().Type() == os.ModeSymlink)
	fmt.Println(followed.Mode().IsRegular())
	fmt.Println(followed.Mode().Type() == 0)
	fmt.Println(regular.Mode()&os.ModePerm == followed.Mode()&os.ModePerm)
}
