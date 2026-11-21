package main

import "fmt"

func (receiver *rootError) Error() string {
	if receiver == nil {
		return "root-nil"
	}
	return "root"
}

func main() {
	fmt.Println(callRoot(nil))
	fmt.Println(callSibling(nil))
}
