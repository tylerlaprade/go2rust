// A loop body whose forward goto jumps over a label that a later goto
// jumps back to, as regexp's tryBacktrack does, and continue and break
// of the loop from inside the goto blocks.
package main

import "fmt"

func visit(seen map[int]bool, pc int) bool {
	if seen[pc] {
		return false
	}
	seen[pc] = true
	return true
}

// walk mirrors regexp's tryBacktrack: a job loop whose body jumps over a
// check on the first pass and back to it from a switch.
func walk(start []int, next map[int][]int) []int {
	var order []int
	seen := map[int]bool{}
	jobs := append([]int(nil), start...)
	for len(jobs) > 0 {
		pc := jobs[len(jobs)-1]
		jobs = jobs[:len(jobs)-1]
		goto Skip
	CheckAndLoop:
		if !visit(seen, pc) {
			continue
		}
	Skip:
		order = append(order, pc)
		switch outs := next[pc]; len(outs) {
		case 0:
		case 1:
			pc = outs[0]
			goto CheckAndLoop
		default:
			jobs = append(jobs, outs[1:]...)
			pc = outs[0]
			goto CheckAndLoop
		}
	}
	return order
}

func main() {
	next := map[int][]int{1: {2, 3}, 2: {4}, 3: {4}, 4: {}}
	fmt.Println(walk([]int{1}, next))
	fmt.Println(walk([]int{4, 3}, next))
	n := 0
	for i := 0; i < 5; i++ {
		if i == 1 {
			goto done
		}
		if i%2 == 0 {
			continue
		}
		n += 100
	done:
		n += i
	}
	fmt.Println(n)
	m := 0
	for i := 0; ; i++ {
		if i > 2 {
			goto out
		}
		m += i
		continue
	out:
		break
	}
	fmt.Println(m)
	// A closure with its own loop inside a goto-lowered loop body: its
	// continue is its own loop's.
	total := 0
	for i := 0; i < 3; i++ {
		if i == 0 {
			goto next
		}
		func() {
			for j := 0; j < 3; j++ {
				if j == 1 {
					continue
				}
				total += j
			}
		}()
	next:
		total += 100
	}
	fmt.Println(total)
}
