package main

import "fmt"

type Path string

func emptyPath() Path {
	return ""
}

func fromString(s string) Path {
	return Path(s)
}

func fromBytes(b []byte) Path {
	return Path(b)
}

func main() {
	fmt.Println(emptyPath())
	fmt.Println(fromString("abc"))
	fmt.Println(fromBytes([]byte("xy")))
}
