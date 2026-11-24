package main

import (
	"fmt"
	"sync"

	"example.com/fmt_stringer_imported_types_goroutines/model"
)

type Celsius struct{ Deg int }

func (c Celsius) String() string { return fmt.Sprintf("%dC", c.Deg) }

func makeC() Celsius { return Celsius{3} }

func main() {
	var s fmt.Stringer = makeC()
	fmt.Println(s.String())
	var v any = Celsius{2}
	fmt.Println(v.(fmt.Stringer))
	fmt.Printf("%T\n", v.(fmt.Stringer))
	_, ok := any(model.Label{Text: "x"}).(fmt.Stringer)
	var w any = model.Label{Text: "y"}
	st, ok2 := w.(fmt.Stringer)
	fmt.Println(ok, ok2, st)

	var wg sync.WaitGroup
	labels := make([]string, 2)
	for i, v := range []any{model.Label{Text: "a"}, Celsius{4}} {
		wg.Add(1)
		go func(i int, v any) {
			defer wg.Done()
			if s, ok := v.(fmt.Stringer); ok {
				labels[i] = s.String()
			}
		}(i, v)
	}
	wg.Wait()
	fmt.Println(labels[0], labels[1])
}
