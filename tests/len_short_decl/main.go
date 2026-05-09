package main

import "fmt"

func take(x int) {
	fmt.Println(x)
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	values := []string{"alpha", "beta", "gamma"}
	i := len(values)
	take(i)
	fmt.Println(values[i-1])
}
