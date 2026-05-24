package main

import "fmt"

func main() {
	lines := []string{"a", "", "b", "", "c"}
	n := 0
	for _, line := range lines {
		if line != "" {
			lines[n] = line
			n++
		}
	}
	lines = lines[0:n]
	fmt.Println(lines)
}
