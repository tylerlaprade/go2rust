package main

import (
	"container/heap"
	"fmt"
	"strings"
)

// IntHeap is the min-heap of container/heap's documentation.
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) { *h = append(*h, x.(int)) }

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

// Task is an entry of a priority queue that tracks its own position, so
// heap.Fix can move it after its priority changes.
type Task struct {
	name     string
	priority int
	index    int
}

type TaskQueue []*Task

func (q TaskQueue) Len() int { return len(q) }

// Less puts the highest priority first.
func (q TaskQueue) Less(i, j int) bool { return q[i].priority > q[j].priority }

func (q TaskQueue) Swap(i, j int) {
	q[i], q[j] = q[j], q[i]
	q[i].index = i
	q[j].index = j
}

func (q *TaskQueue) Push(x any) {
	task := x.(*Task)
	task.index = len(*q)
	*q = append(*q, task)
}

func (q *TaskQueue) Pop() any {
	old := *q
	n := len(old)
	task := old[n-1]
	old[n-1] = nil
	task.index = -1
	*q = old[0 : n-1]
	return task
}

func (q *TaskQueue) update(task *Task, priority int) {
	task.priority = priority
	heap.Fix(q, task.index)
}

// Item is a value entry: the queue holds copies, not pointers.
type Item struct {
	node, dist int
}

type PQ []Item

func (p PQ) Len() int           { return len(p) }
func (p PQ) Less(i, j int) bool { return p[i].dist < p[j].dist }
func (p PQ) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }
func (p *PQ) Push(x any)        { *p = append(*p, x.(Item)) }
func (p *PQ) Pop() any {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

type Edge struct{ to, w int }

func dijkstra(g map[int][]Edge, src, n int) []int {
	dist := make([]int, n)
	for i := range dist {
		dist[i] = 1 << 60
	}
	dist[src] = 0
	pq := &PQ{{src, 0}}
	for pq.Len() > 0 {
		cur := heap.Pop(pq).(Item)
		if cur.dist > dist[cur.node] {
			continue
		}
		for _, e := range g[cur.node] {
			if nd := cur.dist + e.w; nd < dist[e.to] {
				dist[e.to] = nd
				heap.Push(pq, Item{e.to, nd})
			}
		}
	}
	return dist
}

func main() {
	h := &IntHeap{5, 2, 8}
	heap.Init(h)
	heap.Push(h, 3)
	fmt.Printf("minimum: %d, len %d\n", (*h)[0], h.Len())
	var popped []string
	for h.Len() > 0 {
		popped = append(popped, fmt.Sprint(heap.Pop(h)))
	}
	fmt.Println("popped:", strings.Join(popped, " "))

	h2 := &IntHeap{9, 4, 7, 1, 6}
	heap.Init(h2)
	fmt.Println("removed:", heap.Remove(h2, 2), "left:", h2.Len())
	(*h2)[0] = 10
	heap.Fix(h2, 0)
	fmt.Println("after fix:", heap.Pop(h2), heap.Pop(h2), heap.Pop(h2), heap.Pop(h2))

	tasks := map[string]int{"write": 3, "test": 2, "ship": 4}
	q := make(TaskQueue, 0, len(tasks))
	for _, name := range []string{"write", "test", "ship"} {
		q = append(q, &Task{name: name, priority: tasks[name], index: len(q)})
	}
	heap.Init(&q)
	review := &Task{name: "review", priority: 1}
	heap.Push(&q, review)
	q.update(review, 5)
	for q.Len() > 0 {
		task := heap.Pop(&q).(*Task)
		fmt.Printf("%d:%s(index %d) ", task.priority, task.name, task.index)
	}
	fmt.Println()

	g := map[int][]Edge{0: {{1, 4}, {2, 1}}, 2: {{1, 2}, {3, 5}}, 1: {{3, 1}}}
	fmt.Println("dijkstra:", dijkstra(g, 0, 4))
}
