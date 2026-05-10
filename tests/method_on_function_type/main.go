package main

import "fmt"

type formatter func(string) string

func (f formatter) Format(path string) string {
	return f(path)
}

func main() {
	f := formatter(func(path string) string {
		return "pkg:" + path
	})
	fmt.Println(f.Format("fmt"))
}
