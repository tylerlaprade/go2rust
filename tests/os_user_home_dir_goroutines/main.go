package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	ordinaryHome := "/tmp/go2rust-home"
	setHomeErr := os.Setenv("HOME", ordinaryHome)
	setProfileErr := os.Setenv("USERPROFILE", ordinaryHome)
	ordinary, ordinaryErr := os.UserHomeDir()

	unsetHomeErr := os.Unsetenv("HOME")
	unsetProfileErr := os.Unsetenv("USERPROFILE")
	missing, missingErr := os.UserHomeDir()

	rawOK := true
	if os.PathSeparator == '/' {
		raw := string([]byte{'/', 't', 'm', 'p', '/', 0xfa, 'H'})
		setRawErr := os.Setenv("HOME", raw)
		rawHome, rawErr := os.UserHomeDir()
		rawOK = setRawErr == nil && rawErr == nil && rawHome == raw
	}

	fmt.Println(
		setHomeErr == nil && setProfileErr == nil && ordinaryErr == nil && ordinary == ordinaryHome,
		unsetHomeErr == nil && unsetProfileErr == nil,
		missing == "",
		missingErr != nil,
		rawOK,
	)
}
