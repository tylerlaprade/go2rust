package main

import "fmt"

func main() {
	buf := make([]byte, 5)
	n := copy(buf, "hello")
	fmt.Println(n, string(buf))

	buf2 := make([]byte, 3)
	n2 := copy(buf2, "transpile")
	fmt.Println(n2, string(buf2))

	var fingerprint [3]byte
	input := "abcdef"
	n3 := copy(fingerprint[:], input[len(input)-3:])
	fmt.Println(n3, string(fingerprint[:]))

	buf3 := make([]byte, 4)
	copy(buf3, "xxxx")
	n4 := copy(buf3[1:3], buf[0:2])
	fmt.Println(n4, string(buf3))
}
