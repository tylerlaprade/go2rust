// In a program with goroutines: a type assertion to a basic type is an
// operand of arithmetic and of comparisons: v.(int)+1, s.(string)+"pher".
package main

import (
	"errors"
	"fmt"
)

type Celsius float64

func main() {
	results := make(chan any, 3)
	go func() {
		results <- 9
		results <- 2.5
		results <- "go"
	}()
	v, f, s := <-results, <-results, <-results
	var c any = Celsius(20)
	fmt.Println(v.(int)+1, v.(int)*2, 10-v.(int), v.(int)%4)
	fmt.Println(f.(float64)*2, s.(string)+"pher", c.(Celsius)*2)
	fmt.Println(v.(int) > 5, s.(string) == "go", f.(float64) < 1)
	n := v.(int) << 2
	fmt.Println(n)
	vals := []any{1, 2, 3}
	total := 0
	for _, x := range vals {
		total += x.(int) * 10
	}
	fmt.Println(total)
	for _, x := range []any{errors.New("boom"), 7} {
		if err, ok := x.(error); ok {
			fmt.Println("err", err.Error())
		}
		switch w := x.(type) {
		case error:
			fmt.Println("error", x.(error).Error())
		default:
			if n, ok := w.(int); ok {
				fmt.Println("int", n+1)
			}
		}
	}
}
