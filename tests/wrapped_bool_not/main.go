package main

import "fmt"

func isReady(flag bool) bool {
	return flag
}

func main() {
	if !isReady(false) {
		fmt.Println("not ready")
	}
	negated := !isReady(true)
	fmt.Println(negated)
	if !isReady(true) {
		fmt.Println("wrong")
	} else {
		fmt.Println("ready")
	}
}
