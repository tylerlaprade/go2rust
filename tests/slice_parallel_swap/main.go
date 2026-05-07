package main

import "fmt"

type Names []string

func (names Names) Swap(i, j int) {
	names[i], names[j] = names[j], names[i]
}

func (names Names) At(i int) string {
	return names[i]
}

func main() {
	names := Names{"ada", "grace", "lin"}
	names.Swap(0, 2)
	fmt.Println(names.At(0), names.At(2))
}
