// A type assertion to a basic type is an operand of arithmetic and of
// comparisons: v.(int)+1, f.(float64)*2, s.(string)+"pher". The same holds
// for a range value over a []any, and a, b := v, s copies interface values.
package main

import "fmt"

type Celsius float64

func main() {
	var v any = 9
	var f any = 2.5
	var s any = "go"
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
	a, b := v, s
	fmt.Println(a, b)
}
