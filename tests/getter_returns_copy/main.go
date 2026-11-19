package main

import (
	"fmt"
	"strings"
)

type Point struct{ X, Y int }

type User struct {
	name string
	pos  Point
}

func (u *User) Name() string { return u.name }

func (u *User) Pos() Point { return u.pos }

func (u User) Label() string { return u.name }

func nameOf(u *User) string { return u.name }

// A method that returns a field returns a copy of it. The caller's variable
// is its own: changing it does not change the struct.
func main() {
	u := &User{name: "ann", pos: Point{1, 2}}

	a := u.Name()
	a = strings.ToUpper(a)
	fmt.Println(a, u.name)

	b := u.Name()
	b += "!"
	fmt.Println(b, u.name)

	c := u.name
	c += "?"
	fmt.Println(c, u.name)

	d := nameOf(u)
	d = "other"
	fmt.Println(d, u.name)

	e := u.Label()
	e = "label"
	fmt.Println(e, u.name)

	p := u.Pos()
	p.X = 100
	fmt.Println(p, u.pos)
}
