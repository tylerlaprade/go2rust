// A struct that embeds an interface gets the interface's methods. They have
// no method declarations of their own to forward, so a call of one on the
// struct, and the struct's impl of the interface, called themselves until
// the stack overflowed. Through a struct that embeds that struct, the
// methods are promoted twice.
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Animal interface {
	Sound() string
	Name() string
}

type Dog struct{ name string }

func (d Dog) Sound() string { return "woof" }
func (d Dog) Name() string  { return d.name }

// Loud overrides Sound and keeps the embedded Animal's Name.
type Loud struct{ Animal }

func (l Loud) Sound() string { return strings.ToUpper(l.Animal.Sound()) + "!" }

type Store interface {
	Get(key string) (string, bool)
	Put(key, value string)
	keys() []string
}

type mapStore struct{ m map[string]string }

func (s *mapStore) Get(k string) (string, bool) { v, ok := s.m[k]; return v, ok }
func (s *mapStore) Put(k, v string)            { s.m[k] = v }
func (s *mapStore) keys() []string {
	var ks []string
	for k := range s.m {
		ks = append(ks, k)
	}
	sort.Strings(ks)
	return ks
}

// logged overrides Put and keeps Get and the unexported keys.
type logged struct {
	Store
	log []string
}

func (l *logged) Put(k, v string) {
	l.log = append(l.log, "put "+k)
	l.Store.Put(k, v)
}

// upper gets Put from logged and keys from the Store inside logged.
type upper struct {
	*logged
}

func (u upper) Get(k string) (string, bool) {
	v, ok := u.logged.Get(k)
	return strings.ToUpper(v), ok
}

func dump(s Store) {
	for _, k := range s.keys() {
		v, _ := s.Get(k)
		fmt.Print(k, "=", v, " ")
	}
	fmt.Println()
}

func main() {
	var a Animal = Dog{"rex"}
	l := Loud{a}
	fmt.Println(l.Name(), l.Sound())
	var b Animal = l
	fmt.Println(b.Name(), b.Sound())

	base := &mapStore{m: map[string]string{}}
	lg := &logged{Store: base}
	lg.Put("a", "x")
	lg.Put("b", "y")
	v, ok := lg.Get("a")
	fmt.Println(v, ok, lg.log, lg.keys())
	dump(lg)
	u := upper{lg}
	u.Put("c", "z")
	dump(u)
	fmt.Println(u.log, len(u.keys()))
	var s Store = u
	_, ok = s.Get("nope")
	fmt.Println(ok)
}
