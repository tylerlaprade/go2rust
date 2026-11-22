// The same byte-string operations with a goroutine present, so generated
// mutable values use their concurrent wrappers.
package main

import "fmt"

func main() {
	go func() {}()
	check([]byte{66, 250, 67})
	checkConcat()
}

func check(b []byte) {
	s := string(b)
	fmt.Println(len(s), []byte(s))
}

func checkConcat() {
	invalid := string([]byte{250})
	left := invalid + "X"
	many := invalid + "Q" + string([]byte{252})
	assigned := invalid
	assigned += "Z"
	items := []string{invalid}
	items[0] += "S"
	values := map[string]string{"k": string([]byte{250})}
	values["k"] += "M"
	fmt.Println(len(left), []byte(left), left == string([]byte{250, 'X'}))
	fmt.Println(len(many), []byte(many), many == string([]byte{250, 'Q', 252}))
	fmt.Println(len(assigned), []byte(assigned), assigned == string([]byte{250, 'Z'}))
	fmt.Println(len(items[0]), []byte(items[0]), items[0] == string([]byte{250, 'S'}))
	fmt.Println(len(values["k"]), []byte(values["k"]), values["k"] == string([]byte{250, 'M'}))
}
