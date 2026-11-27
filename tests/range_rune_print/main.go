// A rune from ranging over a string prints as its number in Println,
// Print and Sprint, as every other rune does; %c and string(r) print the
// character.
package main

import "fmt"

func main() {
	for i, r := range "aé€" {
		fmt.Println(i, r, string(r))
		fmt.Print(r, " ", r+1, "\n")
		fmt.Printf("%d %c %q %v %U %x\n", r, r, r, r, r, r)
		s := fmt.Sprint(r)
		fmt.Println(s, fmt.Sprintln(r) == fmt.Sprint(r)+"\n")
	}
}
