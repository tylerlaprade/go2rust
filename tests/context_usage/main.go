package main

import (
	"context"
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
}
