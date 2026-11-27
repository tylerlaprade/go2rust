package main

import "fmt"

type Point struct{ X, Y int }

func main() {
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	// A slice of maps as a map value, with a nil map element.
	byGroup := map[string][]map[string]int{
		"a": {{"x": 1, "y": 2}, nil},
		"b": nil,
	}
	fmt.Println(byGroup)
	fmt.Printf("%v %d\n", byGroup["a"], len(byGroup["a"][1]))

	// Replacing a group's slice; appending to a slice of maps and writing
	// into a map in one are #318.
	byGroup["b"] = []map[string]int{{"z": 3}}
	fmt.Println(byGroup)

	// A goroutine adds a group and reports the map it sees.
	done := make(chan string)
	go func() {
		byGroup["c"] = []map[string]int{{"q": 9}, nil}
		done <- fmt.Sprint(byGroup)
	}()
	fmt.Println(<-done)

	// Inner maps of slices print a nil slice as [].
	tags := map[string][]map[string][]string{
		"docs": {{"go": {"spec", "faq"}, "rust": nil}},
	}
	fmt.Println(tags)

	// Inner maps of structs.
	shapes := map[int][]map[string]Point{
		1: {{"origin": {0, 0}}, {"corner": {3, 4}}},
	}
	fmt.Println(shapes)
	fmt.Println(map[string][]map[string]int{})
}
