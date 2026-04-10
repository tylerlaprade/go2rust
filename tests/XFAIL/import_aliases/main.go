package main

import (
	"fmt"
	conv "strconv"
	str "strings"
)

func main() {
	n, _ := conv.Atoi("42")
	fmt.Println(str.ToUpper("go"), n)
}
