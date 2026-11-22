package main

import (
	"fmt"
	"os"
	"os/exec"
)

func output(env []string, script string) {
	cmd := exec.Command("/bin/sh", "-c", script)
	cmd.Env = env
	out, err := cmd.Output()
	fmt.Println(err == nil, []byte(out))
}

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	output(nil, `printf '%s' "$GO2RUST_HOST_RAW"`)
	output([]string{}, `if [ "${GO2RUST_HOST_RAW+x}" = x ]; then printf present; else printf absent; fi`)

	rawChild := "GO2RUST_CHILD=" + string([]byte{0xfb, 'B'})
	nul := "N" + string([]byte{0}) + "X=bad"
	output(
		[]string{"GO2RUST_CHILD=old", rawChild, "BROKEN"},
		`if [ "${GO2RUST_HOST_RAW+x}" = x ]; then printf present; else printf absent; fi; printf ':'; printf '%s' "$GO2RUST_CHILD"`,
	)
	output([]string{rawChild, nul}, `printf should-not-run`)

	cmd := exec.Command("/bin/sh", "-c", `printf '%s' "$PWD"`)
	cmd.Dir = "."
	out, err := cmd.Output()
	wd, _ := os.Getwd()
	fmt.Println(err == nil, string(out) == wd)

	cmd = exec.Command("/bin/sh", "-c", "printf fail; exit 7")
	cmd.Env = []string{}
	out, err = cmd.Output()
	fmt.Println([]byte(out), err != nil)
}
