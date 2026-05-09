package main

import "fmt"

type Version uint32

const (
	V0 Version = iota
	V1
	V2
)

var introduced = [4]Version{V1, 0, V2}

func main() {
	fmt.Println(introduced[0], introduced[1], introduced[2], introduced[3])
}
