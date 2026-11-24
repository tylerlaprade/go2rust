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
	data, err = os.ReadFile(path)
	fmt.Println(len(data), err)

	g, _ := os.Create(path)
	g.WriteString("abc")
	buf := make([]byte, 2)
	n, err = g.ReadAt(buf, 1)
	fmt.Println(n, err, string(buf))
	fmt.Println(g.Close())
	n, err = g.WriteString("x")
	fmt.Println(n, err)

	h, _ := os.Create(path)
	h.WriteString("abc")
	big := make([]byte, 4)
	n, err = h.ReadAt(big, 1)
	fmt.Println(n, err, string(big[:n]))
	_, err = h.ReadAt(big, -1)
	fmt.Println(err != nil)
	fmt.Println(h.Close(), h.Close() != nil)
	_, err = h.Read(big)
	fmt.Println(err != nil)
	// A closed file fails a read into a nil buffer too, but an empty ReadAt
	// reads nothing and succeeds.
	var none []byte
	_, err = h.Read(none)
	fmt.Println(err != nil)
	n, err = h.ReadAt(big[:0], 0)
	fmt.Println(n, err)

	ap, err := os.OpenFile(path, os.O_WRONLY|os.O_APPEND|os.O_TRUNC, 0)
	fmt.Println(err)
	ap.WriteString("x")
	ap.WriteString("y")
	ap.Close()
	data, _ = os.ReadFile(path)
	fmt.Println(string(data))

	sy, err := os.OpenFile(path, os.O_WRONLY|os.O_APPEND|os.O_SYNC, 0)
	fmt.Println(err)
	sy.WriteString("z")
	sy.Close()
	data, _ = os.ReadFile(path)
	fmt.Println(string(data))

	// OpenFile's errors are *PathError values that os.IsNotExist and
	// os.IsExist look inside.
	_, err = os.OpenFile("os_create_write_missing.txt", os.O_RDONLY, 0)
	fmt.Println(os.IsNotExist(err), os.IsExist(err))
	_, err = os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
	fmt.Println(os.IsExist(err), os.IsNotExist(err))

	// A file is not a directory, so nothing can be created inside it.
	_, err = os.Create(path + "/x.txt")
	fmt.Println(err != nil)
}
