// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"fmt"
	"sort"
	"strings"
)

type animal struct{ name string }

func (a animal) Name() string { return a.name }

type dog struct {
	animal
	breed string
}

func (d dog) Sound() string { return "woof" }

type cat struct {
	*animal
	lives int
}

func (c *cat) Sound() string { return "meow" }

type counter struct{ n int }

func (c *counter) Inc() { c.n++ }

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	type speaker interface {
		Name() string
		Sound() string
	}
	var all []speaker
	all = append(all, dog{animal{"rex"}, "lab"})
	all = append(all, &cat{&animal{"tom"}, 9})
	for _, s := range all {
		fmt.Println(s.Name(), "says", s.Sound())
	}

	names := []string{}
	for _, s := range all {
		switch v := s.(type) {
		case dog:
			names = append(names, "dog:"+v.breed)
		case *cat:
			names = append(names, fmt.Sprint("cat:", v.lives))
		}
	}
	sort.Strings(names)
	fmt.Println(strings.Join(names, ","))

	type incer interface{ Inc() }
	c := &counter{}
	var i incer = c
	i.Inc()
	i.Inc()
	fmt.Println("count", c.n)

	type namer interface{ Name() string }
	var n namer = dog{animal{"fido"}, "pug"}
	if sp, ok := n.(speaker); ok {
		fmt.Println("also a speaker:", sp.Sound())
	}
	fmt.Printf("%v\n", n.Name())
}
