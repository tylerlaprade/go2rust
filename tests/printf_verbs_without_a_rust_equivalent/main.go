// Literal formats with verbs that have no Rust format of the same output:
// width and precision together ("%8.2f"), %e, %g, %o, "%+5d", "% d", "%.3d",
// %q and others. Each was copied into the Rust format as text, and the
// argument it was for was left over, so the program did not compile. Such a
// format is now formatted when the program runs, as one that is not a
// literal is.
package main

import "fmt"

type Celsius float64

func (c Celsius) String() string { return fmt.Sprintf("%.1f°C", float64(c)) }

func main() {
	fmt.Printf("%05.1f|%8.2f|%-8.3f|%-6s|%6s|%e|%o|%+5d|% d|%x|%X|%6.3g|%3c|%q|%U|%t|%5t|%08b|%5.1f%%|%10v|%-10v|\n", 3.14159, 2.5, 1.0, "ab", "cd", 12345.678, 8, 42, 7, "hi", 255, 3.14159, 'x', 'y', 0x1F600, true, false, 5, 99.5, "v", 3)
	s := fmt.Sprintf("%6.2f|%g|%g|%.3d|%q", 3.14159, 1e21, 0.5, 7, "a\"b")
	fmt.Println(s)
	err := fmt.Errorf("value %8.3f out of range [%d, %d]", 12.5, 0, 10)
	fmt.Println(err)
	fmt.Printf("%-10s|%10.4f|\n", "name", 2.0/3.0)
	var c Celsius = 21.5
	fmt.Printf("%v %s %8.2v|\n", c, c, 1.5)
	for i, row := range [][]float64{{1, 2.5}, {10.25, 300}} {
		fmt.Printf("row %d: %7.2f %7.2f\n", i, row[0], row[1])
	}
}
