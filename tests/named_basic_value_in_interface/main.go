// A value of a type such as "type Level int" held in an interface{}. Printing
// it stopped the program: format_any had no case for it (DEV-47).
package main

import "fmt"

type Level int

type Name string

type Celsius float64

func (c Celsius) String() string { return fmt.Sprintf("%.1f°C", float64(c)) }

func main() {
	var a any = Level(3)
	var b any = Name("x")
	var c any = Celsius(21.5)
	fmt.Println(a, b, c)
	fmt.Printf("%v %d %s %v|%5v|%-4d|%x\n", a, a, b, c, a, a, a)
	values := []any{Level(1), Name("y"), Celsius(-3)}
	for _, v := range values {
		fmt.Print(v, " ")
	}
	fmt.Println()
}
