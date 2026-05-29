package main

func describe(args ...any) int {
	n := 0
	for _, arg := range args {
		switch arg := arg.(type) {
		case int:
			_ = arg
			n++
		}
	}
	return n
}

func main() {
	println(describe(1, "a", 2, 3))
}
