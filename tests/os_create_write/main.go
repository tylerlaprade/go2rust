// A file made with os.Create takes writes from Fprintf, WriteString, Write
// and a bufio.Writer, and Close leaves them on disk for os.ReadFile.
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Report struct {
	Name  string
	Lines []string
}

func (r Report) Save(path string) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	defer f.Close()
	w := bufio.NewWriter(f)
	fmt.Fprintf(w, "# %s\n", r.Name)
	for i, line := range r.Lines {
		fmt.Fprintf(w, "%d. %s\n", i+1, strings.ToUpper(line))
	}
	return w.Flush()
}

func main() {
	path := "os_create_write_test.txt"
	defer os.Remove(path)
	f, err := os.Create(path)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Fprintf(f, "line %d\n", 1)
	f.WriteString("raw\n")
	n, err := f.Write([]byte("bytes\n"))
	fmt.Println(n, err)
	fmt.Println(f.Close())
	data, err := os.ReadFile(path)
	fmt.Printf("%q %v\n", string(data), err)

	r := Report{Name: "todo", Lines: []string{"milk", "eggs"}}
	fmt.Println(r.Save(path))
	data, _ = os.ReadFile(path)
	fmt.Print(string(data))

	f, err = os.Create(path)
	fmt.Println(err == nil)
	f.Close()
	data, _ = os.ReadFile(path)
	fmt.Println(len(data))

	_, err = os.Create("/nonexistent-dir/x.txt")
	fmt.Println(err != nil)
}
