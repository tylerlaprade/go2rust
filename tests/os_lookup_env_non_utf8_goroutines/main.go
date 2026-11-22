package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	value, ok := os.LookupEnv("GO2RUST_LOOKUP_RAW")
	fmt.Println([]byte(value), ok)

	_ = os.Setenv("GO2RUST_LOOKUP_EMPTY", "")
	value, ok = os.LookupEnv("GO2RUST_LOOKUP_EMPTY")
	fmt.Println([]byte(value), ok)

	_ = os.Unsetenv("GO2RUST_LOOKUP_MISSING")
	value, ok = os.LookupEnv("GO2RUST_LOOKUP_MISSING")
	fmt.Println([]byte(value), ok)

	rawKey := "GO2RUST_LOOKUP_" + string([]byte{0xfb})
	_ = os.Setenv(rawKey, "raw-key")
	value, ok = os.LookupEnv(rawKey)
	fmt.Println([]byte(value), ok)
	_ = os.Unsetenv(rawKey)
}
