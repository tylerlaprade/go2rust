package main

import (
	"fmt"
	"os"
)

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
	fmt.Println([]byte(fmt.Sprintf("<%s|%v|%b|%o|%O|%#x|%X|%c|%q|%U>", invalid, 9, 10, 10, 10, 42, 42, 65, 65, 65)))

	var c capture
	n, err := fmt.Fprintf(&c, "<%s|%v|%b|%o|%O|%#x|%X|%c|%q|%U>", invalid, 9, 10, 10, 10, 42, 42, 65, 65, 65)
	fmt.Println(n, err == nil, c.data)

	n, err = fmt.Fprintf(os.Stdout, "<%s|%v|%b|%o|%O|%#x|%X|%c|%q|%U>\n", invalid, 9, 10, 10, 10, 42, 42, 65, 65, 65)
	fmt.Println(n, err == nil)
}
