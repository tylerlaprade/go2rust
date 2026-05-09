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

func nested(v interface{}) string {
	switch v.(type) {
	case int:
		switch v.(type) {
		case int:
			return "nested-int"
		default:
			return "nested-other"
		}
	default:
		return "other"
	}
}

func main() {
	fmt.Println(classify(1))
	fmt.Println(classify("x"))
	fmt.Println(classify(false))
	fmt.Println(nested(1))
}
