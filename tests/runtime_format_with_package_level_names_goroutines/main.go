// The same program with a goroutine, so the helpers take their concurrent form.
// A package-level variable or constant may have any name, including a name
// the generated formatting code uses for one of its own locals. A Go package
// with "var prefix" used to stop the generated crate from compiling, which
// broke every program that translates go/scanner from source.
package main

import "fmt"

var prefix = "line "
var verb = 'v'
var text = "t"
var spec, out, args, format, digits, sign = 1, 2, 3, 4, 5, 6
var chars, next, items, fill, magnitude, negative = 7, 8, 9, 10, 11, false

const width = 4
const precision = 2

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	f := "%s%d %q %5.2f|%-4s|%x\n"
	fmt.Printf(f, prefix, width, text, 3.14159, "ab", 255)
	g := "%v %c %v %+d\n"
	fmt.Printf(g, verb, verb, width+precision, digits)
	h := "%v %v %v %v %v %v\n"
	fmt.Printf(h, spec, out, args, format, digits, sign)
	fmt.Printf(h, chars, next, items, fill, magnitude, negative)
	fmt.Println(fmt.Sprintf(f, "x", 1, "y", 2.5, "z", 16))
}
