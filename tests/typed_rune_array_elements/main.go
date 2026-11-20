package main

var label = [...]byte{'n', 'o', 'n', 'e', 0}

func first(values ...byte) byte { return values[0] }

func main() {
	var path [1]int8
	path[0] = '@'
	println(label[0], path[0], first('m'))
}
