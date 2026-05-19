package main

import "fmt"

func nilMap() map[string]int {
	return nil
}

func countEntries(values map[string]int) int {
	count := 0
	for key, value := range values {
		fmt.Println("unexpected", key, value)
		count++
	}
	return count
}

func main() {
	fmt.Println(countEntries(nilMap()))
}
