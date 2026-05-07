package main

import (
	"fmt"
	"iter"
)

func words() iter.Seq[string] {
	return func(yield func(string) bool) {
		if !yield("go") {
			return
		}
		yield("rust")
	}
}

func printUntilStop(seq iter.Seq[string]) {
	seq(func(word string) bool {
		fmt.Println("word:", word)
		return word != "go"
	})
}

func main() {
	printUntilStop(words())
}
