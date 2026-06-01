package main

import "fmt"

type entry struct {
	name string
}

type scope struct {
	elems map[string]*entry
}

var global *entry

func (s *scope) lookup(name string) *entry {
	if s.elems[name] == global {
		return nil
	}
	return s.elems[name]
}

func (s *scope) insert(obj *entry) *entry {
	if alt := s.lookup(obj.name); alt != nil {
		return alt
	}
	s.elems[obj.name] = obj
	return nil
}

func def(s *scope, obj *entry) {
	s.insert(obj)
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	s := &scope{elems: map[string]*entry{}}
	global = &entry{name: "any"}
	def(s, global)
	fmt.Println("done")
}
