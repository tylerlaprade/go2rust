package main

import (
	"encoding/json"
	"fmt"
)

type OverlayJSON struct {
	Replace map[string]string `json:"replace,omitempty"`
}

func main() {
	overlays := map[string]string{
		"b.go": "tmp-b",
		"a.go": "tmp-a",
	}

	data, _ := json.Marshal(OverlayJSON{Replace: overlays})
	fmt.Println(string(data))

	empty, _ := json.Marshal(OverlayJSON{})
	fmt.Println(string(empty))
}
