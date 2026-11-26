// Gotos whose scopes would cross: a forward block that opens at an early
// loop's label or closes at its start, gotos to one label from before and
// after it, a label that is a Rust keyword, locals named like the flags,
// and two loops one forward goto jumps over, nested.
package main

import "fmt"

// A goto into the label's loop from before the loop's start, and a forward
// block that opens at the label of an early loop.
func caseA(x, y bool, z int) {
	n := 0
	if x {
		goto Skip
	}
Back:
	n++
	if y && n > 2 {
		goto End
	}
Skip:
	if n < z {
		goto Back
	}
End:
	fmt.Println("A", x, y, z, n)
}

// A forward label at the early loop's start.
func caseB(x, y bool, z int) {
	n := 0
	if x {
		goto Entry
	}
	n += 10
Entry:
	if y {
		goto Skip
	}
Back:
	n++
Skip:
	if n < z {
		goto Back
	}
	fmt.Println("B", x, y, z, n)
}

// Gotos to one label from before and after it.
func caseC(a, b bool, c int) {
	n := 0
	if a {
		goto Back
	}
	if b {
		goto Skip
	}
	n += 100
Back:
	n++
Skip:
	if n < c {
		goto Back
	}
	fmt.Println("C", a, b, c, n)
}

// A label that is a Rust keyword, and a local whose name a flag would take.
func caseD(x bool) {
	__goto_reenter_a := 7
	n := 0
	if x {
		goto skip
	}
match:
	n++
skip:
	if n < 3 {
		goto match
	}
	fmt.Println("D", x, n, __goto_reenter_a)
}

// Two labels whose flag names would clash, with nested early loops.
func caseE(x bool) {
	__goto_reenter_a := 1
	n, m := 0, 0
	if x {
		goto s1
	}
a_:
	if x {
		goto s2
	}
a:
	n++
s2:
	if n < 2 {
		goto a
	}
	m++
s1:
	if m < 2 {
		goto a_
	}
	fmt.Println("E", x, n, m, __goto_reenter_a)
}

// The early loop's label marks the statement with the forward goto.
func caseG(x, y bool) {
	n := 0
	if x {
		goto Skip
	}
Back:
	if y && n > 1 {
		goto End
	}
	n++
Skip:
	if n < 3 {
		goto Back
	}
End:
	fmt.Println("G", x, y, n)
}

// The loop's last statement is a break, after which the lowering closes
// the label's loop.
func caseH(run, skip, retry bool) int {
	n := 0
	for run {
		if skip {
			goto Skip
		}
	Back:
		n++
	Skip:
		if retry && n < 3 {
			goto Back
		}
		break
	}
	return n
}

// A loop label that no goto uses keeps its loop's continue.
func caseF(n int) {
	i := 0
	if n < 0 {
		goto done
	}
outer:
	for j := 0; j < 3; j++ {
		for k := 0; k < 3; k++ {
			if k == j {
				continue outer
			}
			i += k
		}
	}
done:
	fmt.Println("F", n, i)
}

func main() {
	for _, x := range []bool{false, true} {
		for _, y := range []bool{false, true} {
			caseA(x, y, 4)
			caseB(x, y, 3)
			caseC(x, y, 3)
			caseG(x, y)
		}
		caseD(x)
		caseE(x)
	}
	fmt.Println("H", caseH(true, false, false), caseH(true, true, true), caseH(false, true, true), caseH(true, false, true))
	caseF(1)
	caseF(-1)
}
