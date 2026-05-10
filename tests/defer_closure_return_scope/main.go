package main

import "fmt"

func run() (result int) {
	defer func() {
		result = 7
		return
	}()
	return 3
}

func main() {
	fmt.Println(run())
}
