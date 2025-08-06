package main

import "fmt"

func main() {
	// Access map - transpiler needs to know Users is a map, not a slice
	aliceID := Users["alice"]
	fmt.Printf("Alice's ID: %d\n", aliceID)

	// Check map key existence - requires knowing it's a map
	if id, ok := Users["dave"]; ok {
		fmt.Printf("Dave's ID: %d\n", id)
	} else {
		fmt.Println("Dave not found")
	}

	// Access slice - transpiler needs to know Numbers is a slice
	fmt.Printf("First number: %d\n", Numbers[0])
	fmt.Printf("Last number: %d\n", Numbers[len(Numbers)-1])

	// Access map of slices - complex type resolution
	admins := Groups["admins"]
	fmt.Printf("Admin count: %d\n", len(admins))
	fmt.Printf("First admin: %s\n", admins[0])

	// Iterate over map - requires knowing the type
	// Note: map iteration order is non-deterministic, so we'll just count
	count := 0
	for _, _ = range Users {
		count++
	}
	fmt.Printf("User count: %d\n", count)

	// Access slice of maps - another complex case
	firstRecord := Records[0]
	if name, ok := firstRecord["name"].(string); ok {
		fmt.Printf("First record name: %s\n", name)
	}

	// Modify map - requires proper type handling
	Users["dave"] = 4
	fmt.Printf("Dave added with ID: %d\n", Users["dave"])
}
