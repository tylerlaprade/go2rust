package main

import "fmt"

type Tracker struct {
	NeedsChannel bool
	NeedsContext bool
}

func (t *Tracker) WithoutShared() *Tracker {
	if t == nil {
		return nil
	}
	copy := *t
	copy.NeedsChannel = false
	copy.NeedsContext = false
	return &copy
}

func main() {
	tracker := &Tracker{NeedsChannel: true, NeedsContext: true}
	copy := tracker.WithoutShared()
	fmt.Println(tracker.NeedsChannel, tracker.NeedsContext, copy.NeedsChannel, copy.NeedsContext)
}
