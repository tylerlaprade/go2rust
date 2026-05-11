package main

import "fmt"

type LoadMode int

const (
	NeedName LoadMode = 1 << iota
	NeedFiles
	NeedImports
)

var modes = [...]struct {
	mode LoadMode
	name string
}{
	{NeedName, "NeedName"},
	{NeedFiles, "NeedFiles"},
	{NeedImports, "NeedImports"},
}

func strip(mode LoadMode) string {
	out := ""
	for _, item := range modes {
		if (mode & item.mode) != 0 {
			mode ^= item.mode
			if out != "" {
				out += ","
			}
			out += item.name
		}
	}
	if mode != 0 {
		if out != "" {
			out += ","
		}
		out += fmt.Sprintf("%#x", int(mode))
	}
	if out == "" {
		return "none"
	}
	return out
}

func main() {
	fmt.Println(strip(NeedName | NeedImports))
	fmt.Println(strip(NeedFiles | LoadMode(8)))
}
