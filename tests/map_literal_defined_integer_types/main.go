package main

import (
	"fmt"

	"example.com/mapliteral/dep"
)

type Index int
type Indexes map[string]Index

const named Index = 3
const offset = 2

func main() {
	local := map[string]Index{"literal": 2, "named": named}
	fmt.Println(local["literal"], local["named"])

	indexes := Indexes{"computed": 2 + 3}
	fmt.Println(indexes["computed"])

	levels := map[string]dep.Level{"literal": 4, "named": dep.High, "computed": 1 + 2, "local": offset, "imported": dep.Offset}
	fmt.Println(levels["literal"], levels["named"], levels["computed"], levels["local"], levels["imported"])
}
