// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Celsius float64

func sumBy(xs []Celsius, f func(Celsius) Celsius) Celsius {
	var total Celsius
	for _, x := range xs {
		total += f(x)
	}
	return total
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	temps := []Celsius{20.5, 22, 19.5}
	fmt.Println(sumBy(temps, func(c Celsius) Celsius { return c * 2 }))
}
