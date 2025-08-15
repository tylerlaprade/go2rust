package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

type Employee struct {
	Person
	ID int
}

func main() {
	e := Employee{
		Person: Person{Name: "John", Age: 30},
		ID:     123,
	}
	fmt.Println(e.Name)
	fmt.Println(e.ID)
}
