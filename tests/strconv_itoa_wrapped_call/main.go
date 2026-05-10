package main

import (
	"fmt"
	"strconv"
)

type item struct {
	index int
}

func (i item) Index() int {
	return i.index
}

func main() {
	v := item{index: 3}
	fmt.Println(strconv.Itoa(v.Index()))
}
