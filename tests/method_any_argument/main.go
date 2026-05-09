package main

import "fmt"

type store struct{}

func (s *store) Set(value any) any {
	return value
}

func main() {
	var s store
	seen, _ := s.Set(true).(bool)
	fmt.Println(seen)
}
