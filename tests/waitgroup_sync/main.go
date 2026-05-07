package main

import (
	"fmt"
	"sync"
)

type Group struct {
	wg sync.WaitGroup
}

func worker(id int, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Printf("Worker %d starting\n", id)
	fmt.Printf("Worker %d done\n", id)
}

func (g *Group) Run() {
	g.wg.Add(1)
	g.wg.Done()
	g.wg.Wait()
	fmt.Println("Struct WaitGroup done")
}

func main() {
	var wg sync.WaitGroup
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		worker(i, &wg)
	}
	wg.Wait()
	fmt.Println("All workers done")

	group := &Group{}
	group.Run()
}
