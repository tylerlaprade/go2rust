package main

import "fmt"

type K struct{ x, y int }

type Outer struct {
	K    K
	Name string
}

func main() {
	m := map[K]int{{3, 4}: 7}
	for k, v := range m {
		k.x = 9
		_ = v
	}
	fmt.Println(m[K{3, 4}], len(m))
	for k := range m {
		k.y++
		fmt.Println(k)
	}
	fmt.Println(m)
	u := map[struct{ a, b int }]string{{1, 2}: "u"}
	for k := range u {
		k.a = 5
	}
	fmt.Println(u)
	o := map[Outer]bool{{K{1, 1}, "o"}: true}
	for k := range o {
		k.K.x = 2
		k.Name = "changed"
	}
	fmt.Println(o)
	saved := []K{}
	for k := range m {
		saved = append(saved, k)
	}
	saved[0].x = 100
	fmt.Println(m, saved)
}
