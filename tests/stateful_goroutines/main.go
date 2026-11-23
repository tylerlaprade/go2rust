package main

import (
	"fmt"
	"math/rand"
	"sync/atomic"
)

type readOp struct {
	key  int
	resp chan int
}
type writeOp struct {
	key  int
	val  int
	resp chan bool
}

func main() {
	var readOps uint64
	var writeOps uint64

	reads := make(chan readOp)
	writes := make(chan writeOp)
	done := make(chan bool)

	go func() {
		var state = make(map[int]int)
		for {
			select {
			case read := <-reads:
				read.resp <- state[read.key]
			case write := <-writes:
				state[write.key] = write.val
				write.resp <- true
			}
		}
	}()

	for r := 0; r < 100; r++ {
		go func() {
			read := readOp{
				key:  rand.Intn(5),
				resp: make(chan int)}
			reads <- read
			<-read.resp
			atomic.AddUint64(&readOps, 1)
			done <- true
		}()
	}

	for w := 0; w < 10; w++ {
		go func() {
			write := writeOp{
				key:  rand.Intn(5),
				val:  rand.Intn(100),
				resp: make(chan bool)}
			writes <- write
			<-write.resp
			atomic.AddUint64(&writeOps, 1)
			done <- true
		}()
	}

	for i := 0; i < 110; i++ {
		<-done
	}

	readOpsFinal := atomic.LoadUint64(&readOps)
	fmt.Println("readOps:", readOpsFinal)
	writeOpsFinal := atomic.LoadUint64(&writeOps)
	fmt.Println("writeOps:", writeOpsFinal)
}
