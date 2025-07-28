package main

import "fmt"

// Interface for drawing
type Drawable interface {
	Draw() string
}

// Shape types
type Circle struct {
	Radius float64
}

func (c Circle) Draw() string {
	return fmt.Sprintf("Circle(r=%.1f)", c.Radius)
}

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Draw() string {
	return fmt.Sprintf("Rectangle(%.1fx%.1f)", r.Width, r.Height)
}

type Canvas struct {
	Name   string
	Shapes []Drawable
}

// Nested struct definitions
type Address struct {
	Street  string
	City    string
	State   string
	ZipCode string
	Country string
}

type Contact struct {
	Email string
	Phone string
}

type Person struct {
	Name    string
	Age     int
	Address Address
	Contact Contact
}

type Department struct {
	Name      string
	Manager   Person
	Employees []Person
	Budget    float64
}

type Company struct {
	Name         string
	Departments  []Department
	Headquarters Address
}

func main() {
	// Create nested structures
	fmt.Println("=== Creating nested structures ===")

	// Create addresses
	hq := Address{
		Street:  "123 Corporate Blvd",
		City:    "Tech City",
		State:   "CA",
		ZipCode: "90210",
		Country: "USA",
	}

	managerAddr := Address{
		Street:  "456 Manager St",
		City:    "Suburb",
		State:   "CA",
		ZipCode: "90211",
		Country: "USA",
	}

	emp1Addr := Address{
		Street:  "789 Employee Ave",
		City:    "Hometown",
		State:   "CA",
		ZipCode: "90212",
		Country: "USA",
	}

	emp2Addr := Address{
		Street:  "321 Worker Way",
		City:    "Village",
		State:   "CA",
		ZipCode: "90213",
		Country: "USA",
	}

	// Create contacts
	managerContact := Contact{
		Email: "manager@company.com",
		Phone: "555-0001",
	}

	emp1Contact := Contact{
		Email: "emp1@company.com",
		Phone: "555-0002",
	}

	emp2Contact := Contact{
		Email: "emp2@company.com",
		Phone: "555-0003",
	}

	// Create people
	manager := Person{
		Name:    "Alice Manager",
		Age:     45,
		Address: managerAddr,
		Contact: managerContact,
	}

	employee1 := Person{
		Name:    "Bob Employee",
		Age:     30,
		Address: emp1Addr,
		Contact: emp1Contact,
	}

	employee2 := Person{
		Name:    "Carol Worker",
		Age:     28,
		Address: emp2Addr,
		Contact: emp2Contact,
	}

	// Create department
	engineering := Department{
		Name:      "Engineering",
		Manager:   manager,
		Employees: []Person{employee1, employee2},
		Budget:    1000000.0,
	}

	// Create company
	company := Company{
		Name:         "TechCorp Inc",
		Departments:  []Department{engineering},
		Headquarters: hq,
	}

	// Access nested data
	fmt.Println("\n=== Accessing nested data ===")

	fmt.Printf("Company: %s\n", company.Name)
	fmt.Printf("HQ Address: %s, %s, %s %s\n",
		company.Headquarters.Street,
		company.Headquarters.City,
		company.Headquarters.State,
		company.Headquarters.ZipCode)

	fmt.Printf("Department: %s\n", company.Departments[0].Name)
	fmt.Printf("Department Budget: $%.2f\n", company.Departments[0].Budget)

	fmt.Printf("Manager: %s (Age: %d)\n",
		company.Departments[0].Manager.Name,
		company.Departments[0].Manager.Age)

	fmt.Printf("Manager Email: %s\n",
		company.Departments[0].Manager.Contact.Email)

	fmt.Printf("Manager Address: %s, %s\n",
		company.Departments[0].Manager.Address.City,
		company.Departments[0].Manager.Address.State)

	// Iterate through employees
	fmt.Println("\n=== Department employees ===")

	for i, emp := range company.Departments[0].Employees {
		fmt.Printf("Employee %d: %s\n", i+1, emp.Name)
		fmt.Printf("  Age: %d\n", emp.Age)
		fmt.Printf("  Email: %s\n", emp.Contact.Email)
		fmt.Printf("  Phone: %s\n", emp.Contact.Phone)
		fmt.Printf("  Address: %s, %s, %s\n",
			emp.Address.Street, emp.Address.City, emp.Address.State)
		fmt.Println()
	}

	// Nested maps
	fmt.Println("=== Nested maps ===")

	// Map of maps
	inventory := map[string]map[string]int{
		"electronics": {
			"laptops": 50,
			"phones":  100,
			"tablets": 25,
		},
		"furniture": {
			"chairs": 200,
			"desks":  75,
			"lamps":  150,
		},
		"supplies": {
			"pens":    1000,
			"paper":   500,
			"folders": 300,
		},
	}

	fmt.Println("Inventory:")
	for category, items := range inventory {
		fmt.Printf("  %s:\n", category)
		for item, count := range items {
			fmt.Printf("    %s: %d\n", item, count)
		}
	}

	// Access nested map values
	laptopCount := inventory["electronics"]["laptops"]
	fmt.Printf("Laptop count: %d\n", laptopCount)

	// Nested slices
	fmt.Println("\n=== Nested slices ===")

	// Matrix (slice of slices)
	matrix := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}

	fmt.Println("Matrix:")
	for i, row := range matrix {
		fmt.Printf("Row %d: ", i)
		for j, val := range row {
			fmt.Printf("%d ", val)
			if j < len(row)-1 {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}

	// Access nested slice elements
	centerElement := matrix[1][1]
	fmt.Printf("Center element: %d\n", centerElement)

	// 3D slice
	cube := [][][]int{
		{
			{1, 2},
			{3, 4},
		},
		{
			{5, 6},
			{7, 8},
		},
	}

	fmt.Println("\n3D Cube:")
	for i, layer := range cube {
		fmt.Printf("Layer %d:\n", i)
		for j, row := range layer {
			fmt.Printf("  Row %d: ", j)
			for _, val := range row {
				fmt.Printf("%d ", val)
			}
			fmt.Println()
		}
	}

	// Complex nested structure with interfaces
	fmt.Println("\n=== Complex nested with interfaces ===")

	canvas := Canvas{
		Name: "My Drawing",
		Shapes: []Drawable{
			Circle{Radius: 5.0},
			Rectangle{Width: 10.0, Height: 8.0},
			Circle{Radius: 3.0},
		},
	}

	fmt.Printf("Canvas: %s\n", canvas.Name)
	for i, shape := range canvas.Shapes {
		fmt.Printf("Shape %d: %s\n", i+1, shape.Draw())
	}

	// Modify nested structures
	fmt.Println("\n=== Modifying nested structures ===")

	// Update employee contact
	company.Departments[0].Employees[0].Contact.Email = "bob.new@company.com"
	fmt.Printf("Updated employee email: %s\n",
		company.Departments[0].Employees[0].Contact.Email)

	// Add new employee
	newEmployee := Person{
		Name: "Dave Newbie",
		Age:  25,
		Address: Address{
			Street:  "999 New St",
			City:    "Newtown",
			State:   "CA",
			ZipCode: "90214",
			Country: "USA",
		},
		Contact: Contact{
			Email: "dave@company.com",
			Phone: "555-0004",
		},
	}

	company.Departments[0].Employees = append(company.Departments[0].Employees, newEmployee)
	fmt.Printf("Added new employee: %s\n", newEmployee.Name)
	fmt.Printf("Total employees now: %d\n", len(company.Departments[0].Employees))
}
