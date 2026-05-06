package main

import (
	"encoding/base64"
	"fmt"
)

func main() {
	data := "Hello, World!"
	encoded := base64.StdEncoding.EncodeToString([]byte(data))
	fmt.Println("Encoded:", encoded)

	decoded, _ := base64.StdEncoding.DecodeString(encoded)
	fmt.Println("Decoded:", string(decoded))
}
