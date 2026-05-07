package main

import "fmt"

type Names []string

func (names Names) Len() int {
	return len(names)
}

func (names Names) First() string {
	return names[0]
}

func (names Names) Join() string {
	out := ""
	for i, name := range names {
		if i > 0 {
			out += ","
		}
		out += name
	}
	return out
}

func main() {
	names := Names{"ada", "grace"}
	fmt.Println("Len:", names.Len())
	fmt.Println("First:", names.First())
	fmt.Println("Join:", names.Join())
}
