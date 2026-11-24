// An ordinary word count: bufio.Scanner over a strings.Reader, sort.Slice,
// strings.Builder with fmt.Fprintf, and the common strings helpers. bufio,
// strings, bytes and io run Go's own code.
package main

import (
	"bufio"
	"bytes"
	"fmt"
	"sort"
	"strings"
)

func main() {
	text := "the quick brown fox jumps over the lazy dog the end"
	counts := map[string]int{}
	sc := bufio.NewScanner(strings.NewReader(text))
	sc.Split(bufio.ScanWords)
	for sc.Scan() {
		counts[sc.Text()]++
	}
	words := make([]string, 0, len(counts))
	for w := range counts {
		words = append(words, w)
	}
	sort.Slice(words, func(i, j int) bool {
		if counts[words[i]] != counts[words[j]] {
			return counts[words[i]] > counts[words[j]]
		}
		return words[i] < words[j]
	})
	var b strings.Builder
	for _, w := range words[:3] {
		fmt.Fprintf(&b, "%s=%d ", w, counts[w])
	}
	fmt.Println(strings.TrimSpace(b.String()))

	lines := bufio.NewScanner(strings.NewReader("first line\nsecond line\n\nfourth"))
	n := 0
	for lines.Scan() {
		n++
		fmt.Printf("%d: %q\n", n, lines.Text())
	}

	fmt.Println(strings.Fields(" a  b c "), strings.Repeat("ab", 3), strings.ToUpper("go"), strings.ToLower("X"))
	fmt.Println(strings.Split("a,b,c", ","), strings.Join([]string{"x", "y"}, "-"), strings.Contains("hello", "ell"))
	fmt.Println(strings.Index("chicken", "ken"), strings.LastIndex("go gopher", "go"), strings.Count("cheese", "e"))
	fmt.Println(strings.HasPrefix("golang", "go"), strings.TrimLeft("xxhixx", "x"), strings.Map(func(r rune) rune { return r + 1 }, "abc"))
	r := strings.NewReplacer("a", "1", "b", "2", "<", "&lt;")
	fmt.Println(r.Replace("abc <tag>"))
	before, after, found := strings.Cut("key=value", "=")
	fmt.Println(strings.EqualFold("Go", "GO"), strings.SplitN("a,b,c,d", ",", 2), before, after, found)

	var buf bytes.Buffer
	buf.WriteString("hello ")
	buf.Write([]byte("bytes"))
	fmt.Fprintf(&buf, " %d", 42)
	fmt.Println(buf.String(), buf.Len(), bytes.Contains(buf.Bytes(), []byte("byte")))
	fmt.Println(bytes.ToUpper([]byte("abc")), string(bytes.TrimSpace([]byte("  x  "))))

	w := bufio.NewWriter(&buf)
	fmt.Fprint(w, " buffered")
	w.Flush()
	fmt.Println(buf.String())
}
