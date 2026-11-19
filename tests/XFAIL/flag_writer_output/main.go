package main

import (
	"flag"
	"fmt"
)

type sink struct{ text string }

func (s *sink) Write(p []byte) (int, error) {
	s.text += string(p)
	return len(p), nil
}

func main() {
	set := flag.NewFlagSet("probe", flag.ContinueOnError)
	out := &sink{}
	set.SetOutput(out)
	err := set.Parse([]string{"-missing"})
	fmt.Println(err != nil)
	fmt.Print(out.text)
}
