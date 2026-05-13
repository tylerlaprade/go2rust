package main

import "fmt"

var current = map[string]string{
	"a": "old",
}

func main() {
	old := current
	current = map[string]string{
		"a": "new",
	}
	old["b"] = "saved"
	fmt.Println(current["a"])
	fmt.Println(old["a"], old["b"])
}
