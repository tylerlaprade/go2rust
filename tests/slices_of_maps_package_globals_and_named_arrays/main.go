package main

import "fmt"

type Tables [2]map[string]int

var s = []map[string]int{1: {"x": 1}}
var a = [2]map[string]int{}
var a2 = [2]map[string]int{1: {"y": 2}}
var t Tables

func main() {
	fmt.Println(len(s), s[0] == nil, s[1]["x"])
	fmt.Println(a[0] == nil, a2[1]["y"])
	fmt.Println(t[0] == nil)
	t[1] = map[string]int{"z": 3}
	fmt.Println(len(t[1]))
	var lt Tables
	fmt.Println(lt[1] == nil)
}
