package main

import (
	"fmt"
	"slices"
)

func main() {
	// Create and initialize map
	ages := make(map[string]int)
	ages["Alice"] = 25
	ages["Bob"] = 30
	ages["Charlie"] = 35

	fmt.Println("Ages map:", ages)

	// Map literal
	colors := map[string]string{
		"red":   "#FF0000",
		"green": "#00FF00",
		"blue":  "#0000FF",
	}

	fmt.Println("Colors map:", colors)

	// Check if key exists
	age, exists := ages["Alice"]
	if exists {
		fmt.Println("Alice's age:", age)
	}

	// Delete from map
	delete(ages, "Bob")
	fmt.Println("After deleting Bob:", ages)

	// Iterate over map in sorted order for deterministic output
	fmt.Println("All colors:")
	// Collect all keys into a slice
	var keys []string
	for k := range colors {
		keys = append(keys, k)
	}
	// Sort the keys
	slices.Sort(keys)
	// Print in sorted order
	for _, k := range keys {
		fmt.Println(k, "->", colors[k])
	}
}
