package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	rawKey := string([]byte{0xfa, 'K'})
	ordinarySet := os.Setenv("GO2RUST_CLEARENV", "ordinary")
	rawSet := os.Setenv(rawKey, "raw")
	_, ordinaryBefore := os.LookupEnv("GO2RUST_CLEARENV")
	_, rawBefore := os.LookupEnv(rawKey)
	os.Clearenv()
	_, ordinaryAfter := os.LookupEnv("GO2RUST_CLEARENV")
	_, rawAfter := os.LookupEnv(rawKey)
	fmt.Println(ordinarySet == nil, rawSet == nil, ordinaryBefore, rawBefore, !ordinaryAfter, !rawAfter)
}
