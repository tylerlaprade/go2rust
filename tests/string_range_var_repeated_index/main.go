package main

import "fmt"

func main() {
	items := []string{"//foo", "/*bar*/", "//x"}
	for _, c := range items {
		switch c[1] {
		case '/':
			c = c[2:]
			if len(c) == 0 {
				continue
			}
			if c[0] == ' ' {
				c = c[1:]
			}
		case '*':
			c = c[2 : len(c)-2]
		}
		fmt.Println(c)
	}
}
