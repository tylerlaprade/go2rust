package main

import "fmt"

type parsed struct {
	major string
}

func splitVersion(v string) (string, string, bool) {
	if len(v) == 0 {
		return "", "", false
	}
	return v[:1], v[1:], true
}

func parse(v string) (parsed, bool) {
	p := parsed{}
	ok := false
	p.major, v, ok = splitVersion(v)
	fmt.Println("rest:", v)
	return p, ok
}

func main() {
	p, ok := parse("v1")
	fmt.Println(p.major, ok)
}
