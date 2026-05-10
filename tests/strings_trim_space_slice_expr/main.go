package main

import (
	"fmt"
	"strings"
)

func main() {
	errstr := "no such directory   /tmp/missing\n"
	marker := "no such directory"
	abspath := strings.TrimSpace(errstr[strings.Index(errstr, marker)+len(marker):])
	fmt.Println(abspath)
}
