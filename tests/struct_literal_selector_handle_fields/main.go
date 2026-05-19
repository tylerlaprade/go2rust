package main

import "fmt"

type Config struct {
	Env        []string
	BuildFlags []string
	Overlay    map[string]string
}

type Request struct {
	Env        []string
	BuildFlags []string
	Overlay    map[string]string
}

func buildRequest(cfg *Config) Request {
	return Request{
		Env:        cfg.Env,
		BuildFlags: cfg.BuildFlags,
		Overlay:    cfg.Overlay,
	}
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	cfg := &Config{
		Env:        []string{"GOOS=darwin"},
		BuildFlags: []string{"-tags=test"},
		Overlay: map[string]string{
			"file.go": "package main",
		},
	}
	req := buildRequest(cfg)
	fmt.Println(req.Env[0], req.BuildFlags[0], req.Overlay["file.go"])
}
