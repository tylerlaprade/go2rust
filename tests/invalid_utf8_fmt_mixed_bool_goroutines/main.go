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
	done := make(chan struct{})
	go func() { close(done) }()
	<-done

	invalid := string([]byte{0xfa})
	fmt.Println([]byte(fmt.Sprintf("<%s|%t>", invalid, true)))

	var c capture
	n, err := fmt.Fprintf(&c, "<%s|%t>", invalid, true)
	fmt.Println(n, err == nil, c.data)
}
