package main

import "fmt"

func echo(s string) string {
	return s
}

func main() {
	names := []string{"alpha", "beta"}
	for _, name := range names {
		func() {
			fmt.Println(echo(name))
		}()
	}
}
