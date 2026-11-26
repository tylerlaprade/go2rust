package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"testing"
)

// Two backward labels whose early loops would start at the same forward
// goto overlap each other, so both keep the plain lowering. The check must
// not drop one candidate and then accept the other against the shrunken
// set, which would make the plan depend on map iteration order.
func TestOverlappingEarlyLoopsAreRejectedTogether(t *testing.T) {
	src := `package main

func f(x bool) {
	a, b, c := 0, 0, 0
	if x {
		goto F
	}
L1:
	a++
L2:
	b++
F:
	c++
	if a < 3 {
		goto L1
	}
	if b < 3 {
		goto L2
	}
	_ = c
}
`
	file, err := parser.ParseFile(token.NewFileSet(), "f.go", src, 0)
	if err != nil {
		t.Fatal(err)
	}
	body := file.Decls[0].(*ast.FuncDecl).Body.List
	for i := 0; i < 64; i++ {
		plan := buildGotoPlan(body)
		if len(plan.earlyLoopStart) != 0 {
			t.Fatalf("run %d: early loops %v, want none", i, plan.earlyLoopStart)
		}
		for _, label := range []string{"l1", "l2"} {
			if plan.modes[label] == gotoReenterMode {
				t.Fatalf("run %d: label %s uses the reentry lowering", i, label)
			}
		}
	}
}
