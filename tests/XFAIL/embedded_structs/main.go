package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func (p Person) Greet() {
	fmt.Printf("Hello, I'm %s\n", p.Name)
}

func (p Person) GetInfo() string {
	return fmt.Sprintf("%s (%d years old)", p.Name, p.Age)
}

type Address struct {
	Street string
	City   string
	State  string
}

func (a Address) FullAddress() string {
	return fmt.Sprintf("%s, %s, %s", a.Street, a.City, a.State)
}

type Employee struct {
	Person  // Embedded struct
	Address // Embedded struct
	ID      int
	Salary  float64
}

func (e Employee) Work() {
	fmt.Printf("%s is working (ID: %d)\n", e.Name, e.ID)
}

type Manager struct {
	Employee // Embedded struct
	Team     []string
}

func (m Manager) Manage() {
	fmt.Printf("Manager %s is managing team: %v\n", m.Name, m.Team)
}

// Anonymous struct embedding
type CompanyInfo struct {
	Founded int
	CEO     string
}

type Company struct {
	Name string
	CompanyInfo
}

func main() {
	// Basic embedded struct
	fmt.Println("=== Basic embedded struct ===")
	emp := Employee{
		Person: Person{
			Name: "Alice",
			Age:  30,
		},
		Address: Address{
			Street: "123 Main St",
			City:   "Anytown",
			State:  "CA",
		},
		ID:     1001,
		Salary: 75000.0,
	}

	// Access embedded fields directly
	fmt.Printf("Name: %s\n", emp.Name)
	fmt.Printf("Age: %d\n", emp.Age)
	fmt.Printf("Street: %s\n", emp.Street)
	fmt.Printf("ID: %d\n", emp.ID)

	// Call embedded methods
	emp.Greet()
	fmt.Println("Info:", emp.GetInfo())
	fmt.Println("Address:", emp.FullAddress())
	emp.Work()

	// Nested embedding
	fmt.Println("\n=== Nested embedding ===")
	mgr := Manager{
		Employee: Employee{
			Person: Person{
				Name: "Bob",
				Age:  35,
			},
			Address: Address{
				Street: "456 Oak Ave",
				City:   "Somewhere",
				State:  "NY",
			},
			ID:     2001,
			Salary: 95000.0,
		},
		Team: []string{"Alice", "Charlie", "Diana"},
	}

	// Access deeply nested fields
	fmt.Printf("Manager: %s\n", mgr.Name)
	fmt.Printf("Manager ID: %d\n", mgr.ID)
	fmt.Printf("Manager City: %s\n", mgr.City)

	// Call methods from all levels
	mgr.Greet()
	mgr.Work()
	mgr.Manage()

	// Anonymous struct embedding
	fmt.Println("\n=== Anonymous struct embedding ===")
	company := Company{
		Name: "TechCorp",
	}
	company.Founded = 2010
	company.CEO = "John Doe"

	fmt.Printf("Company: %s\n", company.Name)
	fmt.Printf("Founded: %d\n", company.Founded)
	fmt.Printf("CEO: %s\n", company.CEO)

	// Method promotion
	fmt.Println("\n=== Method promotion ===")
	fmt.Println("Employee methods are promoted from Person and Address")
	fmt.Printf("Employee can call: %s\n", emp.GetInfo())
	fmt.Printf("Employee address: %s\n", emp.FullAddress())
}
