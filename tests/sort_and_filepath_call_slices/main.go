// sort and path/filepath are translated from source, and their own calls of
// slices (sort.Ints is slices.Sort, sort.IntsAreSorted is slices.IsSorted)
// are copies of slices' code written once per instantiation.
package main

import (
	"fmt"
	"path/filepath"
	"sort"
)

func main() {
	x := []int{3, 1, 2}
	fmt.Println(sort.IntsAreSorted(x))
	sort.Ints(x)
	fmt.Println(x, sort.IntsAreSorted(x))
	s := []string{"b", "c", "a"}
	sort.Strings(s)
	fmt.Println(s, sort.StringsAreSorted(s), sort.SearchStrings(s, "b"))
	f := []float64{2.5, -1, 0}
	fmt.Println(sort.Float64sAreSorted(f))
	sort.Float64s(f)
	fmt.Println(f, sort.Float64sAreSorted(f))
	fmt.Println(filepath.Join("a", "b", "../c"), filepath.Clean("/x//y/./z/.."), filepath.Base("/x/y.go"), filepath.Ext("f.txt"), filepath.Dir("a/b/c"))
	rel, err := filepath.Rel("/a/b", "/a/c/d")
	fmt.Println(rel, err)
	ok, _ := filepath.Match("*.go", "main.go")
	fmt.Println(ok, filepath.IsAbs("/x"), filepath.SplitList("a:b"))
}
