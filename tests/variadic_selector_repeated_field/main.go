package main

import "fmt"

type source struct {
	dir string
	a   []string
	b   []string
}

type result struct {
	first  []string
	second []string
}

func join(dir string, groups ...[]string) []string {
	var out []string
	for _, group := range groups {
		for _, name := range group {
			out = append(out, dir+"/"+name)
		}
	}
	return out
}

func main() {
	done := make(chan bool, 1)
	go func() {
		src := source{
			dir: "root",
			a:   []string{"a"},
			b:   []string{"b"},
		}
		res := result{
			first:  join(src.dir, src.a),
			second: join(src.dir, src.b),
		}
		fmt.Println(res.first[0])
		fmt.Println(res.second[0])
		done <- true
	}()
	<-done
}
