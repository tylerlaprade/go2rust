package main

import "fmt"

type Counter struct {
	value int
}

// Method with value receiver
func (c Counter) GetValue() int {
	return c.value
}

// Method with pointer receiver
func (c *Counter) Increment() {
	c.value++
}

func (c *Counter) Add(n int) {
	c.value += n
}

// Method with return value
func (c *Counter) Double() int {
	c.value *= 2
	return c.value
}

type Person struct {
	Name string
	Age  int
}

func (p Person) Greet() {
	fmt.Printf("Hello, I'm %s and I'm %d years old\n", p.Name, p.Age)
}

func (p *Person) HaveBirthday() {
	p.Age++
	fmt.Printf("%s is now %d years old\n", p.Name, p.Age)
}

func main() {
	// Counter methods
	counter := &Counter{value: 0}
	fmt.Println("Initial value:", counter.GetValue())

	counter.Increment()
	fmt.Println("After increment:", counter.GetValue())

	counter.Add(5)
	fmt.Println("After adding 5:", counter.GetValue())

	doubled := counter.Double()
	fmt.Println("After doubling:", doubled)

	// Person methods
	person := &Person{Name: "Alice", Age: 25}
	person.Greet()
	person.HaveBirthday()
	person.Greet()
}
