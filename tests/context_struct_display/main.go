package main

import (
	"context"
	"fmt"
)

type holder struct {
	ctx context.Context
}

func main() {
	_ = fmt.Sprintf("%v", holder{ctx: context.Background()})
	fmt.Println(context.Background())
}
