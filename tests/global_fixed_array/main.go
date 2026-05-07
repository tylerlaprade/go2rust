package main

import "fmt"

var labels [40]string

func init() {
	for i := range labels {
		labels[i] = fmt.Sprintf("label-%d", i)
	}
}

func main() {
	fmt.Println(labels[0], labels[1], labels[39])
}
