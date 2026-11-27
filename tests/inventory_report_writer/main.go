package main

// A report over a defined map of struct pointers, written to an io.Writer.
// The struct holding the map has a Display, so the map type needs one.

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
	"strings"
)

type Item struct {
	Name  string
	Qty   int
	Price float64
	Tags  []string
}

type Inventory map[string]*Item

func (inv Inventory) Add(name string, qty int, price float64, tags ...string) {
	if it, ok := inv[name]; ok {
		it.Qty += qty
		it.Tags = append(it.Tags, tags...)
		return
	}
	inv[name] = &Item{Name: name, Qty: qty, Price: price, Tags: tags}
}

func (inv Inventory) Total() float64 {
	t := 0.0
	for _, it := range inv {
		t += float64(it.Qty) * it.Price
	}
	return t
}

type Reporter interface {
	Report(w io.Writer) error
}

type textReport struct {
	inv   Inventory
	title string
}

func (r textReport) Report(w io.Writer) error {
	names := make([]string, 0, len(r.inv))
	for n := range r.inv {
		names = append(names, n)
	}
	sort.SliceStable(names, func(i, j int) bool { return r.inv[names[i]].Qty > r.inv[names[j]].Qty })
	fmt.Fprintf(w, "== %s ==\n", r.title)
	for _, n := range names {
		it := r.inv[n]
		fmt.Fprintf(w, "%-6s %3d x %6.2f [%s]\n", it.Name, it.Qty, it.Price, strings.Join(it.Tags, ","))
	}
	_, err := fmt.Fprintf(w, "total: %.2f\n", r.inv.Total())
	return err
}

func main() {
	inv := Inventory{}
	inv.Add("apple", 3, 0.5, "fruit")
	inv.Add("pear", 1, 0.75, "fruit", "green")
	inv.Add("apple", 2, 0.5, "red")
	inv.Add("nut", 10, 0.1)
	var sb strings.Builder
	var rep Reporter = textReport{inv: inv, title: "stock"}
	if err := rep.Report(&sb); err != nil {
		fmt.Println("error", err)
	}
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()
	fmt.Fprint(out, sb.String())
	var a, b int
	var op string
	n, err := fmt.Sscanf("12 + 30", "%d %s %d", &a, &op, &b)
	fmt.Fprintln(out, n, err, a, op, b, a+b)
	fmt.Fprintf(out, "%5.1f%%|%x|%08b|%q|%v|%T\n", 12.345, 255, 5, 'x', []string{"a"}, inv)
}
