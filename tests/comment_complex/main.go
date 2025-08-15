package main

import "fmt"

// Person represents a person with a name and age
type Person struct {
	// Name is the person's full name
	Name string
	// Age is the person's age in years
	Age int
}

// NewPerson creates a new Person instance
func NewPerson(name string, age int) *Person {
	// Validate inputs
	if age < 0 {
		// Return nil for invalid age
		return nil
	}

	// Create and return the person
	return &Person{
		Name: name,
		Age:  age,
	}
}

// Greet prints a greeting message
func (p *Person) Greet() {
	// Check if person is valid
	if p == nil {
		// Handle nil receiver
		fmt.Println("Invalid person")
		return
	}

	// Print the greeting
	fmt.Printf("Hello, I'm %s and I'm %d years old\n", p.Name, p.Age)
}

func main() {
	// Create a new person
	person := NewPerson("Alice", 30)

	// Call the greeting method
	person.Greet()

	// Try with invalid age
	invalid := NewPerson("Bob", -1)
	if invalid == nil {
		// This should print
		fmt.Println("Failed to create person with invalid age")
	}
}
