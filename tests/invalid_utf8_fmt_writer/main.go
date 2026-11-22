package main

import "fmt"

type capture struct {
	data []byte
}

func (c *capture) Write(p []byte) (int, error) {
	c.data = append(c.data, p...)
	return len(p), nil
}

func main() {
	invalid := string([]byte{0xfa})
	c := &capture{}

	n, err := fmt.Fprint(c, "A", invalid)
	fmt.Println(n, err == nil, c.data)

	c.data = nil
	n, err = fmt.Fprintf(c, "<%s>", invalid)
	fmt.Println(n, err == nil, c.data)

	c.data = nil
	n, err = fmt.Fprintln(c, invalid, "B")
	fmt.Println(n, err == nil, c.data)
}
