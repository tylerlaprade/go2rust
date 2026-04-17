package main

import "fmt"

func main() {
	buf := make([]byte, 5)
	n := copy(buf, "hello")
	fmt.Println(n, string(buf))

	buf2 := make([]byte, 3)
	n2 := copy(buf2, "transpile")
	fmt.Println(n2, string(buf2))
}
