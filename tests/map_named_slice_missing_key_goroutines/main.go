package main

import "fmt"

type Numbers []int

func main() {
	m := map[string]Numbers{}
	one := m["zzz"]
	two, ok := m["zzz"]
	done := make(chan bool)
	go func() {
		fmt.Println(len(one), one == nil, ok, len(two), two == nil)
		s := Numbers{1}
		fmt.Println(s == nil, s != nil)
		s = nil
		fmt.Println(s == nil, s != nil)
		done <- true
	}()
	<-done
}
