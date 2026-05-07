package main

import "fmt"

func regular(v int) string {
	result := "start"
	switch v {
	case 1:
		result = "one"
		break
		result = "bad"
	default:
		result = "other"
	}
	return result
}

func typed(v interface{}) string {
	result := "start"
	switch v.(type) {
	case nil, int:
		result = "simple"
		break
		result = "bad"
	default:
		result = "other"
	}
	return result
}

func main() {
	fmt.Println(regular(1))
	fmt.Println(regular(2))
	if false {
		fmt.Println(typed(1))
	}
}
