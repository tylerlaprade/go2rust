package main

import "fmt"

// A package-level map variable holds a map, as a local one does: a copy of
// it, an argument made from it, a struct field set from it and a pointer to
// it share the map, and assigning another map to it leaves the old map with
// its other copies.

var routes = map[string]int{}

var cache map[string]string

var counts = map[string]int{"a": 1, "b": 2}

type Server struct{ routes map[string]int }

func register(name string, n int) { routes[name] = n }

func add(m map[string]int, k string) { m[k] = len(m) }

func load() (map[string]int, int) { return map[string]int{"l": 9}, 2 }

var loaded int

func keys(m map[string]int) []string {
	out := []string{}
	for k := range m {
		out = append(out, k)
	}
	for i := 1; i < len(out); i++ {
		for j := i; j > 0 && out[j] < out[j-1]; j-- {
			out[j], out[j-1] = out[j-1], out[j]
		}
	}
	return out
}

func init() {
	cache = make(map[string]string)
}

func main() {
	register("x", 1)
	add(routes, "w")
	s := Server{routes: routes}
	s.routes["y"] = 2
	fmt.Println(keys(routes), len(s.routes))

	cache["k"] = "v"
	c := cache
	c["j"] = "w"
	fmt.Println(len(cache), cache["j"])

	old := counts
	local := map[string]int{"m": 1}
	counts = local
	local["n"] = 2
	fmt.Println(keys(counts), keys(old))
	counts = nil
	fmt.Println(counts == nil, len(local), len(old))
	counts = map[string]int{}
	counts["z"] = 1
	fmt.Println(keys(counts), keys(local))

	p := &routes
	(*p)["z"] = 3
	fmt.Println(len(routes), routes["z"])

	delete(routes, "x")
	fmt.Println(keys(routes), keys(s.routes))

	before := counts
	counts, loaded = nil, 0
	fmt.Println(keys(before), counts == nil, loaded)
	counts, loaded = local, 1
	local["o"] = 3
	fmt.Println(keys(counts), loaded)
	counts, loaded = load()
	counts["p"] = 4
	fmt.Println(keys(counts), loaded, keys(local))
}
