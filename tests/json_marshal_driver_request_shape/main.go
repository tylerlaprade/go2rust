package main

import (
	"encoding/json"
	"fmt"
)

type Mode int

type Request struct {
	Mode       Mode              `json:"mode"`
	Env        []string          `json:"env"`
	BuildFlags []string          `json:"build_flags"`
	Tests      bool              `json:"tests"`
	Overlay    map[string][]byte `json:"overlay"`
}

func main() {
	overlay := make(map[string][]byte)
	overlay["b.go"] = []byte{0, 1, 255}
	overlay["a.go"] = []byte("tmp-a")

	req := Request{
		Mode:       3,
		Env:        []string{"B=2", "A=1"},
		BuildFlags: []string{"-tags", "dev"},
		Tests:      true,
		Overlay:    overlay,
	}

	data, _ := json.Marshal(req)
	fmt.Println(string(data))
}
