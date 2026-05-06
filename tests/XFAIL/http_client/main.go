package main

import (
	"bufio"
	"fmt"
	"io"
	"net/http"
	"strings"
)

func main() {
	payload := `{"slideshow":{"author":"Yours Truly","slides":[1,2,3]}}`
	raw := "HTTP/1.1 200 OK\r\nContent-Length: 57\r\n\r\n" + payload
	resp, err := http.ReadResponse(bufio.NewReader(strings.NewReader(raw)), nil)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	text := string(body)
	if len(text) > 100 {
		text = text[:100]
	}
	fmt.Println("Response:", text)
}
