package main

import "fmt"

func main() {
	// Create a Person - transpiler needs to know Person struct fields
	p := Person{Name: "Alice", Age: 30}
	fmt.Printf("Person: %s is %d years old\n", p.Name, p.Age)

	// Create an Address - transpiler needs to know Address struct fields
	addr := Address{
		Street: "123 Main St",
		City:   "Springfield",
		Zip:    "12345",
	}
	fmt.Printf("Address: %s, %s %s\n", addr.Street, addr.City, addr.Zip)

	// Create an Employee - transpiler needs to know nested struct types
	emp := Employee{
		Person:  Person{Name: "Bob", Age: 25},
		Address: addr,
		ID:      42,
	}
	fmt.Printf("Employee %d: %s lives at %s\n", emp.ID, emp.Person.Name, emp.Address.Street)

	// Access nested fields - requires knowing the full type hierarchy
	emp.Person.Age = 26
	fmt.Printf("After birthday: %s is now %d\n", emp.Person.Name, emp.Person.Age)
}
