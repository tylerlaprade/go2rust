package main

import "fmt"

func count(names []string) int {
	return len(names)
}

func main() {
	groups := map[string][]string{
		"letters": {"alpha", "beta"},
	}
	fmt.Println(count(groups["letters"]))
}
