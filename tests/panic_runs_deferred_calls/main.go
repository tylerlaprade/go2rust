// A panic that nothing recovers runs every deferred call on the way out,
// then ends the program with "panic: " and the value on stderr and exit
// status 2. The goroutine trace Go prints after the message is not
// compared.
package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
)

func inner() {
	defer fmt.Println("inner deferred")
	panic(errors.New("boom"))
}

func middle() int {
	defer func() { fmt.Println("middle deferred") }()
	inner()
	fmt.Println("not reached")
	return 1
}

func main() {
	w := bufio.NewWriter(os.Stdout)
	defer w.Flush()
	fmt.Fprintln(w, "buffered until the deferred Flush")
	defer fmt.Println("main deferred")
	fmt.Println("start")
	fmt.Println(middle())
}
