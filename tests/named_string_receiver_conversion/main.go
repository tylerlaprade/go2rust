package main

import "fmt"

type internalError string

func (e internalError) Error() string {
	return "gcimporter: " + string(e)
}

func main() {
	err := internalError("bad import data")
	fmt.Println(err.Error())
}
