package main

import (
	"encoding/json"
	"fmt"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	fmt.Println(
		json.Valid([]byte(`{"x":[true,null,"a\\b"]}`)),
		json.Valid([]byte(" \n [1,2,3] \t")),
		json.Valid([]byte(`"escaped\u263a"`)),
		json.Valid([]byte(`-1.2e+3`)),
		json.Valid([]byte(`1e999`)),
		json.Valid([]byte(`"\ud800"`)),
		json.Valid([]byte{'"', 0xff, '"'}),
		json.Valid([]byte(`{"x":`)),
		json.Valid([]byte(`true false`)),
		json.Valid([]byte(`01`)),
		json.Valid([]byte{}),
		json.Valid([]byte{0xff}),
	)
}
