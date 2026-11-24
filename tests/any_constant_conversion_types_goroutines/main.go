package main

import (
	"fmt"
	"sync"
)

type Box struct{ V any }

func get(n int) any {
	if n == 0 {
		return nil
	}
	return n
}

func main() {
	x := any(5)
	n, ok := x.(int)
	fmt.Println(n, ok)
	fmt.Println(any(2.5).(float64), any('a').(rune), any(-3).(int))
	var y any = 7
	fmt.Println(y.(int))
	const big = 1 << 40
	fmt.Println(any(big).(int), any(int8(4)).(int8), any(uint(9)).(uint))
	fmt.Printf("%T %T %T %T\n", any(5), any(2.5), any('a'), any("s"))
	fmt.Printf("%T %T %T\n", get(1), get(0), Box{V: true}.V)
	b := Box{V: 3.5}
	fmt.Printf("%T %v\n", b.V, b.V)
	xs := []any{1, "a", false}
	fmt.Printf("%T %T %T\n", xs[0], xs[1], xs[2])

	var wg sync.WaitGroup
	names := make([]string, 2)
	for i, v := range []any{4, "go"} {
		wg.Add(1)
		go func(i int, v any) {
			defer wg.Done()
			names[i] = fmt.Sprintf("%T", v)
		}(i, v)
	}
	wg.Wait()
	fmt.Println(names[0], names[1], any(6).(int))
}
