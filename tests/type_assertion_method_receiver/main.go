package main

import "fmt"

type Signature struct {
	name string
}

func (s *Signature) Recv() string {
	return s.name
}

func recvName(v any) string {
	return v.(*Signature).Recv()
}

func main() {
	fmt.Println(recvName(&Signature{name: "receiver"}))
}
