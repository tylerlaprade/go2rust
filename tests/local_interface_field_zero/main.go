package main

import "fmt"

type Reader interface {
	Read() int
}

type holder struct {
	reader Reader
	count  int
}

func zeroHolder() holder {
	return holder{}
}

func main() {
	holder := zeroHolder()
	fmt.Println(holder.count)
}
