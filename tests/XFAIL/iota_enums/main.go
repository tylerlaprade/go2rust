package main

import "fmt"

type Color int

const (
	Red Color = iota
	Green
	Blue
	Yellow
)

func main() {
	fmt.Println("Red:", Red)
	fmt.Println("Green:", Green)
	fmt.Println("Blue:", Blue)
	fmt.Println("Yellow:", Yellow)
}
