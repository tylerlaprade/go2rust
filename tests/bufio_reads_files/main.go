package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const fileName = "bufio_reads_files.txt"

func countLines(path string) (int, error) {
	f, err := os.Open(path)
	if err != nil {
		return 0, err
	}
	defer f.Close()
	sc := bufio.NewScanner(f)
	n := 0
	for sc.Scan() {
		n++
		fmt.Printf("line %d: %s\n", n, strings.ToUpper(sc.Text()))
	}
	return n, sc.Err()
}

func countWords(path string) int {
	f, err := os.Open(path)
	if err != nil {
		return -1
	}
	defer f.Close()
	sc := bufio.NewScanner(f)
	sc.Split(bufio.ScanWords)
	words := 0
	for sc.Scan() {
		words++
	}
	return words
}

func main() {
	content := "alpha beta\ngamma\n\ndelta epsilon zeta\nlast line without newline"
	if err := os.WriteFile(fileName, []byte(content), 0o644); err != nil {
		fmt.Println("write failed:", err)
		return
	}
	defer os.Remove(fileName)

	n, err := countLines(fileName)
	fmt.Println("lines:", n, "err:", err)
	fmt.Println("words:", countWords(fileName))

	g, err := os.Open(fileName)
	if err != nil {
		fmt.Println("open failed:", err)
		return
	}
	data, err := io.ReadAll(g)
	g.Close()
	fmt.Println("readall:", len(data), err, string(data) == content)

	h, _ := os.Open(fileName)
	r := bufio.NewReader(h)
	for i := 0; ; i++ {
		line, err := r.ReadString('\n')
		fmt.Printf("chunk %d: %q err=%v eof=%v\n", i, line, err, err == io.EOF)
		if err != nil {
			break
		}
	}
	h.Close()

	_, err = os.Open("does_not_exist.txt")
	fmt.Println("missing file error:", err != nil)

	sr := bufio.NewReader(strings.NewReader("x\ny"))
	first, err := sr.ReadString('\n')
	fmt.Printf("%q %v\n", first, err)
	second, err := sr.ReadString('\n')
	fmt.Printf("%q %v %v\n", second, err, err == io.EOF)
}
