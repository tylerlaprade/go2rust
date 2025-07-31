package main

import (
	"fmt"
	"net/url"
)

func main() {
	u, err := url.Parse("https://example.com/path?key=value")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	fmt.Println("Scheme:", u.Scheme)
	fmt.Println("Host:", u.Host)
	fmt.Println("Path:", u.Path)
	fmt.Println("Query:", u.RawQuery)
}
