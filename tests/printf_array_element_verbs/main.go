// A verb such as %x or %q applies to each element of an array or a slice,
// and to each field of a struct. For an array, the Rust format wrote the
// verb around the printed text, format!("{:x}", ...), which did not compile,
// and a format known only at run time panicked on %q of a [3]string.
package main

import "fmt"

type Pt struct{ X, Y int }

type Name string

func (n Name) String() string { return "N:" + string(n) }

func main() {
	arr := [...]string{2: "c", 0: "a"}
	fmt.Println(len(arr), arr)
	fmt.Println(fmt.Sprintf("%q", arr))
	nums := [3]int{1, 2, 3}
	fmt.Printf("%v %d %x %T\n", nums, nums, nums, nums)
	fmt.Println(fmt.Sprintf("%5v|%v", nums, [2]bool{true}))
	b := [2]byte{'h', 'i'}
	fmt.Printf("%x %X %s %q %v\n", b, b, b, b, b)
	s := []int{10, 255}
	fmt.Printf("%x %X %o %b %03d\n", s, s, s, s, s)
	fl := [2]float64{1.5, 2.25}
	fmt.Printf("%.1f %e %g\n", fl, fl, fl)
	fmt.Printf("%x %q\n", []string{"hi", "yo"}, []string{"a\n"})
	fmt.Printf("%v %+v %d\n", Pt{1, 2}, Pt{3, 4}, Pt{5, 6})
	fmt.Printf("%s %q %x\n", Name("ab"), Name("cd"), Name("e"))
	format := "%x|%-4d|\n"
	fmt.Printf(format, [3]uint8{1, 2, 255}, [2]int{7, 8})
}
