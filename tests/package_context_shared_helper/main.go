package main

import (
	"context"
	"fmt"

	"example.com/contextshared/event"
)

func main() {
	ctx := event.SetExporter(func(ctx context.Context) context.Context {
		return ctx
	})
	_ = ctx
	fmt.Println("compiled")
}
