package main

import "fmt"

func classify(v interface{}) string {
	switch v.(type) {
	case int:
		return "int"
	case string:
		return "string"
	default:
		return "other"
	}
}

func main() {
	fmt.Println(classify(1))
	fmt.Println(classify("x"))
	fmt.Println(classify(false))
}
