package main

import "fmt"

type Record struct {
	Name string
}

func (r Record) Error() string {
	return "record " + r.Name
}

func run() {
	value := Record{Name: "before"}
	var direct any = value
	var err error = value

	value.Name = "after"
	fmt.Println(value.Name, direct.(Record).Name, err.Error())

	asserted := direct.(Record)
	asserted.Name = "asserted"
	fmt.Println(value.Name, direct.(Record).Name, asserted.Name, err.Error())

	comma, ok := direct.(Record)
	comma.Name = "comma"
	fmt.Println(value.Name, direct.(Record).Name, comma.Name, ok, err.Error())
}

func main() {
	run()
}
