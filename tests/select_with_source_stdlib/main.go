package main

import (
	"fmt"
	"strings"
	"time"
)

func main() {
	words := make(chan string)
	done := make(chan struct{})
	go func() {
		for _, w := range strings.Fields("select works with strings") {
			words <- w
		}
		close(done)
	}()
	var got []string
	for {
		select {
		case w := <-words:
			got = append(got, strings.ToUpper(w))
			continue
		case <-done:
		case <-time.After(5 * time.Second):
			fmt.Println("timeout")
		}
		break
	}
	fmt.Println(strings.Join(got, " "))
	ticks := 0
	timer := time.After(20 * time.Millisecond)
	stop := false
	for !stop {
		select {
		case <-timer:
			stop = true
		default:
			ticks++
			time.Sleep(time.Millisecond)
		}
	}
	fmt.Println(ticks > 0)
}
