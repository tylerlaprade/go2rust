package main

import "fmt"

func main() {
	shapes := []Shape{newCircle(2), newCircle(3)}
	fmt.Println(totalArea(shapes))
}
