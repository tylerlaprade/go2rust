package main

import "fmt"

func main() {
	var f any = func() (string, error) {
		return "ok", nil
	}

	s, err := f.(func() (string, error))()
	fmt.Println(s, err == nil)
}
