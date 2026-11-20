// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Level int

type Name string

type Celsius float64

func (c Celsius) String() string { return fmt.Sprintf("%.1f°C", float64(c)) }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
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
