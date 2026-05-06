package main

import "math"

// Methods for Counter type
func (c *Counter) Increment() {
	c.value++
}

func (c *Counter) Add(n int) {
	c.value += n
}

func (c Counter) Value() int {
	return c.value
}

// Methods for Point type
func (p Point) Distance(other Point) float64 {
	dx := p.X - other.X
	dy := p.Y - other.Y
	return math.Sqrt(dx*dx + dy*dy)
}

func (p *Point) Move(dx, dy float64) {
	p.X += dx
	p.Y += dy
}
