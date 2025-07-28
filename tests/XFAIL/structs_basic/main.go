package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

type Address struct {
	Street string
	City   string
	State  string
}

type Employee struct {
	Person
	Address
	ID     int
	Salary float64
}

func main() {
	// Basic struct creation
	p1 := Person{Name: "Alice", Age: 30}
	fmt.Println("Person 1:", p1)

	// Struct with field names
	p2 := Person{
		Name: "Bob",
		Age:  25,
	}
	fmt.Println("Person 2:", p2)

	// Access and modify fields
	p2.Age = 26
	fmt.Println("Updated Person 2:", p2)

	// Embedded structs
	emp := Employee{
		Person: Person{Name: "Charlie", Age: 35},
		Address: Address{
			Street: "123 Main St",
			City:   "Anytown",
			State:  "CA",
		},
		ID:     1001,
		Salary: 75000.0,
	}

	fmt.Println("Employee:", emp)
	fmt.Println("Employee name:", emp.Name)
	fmt.Println("Employee city:", emp.City)
}
