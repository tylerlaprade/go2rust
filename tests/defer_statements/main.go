package main

import "fmt"

func deferExample() {
	fmt.Println("Start of function")

	defer fmt.Println("Deferred 1")
	defer fmt.Println("Deferred 2")
	defer fmt.Println("Deferred 3")

	fmt.Println("Middle of function")

	defer func() {
		fmt.Println("Anonymous deferred function")
	}()

	fmt.Println("End of function")
}

func deferWithVariables() {
	x := 10
	defer func() {
		fmt.Println("Deferred x:", x)
	}()

	x = 20
	fmt.Println("Current x:", x)
}

func deferInLoop() {
	fmt.Println("Defer in loop:")
	for i := 0; i < 3; i++ {
		defer func(val int) {
			fmt.Printf("Deferred loop value: %d\n", val)
		}(i)
	}
	fmt.Println("Loop finished")
}

func cleanup() {
	fmt.Println("Cleanup function called")
}

func resourceExample() {
	fmt.Println("Acquiring resource")
	defer cleanup()

	fmt.Println("Using resource")
	// Simulate some work
	for i := 0; i < 3; i++ {
		fmt.Printf("Working... %d\n", i+1)
	}
	fmt.Println("Done with resource")
}

func main() {
	fmt.Println("=== Basic defer example ===")
	deferExample()

	fmt.Println("\n=== Defer with variables ===")
	deferWithVariables()

	fmt.Println("\n=== Defer in loop ===")
	deferInLoop()

	fmt.Println("\n=== Resource cleanup example ===")
	resourceExample()

	fmt.Println("\n=== Main function ending ===")
}
