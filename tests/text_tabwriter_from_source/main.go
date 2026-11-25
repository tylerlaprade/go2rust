// text/tabwriter translated from its Go source; the stub's methods panicked.
// Writing a cell appends to the last line through a pointer to an element of
// a [][]cell, and a new line grows b.lines[:n] within its capacity, which in
// Go are zero.
package main

import (
	"fmt"
	"os"
	"strings"
	"text/tabwriter"
)

func main() {
	w := tabwriter.NewWriter(os.Stdout, 0, 8, 1, ' ', 0)
	fmt.Fprintln(w, "NAME\tAGE\tCITY")
	fmt.Fprintln(w, "Ann\t30\tParis")
	fmt.Fprintln(w, "Bartholomew\t5\tRome")
	w.Flush()

	tw := tabwriter.NewWriter(os.Stdout, 6, 4, 2, '.', tabwriter.AlignRight|tabwriter.Debug)
	fmt.Fprintf(tw, "a\tbbb\tc\n")
	fmt.Fprintf(tw, "aaaa\tb\tccccc\n")
	tw.Flush()

	var sb strings.Builder
	t := tabwriter.NewWriter(&sb, 4, 4, 1, ' ', tabwriter.TabIndent)
	for i := 1; i <= 3; i++ {
		fmt.Fprintf(t, "%s\t%d\t%v\n", strings.Repeat("x", i*3), i*i, i%2 == 0)
	}
	t.Flush()
	fmt.Print(sb.String())
	fmt.Println(strings.Count(sb.String(), "\n"))
}
