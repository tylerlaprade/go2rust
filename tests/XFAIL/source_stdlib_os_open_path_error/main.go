package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

func main() {
	const path = "/definitely/missing/go2rust"
	_, err := os.Open(path)
	var target *fs.PathError
	fmt.Println(errors.As(err, &target))
	if target != nil {
		fmt.Println(target.Op, target.Path == path)
		fmt.Println(errors.Is(errors.Unwrap(err), fs.ErrNotExist))
	}
}
