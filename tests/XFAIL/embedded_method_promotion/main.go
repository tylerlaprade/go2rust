package main

import "fmt"

// Base types with methods
type Logger struct {
	prefix string
}

func (l Logger) Log(msg string) {
	fmt.Printf("[%s] %s\n", l.prefix, msg)
}

func (l *Logger) SetPrefix(prefix string) {
	l.prefix = prefix
}

type Counter struct {
	count int
}

func (c Counter) Value() int {
	return c.count
}

func (c *Counter) Increment() {
	c.count++
}

func (c *Counter) Add(n int) {
	c.count += n
}

// Type that embeds both Logger and Counter
type Service struct {
	Logger  // Embedded
	Counter // Embedded
	name    string
}

// Service's own method
func (s Service) Name() string {
	return s.name
}

// Method that shadows embedded method
func (s Service) Value() int {
	// This should shadow Counter.Value()
	return s.Counter.Value() * 10
}

// Type with multiple levels of embedding
type Base struct {
	id int
}

func (b Base) GetID() int {
	return b.id
}

func (b *Base) SetID(id int) {
	b.id = id
}

type Middle struct {
	Base // Embedded
	data string
}

func (m Middle) GetData() string {
	return m.data
}

type Top struct {
	Middle // Embedded
	extra  string
}

func (t Top) GetExtra() string {
	return t.extra
}

func main() {
	// Test basic method promotion
	fmt.Println("=== Basic method promotion ===")
	svc := Service{
		Logger:  Logger{prefix: "SVC"},
		Counter: Counter{count: 0},
		name:    "MyService",
	}

	// Call promoted methods from Logger
	svc.Log("Service started") // Should call Logger.Log
	svc.SetPrefix("SERVICE")   // Should call Logger.SetPrefix
	svc.Log("Prefix changed")

	// Call promoted methods from Counter
	svc.Increment() // Should call Counter.Increment
	svc.Add(5)      // Should call Counter.Add
	fmt.Printf("Counter value (via promoted method): %d\n", svc.Counter.Value())

	// Call Service's own methods
	fmt.Printf("Service name: %s\n", svc.Name())
	fmt.Printf("Shadowed Value method: %d\n", svc.Value()) // Should return count * 10

	// Test method promotion with pointers
	fmt.Println("\n=== Method promotion with pointers ===")
	svcPtr := &Service{
		Logger:  Logger{prefix: "PTR"},
		Counter: Counter{count: 10},
		name:    "PointerService",
	}

	svcPtr.Log("Pointer service")
	svcPtr.Increment()
	fmt.Printf("Pointer service counter: %d\n", svcPtr.Counter.Value())

	// Test multi-level embedding
	fmt.Println("\n=== Multi-level embedding ===")
	top := Top{
		Middle: Middle{
			Base: Base{id: 100},
			data: "middle data",
		},
		extra: "extra data",
	}

	// Methods promoted from Base through Middle
	fmt.Printf("ID (promoted from Base): %d\n", top.GetID())
	top.SetID(200)
	fmt.Printf("ID after SetID: %d\n", top.GetID())

	// Methods promoted from Middle
	fmt.Printf("Data (promoted from Middle): %s\n", top.GetData())

	// Top's own method
	fmt.Printf("Extra: %s\n", top.GetExtra())

	// Test with embedded pointer types would go here
	// but local type definitions with methods aren't supported in functions
	fmt.Println("\n=== End of method promotion tests ===")
}
