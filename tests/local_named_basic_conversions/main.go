package main

import "fmt"

func main() {
	type MyInt int
	type MyString string

	a := MyInt(42)
	b := int(a)
	c := MyInt(b)

	s := MyString("hello")
	t := string(s)
	u := MyString(t)

	fmt.Println(a, b, c, s, t, u)
}
