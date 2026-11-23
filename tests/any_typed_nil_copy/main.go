package main

import "fmt"

func replace(value any) {
	value = 9
}

func main() {
	var value any = (*int)(nil)
	replace(value)
	fmt.Println(value == nil, value.(*int) == nil)

	copy := value
	value = nil
	fmt.Println(value == nil, copy.(*int) == nil)
}
