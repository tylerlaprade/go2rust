package main

import "fmt"

type record struct {
	N int
}

func get() any { return 7 }

func replace(value any) { value = 9 }

func main() {
	var a any = "first"
	f := func() { a = get() }
	f()
	fmt.Println(a)

	var b any = "x"
	c := b
	b = 9
	fmt.Println(b, c)
	var d any = b
	b = 10
	fmt.Println(b, c, d)

	g := func() { b = "from closure" }
	g()
	fmt.Println(b, c)

	var structValue any = record{N: 1}
	structCopy := structValue
	structValue = record{N: 2}
	fmt.Println(structCopy.(record).N, structValue.(record).N)

	firstRecord := &record{N: 3}
	var pointerValue any = firstRecord
	pointerCopy := pointerValue
	pointerValue = &record{N: 4}
	firstRecord.N = 5
	fmt.Println(pointerCopy.(*record).N, pointerValue.(*record).N)

	var functionValue any = func() int { return 6 }
	functionCopy := functionValue
	functionValue = 7
	fmt.Println(functionCopy.(func() int)(), functionValue.(int))

	var argument any = "keep"
	replace(argument)
	fmt.Println(argument)
}
