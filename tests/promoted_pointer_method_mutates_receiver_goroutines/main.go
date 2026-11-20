// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Basic struct {
	id      string
	balance float64
	history []string
}

func (b *Basic) Deposit(a float64) {
	b.balance += a
	b.history = append(b.history, fmt.Sprintf("+%.2f", a))
}

type Savings struct {
	Basic
	rate float64
}

func (s *Savings) AddInterest() {
	s.Deposit(s.balance * s.rate)
}

type Premium struct {
	*Savings
	bonus float64
}

func (p *Premium) Reward() {
	p.AddInterest()
	p.Deposit(p.bonus)
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	s := &Savings{Basic: Basic{id: "bob"}, rate: 0.5}
	s.Deposit(100)
	s.AddInterest()
	fmt.Println(s.id, s.balance, s.history)
	p := &Premium{Savings: &Savings{Basic: Basic{id: "ann"}, rate: 0.1}, bonus: 5}
	p.Deposit(50)
	p.Reward()
	fmt.Printf("%s %.2f %v\n", p.id, p.balance, p.history)
}
