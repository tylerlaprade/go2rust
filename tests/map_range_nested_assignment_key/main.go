package main

import "fmt"

func copyNested(dst, src map[string]map[string]string) {
	for outerKey, srcInner := range src {
		if dst[outerKey] == nil {
			dst[outerKey] = make(map[string]string)
		}
		for innerKey, value := range srcInner {
			dst[outerKey][innerKey] = value
		}
	}
}

func main() {
	dst := map[string]map[string]string{}
	copyNested(dst, map[string]map[string]string{
		"outer": {"inner": "value"},
	})
	fmt.Println(dst["outer"]["inner"])
}
