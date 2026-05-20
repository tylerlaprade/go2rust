package main

import "fmt"

func classify(values []string) {
	for _, value := range values {
		switch value {
		case "go", "rust":
			fmt.Println("systems")
		case "python":
			fmt.Println("scripting")
		default:
			fmt.Println("other")
		}
	}
}

func main() {
	classify([]string{"go", "python", "zig"})
}
