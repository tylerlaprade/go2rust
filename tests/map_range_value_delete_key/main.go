package main

import "fmt"

func main() {
	ids := map[string]bool{
		"real":  true,
		"other": true,
	}
	aliases := map[string]string{
		"alias": "real",
	}

	for _, id := range aliases {
		delete(ids, id)
	}

	fmt.Println(ids["real"], ids["other"])
}
