package main

import "fmt"

// A defined type of a defined map or slice type has the same underlying
// type as the first: it stores the map or slice itself, and none of the
// first type's methods. Conversions between them share the map or slice.

type Lists map[string][]int
type Other Lists
type Third Other

type Nums []int
type More Nums

func (l Lists) Count() int { return len(l) }

func (o Other) Total() int {
	n := 0
	for _, v := range o {
		for _, x := range v {
			n += x
		}
	}
	return n
}

func sum(m More) int {
	n := 0
	for _, x := range m {
		n += x
	}
	return n
}

func mk() Other { return Other{"k": {7}} }

func main() {
	var o Other
	fmt.Println(len(o), o == nil, o["missing"] == nil, o)
	o = Other{"x": {1, 2}}
	o["y"] = append(o["y"], 3)
	fmt.Println(len(o), o["x"], o["y"], o.Total(), o)

	l := Lists(o)
	l["z"] = []int{4}
	fmt.Println(l.Count(), len(o), o.Total(), l)

	var t Third
	fmt.Println(t == nil, len(t))
	t = Third(o)
	delete(t, "x")
	fmt.Println(len(o), len(t))

	var l2 Lists
	fmt.Println(l2 == nil, l2.Count())
	l2 = Lists{}
	fmt.Println(l2 == nil)

	raw := map[string][]int{}
	shared := Lists(raw)
	shared["q"] = nil
	back := map[string][]int(o)
	back["w"] = []int{8}
	fmt.Println(len(raw), len(o), Lists(mk()).Count(), Lists(mk()))

	var m More
	fmt.Println(len(m), m == nil)
	m = More{1, 2}
	m = append(m, 3)
	m[0] = 9
	fmt.Println(len(m), m, sum(m), sum(More(Nums{5, 6})))
}
