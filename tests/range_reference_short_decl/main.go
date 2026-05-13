package main

import "fmt"

func firstAlias(values []string) string {
	for _, value := range values {
		alias := value
		return alias
	}
	return ""
}

func main() {
	done := make(chan string, 1)
	go func() {
		done <- firstAlias([]string{"alpha"})
	}()
	fmt.Println(<-done)
}
