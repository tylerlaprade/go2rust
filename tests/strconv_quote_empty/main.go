package main

import (
	"fmt"
	"strconv"
)

func main() {
	quoted := strconv.Quote("")
	fmt.Println(len(quoted), quoted[1:len(quoted)-1] == "")
	fmt.Println(strconv.Quote("go list"))
}
