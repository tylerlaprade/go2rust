// A file made with os.Create, in a program that needs no shared-state
// wrappers, takes writes from Fprintf, WriteString and Write.
package main

import (
	"fmt"
	"os"
)

func main() {
	path := "os_create_write_plain_test.txt"
	defer os.Remove(path)
	f, err := os.Create(path)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	total := 0
	for i := 1; i <= 3; i++ {
		n, _ := fmt.Fprintf(f, "row %d\n", i)
		total += n
	}
	n, _ := f.WriteString("end\n")
	total += n
	f.Write([]byte{'!', '\n'})
	fmt.Println(total)
	f.Close()
	data, err := os.ReadFile(path)
	fmt.Print(string(data))
	fmt.Println(err)
}
