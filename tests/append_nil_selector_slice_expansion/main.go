package main

import (
	"fmt"
	"strings"
)

type invocation struct {
	Verb       string
	BuildFlags []string
}

func (i *invocation) run() []string {
	goArgs := []string{i.Verb}
	goArgs = append(goArgs, i.BuildFlags...)
	return goArgs
}

func main() {
	inv := &invocation{Verb: "list"}
	fmt.Println(strings.Join(inv.run(), ","))
}
