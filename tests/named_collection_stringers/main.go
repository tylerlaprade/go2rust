// A named slice, array or map type with a String method prints through it,
// wherever it is printed from.
package main

import "fmt"

type Grid [][]rune

func (g Grid) String() string {
	s := ""
	for _, row := range g {
		s += string(row) + "/"
	}
	return s
}

type Tags []string

func (t Tags) String() string { return fmt.Sprint(len(t), " tags") }

type Counts map[string]int

func (c Counts) String() string { return fmt.Sprint(len(c), " counts") }

type Triple [3]int

func (t Triple) String() string { return fmt.Sprintf("<%d,%d,%d>", t[0], t[1], t[2]) }

type Board struct {
	cells Grid
	tags  Tags
}

func makeTags() Tags { return Tags{"x", "y", "z"} }

func main() {
	g := Grid{[]rune("ab"), []rune("cd")}
	g[1][0] = 'X'
	t := Tags{"a", "b"}
	c := Counts{"x": 1}
	tr := Triple{1, 2, 3}
	fmt.Print(g, "\n")
	fmt.Println(g, t, c, tr)
	fmt.Printf("%v %s %v %v\n", g, t, c, tr)
	s := fmt.Sprint(t, "|", tr)
	fmt.Println(s, fmt.Sprintf("%v", c))
	b := Board{cells: g, tags: t}
	fmt.Println(b.cells, b.tags, makeTags())
	err := fmt.Errorf("bad %v", makeTags())
	fmt.Println(err)
	var x any = t
	var y any = c
	fmt.Println(x, y)
}
