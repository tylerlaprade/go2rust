package main

import "fmt"

type Loader struct{}

func (l *Loader) Load(patterns []string) int {
	return len(patterns)
}

func main() {
	loader := &Loader{}
	fmt.Println(loader.Load([]string{"."}))
}
