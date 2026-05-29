package main

type Pos int

type Position struct {
	Offset int
}

func position(p Pos) Position {
	return Position{Offset: int(p)}
}

func set(args ...any) int {
	for i, arg := range args {
		switch arg := arg.(type) {
		case Pos:
			args[i] = position(arg)
		}
	}
	return len(args)
}

func main() {
	println(set(Pos(42), "a", Pos(7)))
}
