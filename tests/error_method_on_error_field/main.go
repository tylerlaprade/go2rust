package main

import (
	"errors"
	"fmt"
)

type holder struct {
	err error
}

func main() {
	h := holder{err: errors.New("boom")}
	fmt.Println(h.err.Error())
}
