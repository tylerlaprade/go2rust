package main

import (
	"context"
	"fmt"
)

func fail(ctx context.Context) error {
	return ctx.Err()
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	err := fail(ctx)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("no error")
}
