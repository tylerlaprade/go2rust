// The same program with a goroutine, so every value is behind a lock.
// type Money int64 with a String method that reassigns its receiver, and a
// named Money result that starts at its zero value, in an inventory that
// sorts, wraps errors and recovers from a panic.
package main

import (
	"errors"
	"fmt"
	"sort"
	"strings"
)

type Money int64

func (m Money) String() string {
	sign := ""
	if m < 0 {
		sign = "-"
		m = -m
	}
	return fmt.Sprintf("%s$%d.%02d", sign, m/100, m%100)
}

type Base struct {
	ID      int
	Created string
}

func (b Base) Describe() string { return fmt.Sprintf("#%d (%s)", b.ID, b.Created) }

type Item struct {
	Base
	Name  string
	Price Money
	Qty   int
	Tags  []string
}

func (i *Item) Total() Money { return i.Price * Money(i.Qty) }

func (i Item) String() string {
	return fmt.Sprintf("%s x%d @ %v", i.Name, i.Qty, i.Price)
}

type Inventory struct {
	items  map[string]*Item
	nextID int
	log    []string
}

var ErrNotFound = errors.New("not found")

type StockError struct {
	Name       string
	Have, Want int
}

func (e StockError) Error() string {
	return fmt.Sprintf("not enough %s: have %d, want %d", e.Name, e.Have, e.Want)
}

func NewInventory() *Inventory {
	return &Inventory{items: make(map[string]*Item), nextID: 1}
}

func (inv *Inventory) Add(name string, price Money, qty int, tags ...string) *Item {
	if it, ok := inv.items[name]; ok {
		it.Qty += qty
		inv.log = append(inv.log, "restock "+name)
		return it
	}
	it := &Item{Base: Base{ID: inv.nextID, Created: "today"}, Name: name, Price: price, Qty: qty, Tags: tags}
	inv.nextID++
	inv.items[name] = it
	inv.log = append(inv.log, "add "+name)
	return it
}

func (inv *Inventory) Take(name string, qty int) (Money, error) {
	it, ok := inv.items[name]
	if !ok {
		return 0, fmt.Errorf("take %q: %w", name, ErrNotFound)
	}
	if it.Qty < qty {
		return 0, StockError{name, it.Qty, qty}
	}
	it.Qty -= qty
	if it.Qty == 0 {
		delete(inv.items, name)
		inv.log = append(inv.log, "sold out "+name)
	}
	return it.Price * Money(qty), nil
}

func (inv *Inventory) Names() []string {
	names := make([]string, 0, len(inv.items))
	for n := range inv.items {
		names = append(names, n)
	}
	sort.Strings(names)
	return names
}

func (inv *Inventory) Value() (total Money) {
	for _, it := range inv.items {
		total += it.Total()
	}
	return
}

func mustPositive(n int) (result int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("recovered: %v", r)
		}
	}()
	if n < 0 {
		panic(fmt.Sprintf("negative: %d", n))
	}
	return n * 2, nil
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	inv := NewInventory()
	inv.Add("widget", 250, 10, "metal", "small")
	inv.Add("gadget", 1999, 2)
	inv.Add("widget", 250, 5)
	apple := inv.Add("apple", 45, 30, "food")
	fmt.Println(apple.Describe(), apple)
	fmt.Println("names:", inv.Names(), "value:", inv.Value())

	for _, req := range []struct {
		name string
		qty  int
	}{{"widget", 3}, {"gadget", 5}, {"pear", 1}, {"gadget", 2}} {
		cost, err := inv.Take(req.name, req.qty)
		var se StockError
		switch {
		case errors.Is(err, ErrNotFound):
			fmt.Println("missing:", err)
		case errors.As(err, &se):
			fmt.Printf("short by %d: %v\n", se.Want-se.Have, err)
		case err != nil:
			fmt.Println("other:", err)
		default:
			fmt.Printf("took %d %s for %v\n", req.qty, req.name, cost)
		}
	}
	fmt.Println(strings.Join(inv.log, "; "))
	fmt.Printf("%v %s %d\n", Money(-1234), Money(5), inv.items["apple"].Qty)
	for _, n := range []int{4, -1} {
		r, err := mustPositive(n)
		fmt.Println(r, err)
	}
	byTag := map[string][]string{}
	for _, name := range inv.Names() {
		for _, t := range inv.items[name].Tags {
			byTag[t] = append(byTag[t], name)
		}
	}
	tags := make([]string, 0)
	for t := range byTag {
		tags = append(tags, t)
	}
	sort.Strings(tags)
	for _, t := range tags {
		fmt.Printf("%-6s %v\n", t, byTag[t])
	}
}
