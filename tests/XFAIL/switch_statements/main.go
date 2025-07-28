package main

import "fmt"

func basicSwitch(day int) {
	switch day {
	case 1:
		fmt.Println("Monday")
	case 2:
		fmt.Println("Tuesday")
	case 3:
		fmt.Println("Wednesday")
	case 4:
		fmt.Println("Thursday")
	case 5:
		fmt.Println("Friday")
	case 6, 7:
		fmt.Println("Weekend")
	default:
		fmt.Println("Invalid day")
	}
}

func switchWithExpression() {
	x := 10
	switch x * 2 {
	case 20:
		fmt.Println("x * 2 equals 20")
	case 30:
		fmt.Println("x * 2 equals 30")
	default:
		fmt.Println("x * 2 is something else")
	}
}

func switchWithoutExpression() {
	score := 85
	switch {
	case score >= 90:
		fmt.Println("Grade: A")
	case score >= 80:
		fmt.Println("Grade: B")
	case score >= 70:
		fmt.Println("Grade: C")
	case score >= 60:
		fmt.Println("Grade: D")
	default:
		fmt.Println("Grade: F")
	}
}

func switchWithFallthrough(num int) {
	switch num {
	case 1:
		fmt.Println("One")
		fallthrough
	case 2:
		fmt.Println("Two or after One")
		fallthrough
	case 3:
		fmt.Println("Three or after Two or after One")
	default:
		fmt.Println("Other number")
	}
}

func typeSwitch(value interface{}) {
	switch v := value.(type) {
	case int:
		fmt.Printf("Integer: %d\n", v)
	case string:
		fmt.Printf("String: %s\n", v)
	case bool:
		fmt.Printf("Boolean: %t\n", v)
	case float64:
		fmt.Printf("Float: %.2f\n", v)
	default:
		fmt.Printf("Unknown type: %T\n", v)
	}
}

func main() {
	fmt.Println("=== Basic switch ===")
	basicSwitch(1)
	basicSwitch(6)
	basicSwitch(10)

	fmt.Println("\n=== Switch with expression ===")
	switchWithExpression()

	fmt.Println("\n=== Switch without expression ===")
	switchWithoutExpression()

	fmt.Println("\n=== Switch with fallthrough ===")
	switchWithFallthrough(1)
	fmt.Println("---")
	switchWithFallthrough(4)

	fmt.Println("\n=== Type switch ===")
	typeSwitch(42)
	typeSwitch("hello")
	typeSwitch(true)
	typeSwitch(3.14)
	typeSwitch([]int{1, 2, 3})
}
