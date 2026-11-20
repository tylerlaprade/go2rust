package main

import "fmt"

func safeDivide(a, b float64) (result float64, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("panic occurred: %v", r)
			result = 0
		}
	}()

	if b == 0 {
		panic("division by zero")
	}

	result = a / b
	return result, nil
}

func processSlice(slice []int, index int) (value int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("index out of bounds: %v", r)
			value = -1
		}
	}()

	value = slice[index]
	return value, nil
}

func nestedPanic() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("Recovered from nested panic: %v\n", r)
		}
	}()

	func() {
		defer func() {
			if r := recover(); r != nil {
				fmt.Printf("Inner recovery: %v\n", r)
				panic("re-panicking from inner function")
			}
		}()

		panic("original panic")
	}()
}

func demonstratePanicTypes() {
	// String panic
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("Recovered string panic: %v\n", r)
		}
	}()

	defer func() {
		panic("string panic message")
	}()

	defer func() {
		panic(42) // Integer panic
	}()

	defer func() {
		panic(fmt.Errorf("error panic"))
	}()
}

func chainedDefers() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("Final recovery: %v\n", r)
		}
	}()

	defer func() {
		fmt.Println("Defer 1: This runs")
	}()

	defer func() {
		fmt.Println("Defer 2: This also runs")
		panic("panic from defer")
	}()

	defer func() {
		fmt.Println("Defer 3: This runs first")
	}()

	fmt.Println("About to return normally")
}

func main() {
	fmt.Println("=== Safe divide examples ===")

	result, err := safeDivide(10, 2)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("10 / 2 = %.2f\n", result)
	}

	result, err = safeDivide(10, 0)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Result: %.2f\n", result)
	}

	fmt.Println("\n=== Slice access examples ===")

	numbers := []int{1, 2, 3, 4, 5}

	value, err := processSlice(numbers, 2)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("numbers[2] = %d\n", value)
	}

	value, err = processSlice(numbers, 10)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Value: %d\n", value)
	}

	fmt.Println("\n=== Nested panic example ===")
	nestedPanic()

	fmt.Println("\n=== Different panic types ===")
	demonstratePanicTypes()

	fmt.Println("\n=== Chained defers with panic ===")
	chainedDefers()

	fmt.Println("\n=== Program completed successfully ===")
}
