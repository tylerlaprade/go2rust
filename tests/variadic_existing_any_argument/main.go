package main

import "fmt"

func count(args ...any) int {
	return len(args)
}

func label(value any) string {
	switch v := value.(type) {
	case string:
		return v
	default:
		return "other"
	}
}

func main() {
	var value any = "kept"
	fmt.Println(count(value))
	fmt.Println(label(value))
}
