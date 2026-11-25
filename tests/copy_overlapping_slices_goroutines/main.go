// In a program with goroutines: copy between overlapping parts of one slice
// moves the values as memmove does, in a goroutine too.
package main

import "fmt"

func insert(s []int, i, x int) []int {
	s = append(s, 0)
	copy(s[i+1:], s[i:])
	s[i] = x
	return s
}

func main() {
	s := []int{1, 2, 3, 4, 5}
	s = insert(s, 1, 9)
	fmt.Println(s)
	a := [6]int{1, 2, 3, 4, 5, 6}
	copy(a[2:], a[:4])
	fmt.Println(a)
	b := []int{1, 2, 3, 4, 5, 6}
	copy(b[1:], b)
	fmt.Println(b)
	n := copy(b, b[2:])
	fmt.Println(n, b)

	arr := []int{1, 2, 3}
	dst := arr[1:1]
	src := arr[0:2]
	dst = append(dst, src...)
	fmt.Println(arr, dst)

	words := []string{"a", "b", "c", "d"}
	words = append(words[:1], words[2:]...)
	fmt.Println(words, len(words))
	r := []rune("hello")
	copy(r[1:], r[:3])
	fmt.Println(string(r))
	done := make(chan []int)
	go func() {
		t := []int{1, 2, 3, 4}
		done <- insert(t, 0, 7)
	}()
	got := <-done
	fmt.Println(got)
}
