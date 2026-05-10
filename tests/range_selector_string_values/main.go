package main

import (
	"fmt"
	"strings"
)

type Command struct {
	Args []string
}

func quote(s string) string {
	return "<" + s + ">"
}

func debug(cmd Command) string {
	var args []string
	for _, arg := range cmd.Args {
		quoted := quote(arg)
		if quoted[1:len(quoted)-1] != arg || strings.Contains(arg, " ") {
			args = append(args, quoted)
		} else {
			args = append(args, arg)
		}
	}
	return strings.Join(args, " ")
}

func main() {
	fmt.Println(debug(Command{Args: []string{"go", "list ./..."}}))
}
