package main

import "fmt"

func main() {
	i := 0

Loop:
	if i < 5 {
		fmt.Printf("i = %d\n", i)
		i++
		goto Loop
	}

	fmt.Println("First loop done")

	// Goto to skip code
	x := 1
	if x > 0 {
		goto Skip
	}
	fmt.Println("This won't print")

Skip:
	fmt.Println("Skipped to here")

	// More complex goto pattern
	for j := 0; j < 3; j++ {
		for k := 0; k < 3; k++ {
			if j == 1 && k == 1 {
				goto Done
			}
			fmt.Printf("j=%d, k=%d\n", j, k)
		}
	}

Done:
	fmt.Println("All done")
}
