package main

import "fmt"

func main() {
	values := []string{"alpha", "beta"}
	ch := make(chan string, len(values))
	for _, value := range values {
		ch <- value
	}
	close(ch)

	for value := range ch {
		fmt.Println(value)
	}
}
