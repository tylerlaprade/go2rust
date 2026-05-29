package main

type Counter struct{ n int }

func (c *Counter) Step() { c.n++ }
