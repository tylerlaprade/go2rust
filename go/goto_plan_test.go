package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"testing"
)

// Two backward labels that one forward goto jumps over both start their
// loops at that goto, nested, the same way on every run.
func TestEarlyLoopsCrossedByOneGotoShareTheirStart(t *testing.T) {
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
		if len(plan.earlyLoopStart) != 2 || plan.earlyLoopStart["l1"] != 1 || plan.earlyLoopStart["l2"] != 1 {
			t.Fatalf("run %d: early loops %v, want l1 and l2 at 1", i, plan.earlyLoopStart)
		}
		if !gotoScopesNest(plan, len(body)) {
			t.Fatalf("run %d: scopes do not nest", i)
		}
		for _, label := range []string{"l1", "l2"} {
			if plan.modes[label] != gotoReenterMode {
				t.Fatalf("run %d: label %s does not use the reentry lowering", i, label)
			}
		}
	}
}
