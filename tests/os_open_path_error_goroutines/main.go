package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

const missingPath = "/definitely/missing/go2rust"

func inspect(done chan struct{}) {
	_, err := os.Open(missingPath)
	var pathErr *os.PathError
	fmt.Println(errors.As(err, &pathErr))
	if pathErr != nil {
		fmt.Println(pathErr.Op)
		fmt.Println(pathErr.Path)
		fmt.Println(errors.Is(pathErr.Err, fs.ErrNotExist))
		fmt.Println(errors.Is(err, fs.ErrNotExist))
		cause := errors.Unwrap(err)
		fmt.Println(errors.Is(fs.ErrNotExist, cause))
		_, another := os.Open(missingPath)
		fmt.Println(errors.Is(cause, errors.Unwrap(another)))
	}
	close(done)
}

func main() {
	done := make(chan struct{})
	go inspect(done)
	<-done
}
