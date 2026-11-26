// Go's os.Stdout is not buffered, so a line without a newline is out
// before the program reads stdin or writes to stderr: a prompt shows before
// its input is read, and a partial line comes before what stderr gets next.
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	fmt.Print("Name: ")
	var name string
	fmt.Scanln(&name)
	fmt.Printf("Hello, %s! Age? ", name)
	in := bufio.NewScanner(os.Stdin)
	in.Scan()
	fmt.Fprint(os.Stdout, "got ", in.Text())
	fmt.Fprintln(os.Stderr, " [stderr]")
	fmt.Print("partial ")
	fmt.Fprint(os.Stderr, "then stderr")
	fmt.Println(" done")
}
