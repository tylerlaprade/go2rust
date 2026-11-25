// os.Stdout and os.Stderr passed on as values write to the process's
// output: through a bufio.Writer, an io.Writer parameter, a variable, and
// the File's own Write and WriteString.
package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

func report(w io.Writer, name string, n int) {
	fmt.Fprintf(w, "%s: %d\n", name, n)
}

func main() {
	w := bufio.NewWriter(os.Stdout)
	defer w.Flush()
	fmt.Println("before the buffer")
	fmt.Fprintln(w, "buffered line")
	fmt.Fprintf(w, "buffered %d\n", 2)
	fmt.Println("buffered bytes:", w.Buffered())

	report(os.Stdout, "direct", 1)
	out := os.Stdout
	fmt.Fprintln(out, "through a variable")
	io.WriteString(os.Stdout, "io.WriteString\n")
	os.Stdout.WriteString("WriteString\n")
	n, err := os.Stdout.Write([]byte("Write\n"))
	fmt.Println(n, err)

	e := bufio.NewWriter(os.Stderr)
	fmt.Fprintln(e, "to stderr")
	e.Flush()
	report(os.Stderr, "stderr", 3)
}
