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

func nestedRegular(v int) string {
	result := "start"
	switch v {
	case 1:
		if true {
			result = "one"
			break
		}
		result = "bad"
	default:
		result = "other"
	}
	return result + "-done"
}

func nestedTyped(v interface{}) string {
	result := "start"
	switch v.(type) {
	case int:
		if true {
			result = "int"
			break
		}
		result = "bad"
	default:
		result = "other"
	}
	return result + "-done"
}

func main() {
	fmt.Println(regular(1))
	fmt.Println(regular(2))
	fmt.Println(nestedRegular(1))
	fmt.Println(nestedRegular(2))
	fmt.Println(nestedTyped(1))
	fmt.Println(nestedTyped("x"))
	if false {
		fmt.Println(typed(1))
	}
}
