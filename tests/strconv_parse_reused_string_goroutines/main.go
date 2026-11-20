// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	total := 0
	for _, field := range strings.Split("1, 2,x, 30", ",") {
		field = strings.TrimSpace(field)
		n, err := strconv.Atoi(field)
		if err != nil {
			fmt.Println("skip", field, err)
			continue
		}
		total += n
		fmt.Println(field, "->", n)
	}
	for _, word := range []string{"true", "nope"} {
		b, err := strconv.ParseBool(word)
		fmt.Println(word, b, err)
	}
	fmt.Println(total)
}
