// In a program with goroutines: function values of a named func type are
// called from a range over a slice of them, from a map of them, and from a
// variable.
package main

import (
	"errors"
	"fmt"
	"strings"
	"sync"
)

type Op func(acc *int) error
type Transform func(string) string

var errNegative = errors.New("negative")

func main() {
	acc := 0
	ops := []Op{
		func(a *int) error { *a += 5; return nil },
		func(a *int) error { *a -= 10; if *a < 0 { return errNegative }; return nil },
		func(a *int) error { *a *= 3; return nil },
	}
	for i, op := range ops {
		if err := op(&acc); err != nil {
			fmt.Println(i, "error:", err, acc)
			continue
		}
		fmt.Println(i, "ok", acc)
	}
	steps := map[string]Transform{"upper": strings.ToUpper, "trim": strings.TrimSpace}
	for _, name := range []string{"trim", "upper"} {
		fmt.Printf("%s=%q\n", name, steps[name]("  go to rust  "))
	}
	var t Transform = func(s string) string { return s + "!" }
	for _, f := range []Transform{t, strings.ToLower} {
		fmt.Println(f("Hi"))
	}
	var wg sync.WaitGroup
	results := make([]string, 2)
	for i, f := range []Transform{strings.ToUpper, t} {
		wg.Add(1)
		go func(i int, f Transform) {
			defer wg.Done()
			results[i] = f("go")
		}(i, f)
	}
	wg.Wait()
	fmt.Println(results)
}
