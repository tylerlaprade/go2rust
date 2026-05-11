package main

import "fmt"

type Config struct {
	Name string
}

type loader struct {
	Config
}

func use(cfg *Config, patterns ...string) string {
	return cfg.Name + ":" + patterns[0]
}

func main() {
	ld := &loader{Config: Config{Name: "cfg"}}
	fmt.Println(use(&ld.Config, "pat"))
}
