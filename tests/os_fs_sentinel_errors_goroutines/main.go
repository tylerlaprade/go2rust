// In a program with goroutines: os.ErrNotExist, ErrExist and ErrPermission,
// and fs's names for them, print Go's text, match through errors.Is and %w,
// match an OS error of their kind, and are each one error.
package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"sync"
)

func find(name string) error {
	if name == "" {
		return fmt.Errorf("find %q: %w", name, os.ErrNotExist)
	}
	return nil
}

func main() {
	err := find("")
	fmt.Println(err)
	fmt.Println(errors.Is(err, os.ErrNotExist), errors.Is(err, fs.ErrNotExist), errors.Is(err, os.ErrExist))
	fmt.Println(os.ErrNotExist, fs.ErrExist, os.ErrPermission)
	_, openErr := os.Open("/definitely/not/here")
	fmt.Println(errors.Is(openErr, os.ErrNotExist), errors.Is(openErr, os.ErrPermission), os.IsNotExist(openErr))
	wrapped := fmt.Errorf("stage: %w", fs.ErrPermission)
	fmt.Println(errors.Is(wrapped, os.ErrPermission), wrapped)
	fmt.Println(os.ErrNotExist == fs.ErrNotExist, errors.Is(os.ErrNotExist, os.ErrNotExist))

	var wg sync.WaitGroup
	found := make([]bool, 2)
	for i, name := range []string{"", "x"} {
		wg.Add(1)
		go func(i int, name string) {
			defer wg.Done()
			found[i] = errors.Is(find(name), fs.ErrNotExist)
		}(i, name)
	}
	wg.Wait()
	fmt.Println(found)
}
