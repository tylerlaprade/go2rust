package main

import (
	"fmt"
	"os"
)

func main() {
	// Mixed output to stdout and stderr
	fmt.Println("=== Mixed Output Test ===")

	// Standard output
	fmt.Println("This goes to stdout via fmt.Println")
	fmt.Printf("This goes to stdout via fmt.Printf: %d\n", 42)

	// Standard error
	fmt.Fprintln(os.Stderr, "This goes to stderr via fmt.Fprintln")
	fmt.Fprintf(os.Stderr, "This goes to stderr via fmt.Fprintf: %s\n", "error message")

	// Built-in println (goes to stderr)
	println("This goes to stderr via built-in println")

	// More mixed output
	fmt.Println("Back to stdout")
	fmt.Fprintln(os.Stderr, "Back to stderr")

	// Multiple values
	fmt.Println("Multiple", "values", "to", "stdout")
	fmt.Fprintln(os.Stderr, "Multiple", "values", "to", "stderr")

	// Numbers and strings mixed
	fmt.Printf("Number: %d, String: %s, Float: %.2f\n", 123, "hello", 3.14)
	fmt.Fprintf(os.Stderr, "Error code: %d, Message: %s\n", 404, "Not Found")

	// Final messages
	fmt.Println("Program completed successfully")
	fmt.Fprintln(os.Stderr, "No errors occurred")
}
