package main

import "fmt"

// Maps of interface{} values box what goes in, from a literal or an
// assignment, and print each value as fmt does. A struct field holding a map
// prints as the map does when printed by itself.

type Point struct{ X, Y int }

type Record struct {
	ID     int
	Fields map[string]any
	Lists  map[string][]int
	Nested map[string]map[string]int
	Pairs  map[[2]int]string
}

func describe(v any) string {
	switch x := v.(type) {
	case []int:
		return fmt.Sprintf("ints %v", x)
	case []any:
		return fmt.Sprintf("anys %d", len(x))
	case map[string]int:
		return fmt.Sprintf("counts %d", len(x))
	case Point:
		return fmt.Sprintf("point %d", x.X+x.Y)
	case nil:
		return "nil"
	default:
		return fmt.Sprintf("%T %v", x, x)
	}
}

func main() {
	m := map[string]any{"b": []int{1, 2}, "a": true, "c": map[string]int{"k": 1}, "d": Point{3, 4}, "e": []any{"x", 2}, "f": nil}
	fmt.Println(m, len(m))
	m["g"] = 2.5
	m["h"] = "text"
	m["i"] = []string{"p", "q"}
	set := func(k string, v any) { m[k] = v }
	set("j", 7)
	for _, k := range []string{"a", "b", "c", "d", "e", "f", "j"} {
		fmt.Println(k, describe(m[k]))
	}
	if xs, ok := m["b"].([]int); ok {
		fmt.Println("b[1] =", xs[1])
	}
	fmt.Printf("%v\n%d %T\n", m, len(m), m["e"])

	var x any = []any{1, "two", []any{3}}
	y := x
	fmt.Println(x, y)

	var empty map[string]any
	fmt.Println(empty, len(empty), empty["missing"] == nil)

	r := Record{
		ID:     1,
		Fields: map[string]any{"n": 2, "s": "x"},
		Lists:  map[string][]int{"x": {1, 2}, "n": nil},
		Nested: map[string]map[string]int{"o": {"i": 1}},
		Pairs:  map[[2]int]string{{1, 2}: "p"},
	}
	r.Fields["t"] = true
	fmt.Println(r)
	fmt.Printf("%+v\n", r)
}
