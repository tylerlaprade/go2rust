package main

import "fmt"

func main() {
	check([]byte{66, 250, 67})
	check([]byte{0xef, 0xb7, 0x90, 0xef, 0xb7, 0x91, 65})
	checkConcat()
}

func check(b []byte) {
	s := string(b)
	fmt.Println(len(s), []byte(s))
}

func checkConcat() {
	invalid := string([]byte{250})
	left := invalid + "X"
	right := "Y" + invalid
	both := invalid + string([]byte{251})
	many := invalid + "Q" + string([]byte{252})
	assigned := invalid
	assigned += "Z"
	assignedRight := "W"
	assignedRight += invalid
	items := []string{invalid}
	items[0] += "S"
	values := map[string]string{"k": string([]byte{250})}
	values["k"] += "M"
	fmt.Println(len(left), []byte(left), left == string([]byte{250, 'X'}))
	fmt.Println(len(right), []byte(right), right == string([]byte{'Y', 250}))
	fmt.Println(len(both), []byte(both), both == string([]byte{250, 251}))
	fmt.Println(len(many), []byte(many), many == string([]byte{250, 'Q', 252}))
	fmt.Println(len(assigned), []byte(assigned), assigned == string([]byte{250, 'Z'}))
	fmt.Println(len(assignedRight), []byte(assignedRight), assignedRight == string([]byte{'W', 250}))
	fmt.Println(len(items[0]), []byte(items[0]), items[0] == string([]byte{250, 'S'}))
	fmt.Println(len(values["k"]), []byte(values["k"]), values["k"] == string([]byte{250, 'M'}))
}
