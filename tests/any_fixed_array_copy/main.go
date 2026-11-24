package main

import (
	"fmt"

	"example.com/anyfixedarraycopy/check"
)

func main() {
	var original any = [2]int{1, 2}
	fmt.Println(check.Preserved(original), original == nil)
	copied := original
	original = nil
	fmt.Println(original == nil, copied == nil)

	var nested any = [1][2]int{{3, 4}}
	nestedCopy := nested
	nested = nil
	fmt.Println(nested == nil, nestedCopy == nil)
}
