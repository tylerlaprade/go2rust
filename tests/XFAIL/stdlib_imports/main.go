package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	// Testing multiple standard library imports
	fmt.Println("=== Testing multiple stdlib imports ===")

	// strings package
	fmt.Println("\n--- strings package ---")
	upper := strings.ToUpper("hello world")
	fmt.Println("Upper:", upper)

	lower := strings.ToLower("HELLO WORLD")
	fmt.Println("Lower:", lower)

	trimmed := strings.TrimSpace("  hello  ")
	fmt.Println("Trimmed:", trimmed)

	split := strings.Split("a,b,c", ",")
	fmt.Printf("Split: %v\n", split)

	// strconv package
	fmt.Println("\n--- strconv package ---")
	num := 42
	str := strconv.Itoa(num)
	fmt.Println("Number as string:", str)

	parsed, err := strconv.Atoi("123")
	if err != nil {
		fmt.Println("Parse error:", err)
	} else {
		fmt.Println("Parsed number:", parsed)
	}

	floatStr := strconv.FormatFloat(3.14159, 'f', 2, 64)
	fmt.Println("Float as string:", floatStr)

	// math package
	fmt.Println("\n--- math package ---")
	fmt.Printf("Pi: %.6f\n", math.Pi)
	fmt.Printf("E: %.6f\n", math.E)
	fmt.Printf("Sqrt(16): %.2f\n", math.Sqrt(16))
	fmt.Printf("Pow(2, 8): %.0f\n", math.Pow(2, 8))
	fmt.Printf("Max(10, 20): %.0f\n", math.Max(10, 20))
	fmt.Printf("Min(10, 20): %.0f\n", math.Min(10, 20))

	// time package
	fmt.Println("\n--- time package ---")
	now := time.Now()
	fmt.Println("Current time:", now.Format("2006-01-02 15:04:05"))

	duration := 5 * time.Second
	fmt.Printf("Duration: %v\n", duration)

	// Sleep for a short time
	fmt.Println("Sleeping for 100ms...")
	time.Sleep(100 * time.Millisecond)
	fmt.Println("Done sleeping")

	// os package
	fmt.Println("\n--- os package ---")
	hostname, err := os.Hostname()
	if err != nil {
		fmt.Println("Hostname error:", err)
	} else {
		fmt.Println("Hostname:", hostname)
	}

	// Environment variables
	path := os.Getenv("PATH")
	if path != "" {
		fmt.Printf("PATH length: %d characters\n", len(path))
	} else {
		fmt.Println("PATH not found")
	}

	// Working directory
	wd, err := os.Getwd()
	if err != nil {
		fmt.Println("Working directory error:", err)
	} else {
		fmt.Println("Working directory:", wd)
	}

	// Combined usage
	fmt.Println("\n--- Combined usage ---")
	timestamp := time.Now().Unix()
	timestampStr := strconv.FormatInt(timestamp, 10)
	message := strings.Join([]string{"Timestamp:", timestampStr}, " ")
	fmt.Println(message)

	// Mathematical calculation with string formatting
	result := math.Sqrt(math.Pow(3, 2) + math.Pow(4, 2))
	resultStr := strconv.FormatFloat(result, 'f', 2, 64)
	fmt.Printf("Hypotenuse of 3,4 triangle: %s\n", resultStr)
}
