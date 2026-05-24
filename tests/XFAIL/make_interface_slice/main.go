package main

import "fmt"

type Animal interface {
	Sound() string
}

type Cat struct{ Name string }

func (c Cat) Sound() string { return c.Name + ": meow" }

type Dog struct{ Name string }

func (d Dog) Sound() string { return d.Name + ": woof" }

func main() {
	pets := make([]Animal, 2)
	pets[0] = Cat{Name: "whiskers"}
	pets[1] = Dog{Name: "rex"}
	for _, p := range pets {
		fmt.Println(p.Sound())
	}
}
