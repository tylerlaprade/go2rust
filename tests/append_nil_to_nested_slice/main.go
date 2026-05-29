package main

// append(stack, nil) where stack is [][]int: the inner []int element is stored
// as a raw Vec, so a nil element is the zero value (empty Vec), not a wrapped
// None to deref.

func main() {
	var stack [][]int
	stack = append(stack, nil)
	stack = append(stack, []int{1, 2})
	total := 0
	for _, row := range stack {
		for _, v := range row {
			total += v
		}
	}
	println(len(stack))
	println(total)
}
