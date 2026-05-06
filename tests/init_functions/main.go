package main

import "fmt"

// Package-level variables
var globalCounter int
var initialized bool
var configData map[string]string

// First init function
func init() {
	fmt.Println("First init function called")
	globalCounter = 10
	initialized = true
}

// Second init function (they run in order)
func init() {
	fmt.Println("Second init function called")
	globalCounter += 5

	// Initialize map
	configData = make(map[string]string)
	configData["version"] = "1.0"
	configData["author"] = "go2rust"
}

// Third init function
func init() {
	fmt.Println("Third init function called")
	if initialized {
		fmt.Printf("Global counter initialized to: %d\n", globalCounter)
	}

	// Add more config
	configData["build"] = "debug"
	configData["target"] = "rust"
}

// Package-level variable with initialization
var computedValue = computeInitialValue()

func computeInitialValue() int {
	fmt.Println("Computing initial value during package initialization")
	return 42 * 2
}

// Another init function that runs after variable initialization
func init() {
	fmt.Println("Fourth init function called")
	fmt.Printf("Computed value is: %d\n", computedValue)

	// Modify the computed value
	computedValue += 10
}

// Struct with initialization
type Config struct {
	Name    string
	Version string
	Debug   bool
}

var appConfig Config

func init() {
	fmt.Println("Fifth init function - initializing app config")
	appConfig = Config{
		Name:    "Go2Rust Transpiler",
		Version: "0.1.0",
		Debug:   true,
	}
}

// Init function that might panic (for testing error handling)
func init() {
	fmt.Println("Sixth init function - with potential panic handling")
	fmt.Println("Sixth init function completed successfully")
}

// Helper function for init
func setupLogging() {
	fmt.Println("Setting up logging system...")
}

func init() {
	fmt.Println("Seventh init function - setting up subsystems")
	setupLogging()

	// Validate configuration
	if len(configData) == 0 {
		fmt.Println("Warning: No configuration data found")
	} else {
		fmt.Printf("Configuration loaded with %d entries\n", len(configData))
	}
}

func main() {
	fmt.Println("\n=== Main function started ===")

	// Show that all init functions have run
	fmt.Printf("Global counter: %d\n", globalCounter)
	fmt.Printf("Initialized flag: %t\n", initialized)
	fmt.Printf("Computed value: %d\n", computedValue)

	fmt.Println("\nConfiguration data:")
	fmt.Printf("  version: %s\n", configData["version"])
	fmt.Printf("  author: %s\n", configData["author"])
	fmt.Printf("  build: %s\n", configData["build"])
	fmt.Printf("  target: %s\n", configData["target"])

	fmt.Printf("\nApp config: {Name:%s Version:%s Debug:%t}\n", appConfig.Name, appConfig.Version, appConfig.Debug)

	// Demonstrate that init functions only run once
	fmt.Println("\n=== Calling functions that were used in init ===")
	fmt.Printf("Calling computeInitialValue() again: %d\n", computeInitialValue())
	setupLogging() // This will run again since it's called explicitly

	// Show that package variables retain their init values
	fmt.Printf("Global counter still: %d\n", globalCounter)

	// Modify package variables
	globalCounter = 100
	fmt.Printf("Modified global counter: %d\n", globalCounter)

	fmt.Println("\n=== Demonstrating init execution order ===")
	fmt.Println("1. Package-level variable declarations")
	fmt.Println("2. Package-level variable initializations (like computedValue)")
	fmt.Println("3. Init functions in the order they appear in source")
	fmt.Println("4. Main function")

	fmt.Println("\n=== Main function completed ===")
}
