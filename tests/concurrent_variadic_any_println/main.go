package main

import "fmt"

func log(values ...any) {
	fmt.Println(values...)
}

func main() {
	go func() {}()
	log("x", 7, true)
}
