package main

import "fmt"

// Bit flags using iota
const (
	FlagRead    = 1 << iota // 1 << 0 = 1
	FlagWrite               // 1 << 1 = 2
	FlagExecute             // 1 << 2 = 4
	FlagDelete              // 1 << 3 = 8
)

// Complex iota expressions
const (
	_  = iota             // ignore first value
	KB = 1 << (10 * iota) // 1 << 10 = 1024
	MB                    // 1 << 20 = 1048576
	GB                    // 1 << 30 = 1073741824
	TB                    // 1 << 40 = 1099511627776
)

// Multiple iotas in same expression
const (
	A, B = iota, iota * 10 // 0, 0
	C, D                   // 1, 10
	E, F                   // 2, 20
)

// Reset iota in new const block
const (
	First  = iota // 0
	Second        // 1
)

const (
	Third  = iota // 0 (reset)
	Fourth        // 1
)

func main() {
	// Test bit flags
	perms := FlagRead | FlagWrite
	fmt.Printf("Permissions: %d (Read=%d, Write=%d)\n", perms, FlagRead, FlagWrite)

	// Test size constants
	fmt.Printf("KB=%d, MB=%d, GB=%d\n", KB, MB, GB)

	// Test multiple iotas
	fmt.Printf("A=%d, B=%d, C=%d, D=%d, E=%d, F=%d\n", A, B, C, D, E, F)

	// Test reset
	fmt.Printf("First=%d, Second=%d, Third=%d, Fourth=%d\n", First, Second, Third, Fourth)
}
