package main

import "fmt"

type named struct {
	id string
}

type pkgReader struct {
	laterFns  []func()
	laterFors map[*named]int
	hits      int
}

func (pr *pkgReader) laterFor(t *named, fn func()) {
	if pr.laterFors == nil {
		pr.laterFors = make(map[*named]int)
	}
	pr.laterFors[t] = len(pr.laterFns)
	pr.laterFns = append(pr.laterFns, fn)
}

func schedule(pr *pkgReader, named, rhs *named) {
	pk := pr
	pk.laterFor(named, func() {
		delete(pk.laterFors, named)
		if i, ok := pk.laterFors[rhs]; ok {
			f := pk.laterFns[i]
			pk.laterFns[i] = func() {}
			f()
		}
		pk.hits++
	})
}

func main() {
	pr := &pkgReader{}
	a := &named{id: "a"}
	b := &named{id: "b"}
	pr.laterFor(b, func() {
		pr.hits += 10
	})
	schedule(pr, a, b)
	pr.laterFns[1]()
	fmt.Println(pr.hits)
}
