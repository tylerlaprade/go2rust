package main

import (
	"errors"
	"fmt"
)

func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("division by zero")
	}
	return a / b, nil
}

func sqrt(x float64) (float64, error) {
	if x < 0 {
		return 0, fmt.Errorf("cannot take square root of negative number: %f", x)
	}
	// Simple approximation
	result := x / 2
	for i := 0; i < 10; i++ {
		result = (result + x/result) / 2
	}
	return result, nil
}

type CustomError struct {
	Code    int
	Message string
}

func (e CustomError) Error() string {
	return fmt.Sprintf("Error %d: %s", e.Code, e.Message)
}

func processValue(val int) error {
	if val < 0 {
		return CustomError{Code: 100, Message: "negative value not allowed"}
	}
	if val > 100 {
		return CustomError{Code: 200, Message: "value too large"}
	}
	return nil
}

func main() {
	// Basic error handling
	result, err := divide(10, 2)
	if err != nil {
		fmt.Println("Error:", err)
	} else {
		fmt.Println("10 / 2 =", result)
	}

	// Error case
	result, err = divide(10, 0)
	if err != nil {
		fmt.Println("Error:", err)
	} else {
		fmt.Println("Result:", result)
	}

	// Formatted error
	sqrtResult, err := sqrt(-4)
	if err != nil {
		fmt.Println("Sqrt error:", err)
	} else {
		fmt.Println("Sqrt result:", sqrtResult)
	}

	// Custom error
	err = processValue(-5)
	if err != nil {
		fmt.Println("Process error:", err)
	}

	err = processValue(150)
	if err != nil {
		fmt.Println("Process error:", err)
	}

	err = processValue(50)
	if err != nil {
		fmt.Println("Process error:", err)
	} else {
		fmt.Println("Value processed successfully")
	}
}
