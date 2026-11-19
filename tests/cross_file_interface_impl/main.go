package main

import "fmt"

func main() {
	shapes := []Shape{newCircle(2), newCircle(3)}
	fmt.Println(totalArea(shapes))
	type measure interface{ Area() int }
	var m measure = Circle{r: 4}
	fmt.Println(m.Area())
}
