// time.Sleep with a constant such as time.Millisecond, or with a Duration
// variable, did not compile: the argument was taken to be a count of
// nanoseconds and cast to u64.
package main

import (
	"fmt"
	"time"
)

func pause(d time.Duration) {
	time.Sleep(d)
}

func main() {
	time.Sleep(time.Millisecond)
	time.Sleep(2 * time.Millisecond)
	d := 3 * time.Millisecond
	time.Sleep(d)
	pause(d)
	n := 2
	time.Sleep(time.Duration(n) * time.Millisecond)
	time.Sleep(500 * time.Microsecond)
	fmt.Println("slept")
}
