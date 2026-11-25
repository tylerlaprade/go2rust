// In a program with goroutines: fmt.Printf of values of the package's
// interfaces, from a goroutine too.
package main

import "fmt"

type Named interface{ Name() string }

type Person struct{ First, Last string }

func (p *Person) Name() string { return p.First + " " + p.Last }

type Pet struct{ Kind string }

func (p Pet) Name() string   { return "a " + p.Kind }
func (p Pet) String() string { return "Pet<" + p.Kind + ">" }

type Robot struct{ ID int }

func (r *Robot) Name() string   { return fmt.Sprint("robot ", r.ID) }
func (r *Robot) String() string { return fmt.Sprintf("R%d", r.ID) }

func describe(n Named) string {
	return fmt.Sprintf("%v (%T) is %s", n, n, n.Name())
}

func main() {
	all := []Named{&Person{"Ada", "L"}, Pet{"cat"}, &Robot{7}}
	for _, n := range all {
		fmt.Println(describe(n))
	}
	var none Named
	fmt.Printf("%v %T\n", none, none)
	first := all[0]
	fmt.Printf("%+v|%s\n", first, all[2])
	err := fmt.Errorf("bad %v", all[1])
	fmt.Println(err)
	done := make(chan string)
	go func() { done <- describe(all[2]) }()
	fmt.Println(<-done)
}
