package main

import (
	"fmt"
	"slices"
)

type Config struct {
	Env []string
	Dir string
}

func buildEnv(cfg *Config) []string {
	return append(slices.Clip(cfg.Env), "PWD="+cfg.Dir)
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	cfg := &Config{
		Env: []string{"A=1"},
		Dir: "/tmp/work",
	}
	env := buildEnv(cfg)
	fmt.Println(len(env), env[0], env[1])
}
