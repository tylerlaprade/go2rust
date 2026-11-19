package main

import "fmt"

type Point struct{ X, Y int }

type IntList []int

type Registry map[string]int

type Scale func(int) int

type Holder struct {
	V interface{}
}

// classify reports the dynamic type of a boxed value. A pointer, slice, map
// or function has to come back out of the interface as the same kind of thing
// that went in.
func classify(value interface{}) string {
	switch v := value.(type) {
	case int:
		return fmt.Sprintf("int %d", v)
	case string:
		return "string " + v
	case []int:
		return fmt.Sprintf("[]int len %d", len(v))
	case []string:
		return fmt.Sprintf("[]string len %d", len(v))
	case map[string]int:
		return fmt.Sprintf("map len %d", len(v))
	case *Point:
		return fmt.Sprintf("*Point %d", v.X)
	case Point:
		return fmt.Sprintf("Point %d", v.X)
	case IntList:
		return fmt.Sprintf("IntList len %d", len(v))
	case Registry:
		return fmt.Sprintf("Registry len %d", len(v))
	case func() int:
		return fmt.Sprintf("func %d", v())
	case nil:
		return "nil"
	default:
		return fmt.Sprintf("other %T", v)
	}
}

func typeName(value interface{}) string {
	return fmt.Sprintf("%T", value)
}

func boxSlice(xs []int) interface{} {
	return xs
}

func boxWithError(xs []int) (interface{}, error) {
	return xs, nil
}

func boxNamedResult(xs []int) (result interface{}) {
	result = xs
	return
}

func classifyAll(args ...interface{}) string {
	out := ""
	for _, arg := range args {
		switch arg.(type) {
		case []int:
			out += "[]int "
		case map[string]int:
			out += "map "
		case *Point:
			out += "*Point "
		default:
			out += "other "
		}
	}
	return out
}

type Pair struct {
	Left, Right interface{}
}

type Shelf struct {
	Items IntList
}

func (h *Holder) set(v interface{}) {
	h.V = v
}

func boxPointer(p *Point) interface{} {
	return p
}

func sum(xs []int) int {
	total := 0
	for _, x := range xs {
		total += x
	}
	return total
}

