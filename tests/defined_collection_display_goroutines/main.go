package main

import "fmt"

// A goroutine makes every variable a locked one.

// Defined map types whose values are slices, arrays and structs, and defined
// collections held in interfaces, print as fmt does wherever they sit.

type Lists map[string][]int

type Pairs map[string][2]int

type Point struct{ X, Y int }

type Places map[string]Point

type Counts map[string]int

type Ints []int

type Pair [2]int

type Holder struct {
	L Lists
	P Places
}

func main() {
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	lists := Lists{"b": {1, 2}, "a": nil}
	pairs := Pairs{"k": {3, 4}}
	places := Places{"p": {1, 2}}

	// Directly, and as slice elements, struct fields and map values.
	fmt.Println(lists, pairs, places)
	fmt.Println([]Lists{lists}, []Pairs{pairs}, []Places{places})
	fmt.Println(Holder{L: lists, P: places})
	fmt.Println(map[string]Lists{"x": lists}, map[string]Places{"y": places})
	fmt.Printf("%v %d\n", []Lists{lists, nil}, len(lists))

	// A map literal shares the map a variable holds, as Go's copy of the
	// map header does.
	counts := Counts{"a": 1}
	byName := map[string]Counts{"c": counts}
	counts["b"] = 2
	fmt.Println(len(byName["c"]), byName)
	ints := Ints{5}
	bySlice := map[string]Ints{"i": ints}
	ints[0] = 6
	fmt.Println(bySlice)

	// Defined collections held in interfaces.
	values := []any{counts, ints, Pair{7, 8}, lists, places}
	for _, v := range values {
		fmt.Println(v)
	}
	var x any = pairs
	fmt.Printf("%v\n", x)
}
