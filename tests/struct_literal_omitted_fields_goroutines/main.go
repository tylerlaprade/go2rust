// A struct literal that leaves out fields gives them their zero values,
// also when one of the fields left out is itself a struct.
// A goroutine makes every variable a locked one.
package main

import "fmt"

type Vec struct{ X, Y int }

type Stats struct {
	Seen  map[string]int
	Count int
}

type Player struct {
	Name  string
	Pos   Vec
	HP    int
	Tags  []string
	Score [2]int
	Inv   map[string]int
	Next  *Player
	Alive bool
	Stats Stats
}

func main() {
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
	p := Player{Name: "a"}
	q := &Player{Pos: Vec{1, 2}}
	r := Player{Name: "r", Stats: Stats{Count: 3}}
	fmt.Println(p.HP, p.Score, len(p.Tags), p.Inv == nil, p.Next == nil, p.Alive, p.Pos, p.Stats.Count)
	fmt.Println(q.Name == "", q.HP, q.Score, q.Pos, q.Stats.Seen == nil)
	fmt.Println(r.Pos, r.HP, r.Stats.Count)
	p.HP += 5
	p.Score[1] = 3
	p.Tags = append(p.Tags, "x")
	p.Stats.Count++
	q.Score[0] = 7
	fmt.Println(p.HP, p.Score, p.Tags, p.Stats.Count, q.Score)
}
