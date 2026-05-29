package main

type Circle struct{ r int }

func (c Circle) Area() int { return c.r * c.r }

func newCircle(r int) Shape { return Circle{r: r} }