func main() {
	xs := []int{1, 2, 3}
	names := []string{"a", "b"}
	counts := map[string]int{"a": 1}
	p := &Point{7, 8}
	pv := Point{9, 10}
	list := IntList{4, 5}
	reg := Registry{"x": 1, "y": 2}

	// Every way of putting a value into an interface.
	fmt.Println("argument:", classify(xs), "|", classify(names), "|", classify(counts))
	fmt.Println("argument:", classify(p), "|", classify(pv), "|", classify(func() int { return 5 }))
	fmt.Println("literal:", classify([]int{4, 5}), "|", classify(&Point{1, 2}), "|", classify(map[string]int{}))
	fmt.Println("defined:", classify(list), "|", classify(reg), "|", classify(IntList{6}))
	fmt.Println("scalar:", classify(42), "|", classify("hi"), "|", classify(3.5))

	var declared interface{} = xs
	fmt.Println("declared:", classify(declared))
	var assigned interface{}
	fmt.Println("unset:", classify(assigned))
	assigned = counts
	fmt.Println("assigned:", classify(assigned))
	assigned = p
	fmt.Println("assigned:", classify(assigned))
	assigned = pv
	fmt.Println("assigned:", classify(assigned))
	fmt.Println("returned:", classify(boxSlice(xs)), "|", classify(boxPointer(p)))

	both, _ := boxWithError(xs)
	fmt.Println("returned with error:", classify(both), "|", classify(boxNamedResult(xs)))
	fmt.Println("converted:", classify(any(xs)), "|", classify(any(counts)), "|", classify(any(p)))
	fmt.Println("variadic:", classifyAll(xs, p, counts, 1))
	pair := Pair{xs, p}
	fmt.Println("positional field:", classify(pair.Left), "|", classify(pair.Right))
	viaMethod := &Holder{}
	viaMethod.set(counts)
	fmt.Println("method argument:", classify(viaMethod.V))
	produce := func() interface{} { return xs }
	fmt.Println("closure result:", classify(produce()))

	// A nil value of a defined slice type is still a value of that type.
	var noList IntList
	shelf := Shelf{}
	fmt.Println("nil defined:", classify(noList), "|", classify(shelf.Items))
	shelf.Items = IntList{1}
	fmt.Println("defined field:", classify(shelf.Items))

	holder := Holder{V: xs}
	fmt.Println("field:", classify(holder.V))
	holder.V = p
	fmt.Println("field:", classify(holder.V))

	byName := map[string]interface{}{"xs": xs, "p": p}
	fmt.Println("map value:", classify(byName["xs"]), "|", classify(byName["p"]))

	items := []interface{}{xs, counts, p, pv, list, 7}
	for _, item := range items {
		switch v := item.(type) {
		case []int:
			fmt.Println("element: []int", len(v))
		case map[string]int:
			fmt.Println("element: map", len(v))
		case *Point:
			fmt.Println("element: *Point", v.X)
		case Point:
			fmt.Println("element: Point", v.X)
		case IntList:
			fmt.Println("element: IntList", len(v))
		default:
			fmt.Println("element: other")
		}
	}
	items = append(items, names)
	appended := items[len(items)-1].([]string)
	fmt.Println("appended:", len(appended), appended[0])

	// Assertions, in both forms.
	if got, ok := declared.([]int); ok {
		fmt.Println("comma-ok slice:", len(got), got[0])
	}
	if _, ok := declared.([]string); !ok {
		fmt.Println("comma-ok slice: not []string")
	}
	if _, ok := declared.(IntList); !ok {
		fmt.Println("comma-ok slice: not IntList")
	}
	var boxedMap interface{} = counts
	if got, ok := boxedMap.(map[string]int); ok {
		fmt.Println("comma-ok map:", got["a"])
	}
	var boxedPointer interface{} = p
	if got, ok := boxedPointer.(*Point); ok {
		fmt.Println("comma-ok pointer:", got.Y)
	}
	if _, ok := boxedPointer.(Point); !ok {
		fmt.Println("comma-ok pointer: not Point")
	}
	var boxedList interface{} = list
	if got, ok := boxedList.(IntList); ok {
		fmt.Println("comma-ok defined:", len(got))
	}
	if _, ok := boxedList.([]int); !ok {
		fmt.Println("comma-ok defined: not []int")
	}

	plain := declared.([]int)
	fmt.Println("single slice:", len(plain), plain[2], sum(declared.([]int)))
	lookup := boxedMap.(map[string]int)
	fmt.Println("single map:", len(lookup), lookup["a"])
	first := items[0].([]int)
	second := items[1].(map[string]int)
	third := items[2].(*Point)
	fmt.Println("single element:", len(first), second["a"], third.X)
	var empty []int
	var boxedEmpty interface{} = empty
	nothing := boxedEmpty.([]int)
	fmt.Println("single nil slice:", len(nothing))

	// The boxed value is the same slice and the same pointee, not a copy.
	if shared, ok := declared.([]int); ok {
		shared[0] = 99
	}
	fmt.Println("shared slice:", xs[0])
	switch shared := boxedPointer.(type) {
	case *Point:
		shared.X = 70
	}
	fmt.Println("shared pointer:", p.X)

	// A function value is called through the interface, and keeps its state.
	var scale Scale = func(x int) int { return x * 10 }
	var boxedScale interface{} = scale
	fmt.Println("call defined func:", boxedScale.(Scale)(4))
	calls := 0
	var counter interface{} = func() int {
		calls++
		return calls
	}
	counter.(func() int)()
	fmt.Println("call func:", counter.(func() int)(), calls)

	// An argument may itself call the same boxed function. Go finishes the
	// inner call before the outer one starts.
	var step interface{} = func(n int) int {
		fmt.Println("step", n)
		return n + 1
	}
	fmt.Println("nested call:", step.(func(int) int)(step.(func(int) int)(1)))

	// An element of a slice of slices, and the loop variable over one, is
	// still a []int when it is put in an interface. So is a map's value.
	matrix := [][]int{{1}, {2, 3}}
	fmt.Println("element:", classify(matrix[1]))
	for _, row := range matrix {
		fmt.Println("range value:", classify(row))
	}
	var firstRow interface{} = matrix[0]
	if row, ok := firstRow.([]int); ok {
		fmt.Println("asserted element:", len(row))
	}
	rowsByName := map[string][]int{"k": {4, 5, 6}}
	fmt.Println("slice in map:", classify(rowsByName["k"]))
	for _, value := range rowsByName {
		fmt.Println("slice in map, range value:", classify(value))
	}
	tables := []map[string]int{{"a": 1, "b": 2}}
	fmt.Println("slice of maps:", classify(tables[0]))
	for _, table := range tables {
		fmt.Println("range over maps:", classify(table))
	}

	// %T names the dynamic type, and the static type of a typed variable.
	fmt.Println("dynamic:", typeName(xs), typeName(names), typeName(counts))
	fmt.Println("dynamic:", typeName(p), typeName(pv), typeName([]float64{1}))
	fmt.Println("dynamic:", typeName(7), typeName("s"), typeName(true), typeName(2.5))
	fmt.Println("unmatched:", classify([]float64{1}), "|", classify(map[string]bool{}))
	var none []int
	var nowhere *Point
	fmt.Printf("static: %T %T %T %T %T\n", xs, counts, p, pv, list)
	fmt.Printf("static nil: %T %T\n", none, nowhere)
}
