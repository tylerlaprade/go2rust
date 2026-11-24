// %s prints a []byte as its text in a format the transpiler lowers to
// print!, with and without a width, in Printf and Sprintf; %x, %X and %q
// stay as they were.
package main

import "fmt"


func main() {
	bs := []byte("abc")
	fmt.Printf("%s\n", bs)
	fmt.Printf("%x\n", bs)
	fmt.Printf("%X\n", bs)
	fmt.Printf("%q\n", bs)
	fmt.Printf("%5s|%-5s|\n", bs, bs)
	s := fmt.Sprintf("<%s>", bs[1:])
	fmt.Println(s)
	fmt.Printf("%s and %d\n", bs, len(bs))
}
