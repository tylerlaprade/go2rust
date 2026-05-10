package main

import "fmt"

type Config struct {
	Mode       string
	Env        string
	BuildFlags string
	Tests      bool
}

type Request struct {
	Mode       string
	Env        string
	BuildFlags string
	Tests      bool
}

func makeRequest(prefix string) func(Config) Request {
	return func(cfg Config) Request {
		return Request{
			Mode:       prefix + cfg.Mode,
			Env:        cfg.Env,
			BuildFlags: cfg.BuildFlags,
			Tests:      cfg.Tests,
		}
	}
}

func main() {
	build := makeRequest("driver:")
	req := build(Config{
		Mode:       "load",
		Env:        "GOOS=darwin",
		BuildFlags: "-mod=vendor",
		Tests:      true,
	})
	fmt.Println(req.Mode)
	fmt.Println(req.Env)
	fmt.Println(req.BuildFlags)
	fmt.Println(req.Tests)
}
