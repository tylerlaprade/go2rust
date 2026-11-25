// Workers that receive batches of words and send back maps of counts, with
// v, ok := <-ch, append(acc, <-ch...) and a select on a channel of slices.
package main

import (
	"fmt"
	"sort"
	"strings"
	"sync"
)

func worker(id int, jobs <-chan []string, results chan<- map[string]int, wg *sync.WaitGroup) {
	defer wg.Done()
	for batch := range jobs {
		counts := map[string]int{}
		for _, w := range batch {
			counts[strings.ToLower(w)]++
		}
		results <- counts
	}
}

func main() {
	jobs := make(chan []string, 3)
	results := make(chan map[string]int, 3)
	var wg sync.WaitGroup
	for i := 0; i < 2; i++ {
		wg.Add(1)
		go worker(i, jobs, results, &wg)
	}
	words := strings.Fields("Go go rust Rust go zig")
	jobs <- words[:2]
	jobs <- words[2:4]
	jobs <- []string{"zig", "Go"}
	close(jobs)
	wg.Wait()
	close(results)
	total := map[string]int{}
	for counts := range results {
		for k, v := range counts {
			total[k] += v
		}
	}
	keys := make([]string, 0, len(total))
	for k := range total {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Println(k, total[k])
	}

	nums := make(chan []int, 2)
	nums <- nil
	nums <- []int{4, 5}
	var acc []int
	first, ok := <-nums
	fmt.Println(ok, len(first))
	acc = append(acc, <-nums...)
	fmt.Println(acc, len(acc))
	sel := make(chan []int, 1)
	sel <- []int{7}
	select {
	case v := <-sel:
		fmt.Println("got", v, v[0])
	default:
		fmt.Println("none")
	}
}
