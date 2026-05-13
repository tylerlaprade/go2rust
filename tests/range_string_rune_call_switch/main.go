package main

import "fmt"

func isUpper(r rune) bool {
	return r >= 'A' && r <= 'Z'
}

func toLower(r rune) byte {
	if r >= 'A' && r <= 'Z' {
		return byte(r + ('a' - 'A'))
	}
	return byte(r)
}

func classify(r rune) string {
	switch r {
	case '\n':
		return "newline"
	case 'A':
		return "upper-a"
	default:
		return "other"
	}
}

func main() {
	for _, r := range "A\nz" {
		rangeClass := "other"
		switch r {
		case '\n':
			rangeClass = "range-newline"
		case 'A':
			rangeClass = "range-upper-a"
		}
		control := false
		if r < 0x20 || r == 0x7f || r > 0x7e {
			control = true
		}
		fmt.Println(isUpper(r), int(toLower(r)), classify(r), rangeClass, control)
	}
}
