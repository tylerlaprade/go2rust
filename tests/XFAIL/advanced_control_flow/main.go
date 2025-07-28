package main

import "fmt"

func main() {
	// Nested loops with labels
	fmt.Println("=== Nested loops with labels ===")

outer:
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			if i == 2 && j == 2 {
				fmt.Printf("Breaking outer loop at i=%d, j=%d\n", i, j)
				break outer
			}
			fmt.Printf("i=%d, j=%d\n", i, j)
		}
	}

	// Continue with labels
	fmt.Println("\n=== Continue with labels ===")

outerContinue:
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			if j == 2 {
				fmt.Printf("Continuing outer loop at i=%d, j=%d\n", i, j)
				continue outerContinue
			}
			fmt.Printf("i=%d, j=%d\n", i, j)
		}
	}

	// Complex switch with fallthrough
	fmt.Println("\n=== Complex switch with fallthrough ===")

	for num := 1; num <= 5; num++ {
		fmt.Printf("Number %d: ", num)
		switch num {
		case 1:
			fmt.Print("One")
			fallthrough
		case 2:
			fmt.Print(" Two-ish")
		case 3:
			fmt.Print("Three")
		case 4, 5:
			fmt.Print(" Four-or-Five")
		default:
			fmt.Print(" Other")
		}
		fmt.Println()
	}

	// Nested switch statements
	fmt.Println("\n=== Nested switch statements ===")

	for category := 1; category <= 2; category++ {
		for item := 1; item <= 2; item++ {
			fmt.Printf("Category %d, Item %d: ", category, item)

			switch category {
			case 1:
				switch item {
				case 1:
					fmt.Println("Electronics - Phone")
				case 2:
					fmt.Println("Electronics - Laptop")
				}
			case 2:
				switch item {
				case 1:
					fmt.Println("Books - Fiction")
				case 2:
					fmt.Println("Books - Non-fiction")
				}
			}
		}
	}

	// Complex for loop conditions
	fmt.Println("\n=== Complex for loop conditions ===")

	// Multiple variables in for loop
	for i, j := 0, 10; i < j; i, j = i+1, j-1 {
		fmt.Printf("i=%d, j=%d, sum=%d\n", i, j, i+j)
		if i >= 3 {
			break
		}
	}

	// For loop with complex condition
	fmt.Println("\n=== For loop with complex condition ===")

	x, y := 1, 1
	for x*y < 100 && x < 10 {
		fmt.Printf("x=%d, y=%d, product=%d\n", x, y, x*y)
		if x%2 == 0 {
			y += 2
		} else {
			y += 1
		}
		x++
	}

	// Goto statements (rarely used, but valid Go)
	fmt.Println("\n=== Goto statements ===")

	counter := 0

start:
	counter++
	fmt.Printf("Counter: %d\n", counter)

	if counter < 3 {
		goto start
	}

	fmt.Println("Done with goto")

	// Complex if-else chains
	fmt.Println("\n=== Complex if-else chains ===")

	for score := 0; score <= 100; score += 25 {
		var grade string
		var message string

		if score >= 90 {
			grade = "A"
			if score >= 95 {
				message = "Excellent!"
			} else {
				message = "Great job!"
			}
		} else if score >= 80 {
			grade = "B"
			if score >= 85 {
				message = "Good work!"
			} else {
				message = "Not bad!"
			}
		} else if score >= 70 {
			grade = "C"
			message = "Average"
		} else if score >= 60 {
			grade = "D"
			message = "Below average"
		} else {
			grade = "F"
			message = "Needs improvement"
		}

		fmt.Printf("Score %d: Grade %s - %s\n", score, grade, message)
	}

	// Range with complex break/continue logic
	fmt.Println("\n=== Range with complex break/continue ===")

	numbers := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

	fmt.Println("Processing numbers:")
	for i, num := range numbers {
		if num%2 == 0 {
			if num > 6 {
				fmt.Printf("Stopping at even number %d (index %d)\n", num, i)
				break
			}
			fmt.Printf("Skipping even number %d (index %d)\n", num, i)
			continue
		}

		if num == 7 {
			fmt.Printf("Found lucky number %d at index %d\n", num, i)
			continue
		}

		fmt.Printf("Processing odd number %d (index %d)\n", num, i)
	}

	// Nested range loops
	fmt.Println("\n=== Nested range loops ===")

	matrix := [][]string{
		{"a", "b", "c"},
		{"d", "e", "f"},
		{"g", "h", "i"},
	}

	for rowIdx, row := range matrix {
		for colIdx, cell := range row {
			if cell == "e" {
				fmt.Printf("Found center at [%d][%d]: %s\n", rowIdx, colIdx, cell)
				continue
			}
			if rowIdx == 2 && colIdx == 2 {
				fmt.Printf("Last cell [%d][%d]: %s\n", rowIdx, colIdx, cell)
				break
			}
			fmt.Printf("[%d][%d]: %s ", rowIdx, colIdx, cell)
		}
		fmt.Println()
	}

	// Select with complex channel operations
	fmt.Println("\n=== Select with complex channel operations ===")

	ch1 := make(chan int, 2)
	ch2 := make(chan string, 2)
	done := make(chan bool)

	// Fill channels
	ch1 <- 1
	ch1 <- 2
	ch2 <- "hello"
	ch2 <- "world"

	go func() {
		count := 0
		for {
			select {
			case val := <-ch1:
				fmt.Printf("Received int: %d\n", val)
				count++
			case val := <-ch2:
				fmt.Printf("Received string: %s\n", val)
				count++
			default:
				if count >= 4 {
					done <- true
					return
				}
			}
		}
	}()

	<-done
	fmt.Println("Channel processing complete")

	// Complex error handling flow
	fmt.Println("\n=== Complex error handling flow ===")

	processData := func(data []int) error {
		if len(data) == 0 {
			return fmt.Errorf("empty data")
		}

		for i, val := range data {
			if val < 0 {
				return fmt.Errorf("negative value at index %d: %d", i, val)
			}
			if val > 100 {
				return fmt.Errorf("value too large at index %d: %d", i, val)
			}
		}

		return nil
	}

	testData := [][]int{
		{1, 2, 3},
		{},
		{1, -2, 3},
		{1, 200, 3},
		{10, 20, 30},
	}

	for i, data := range testData {
		fmt.Printf("Testing dataset %d: %v\n", i+1, data)
		if err := processData(data); err != nil {
			fmt.Printf("  Error: %v\n", err)
			continue
		}
		fmt.Printf("  Success: data is valid\n")
	}
}
