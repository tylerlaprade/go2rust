package main

import (
	"fmt"
	"os"
	"os/exec"
)

func dump(env []string) {
	fmt.Println(len(env))
	for _, entry := range env {
		fmt.Println([]byte(entry))
	}
}

func main() {
	cmd := exec.Command("ignored")
	foundRaw := false
	for _, entry := range cmd.Environ() {
		prefix := "GO2RUST_CMD_LIST="
		if len(entry) >= len(prefix) && entry[:len(prefix)] == prefix {
			fmt.Println([]byte(entry))
			foundRaw = true
		}
	}
	fmt.Println(foundRaw)

	rawKey := "R" + string([]byte{0xfb})
	nul := "N" + string([]byte{0}) + "X=bad"
	cmd.Env = []string{
		rawKey + "=old",
		"BROKEN",
		"A=old",
		rawKey + "=new",
		"A=new",
		"BROKEN",
		"",
		"B=1",
		"B=2",
		nul,
	}
	dump(cmd.Environ())

	cmd.Env = []string{}
	fmt.Println(len(cmd.Environ()))

	cmd.Env = nil
	cmd.Dir = "."
	wd, _ := os.Getwd()
	foundPWD := false
	for _, entry := range cmd.Environ() {
		if entry == "PWD="+wd {
			foundPWD = true
		}
	}
	fmt.Println(foundPWD)
}
