package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	operationDone := time.After(500 * time.Millisecond)

	select {
	case <-operationDone:
		fmt.Println("Operation completed")
	case <-ctx.Done():
		fmt.Println("Context cancelled:", ctx.Err())
	}

	ctx2, cancel2 := context.WithCancelCause(context.Background())
	cancel2(errors.New("boom"))
	<-ctx2.Done()
	fmt.Println("Cause cancel:", ctx2.Err())
}
