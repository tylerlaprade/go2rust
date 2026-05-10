package main

import "fmt"

type item struct {
	name string
}

func first(items []item) item {
	for _, item := range items {
		return item
	}
	return item{}
}

func score(item item) int {
	return len(item.name)
}

type scorer struct{}

func (s *scorer) score(item item) int {
	return len(item.name)
}

type holder struct {
	s *scorer
}

func totalScore(items []item) int {
	total := 0
	for _, item := range items {
		total += score(item)
	}
	return total
}

func totalMethodScore(h *holder, items []item) int {
	total := 0
	for _, item := range items {
		total += h.s.score(item)
	}
	return total
}

func main() {
	items := []item{{name: "alpha"}, {name: "beta"}}
	h := &holder{s: &scorer{}}
	fmt.Println(first(items).name)
	fmt.Println(totalScore(items))
	fmt.Println(totalMethodScore(h, items))
}
