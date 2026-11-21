// The same program with a goroutine, so every value is behind a lock.
// Rune literals stored in byte fields of struct literals, named and
// anonymous, with and without field names.
package main

import "fmt"

type pair struct {
	bit int
	ch  byte
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	named := []pair{{1, 'r'}, {2, 'w'}}
	anon := []struct {
		bit int
		ch  byte
	}{{1, 'r'}, {2, 'w'}}
	one := struct{ ch byte }{'x'}
	p := pair{ch: 'z'}
	fmt.Println(named[0].ch, anon[1].ch, one.ch, p.ch, string(anon[0].ch))
}
