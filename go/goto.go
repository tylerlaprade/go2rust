package main

import (
	"go/ast"
	"go/token"
	"strings"
)

var currentGotoLabelModes map[string]string

func functionHasGoto(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Body == nil {
		return false
	}
	hasGoto := false
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if hasGoto {
			return false
		}
		if branch, ok := node.(*ast.BranchStmt); ok && branch.Tok == token.GOTO {
			hasGoto = true
			return false
		}
		return true
	})
	return hasGoto
}

type gotoPlan struct {
	labelIndex map[string]int
	backward   map[string]bool
	forward    map[string]bool
	modes      map[string]string
}

func buildGotoPlan(stmts []ast.Stmt) gotoPlan {
	plan := gotoPlan{
		labelIndex: make(map[string]int),
		backward:   make(map[string]bool),
		forward:    make(map[string]bool),
		modes:      make(map[string]string),
	}
	labelPos := make(map[string]token.Pos)
	for i, stmt := range stmts {
		if labeled, ok := stmt.(*ast.LabeledStmt); ok {
			label := ToSnakeCase(labeled.Label.Name)
			plan.labelIndex[label] = i
			labelPos[label] = labeled.Pos()
		}
	}

	for _, stmt := range stmts {
		ast.Inspect(stmt, func(node ast.Node) bool {
			branch, ok := node.(*ast.BranchStmt)
			if !ok || branch.Tok != token.GOTO || branch.Label == nil {
				return true
			}
			label := ToSnakeCase(branch.Label.Name)
			pos, exists := labelPos[label]
			if !exists {
				return true
			}
			if pos < branch.Pos() {
				plan.backward[label] = true
				plan.modes[label] = "continue"
			} else if !plan.backward[label] {
				plan.forward[label] = true
				plan.modes[label] = "break"
			}
			return true
		})
	}
	return plan
}

func TranspileGotoStatementList(out *strings.Builder, stmts []ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) ast.Stmt {
	plan := buildGotoPlan(stmts)
	oldModes := currentGotoLabelModes
	currentGotoLabelModes = plan.modes
	defer func() { currentGotoLabelModes = oldModes }()

	var prevStmt ast.Stmt
	for i := 0; i < len(stmts); {
		stmt := stmts[i]
		if labeled, ok := stmt.(*ast.LabeledStmt); ok {
			label := ToSnakeCase(labeled.Label.Name)
			if plan.backward[label] {
				writeBlankLineBetweenGotoStatements(out, prevStmt, stmt, fileSet)
				out.WriteString(indent)
				out.WriteString("'")
				out.WriteString(label)
				out.WriteString(": loop {\n")
				emitGotoStatement(out, labeled.Stmt, fnType, fileSet, comments, lastPos, indent+"    ")
				out.WriteString(indent)
				out.WriteString("    break '")
				out.WriteString(label)
				out.WriteString(";\n")
				out.WriteString(indent)
				out.WriteString("}\n")
				prevStmt = stmt
				i++
				continue
			}
			emitGotoStatement(out, labeled.Stmt, fnType, fileSet, comments, lastPos, indent)
			prevStmt = stmt
			i++
			continue
		}

		nextForwardLabel := findNextForwardLabel(stmts, plan, i)
		if nextForwardLabel >= 0 {
			labeled := stmts[nextForwardLabel].(*ast.LabeledStmt)
			label := ToSnakeCase(labeled.Label.Name)
			blockStart := findFirstGotoToLabel(stmts, label, i, nextForwardLabel)
			for j := i; j < blockStart; j++ {
				writeBlankLineBetweenGotoStatements(out, prevStmt, stmts[j], fileSet)
				emitGotoStatement(out, stmts[j], fnType, fileSet, comments, lastPos, indent)
				prevStmt = stmts[j]
			}
			writeBlankLineBetweenGotoStatements(out, prevStmt, stmts[blockStart], fileSet)
			out.WriteString(indent)
			out.WriteString("'")
			out.WriteString(label)
			out.WriteString(": {\n")
			for j := blockStart; j < nextForwardLabel; j++ {
				emitGotoStatement(out, stmts[j], fnType, fileSet, comments, lastPos, indent+"    ")
				prevStmt = stmts[j]
			}
			out.WriteString(indent)
			out.WriteString("}\n")
			emitGotoStatement(out, labeled.Stmt, fnType, fileSet, comments, lastPos, indent)
			prevStmt = labeled
			i = nextForwardLabel + 1
			continue
		}

		writeBlankLineBetweenGotoStatements(out, prevStmt, stmt, fileSet)
		emitGotoStatement(out, stmt, fnType, fileSet, comments, lastPos, indent)
		prevStmt = stmt
		i++
	}
	return prevStmt
}

func findNextForwardLabel(stmts []ast.Stmt, plan gotoPlan, start int) int {
	for i := start + 1; i < len(stmts); i++ {
		labeled, ok := stmts[i].(*ast.LabeledStmt)
		if !ok {
			continue
		}
		label := ToSnakeCase(labeled.Label.Name)
		if plan.forward[label] {
			return i
		}
		if plan.backward[label] {
			return -1
		}
	}
	return -1
}

func findFirstGotoToLabel(stmts []ast.Stmt, label string, start, end int) int {
	for i := start; i < end; i++ {
		if stmtContainsGotoToLabel(stmts[i], label) {
			return i
		}
	}
	return start
}

func stmtContainsGotoToLabel(stmt ast.Stmt, label string) bool {
	found := false
	ast.Inspect(stmt, func(node ast.Node) bool {
		if found {
			return false
		}
		branch, ok := node.(*ast.BranchStmt)
		if !ok || branch.Tok != token.GOTO || branch.Label == nil {
			return true
		}
		if ToSnakeCase(branch.Label.Name) == label {
			found = true
			return false
		}
		return true
	})
	return found
}

func writeBlankLineBetweenGotoStatements(out *strings.Builder, prevStmt, stmt ast.Stmt, fileSet *token.FileSet) {
	if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
		out.WriteString("\n")
	}
}

func emitGotoStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	out.WriteString(indent)
	TranspileStatement(out, stmt, fnType, fileSet, comments, lastPos, indent)
	out.WriteString("\n")
}
