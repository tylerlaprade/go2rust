package main

func set(args ...any) int {
	if len(args) > 0 {
		args[0] = 42
	}
	return len(args)
}

func main() {
	println(set(1, "a", 3))
}
