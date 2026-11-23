// Goroutines exchange interface values over unbuffered channels, and a bus
// keeps a slice of channels per topic.
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Event interface {
	Kind() string
}

type Click struct{ X, Y int }

func (c Click) Kind() string { return fmt.Sprintf("click(%d,%d)", c.X, c.Y) }

type Key struct{ Code rune }

func (k *Key) Kind() string { return "key " + string(k.Code) }

type Bus struct {
	mu       sync.Mutex
	handlers map[string][]chan Event
}

func (b *Bus) Subscribe(topic string) <-chan Event {
	b.mu.Lock()
	defer b.mu.Unlock()
	ch := make(chan Event, 4)
	b.handlers[topic] = append(b.handlers[topic], ch)
	return ch
}

func (b *Bus) Publish(topic string, e Event) {
	b.mu.Lock()
	defer b.mu.Unlock()
	for _, ch := range b.handlers[topic] {
		ch <- e
	}
}

func (b *Bus) Close() {
	b.mu.Lock()
	defer b.mu.Unlock()
	for _, chans := range b.handlers {
		for _, ch := range chans {
			close(ch)
		}
	}
}

func main() {
	events := make(chan Event)
	done := make(chan []string)
	go func() {
		var seen []string
		for e := range events {
			seen = append(seen, e.Kind())
		}
		done <- seen
	}()
	var wg sync.WaitGroup
	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(n int) {
			defer wg.Done()
			events <- Click{n, n * 2}
			events <- &Key{rune('a' + n)}
		}(i)
	}
	wg.Wait()
	close(events)
	seen := <-done
	sort.Strings(seen)
	fmt.Println(seen)

	bus := &Bus{handlers: map[string][]chan Event{}}
	a := bus.Subscribe("ui")
	b := bus.Subscribe("ui")
	bus.Publish("ui", Click{1, 1})
	bus.Publish("ui", &Key{'z'})
	bus.Close()
	for e := range a {
		fmt.Println("a got", e.Kind())
	}
	for {
		e, ok := <-b
		if !ok {
			break
		}
		fmt.Println("b got", e.Kind())
	}

	results := make(chan Event)
	go func() {
		defer close(results)
		for _, e := range []Event{Click{3, 4}, nil, &Key{'q'}} {
			results <- e
		}
	}()
	for e := range results {
		if e == nil {
			fmt.Println("nil event")
			continue
		}
		if c, ok := e.(Click); ok {
			fmt.Println("click at", c.X, c.Y)
		} else {
			fmt.Println("other", e.Kind())
		}
	}
}
