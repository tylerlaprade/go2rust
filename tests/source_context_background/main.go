package main

import (
	"context"
	"fmt"
)

func main() {
	ctx := context.Background()
	fmt.Println(ctx.Err() == nil)
	fmt.Println(ctx.Done() == nil)
}
