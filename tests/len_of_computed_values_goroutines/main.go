// The same program with a goroutine, so every value is behind a lock.
// len of values computed in the call: a string concatenation, a call result,
// a slice expression, a conversion and an append.
package main

import (
	"fmt"
	"strings"
)

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	s := "go"
	fmt.Println(len(s+"!"), len(strings.Repeat(s, 3)), len(s[1:]), len([]byte(s)), len(fmt.Sprint(42)))
	words := []string{"a", "bb"}
	fmt.Println(len(words[1]+words[0]), len(append(words, "c")), len(words[:1]))
}
