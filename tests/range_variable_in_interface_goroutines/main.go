// The same program with a goroutine, so every value is behind a lock.
// A range variable put in an interface{}: the interface value holds its own
// copy. Over a slice the generated range variable is a reference into the
// slice, and it used to be boxed as that reference, which does not live long
// enough: "var x any = s", a ...any argument and a format known only at run
// time all failed to compile.
package main

import "fmt"

type item struct{ name string }

func show(args ...any) { fmt.Println(args...) }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	format := "%s (first)\n"
	for _, dir := range []string{"a", "b"} {
		fmt.Printf(format, dir)
		format = "%s\n"
		show(dir, 1)
		var x any = dir
		fmt.Println(x)
	}

	f := "%d:%v\n"
	for i, it := range []item{{"x"}, {"y"}} {
		fmt.Printf(f, i, it.name)
		fmt.Print(fmt.Sprintf(f, i, it))
		var boxed any = it
		it.name = "changed"
		fmt.Println(boxed, it.name)
	}

	m := map[string]int{"k": 1}
	for k, v := range m {
		fmt.Printf(f, v, k)
		show(k, v)
	}

	for _, n := range []int{3, 4} {
		show(n)
	}
}
