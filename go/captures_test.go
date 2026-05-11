package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestClosureCapturesUseTypeInfoScopes(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "encoding/binary"

func makeSeq(pkgs []string) func(func(string) bool) {
	return func(yield func(string) bool) {
		for _, pkg := range pkgs {
			var seen [2]byte
			var visit func(int) bool
			visit = func(i int) bool {
				bit := byte(1) << (i % 8)
				if seen[i/8]&bit == 0 {
					seen[i/8] |= bit
					var data []byte
					_, _ = binary.Uvarint(data)
					if i > 0 && !visit(i-1) {
						return false
					}
					if !yield(pkg) {
						return false
					}
				}
				return true
			}
			if !visit(0) {
				return
			}
		}
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	var returnStmt *ast.ReturnStmt
	var rangeStmt *ast.RangeStmt
	var assignStmt *ast.AssignStmt
	ast.Inspect(file, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.ReturnStmt:
			if returnStmt == nil {
				returnStmt = node
			}
		case *ast.RangeStmt:
			if rangeStmt == nil {
				rangeStmt = node
			}
		case *ast.AssignStmt:
			if len(node.Rhs) == 1 {
				if _, ok := node.Rhs[0].(*ast.FuncLit); ok {
					assignStmt = node
				}
			}
		}
		return true
	})
	if returnStmt == nil {
		t.Fatal("did not find return statement with outer closure")
	}
	if rangeStmt == nil {
		t.Fatal("did not find range statement containing inner closure")
	}
	if assignStmt == nil {
		t.Fatal("did not find assignment statement with inner closure")
	}

	sp := NewStatementPreprocessor(fset)
	outerInfo := sp.PreprocessStatement(returnStmt, nil)
	if outerInfo == nil {
		t.Fatal("outer closure should capture pkgs")
	}
	if _, ok := outerInfo.CapturedVars["pkgs"]; !ok {
		t.Fatalf("outer closure should capture pkgs, got %#v", outerInfo.CapturedVars)
	}
	for _, name := range []string{"binary", "byte", "seen", "uint64", "visit", "yield"} {
		if _, ok := outerInfo.CapturedVars[name]; ok {
			t.Fatalf("outer closure should not capture %q, got %#v", name, outerInfo.CapturedVars)
		}
	}

	rangeInfo := sp.PreprocessStatement(rangeStmt, nil)
	if rangeInfo != nil && len(rangeInfo.CapturedVars) > 0 {
		t.Fatalf("range statement should leave body closure captures to body statements, got %#v", rangeInfo.CapturedVars)
	}

	innerInfo := sp.PreprocessStatement(assignStmt, nil)
	if innerInfo == nil {
		t.Fatal("inner closure should capture outer closure locals")
	}
	for _, name := range []string{"pkg", "seen", "visit", "yield"} {
		if _, ok := innerInfo.CapturedVars[name]; !ok {
			t.Fatalf("inner closure should capture %q, got %#v", name, innerInfo.CapturedVars)
		}
	}
	for _, name := range []string{"binary", "byte"} {
		if _, ok := innerInfo.CapturedVars[name]; ok {
			t.Fatalf("inner closure should not capture %q, got %#v", name, innerInfo.CapturedVars)
		}
	}

	var clones strings.Builder
	sp.GenerateCloneStatements(&clones, innerInfo)
	got := clones.String()
	if strings.Contains(got, " = yield.clone()") {
		t.Fatalf("keyword parameter capture should be escaped, got %q", got)
	}
	if !strings.Contains(got, "r#yield.clone()") {
		t.Fatalf("keyword parameter capture should use raw identifier, got %q", got)
	}
}

func TestClosureParameterDoesNotCaptureOuterName(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func use(T any) func(any) any {
	visit := func(T any) any {
		return T
	}
	return visit
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	var assignStmt *ast.AssignStmt
	ast.Inspect(file, func(n ast.Node) bool {
		if assignStmt != nil {
			return false
		}
		assign, ok := n.(*ast.AssignStmt)
		if !ok || len(assign.Rhs) != 1 {
			return true
		}
		if _, ok := assign.Rhs[0].(*ast.FuncLit); ok {
			assignStmt = assign
			return false
		}
		return true
	})
	if assignStmt == nil {
		t.Fatal("did not find assignment statement with closure")
	}

	sp := NewStatementPreprocessor(fset)
	info := sp.PreprocessStatement(assignStmt, nil)
	if info != nil {
		if _, ok := info.CapturedVars["T"]; ok {
			t.Fatalf("closure parameter T should not capture outer T, got %#v", info.CapturedVars)
		}
	}
}

func TestCachedFuncLitCapturesReturnCopy(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func outer(prefix string) func(string) string {
	return func(name string) string {
		return prefix + name
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	var funcLit *ast.FuncLit
	ast.Inspect(file, func(n ast.Node) bool {
		if funcLit != nil {
			return false
		}
		if node, ok := n.(*ast.FuncLit); ok {
			funcLit = node
			return false
		}
		return true
	})
	if funcLit == nil {
		t.Fatal("did not find closure")
	}

	sp := NewStatementPreprocessor(fset)
	captured := sp.CapturedVarsForFuncLit(funcLit)
	if _, ok := captured["prefix"]; !ok {
		t.Fatalf("closure should capture prefix, got %#v", captured)
	}
	captured["name"] = true
	captured["extra"] = true

	again := sp.CapturedVarsForFuncLit(funcLit)
	if _, ok := again["prefix"]; !ok {
		t.Fatalf("cached closure should still capture prefix, got %#v", again)
	}
	for _, name := range []string{"name", "extra"} {
		if _, ok := again[name]; ok {
			t.Fatalf("cached closure captures should not include caller mutation %q, got %#v", name, again)
		}
	}
	if len(sp.funcLitCaptures) != 1 {
		t.Fatalf("expected one cached closure capture entry, got %d", len(sp.funcLitCaptures))
	}
}

func TestClosureStructLiteralFieldKeysAreNotCaptured(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Config struct {
	Mode string
	BuildFlags string
}

type Request struct {
	Mode string
	BuildFlags string
}

func makeRequest(prefix string) func(Config) Request {
	return func(cfg Config) Request {
		return Request{
			Mode: cfg.Mode,
			BuildFlags: prefix + cfg.BuildFlags,
		}
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	var returnStmt *ast.ReturnStmt
	ast.Inspect(file, func(n ast.Node) bool {
		if returnStmt != nil {
			return false
		}
		node, ok := n.(*ast.ReturnStmt)
		if !ok || len(node.Results) != 1 {
			return true
		}
		if _, ok := node.Results[0].(*ast.FuncLit); ok {
			returnStmt = node
			return false
		}
		return true
	})
	if returnStmt == nil {
		t.Fatal("did not find return statement with closure")
	}

	sp := NewStatementPreprocessor(fset)
	info := sp.PreprocessStatement(returnStmt, nil)
	if info == nil {
		t.Fatal("closure should capture prefix")
	}
	if _, ok := info.CapturedVars["prefix"]; !ok {
		t.Fatalf("closure should capture prefix, got %#v", info.CapturedVars)
	}
	for _, name := range []string{"Mode", "BuildFlags", "cfg"} {
		if _, ok := info.CapturedVars[name]; ok {
			t.Fatalf("closure should not capture %q, got %#v", name, info.CapturedVars)
		}
	}
}
