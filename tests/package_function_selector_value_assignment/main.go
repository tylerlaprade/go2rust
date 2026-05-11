package main

import (
	"fmt"
	"log"
)

type Config struct {
	Logf func(format string, args ...any)
}

type loader struct {
	Config Config
}

func main() {
	var cfg Config
	cfg.Logf = log.Printf
	var copied Config
	copied.Logf = cfg.Logf
	var ld loader
	ld.Config.Logf = cfg.Logf
	ld.Config.Logf = log.Printf
	fmt.Println("assigned")
}
