package main

import "fmt"

func isString(v interface{}) bool {
	switch v.(type) {
	case string:
		return true
	default:
		return false
	}
}

func classify(v interface{}) string {
	switch v.(type) {
	case string:
		if isString(v) {
			return "string"
		}
	default:
		return "other"
	}
	return "other"
}

func main() {
	fmt.Println(classify("hello"))
	fmt.Println(classify(42))
}
