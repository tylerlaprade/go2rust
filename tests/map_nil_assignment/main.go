package main

import "fmt"

type holder struct {
	values map[string]int
}

func key() string {
	fmt.Println("key")
	return "new"
}

func value() int {
	fmt.Println("value")
	return 7
}

func directAssignment() {
	var values map[string]int
	missing, ok := values["missing"]
	fmt.Println(values == nil, len(values), missing, ok)
	for range values {
		fmt.Println("unexpected range value")
	}
	defer func() { fmt.Println("direct:", recover()) }()
	values[key()] = value()
	fmt.Println("write succeeded")
}

func compoundAssignment() {
	var counts map[string]int
	defer func() { fmt.Println("compound:", recover()) }()
	counts[key()]++
}

func fieldChecks() {
	var h holder
	plain := h.values["plain"]
	missing, ok := h.values["missing"]
	fmt.Println(h.values == nil, len(h.values), plain, missing, ok)
	for range h.values {
		fmt.Println("unexpected field range value")
	}
}

func main() {
	made := make(map[string]int)
	made["ok"] = 3
	fmt.Println(made == nil, made["ok"])
	fieldChecks()
	directAssignment()
	compoundAssignment()
}
