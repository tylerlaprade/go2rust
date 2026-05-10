package main

import "fmt"

type Holder struct {
	Items []string
}

func main() {
	holder := Holder{Items: []string{"beta", "gamma"}}
	values := []string{"alpha"}
	values = append(values, holder.Items...)
	fmt.Println(len(values), values[2])
}
