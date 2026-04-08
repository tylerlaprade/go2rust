package main

import "fmt"

func main() {
	counts := make(map[string]int, 4)
	counts["go"]++
	counts["rust"] += 2
	fmt.Println(counts["go"], counts["rust"], len(counts))

	seen := make(map[int]bool, 3)
	seen[10] = true
	seen[20] = true
	fmt.Println(seen[10], seen[30], len(seen))
}
