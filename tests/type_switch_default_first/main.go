package main

import "fmt"

func classify(v interface{}) string {
	result := ""
	switch v.(type) {
	default:
		result = "other"
	case int:
		result = "int"
	}
	return result
}

func main() {
	fmt.Println(classify(7))
	fmt.Println(classify("x"))
}
