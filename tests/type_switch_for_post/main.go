package main

import "fmt"

func classify(value interface{}) string {
	for i := 0; i < 2; i++ {
		switch v := value.(type) {
		case int:
			if i == 0 {
				continue
			}
			return fmt.Sprintf("int:%d", v)
		default:
			return "other"
		}
	}
	return "none"
}

func main() {
	fmt.Println(classify(3))
}
