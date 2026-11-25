package main

import (
	"flag"
	"fmt"
	"time"
)

func main() {
	fs := flag.NewFlagSet("demo", flag.ContinueOnError)
	n := fs.Int("n", 1, "count")
	name := fs.String("name", "x", "name")
	verbose := fs.Bool("v", false, "verbose")
	d := fs.Duration("d", time.Second, "delay")
	var limit int
	fs.IntVar(&limit, "limit", 10, "limit")
	err := fs.Parse([]string{"-n", "5", "-name", "gopher", "-v", "-d", "250ms", "-limit=3", "rest", "args"})
	fmt.Println(err, *n, *name, *verbose, *d, limit, fs.Args(), fs.NArg())
	fs.VisitAll(func(f *flag.Flag) { fmt.Println(f.Name, f.Value.String(), f.DefValue) })
}
