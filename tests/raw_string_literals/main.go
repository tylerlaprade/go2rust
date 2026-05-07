package main

import "fmt"

func main() {
	const manifest = `[package]
name = "demo"
`

	fmt.Println(`raw\ntext`)
	fmt.Printf(`[package]
name = "%s"
`, "demo")
	fmt.Print(manifest)
}
