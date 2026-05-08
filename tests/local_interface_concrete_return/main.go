package main

import "fmt"

type Reader interface {
	Read() int
}

type counter struct {
	n int
}

func (c *counter) Read() int {
	return c.n
}

func NewReader() Reader {
	return &counter{n: 7}
}

type Valuer interface {
	Value() int
}

type number struct {
	n int
}

func (n number) Value() int {
	return n.n
}

func NewValuer() Valuer {
	return number{n: 11}
}

func main() {
	reader := NewReader()
	valuer := NewValuer()
	fmt.Println(reader.Read(), valuer.Value())
}
