package main

import "fmt"

type Inner struct {
	Value int
}

func (i Inner) GetValue() int {
	return i.Value
}

type Outer struct {
	Inner
	Name string
}

func main() {
	o := Outer{
		Inner: Inner{Value: 42},
		Name:  "test",
	}
	
	// Direct field access
	fmt.Println("Value:", o.Value)
	fmt.Println("Name:", o.Name)
	
	// Method call
	fmt.Println("GetValue:", o.GetValue())
}
