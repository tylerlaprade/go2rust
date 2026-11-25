// A *T held in one of the program's interfaces prints as Go prints it: its
// String() or Error() when it has one, &{...} for a pointer to a struct,
// with field names for %+v, and <nil> for a nil interface.
package main

import (
	"errors"
	"fmt"
	"strings"
)

type Named interface{ Name() string }
type Sized interface{ Size() int }
type Item interface {
	Named
	Sized
}

type File struct {
	name string
	size int
}

func (f *File) Name() string { return f.name }
func (f *File) Size() int    { return f.size }

type Dir struct{ name string }

func (d *Dir) Name() string   { return d.name }
func (d *Dir) Size() int      { return 0 }
func (d *Dir) String() string { return "dir:" + d.name }

type Problem struct{ code int }

func (p *Problem) Name() string  { return "problem" }
func (p *Problem) Error() string { return fmt.Sprintf("problem %d", p.code) }

type Holder struct{ item Item }

type Counter int

func (c *Counter) Name() string { return "counter" }

func show(n Named) string { return fmt.Sprint(n) }

func makeItem() Item { return &File{"made", 3} }

func main() {
	items := []Item{&File{"a.txt", 10}, &Dir{"src"}}
	for _, it := range items {
		fmt.Println(it, it.Name(), it.Size())
		fmt.Printf("%v|%+v|%s\n", it, it, show(it))
	}
	h := Holder{item: &File{"held", 1}}
	fmt.Println(h.item)
	fmt.Println(makeItem())
	var n Named = &Problem{7}
	fmt.Println(n)
	fmt.Println(fmt.Sprintf("<%v>", n), fmt.Errorf("wrapped: %v", n))
	var c Counter = 4
	var cn Named = &c
	fmt.Println(strings.HasPrefix(fmt.Sprint(cn), "0x"), cn.Name())
	var none Item
	fmt.Println(none)
	err := errors.New("plain")
	fmt.Println(err)
}
