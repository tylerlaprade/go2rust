package main

import "fmt"

// An unnamed struct type keys a map: a grid of positions, a search that
// records the distance to each, and a count by name and flag, with a
// goroutine filling one of the maps.

func neighbors(p struct{ x, y int }) []struct{ x, y int } {
	return []struct{ x, y int }{{p.x + 1, p.y}, {p.x, p.y + 1}}
}

func main() {
	seen := map[struct{ x, y int }]bool{}
	done := make(chan bool)
	go func() {
		seen[struct{ x, y int }{1, 2}] = true
		done <- true
	}()
	<-done
	fmt.Println(len(seen), seen[struct{ x, y int }{1, 2}], seen[struct{ x, y int }{0, 0}])

	dist := map[struct{ x, y int }]int{{0, 0}: 0}
	queue := []struct{ x, y int }{{0, 0}}
	for len(queue) > 0 && len(dist) < 6 {
		cur := queue[0]
		queue = queue[1:]
		for _, n := range neighbors(cur) {
			if _, ok := dist[n]; !ok {
				dist[n] = dist[cur] + 1
				queue = append(queue, n)
			}
		}
	}
	fmt.Println(len(dist), dist[struct{ x, y int }{1, 1}])
	for k, v := range dist {
		if v == 2 {
			fmt.Println(k.x+k.y, v)
		}
	}

	a := struct{ x, y int }{3, 4}
	grid := map[struct{ x, y int }]string{a: "a"}
	fmt.Println(grid[struct{ x, y int }{3, 4}], len(grid))

	counts := map[struct {
		name string
		ok   bool
	}]int{}
	counts[struct {
		name string
		ok   bool
	}{"x", true}]++
	counts[struct {
		name string
		ok   bool
	}{"x", true}]++
	fmt.Println(len(counts), counts[struct {
		name string
		ok   bool
	}{"x", true}])
}
