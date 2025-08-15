package main

import "fmt"

// Function with anonymous struct parameter
func printPerson(p struct {
	Name string
	Age  int
}) {
	fmt.Printf("Person: %s is %d years old\n", p.Name, p.Age)
}

// Function returning anonymous struct
func getPoint() struct {
	X, Y int
} {
	return struct{ X, Y int }{X: 10, Y: 20}
}

// Function with multiple anonymous struct parameters
func comparePoints(
	p1 struct{ X, Y int },
	p2 struct{ X, Y int },
) bool {
	return p1.X == p2.X && p1.Y == p2.Y
}

// Function returning multiple values including anonymous struct
func getConfig() (string, struct {
	Port    int
	Timeout int
}) {
	return "server", struct {
		Port    int
		Timeout int
	}{Port: 8080, Timeout: 30}
}

// Function with anonymous struct pointer parameter
func updateSettings(s *struct {
	Debug   bool
	Verbose bool
}) {
	s.Debug = true
	s.Verbose = true
}

// Function with anonymous struct in channel
func processEvents(ch chan struct {
	Type    string
	Message string
}) {
	for event := range ch {
		fmt.Printf("Event [%s]: %s\n", event.Type, event.Message)
	}
}

func main() {
	// Test function with anonymous struct parameter
	printPerson(struct {
		Name string
		Age  int
	}{Name: "Alice", Age: 30})

	// Test function returning anonymous struct
	point := getPoint()
	fmt.Printf("Point: (%d, %d)\n", point.X, point.Y)

	// Test function with multiple anonymous struct parameters
	p1 := struct{ X, Y int }{X: 5, Y: 10}
	p2 := struct{ X, Y int }{X: 5, Y: 10}
	p3 := struct{ X, Y int }{X: 10, Y: 20}
	fmt.Printf("p1 == p2: %v\n", comparePoints(p1, p2))
	fmt.Printf("p1 == p3: %v\n", comparePoints(p1, p3))

	// Test function returning multiple values including anonymous struct
	name, config := getConfig()
	fmt.Printf("Config for %s: Port=%d, Timeout=%d\n", name, config.Port, config.Timeout)

	// Test function with anonymous struct pointer
	settings := &struct {
		Debug   bool
		Verbose bool
	}{Debug: false, Verbose: false}
	fmt.Printf("Settings before: Debug=%v, Verbose=%v\n", settings.Debug, settings.Verbose)
	updateSettings(settings)
	fmt.Printf("Settings after: Debug=%v, Verbose=%v\n", settings.Debug, settings.Verbose)

	// Test function with anonymous struct in channel
	eventCh := make(chan struct {
		Type    string
		Message string
	}, 2)
	eventCh <- struct {
		Type    string
		Message string
	}{Type: "info", Message: "System started"}
	eventCh <- struct {
		Type    string
		Message string
	}{Type: "error", Message: "Connection failed"}
	close(eventCh)
	processEvents(eventCh)
}
