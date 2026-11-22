package main

import (
	"fmt"
	"os"
)

func main() {
	const key = "GO2RUST_MUTATE_ENV"
	_ = os.Unsetenv(key)

	err := os.Setenv(key, "ok")
	fmt.Println(err == nil, os.Getenv(key))
	err = os.Setenv(key, string([]byte{0xfa}))
	fmt.Println(err == nil, []byte(os.Getenv(key)))
	err = os.Unsetenv(key)
	fmt.Println(err == nil, os.Getenv(key) == "")

	badKey := "GO2RUST_" + string([]byte{0xfb})
	err = os.Setenv(badKey, "raw-key")
	fmt.Println(err == nil, os.Getenv(badKey))
	err = os.Unsetenv(badKey)
	fmt.Println(err == nil, os.Getenv(badKey) == "")

	nul := string([]byte{'A', 0, 'B'})
	fmt.Println(os.Setenv("", "x") != nil)
	fmt.Println(os.Setenv("A=B", "x") != nil)
	fmt.Println(os.Setenv(nul, "x") != nil)
	fmt.Println(os.Setenv("GO2RUST_NUL", nul) != nil)
	fmt.Println(os.Unsetenv("") == nil, os.Unsetenv("A=B") == nil, os.Unsetenv(nul) == nil)
}
