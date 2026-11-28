package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
	defer cancel()
	<-ctx.Done()
	fmt.Println(ctx.Err(), errors.Is(ctx.Err(), context.DeadlineExceeded), ctx.Err() == context.DeadlineExceeded)
	c2, cancel2 := context.WithCancel(context.Background())
	cancel2()
	fmt.Println(c2.Err(), errors.Is(c2.Err(), context.Canceled), errors.Is(c2.Err(), context.DeadlineExceeded))
	wrapped := fmt.Errorf("op: %w", c2.Err())
	fmt.Println(errors.Is(wrapped, context.Canceled), wrapped)
}
