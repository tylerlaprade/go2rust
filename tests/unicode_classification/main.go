// The unicode package works without opting in: it is translated from Go
// source by default. Classification, case mapping and range tables.
package main

import (
	"fmt"
	"strings"
	"unicode"
)

func classify(r rune) string {
	var kinds []string
	if unicode.IsLetter(r) {
		kinds = append(kinds, "letter")
	}
	if unicode.IsDigit(r) {
		kinds = append(kinds, "digit")
	}
	if unicode.IsSpace(r) {
		kinds = append(kinds, "space")
	}
	if unicode.IsPunct(r) {
		kinds = append(kinds, "punct")
	}
	if unicode.IsUpper(r) {
		kinds = append(kinds, "upper")
	}
	if unicode.IsLower(r) {
		kinds = append(kinds, "lower")
	}
	if unicode.Is(unicode.Han, r) {
		kinds = append(kinds, "han")
	}
	if len(kinds) == 0 {
		return "other"
	}
	return strings.Join(kinds, ",")
}

func main() {
	for _, r := range "aZ9 \t!é漢ß٣ " {
		fmt.Printf("%q %s %q %q\n", r, classify(r), unicode.ToUpper(r), unicode.ToLower(r))
	}
	words := 0
	inWord := false
	for _, r := range "  hello,  wörld\nand   more  " {
		if unicode.IsSpace(r) || unicode.IsPunct(r) {
			inWord = false
		} else if !inWord {
			inWord = true
			words++
		}
	}
	fmt.Println("words:", words, unicode.IsTitle('ǅ'), string(unicode.ToTitle('ǆ')), unicode.SimpleFold('A'))
}
