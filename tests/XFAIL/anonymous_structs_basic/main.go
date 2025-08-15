package main

import "fmt"

func main() {
	// Anonymous struct as variable
	var point struct {
		X int
		Y int
	}
	point.X = 10
	point.Y = 20
	fmt.Printf("Point: (%d, %d)\n", point.X, point.Y)

	// Anonymous struct literal
	person := struct {
		Name string
		Age  int
	}{
		Name: "Alice",
		Age:  30,
	}
	fmt.Printf("Person: %s, %d years old\n", person.Name, person.Age)

	// Anonymous struct with nested fields
	config := struct {
		Host     string
		Port     int
		Settings struct {
			Debug   bool
			Verbose bool
		}
	}{
		Host: "localhost",
		Port: 8080,
	}
	config.Settings.Debug = true
	config.Settings.Verbose = false
	fmt.Printf("Config: %s:%d (Debug: %v, Verbose: %v)\n",
		config.Host, config.Port, config.Settings.Debug, config.Settings.Verbose)

	// Array of anonymous structs
	var items [2]struct {
		ID    int
		Value string
	}
	items[0].ID = 1
	items[0].Value = "first"
	items[1].ID = 2
	items[1].Value = "second"
	for i, item := range items {
		fmt.Printf("Item %d: {ID: %d, Value: %s}\n", i, item.ID, item.Value)
	}

	// Slice of anonymous structs
	events := []struct {
		Type    string
		Message string
	}{
		{Type: "info", Message: "System started"},
		{Type: "warning", Message: "Low memory"},
		{Type: "error", Message: "Connection failed"},
	}
	for _, event := range events {
		fmt.Printf("Event [%s]: %s\n", event.Type, event.Message)
	}

	// Map with anonymous struct values
	users := map[string]struct {
		Email string
		Admin bool
	}{
		"alice": {Email: "alice@example.com", Admin: true},
		"bob":   {Email: "bob@example.com", Admin: false},
	}
	for name, user := range users {
		fmt.Printf("User %s: %s (admin: %v)\n", name, user.Email, user.Admin)
	}
}
