package main

func count(args ...any) int {
	n := 0
	for _, arg := range args {
		_ = arg
		n++
	}
	return n
}

func main() {
	println(count(1, "a", 3))
}
