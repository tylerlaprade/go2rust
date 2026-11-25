// In a program with goroutines: slices, maps and cmp translated from their
// Go source: each generic
// function the program calls is written once per instantiation, so a
// []*Task, a []Item, a named slice and a named float are ordinary slices of
// their elements. Insert and Replace with overlapping values, Delete's
// zeroed tail, NaN ordering, and the iterator functions.
package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
)

type Celsius float64

type IDs []int

type Item struct {
	Name  string
	Price int
}

type Task struct {
	Name string
	Pri  int
}

func lower(s string) string {
	b := []byte(s)
	for i, c := range b {
		if 'A' <= c && c <= 'Z' {
			b[i] = c + 'a' - 'A'
		}
	}
	return string(b)
}

func main() {
	// Insert and Replace, inside the capacity and past it, with values that
	// overlap the slice they go into.
	s := make([]int, 5, 10)
	for i := range s {
		s[i] = i + 1
	}
	s = slices.Insert(s, 2, 10, 11)
	fmt.Println(s, len(s), cap(s))
	s = slices.Insert(s, 1, s[4:6]...)
	fmt.Println(s)
	s = slices.Replace(s, 1, 3, 7, 8, 9)
	fmt.Println(s)
	s = slices.Replace(s, 0, 2, s[3:7]...)
	fmt.Println(s)
	ins := slices.Insert([]int{1, 4}, 1, 2, 3)
	del := slices.Delete(ins, 0, 2)
	fmt.Println(ins, del)
	fmt.Println(slices.Compact([]int{1, 1, 2, 2, 2, 3, 1}), slices.CompactFunc([]string{"a", "A", "b"}, func(a, b string) bool { return lower(a) == lower(b) }))
	fmt.Println(slices.Concat([]int{1}, nil, []int{2, 3}), slices.Repeat([]string{"x", "y"}, 3))
	g := slices.Grow([]int{1}, 10)
	fmt.Println(len(g), cap(g) >= 11, len(slices.Clip(g)), cap(slices.Clip(g)))
	for c := range slices.Chunk([]int{1, 2, 3, 4, 5}, 2) {
		fmt.Print(c, " ")
	}
	fmt.Println()
	for i, v := range slices.Backward([]string{"a", "b", "c"}) {
		fmt.Print(i, v, " ")
	}
	fmt.Println()
	fmt.Println(slices.Compare([]int{1, 2}, []int{1, 3}), slices.Compare([]string{"b"}, []string{"a", "z"}))
	fmt.Println(slices.EqualFunc([]int{1, 2}, []string{"1", "2"}, func(a int, b string) bool { return fmt.Sprint(a) == b }))

	// Structs, sorted stably and searched.
	items := []Item{{"pen", 3}, {"cup", 5}, {"ink", 3}, {"box", 1}}
	slices.SortStableFunc(items, func(a, b Item) int { return cmp.Compare(a.Price, b.Price) })
	fmt.Println(items, slices.IsSortedFunc(items, func(a, b Item) int { return a.Price - b.Price }))
	n, ok := slices.BinarySearchFunc(items, 5, func(it Item, p int) int { return cmp.Compare(it.Price, p) })
	fmt.Println(n, ok)
	fmt.Println(slices.MinFunc(items, func(a, b Item) int { return cmp.Compare(a.Name, b.Name) }), slices.MaxFunc(items, func(a, b Item) int { return a.Price - b.Price }))
	items = slices.DeleteFunc(items, func(it Item) bool { return it.Price == 3 })
	fmt.Println(items, slices.Index(items, Item{"cup", 5}), slices.Contains(items, Item{"pen", 3}))

	// Pointers: the slice holds the program's own *Task values.
	tasks := []*Task{{"write", 2}, {"test", 1}, {"ship", 3}}
	slices.SortFunc(tasks, func(a, b *Task) int { return cmp.Compare(a.Pri, b.Pri) })
	for _, t := range tasks {
		fmt.Print(t.Name, " ")
	}
	fmt.Println()
	i := slices.IndexFunc(tasks, func(t *Task) bool { return t.Name == "ship" })
	tasks[0].Pri = 10
	fmt.Println(i, tasks[i].Pri, slices.ContainsFunc(tasks, func(t *Task) bool { return t.Pri == 10 }), slices.Index(tasks, tasks[1]))

	// Named types and floats.
	temps := []Celsius{21.5, -3, 37, 0}
	slices.Sort(temps)
	fmt.Println(temps, slices.Max(temps), slices.Min(temps), cmp.Less(temps[0], temps[1]))
	ids := IDs{9, 4, 7}
	slices.Sort(ids)
	idx, found := slices.BinarySearch(ids, 7)
	fmt.Printf("%v %T %d %v\n", ids, slices.Clone(ids), idx, found)
	zero := 0.0
	nan, negInf := zero/zero, -1/zero
	fl := []float64{3, nan, 1, negInf}
	slices.Sort(fl)
	fmt.Println(fl, cmp.Compare(nan, 1.0), cmp.Or(0, 0, 4, 2), cmp.Or("", "x"))
	big := make([]int, 200)
	for i := range big {
		big[i] = (i * 7919) % 200
	}
	slices.Sort(big)
	fmt.Println(big[:5], big[195:], slices.IsSorted(big))
	slices.SortFunc(big, func(a, b int) int { return b - a })
	fmt.Println(big[:3])
	slices.Reverse(big)
	fmt.Println(big[:3], len(big))

	// maps, and the iterators between maps and slices.
	m := map[string]int{"a": 1, "b": 2, "c": 3}
	fmt.Println(slices.Sorted(maps.Keys(m)), slices.Sorted(maps.Values(m)))
	fmt.Println(maps.Collect(slices.All([]string{"x", "y"})))
	maps.Copy(m, map[string]int{"d": 4, "a": 10})
	maps.DeleteFunc(m, func(k string, v int) bool { return v%2 == 0 })
	fmt.Println(m, maps.EqualFunc(m, map[string]int{"c": 3}, func(a, b int) bool { return a == b }))
	maps.Insert(m, maps.All(map[string]int{"z": 26}))
	m2 := maps.Clone(m)
	m2["y"] = 25
	fmt.Println(len(m), len(m2), m2["z"], maps.Equal(m, m2))
	var nilMap map[string]int
	fmt.Println(maps.Clone(nilMap) == nil, slices.Collect(maps.Keys(nilMap)) == nil)

	done := make(chan []int)
	go func() {
		w := []int{5, 3, 9}
		slices.Sort(w)
		done <- slices.Insert(w, 1, w[1:]...)
	}()
	fmt.Println(<-done)
}
